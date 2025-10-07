use crate::{
    database::Database,
    error::CryptoTradeError,
    models::*,
    Result,
};
use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::Row;
use uuid::Uuid;
use validator::Validate;

#[derive(Clone)]
pub struct OrderService {
    db: Database,
}

impl OrderService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create_order(&self, user_id: Uuid, request: CreateOrderRequest) -> Result<Order> {
        request.validate().map_err(|e| CryptoTradeError::Validation {
            message: e.to_string(),
        })?;

        // Validate trading pair
        let trading_pair = self.get_trading_pair(request.trading_pair_id).await?;
        if !trading_pair.is_active.unwrap_or(false) {
            return Err(CryptoTradeError::TradingPairNotActive);
        }

        let quantity = Decimal::from_f64_retain(request.quantity)
            .ok_or(CryptoTradeError::InvalidQuantity)?;

        let min_size = trading_pair.min_order_size.unwrap_or(Decimal::ZERO);
        let max_size = trading_pair.max_order_size.unwrap_or(Decimal::from(1000000));

        if quantity < min_size || quantity > max_size {
            return Err(CryptoTradeError::InvalidQuantity);
        }

        if matches!(request.order_type, OrderType::Limit | OrderType::StopLossLimit | OrderType::TakeProfitLimit) {
            if request.price.is_none() {
                return Err(CryptoTradeError::InvalidPrice);
            }
        }

        let required_currency = match request.side {
            OrderSide::Buy => &trading_pair.quote_currency,
            OrderSide::Sell => &trading_pair.base_currency,
        };

        let required_amount = match request.side {
            OrderSide::Buy => quantity * request.price.unwrap_or(Decimal::ZERO),
            OrderSide::Sell => quantity,
        };

        self.lock_balance(user_id, required_currency, required_amount).await?;

        let order_id = Uuid::new_v4();
        let now = Utc::now();

        let order = sqlx::query_as::<_, Order>(
            "INSERT INTO orders (id, user_id, trading_pair_id, order_type, side, quantity, price, filled_quantity, remaining_quantity, status, time_in_force, stop_price, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, 0, $6, 'pending', $8, $9, $10, $10) RETURNING *"
        )
        .bind(order_id)
        .bind(user_id)
        .bind(request.trading_pair_id)
        .bind(request.order_type)
        .bind(request.side)
        .bind(quantity)
        .bind(request.price)
        .bind(request.time_in_force.unwrap_or(TimeInForce::GTC))
        .bind(request.stop_price)
        .bind(now)
        .fetch_one(&self.db)
        .await?;

        self.submit_to_matching_engine(&order).await?;

