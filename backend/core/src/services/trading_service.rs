use crate::{
    database::Database,
    error::CryptoTradeError,
    models::*,
    Result,
};
use chrono::Utc;
use rust_decimal::Decimal;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Clone)]
pub struct TradingService {
    db: Database,
}

impl TradingService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn execute_trade(
        &self,
        buyer_order: &Order,
        seller_order: &Order,
        price: Decimal,
        quantity: Decimal,
    ) -> Result<Trade> {
        let trade_id = Uuid::new_v4();
        let now = Utc::now();

        let trading_pair = self.get_trading_pair(buyer_order.trading_pair_id).await?;

        let trade_value = price * quantity;
        let buyer_fee = trade_value * trading_pair.taker_fee.unwrap_or(Decimal::from_str("0.001").unwrap());
        let seller_fee = trade_value * trading_pair.maker_fee.unwrap_or(Decimal::from_str("0.001").unwrap());

        let trade = sqlx::query_as::<_, Trade>(
            "INSERT INTO trades (id, trading_pair_id, buyer_order_id, seller_order_id, buyer_user_id, seller_user_id, price, quantity, buyer_fee, seller_fee, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING *"
        )
        .bind(trade_id)
        .bind(buyer_order.trading_pair_id)
        .bind(buyer_order.id)
        .bind(seller_order.id)
        .bind(buyer_order.user_id)
        .bind(seller_order.user_id)
        .bind(price)
        .bind(quantity)
        .bind(buyer_fee)
        .bind(seller_fee)
        .bind(now)
        .fetch_one(&self.db)
        .await?;

        // Update orders
        self.update_order_fill(buyer_order.id, quantity).await?;
        self.update_order_fill(seller_order.id, quantity).await?;

        // Update account balances
        self.update_balances_after_trade(&trade, &trading_pair).await?;

        Ok(trade)
    }

    pub async fn get_recent_trades(&self, trading_pair_id: Uuid, limit: Option<i64>) -> Result<Vec<Trade>> {
        let limit = limit.unwrap_or(100).min(1000);

        sqlx::query_as::<_, Trade>(
            "SELECT * FROM trades WHERE trading_pair_id = $1 ORDER BY created_at DESC LIMIT $2"
        )
        .bind(trading_pair_id)
        .bind(limit)
        .fetch_all(&self.db)
        .await
        .map_err(Into::into)
    }

    pub async fn get_user_trades(&self, user_id: Uuid, limit: Option<i64>) -> Result<Vec<Trade>> {
        let limit = limit.unwrap_or(100).min(1000);

        sqlx::query_as::<_, Trade>(
            "SELECT * FROM trades WHERE buyer_user_id = $1 OR seller_user_id = $1 ORDER BY created_at DESC LIMIT $2"
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(&self.db)
        .await
        .map_err(Into::into)
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

    async fn update_order_fill(&self, order_id: Uuid, quantity: Decimal) -> Result<()> {
        sqlx::query(
            "UPDATE orders SET filled_quantity = filled_quantity + $1, remaining_quantity = remaining_quantity - $1, status = CASE WHEN remaining_quantity - $1 <= 0 THEN 'filled' ELSE 'partially_filled' END, updated_at = $2 WHERE id = $3"
        )
        .bind(quantity)
        .bind(Utc::now())
        .bind(order_id)
        .execute(&self.db)
        .await?;

        Ok(())
    }

    async fn update_balances_after_trade(&self, trade: &Trade, trading_pair: &TradingPair) -> Result<()> {
        let base_currency = &trading_pair.base_currency;
        let quote_currency = &trading_pair.quote_currency;

        // Handle Option types properly
        let trade_price = trade.price.unwrap_or(Decimal::ZERO);
        let trade_quantity = trade.quantity.unwrap_or(Decimal::ZERO);
        let buyer_fee = trade.buyer_fee.unwrap_or(Decimal::ZERO);
        let seller_fee = trade.seller_fee.unwrap_or(Decimal::ZERO);

        // Buyer receives base currency, pays quote currency + fee
        let buyer_base_amount = trade_quantity;
        let buyer_quote_amount = trade_price * trade_quantity + buyer_fee;

        // Seller receives quote currency - fee, loses base currency
        let seller_quote_amount = trade_price * trade_quantity - seller_fee;
        let seller_base_amount = trade_quantity;

        // Update buyer balances
        self.update_account_balance(trade.buyer_user_id, base_currency, buyer_base_amount, true).await?;
        self.update_account_balance(trade.buyer_user_id, quote_currency, buyer_quote_amount, false).await?;

        // Update seller balances
        self.update_account_balance(trade.seller_user_id, quote_currency, seller_quote_amount, true).await?;
        self.update_account_balance(trade.seller_user_id, base_currency, seller_base_amount, false).await?;

        Ok(())
    }

    async fn update_account_balance(&self, user_id: Uuid, currency: &str, amount: Decimal, is_credit: bool) -> Result<()> {
        if is_credit {
            sqlx::query(
                "UPDATE accounts SET balance = balance + $1, available_balance = available_balance + $1 WHERE user_id = $2 AND currency = $3"
            )
            .bind(amount)
            .bind(user_id)
            .bind(currency)
            .execute(&self.db)
            .await?;
        } else {
            sqlx::query(
                "UPDATE accounts SET balance = balance - $1, locked_balance = locked_balance - $1 WHERE user_id = $2 AND currency = $3"
            )
            .bind(amount)
            .bind(user_id)
            .bind(currency)
            .execute(&self.db)
            .await?;
        }

        Ok(())
    }
}
