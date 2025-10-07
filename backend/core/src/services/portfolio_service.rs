use crate::{
    database::Database,
    models::*,
    Result,
};
use rust_decimal::Decimal;
use sqlx::Row;
use uuid::Uuid;

#[derive(Clone)]
pub struct PortfolioService {
    db: Database,
}

impl PortfolioService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn get_portfolio(&self, user_id: Uuid) -> Result<Portfolio> {
        let accounts = sqlx::query_as::<_, Account>(
            "SELECT * FROM accounts WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_all(&self.db)
        .await?;

        let mut account_balances = Vec::new();
        let mut total_value_usd = Decimal::ZERO;

        for account in accounts {
            let balance = account.balance.unwrap_or(Decimal::ZERO);
            let available_balance = account.available_balance.unwrap_or(Decimal::ZERO);
            let locked_balance = account.locked_balance.unwrap_or(Decimal::ZERO);

            let usd_value = self.get_usd_value(&account.currency, balance).await?;
            total_value_usd += usd_value;

            account_balances.push(AccountBalance {
                currency: account.currency,
                balance,
                available_balance,
                locked_balance,
                usd_value,
                percentage: Decimal::ZERO, // Will be calculated after total is known
            });
        }

        // Calculate percentages
        for account_balance in &mut account_balances {
            if total_value_usd > Decimal::ZERO {
                account_balance.percentage = (account_balance.usd_value / total_value_usd) * Decimal::from(100);
            }
        }

        let open_orders_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM orders WHERE user_id = $1 AND status IN ('open', 'partially_filled')"
        )
        .bind(user_id)
        .fetch_one(&self.db)
        .await?;

        let total_trades = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM trades WHERE buyer_user_id = $1 OR seller_user_id = $1"
        )
        .bind(user_id)
        .fetch_one(&self.db)
        .await?;

        let performance_24h = self.calculate_24h_performance(user_id).await?;

        Ok(Portfolio {
            user_id,
            total_value_usd,
            accounts: account_balances,
            performance_24h,
            open_orders_count,
            total_trades,
        })
    }

    pub async fn get_portfolio_history(&self, user_id: Uuid, days: i32) -> Result<Vec<PortfolioSnapshot>> {
        sqlx::query_as::<_, PortfolioSnapshot>(
            "SELECT * FROM portfolio_snapshots WHERE user_id = $1 AND created_at >= NOW() - INTERVAL '$2 days' ORDER BY created_at DESC"
        )
        .bind(user_id)
        .bind(days.to_string())
        .fetch_all(&self.db)
        .await
        .map_err(Into::into)
    }

    async fn calculate_24h_performance(&self, user_id: Uuid) -> Result<PerformanceMetrics> {
        let now = chrono::Utc::now();
        let yesterday = now - chrono::Duration::hours(24);

        let trade_stats = sqlx::query(
            r#"
            SELECT
                COALESCE(SUM(CASE WHEN buyer_user_id = $1 THEN price * quantity ELSE 0 END), 0) as buy_volume,
                COALESCE(SUM(CASE WHEN seller_user_id = $1 THEN price * quantity ELSE 0 END), 0) as sell_volume,
                COALESCE(SUM(CASE WHEN buyer_user_id = $1 THEN buyer_fee ELSE seller_fee END), 0) as total_fees
            FROM trades
            WHERE (buyer_user_id = $1 OR seller_user_id = $1)
              AND created_at >= $2
            "#
        )
        .bind(user_id)
        .bind(yesterday)
        .fetch_one(&self.db)
        .await?;

        let buy_volume: Decimal = trade_stats.get("buy_volume");
        let sell_volume: Decimal = trade_stats.get("sell_volume");
        let total_fees: Decimal = trade_stats.get("total_fees");
        let total_volume_24h = buy_volume + sell_volume;

        // For now, calculate PnL as sell volume minus buy volume
        // In a real system, this would be more sophisticated
        let pnl_24h = sell_volume - buy_volume - total_fees;
        let pnl_percentage_24h = if buy_volume > Decimal::ZERO {
            (pnl_24h / buy_volume) * Decimal::from(100)
        } else {
            Decimal::ZERO
        };

        Ok(PerformanceMetrics {
            pnl_24h,
            pnl_percentage_24h,
            total_volume_24h,
            total_fees_24h: total_fees,
        })
    }

    async fn get_usd_value(&self, currency: &str, amount: Decimal) -> Result<Decimal> {
        if currency == "USD" || currency == "USDT" {
            return Ok(amount);
        }

        // For other currencies, we would typically look up the current exchange rate
        // For now, return a placeholder value
        match currency {
            "BTC" => Ok(amount * Decimal::from(50000)), // Placeholder BTC price
            "ETH" => Ok(amount * Decimal::from(3000)),  // Placeholder ETH price
            _ => Ok(Decimal::ZERO),
        }
    }
}