        Ok(order)
    }

    pub async fn cancel_order(&self, user_id: Uuid, order_id: Uuid) -> Result<Order> {
        let order = sqlx::query_as::<_, Order>("SELECT * FROM orders WHERE id = $1 AND user_id = $2")
            .bind(order_id)
            .bind(user_id)
            .fetch_optional(&self.db)
            .await?
            .ok_or(CryptoTradeError::NotFound {
                message: "Order not found".to_string(),
            })?;

        if !matches!(order.status, Some(OrderStatus::Open) | Some(OrderStatus::PartiallyFilled)) {
            return Err(CryptoTradeError::OrderNotCancellable);
        }

        let updated_order = sqlx::query_as::<_, Order>(
            "UPDATE orders SET status = 'cancelled', updated_at = $1 WHERE id = $2 RETURNING *"
        )
        .bind(Utc::now())
        .bind(order_id)
        .fetch_one(&self.db)
        .await?;

        // Release locked balance
        let trading_pair = self.get_trading_pair(order.trading_pair_id).await?;
        let currency = match order.side {
            Some(OrderSide::Buy) => &trading_pair.quote_currency,
            Some(OrderSide::Sell) => &trading_pair.base_currency,
            None => return Err(CryptoTradeError::InvalidOrderType),
        };

        let remaining_quantity = order.remaining_quantity.unwrap_or(Decimal::ZERO);
        let order_price = order.price.unwrap_or(Decimal::ZERO);

        let amount_to_release = match order.side {
            Some(OrderSide::Buy) => remaining_quantity * order_price,
            Some(OrderSide::Sell) => remaining_quantity,
            None => return Err(CryptoTradeError::InvalidOrderType),
        };

        self.unlock_balance(user_id, currency, amount_to_release).await?;

        Ok(updated_order)
    }

    pub async fn get_user_orders(&self, user_id: Uuid, status: Option<OrderStatus>, limit: Option<i64>) -> Result<Vec<Order>> {
        let limit = limit.unwrap_or(100).min(1000);

        let orders = if let Some(status) = status {
            sqlx::query_as::<_, Order>(
                "SELECT * FROM orders WHERE user_id = $1 AND status = $2 ORDER BY created_at DESC LIMIT $3"
            )
            .bind(user_id)
            .bind(status)
            .bind(limit)
            .fetch_all(&self.db)
            .await?
        } else {
            sqlx::query_as::<_, Order>(
                "SELECT * FROM orders WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2"
            )
            .bind(user_id)
            .bind(limit)
            .fetch_all(&self.db)
            .await?
        };

        Ok(orders)
    }

    pub async fn get_order_book(&self, trading_pair_id: Uuid, depth: Option<usize>) -> Result<OrderBook> {
        let depth = depth.unwrap_or(20).min(100);

        let bids = sqlx::query(
            "SELECT price, SUM(remaining_quantity) as total_quantity, COUNT(*) as order_count FROM orders WHERE trading_pair_id = $1 AND side = 'buy' AND status IN ('open', 'partially_filled') GROUP BY price ORDER BY price DESC LIMIT $2"
        )
        .bind(trading_pair_id)
        .bind(depth as i64)
        .fetch_all(&self.db)
        .await?;

        let asks = sqlx::query(
            "SELECT price, SUM(remaining_quantity) as total_quantity, COUNT(*) as order_count FROM orders WHERE trading_pair_id = $1 AND side = 'sell' AND status IN ('open', 'partially_filled') GROUP BY price ORDER BY price ASC LIMIT $2"
        )
        .bind(trading_pair_id)
        .bind(depth as i64)
        .fetch_all(&self.db)
        .await?;

        let bid_levels: Vec<OrderBookLevel> = bids.into_iter().map(|row| {
            OrderBookLevel {
                price: row.get("price"),
                quantity: row.get("total_quantity"),
                count: row.get::<i64, _>("order_count") as i32,
            }
        }).collect();

        let ask_levels: Vec<OrderBookLevel> = asks.into_iter().map(|row| {
            OrderBookLevel {
                price: row.get("price"),
                quantity: row.get("total_quantity"),
                count: row.get::<i64, _>("order_count") as i32,
            }
        }).collect();

        // Get trading pair symbol
        let trading_pair = self.get_trading_pair(trading_pair_id).await?;

        Ok(OrderBook {
            trading_pair_id,
            symbol: trading_pair.symbol,
            bids: bid_levels,
            asks: ask_levels,
            timestamp: Utc::now(),
        })
    }

    async fn get_trading_pair(&self, trading_pair_id: Uuid) -> Result<TradingPair> {
        sqlx::query_as::<_, TradingPair>("SELECT * FROM trading_pairs WHERE id = $1")
            .bind(trading_pair_id)
            .fetch_optional(&self.db)
            .await?
            .ok_or(CryptoTradeError::NotFound {
                message: "Trading pair not found".to_string(),
            })
    }

    async fn get_account(&self, user_id: Uuid, currency: &str) -> Result<Account> {
        sqlx::query_as::<_, Account>(
            "SELECT * FROM accounts WHERE user_id = $1 AND currency = $2"
        )
        .bind(user_id)
        .bind(currency)
        .fetch_optional(&self.db)
        .await?
        .ok_or(CryptoTradeError::InsufficientBalance)
    }

    async fn lock_balance(&self, user_id: Uuid, currency: &str, amount: Decimal) -> Result<()> {
        sqlx::query(
            "UPDATE accounts SET available_balance = available_balance - $1, locked_balance = locked_balance + $1 WHERE user_id = $2 AND currency = $3"
        )
        .bind(amount)
        .bind(user_id)
        .bind(currency)
        .execute(&self.db)
        .await?;

        Ok(())
    }

    async fn unlock_balance(&self, user_id: Uuid, currency: &str, amount: Decimal) -> Result<()> {
        sqlx::query(
            "UPDATE accounts SET available_balance = available_balance + $1, locked_balance = locked_balance - $1 WHERE user_id = $2 AND currency = $3"
        )
        .bind(amount)
        .bind(user_id)
        .bind(currency)
        .execute(&self.db)
        .await?;

        Ok(())
    }

    async fn submit_to_matching_engine(&self, _order: &Order) -> Result<()> {
        // In a real implementation, this would send the order to a message queue
        // or matching engine service
        sqlx::query("UPDATE orders SET status = 'open' WHERE id = $1")
            .bind(_order.id)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}
