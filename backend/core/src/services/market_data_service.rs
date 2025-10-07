use crate::{
    database::Database,
    error::CryptoTradeError,
    models::*,
    Result,
};
use chrono::{Duration, Utc};
use rust_decimal::Decimal;
use sqlx::Row;
use uuid::Uuid;

#[derive(Clone)]
pub struct MarketDataService {
    db: Database,
}

impl MarketDataService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn get_market_data(&self, trading_pair_id: Uuid) -> Result<MarketData> {
        let now = Utc::now();
        let yesterday = now - Duration::hours(24);

        let stats = sqlx::query(
            r#"
            SELECT
                tp.id as trading_pair_id,
                tp.symbol,
                COALESCE(t.last_price, 0) as last_price,
                COALESCE(t.volume_24h, 0) as volume_24h,
                COALESCE(t.high_24h, 0) as high_24h,
                COALESCE(t.low_24h, 0) as low_24h,
                COALESCE(t.first_price_24h, 0) as first_price_24h
            FROM trading_pairs tp
            LEFT JOIN (
                SELECT
                    trading_pair_id,
                    (SELECT price FROM trades t2
                     WHERE t2.trading_pair_id = t1.trading_pair_id
                       AND t2.created_at >= $1
                     ORDER BY t2.created_at DESC
                     LIMIT 1) as last_price,
                    SUM(quantity * price) as volume_24h,
                    MAX(price) as high_price,
                    MIN(price) as low_price,
                    (SELECT price FROM trades t3
                     WHERE t3.trading_pair_id = t1.trading_pair_id
                       AND t3.created_at >= $1
                     ORDER BY t3.created_at ASC
                     LIMIT 1) as first_price_24h
                FROM trades t1
                WHERE t1.created_at >= $1
                GROUP BY trading_pair_id
            ) t ON tp.id = t.trading_pair_id
            WHERE tp.is_active = true AND tp.id = $2
            "#
        )
        .bind(yesterday)
        .bind(trading_pair_id)
        .fetch_optional(&self.db)
        .await?;

        if let Some(stat) = stats {
            let last_price: Decimal = stat.get("last_price");
            let first_price: Decimal = stat.get("first_price_24h");
            let price_change = last_price - first_price;
            let price_change_percent = if first_price > Decimal::ZERO {
                (price_change / first_price) * Decimal::from(100)
            } else {
                Decimal::ZERO
            };

            Ok(MarketData {
                trading_pair_id,
                symbol: stat.get("symbol"),
                last_price,
                volume_24h: stat.get("volume_24h"),
                high_24h: stat.get("high_24h"),
                low_24h: stat.get("low_24h"),
                price_change_24h: price_change,
                price_change_percent_24h: price_change_percent,
                bid_price: None,
                ask_price: None,
                updated_at: now,
            })
        } else {
            Err(CryptoTradeError::NotFound {
                message: "Trading pair not found".to_string(),
            })
        }
    }

    pub async fn get_all_market_data(&self) -> Result<Vec<MarketData>> {
        let trading_pairs = sqlx::query("SELECT id FROM trading_pairs WHERE is_active = true")
            .fetch_all(&self.db)
            .await?;

        let mut market_data = Vec::new();
        for row in trading_pairs {
            let trading_pair_id: Uuid = row.get("id");
            if let Ok(data) = self.get_market_data(trading_pair_id).await {
                market_data.push(data);
            }
        }

        Ok(market_data)
    }

    pub async fn get_candlestick_data(
        &self,
        trading_pair_id: Uuid,
        interval: String,
        start_time: Option<chrono::DateTime<Utc>>,
        end_time: Option<chrono::DateTime<Utc>>,
        limit: Option<i32>,
    ) -> Result<Vec<Candlestick>> {
        let start = start_time.unwrap_or_else(|| Utc::now() - Duration::days(1));
        let end = end_time.unwrap_or_else(|| Utc::now());
        let limit = limit.unwrap_or(1000).min(5000);

        let interval_minutes = match interval.as_str() {
            "1m" => 1,
            "5m" => 5,
            "15m" => 15,
            "1h" => 60,
            "4h" => 240,
            "1d" => 1440,
            _ => 60,
        };

        let rows = sqlx::query(
            r#"
            SELECT
                date_trunc('minute', created_at) - INTERVAL '1 minute' * (EXTRACT(MINUTE FROM created_at)::int % $4) as bucket_time,
                (SELECT price FROM trades t1 WHERE t1.trading_pair_id = $1 AND date_trunc('minute', t1.created_at) - INTERVAL '1 minute' * (EXTRACT(MINUTE FROM t1.created_at)::int % $4) = bucket_time ORDER BY t1.created_at ASC LIMIT 1) as open_price,
                MAX(price) as high_price,
                MIN(price) as low_price,
                (SELECT price FROM trades t2 WHERE t2.trading_pair_id = $1 AND date_trunc('minute', t2.created_at) - INTERVAL '1 minute' * (EXTRACT(MINUTE FROM t2.created_at)::int % $4) = bucket_time ORDER BY t2.created_at DESC LIMIT 1) as close_price,
                SUM(quantity) as volume
            FROM trades
            WHERE trading_pair_id = $1
              AND created_at >= $2
              AND created_at <= $3
            GROUP BY bucket_time
            ORDER BY bucket_time
            LIMIT $5
            "#
        )
        .bind(trading_pair_id)
        .bind(start)
        .bind(end)
        .bind(interval_minutes)
        .bind(limit)
        .fetch_all(&self.db)
        .await?;

        let candlesticks: Vec<Candlestick> = rows.into_iter().map(|row| {
            Candlestick {
                timestamp: row.get("bucket_time"),
                open: row.get("open_price"),
                high: row.get("high_price"),
                low: row.get("low_price"),
                close: row.get("close_price"),
                volume: row.get("volume"),
                interval_minutes,
            }
        }).collect();

        Ok(candlesticks)
    }
}
