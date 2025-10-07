use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_verified: Option<bool>,
    pub is_active: Option<bool>,
    pub two_fa_enabled: Option<bool>,
    pub two_fa_secret: Option<String>,
    pub kyc_status: Option<KycStatus>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "kyc_status", rename_all = "lowercase")]
pub enum KycStatus {
    Pending,
    Approved,
    Rejected,
    Required,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Account {
    pub id: Uuid,
    pub user_id: Uuid,
    pub currency: String,

    #[schema(value_type = String)]
    pub balance: Option<Decimal>,

    #[schema(value_type = String)]
    pub available_balance: Option<Decimal>,

    #[schema(value_type = String)]
    pub locked_balance: Option<Decimal>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow,ToSchema)]
pub struct TradingPair {
    pub id: Uuid,
    pub symbol: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub is_active: Option<bool>,

    #[schema(value_type = String)]
    pub min_order_size: Option<Decimal>,

    #[schema(value_type = String)]
    pub max_order_size: Option<Decimal>,

    pub price_precision: Option<i32>,
    pub quantity_precision: Option<i32>,

    #[schema(value_type = String)]
    pub maker_fee: Option<Decimal>,

    #[schema(value_type = String)]
    pub taker_fee: Option<Decimal>,

    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub trading_pair_id: Uuid,
    pub order_type: Option<OrderType>,
    pub side: Option<OrderSide>,

    #[schema(value_type = String)]
    pub quantity: Option<Decimal>,

    #[schema(value_type = String)]
    pub price: Option<Decimal>,

    #[schema(value_type = String)]
    pub filled_quantity: Option<Decimal>,

    #[schema(value_type = String)]
    pub remaining_quantity: Option<Decimal>,

    pub status: Option<OrderStatus>,
    pub time_in_force: Option<TimeInForce>,

    #[schema(value_type = String)]
    pub stop_price: Option<Decimal>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "order_type", rename_all = "lowercase")]
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    TakeProfit,
    StopLossLimit,
    TakeProfitLimit,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "order_side", rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "order_status", rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Open,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "time_in_force", rename_all = "lowercase")]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
    GTD,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Trade {
    pub id: Uuid,
    pub trading_pair_id: Uuid,
    pub buyer_order_id: Uuid,
    pub seller_order_id: Uuid,
    pub buyer_user_id: Uuid,
    pub seller_user_id: Uuid,

    #[schema(value_type = String)]
    pub price: Option<Decimal>,

    #[schema(value_type = String)]
    pub quantity: Option<Decimal>,

    #[schema(value_type = String)]
    pub buyer_fee: Option<Decimal>,

    #[schema(value_type = String)]
    pub seller_fee: Option<Decimal>,

    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct MarketData {
    pub trading_pair_id: Uuid,
    pub symbol: String,

    #[schema(value_type = String)]
    pub last_price: Decimal,

    #[schema(value_type = String)]
    pub volume_24h: Decimal,

    #[schema(value_type = String)]
    pub high_24h: Decimal,

    #[schema(value_type = String)]
    pub low_24h: Decimal,

    #[schema(value_type = String)]
    pub price_change_24h: Decimal,

    #[schema(value_type = String)]
    pub price_change_percent_24h: Decimal,

    #[schema(value_type = String)]
    pub bid_price: Option<Decimal>,

    #[schema(value_type = String)]
    pub ask_price: Option<Decimal>,

    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OrderBook {
    pub trading_pair_id: Uuid,
    pub symbol: String,
    pub bids: Vec<OrderBookLevel>,
    pub asks: Vec<OrderBookLevel>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OrderBookLevel {
    #[schema(value_type = String)]
    pub price: Decimal,

    #[schema(value_type = String)]
    pub quantity: Decimal,

    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Candlestick {
    pub timestamp: Option<DateTime<Utc>>,

    #[schema(value_type = String)]
    pub open: Option<Decimal>,

    #[schema(value_type = String)]
    pub high: Option<Decimal>,

    #[schema(value_type = String)]
    pub low: Option<Decimal>,

    #[schema(value_type = String)]
    pub close: Option<Decimal>,

    #[schema(value_type = String)]
    pub volume: Option<Decimal>,

    pub interval_minutes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct PortfolioSnapshot {
    pub id: Uuid,
    pub user_id: Uuid,

    #[schema(value_type = String)]
    pub total_value_usd: Decimal,

    pub snapshot_date: chrono::NaiveDate,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TwoFactorResponse {
    pub secret: String,
    pub qr_code: String,
    pub backup_codes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ConfirmTwoFactorRequest {
    #[validate(length(equal = 6))]
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SuccessResponse {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub totp_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub user: UserProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserProfile {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_verified: bool,
    pub two_fa_enabled: bool,
    pub kyc_status: KycStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateOrderRequest {
    pub trading_pair_id: Uuid,
    pub order_type: OrderType,
    pub side: OrderSide,
    #[validate(range(min = 0.0))]
    pub quantity: f64,

    #[schema(value_type = String)]
    pub price: Option<Decimal>,

    pub time_in_force: Option<TimeInForce>,

    #[schema(value_type = String)]
    pub stop_price: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Portfolio {
    pub user_id: Uuid,

    #[schema(value_type = String)]
    pub total_value_usd: Decimal,

    pub accounts: Vec<AccountBalance>,
    pub performance_24h: PerformanceMetrics,
    pub open_orders_count: i64,
    pub total_trades: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AccountBalance {
    pub currency: String,

    #[schema(value_type = String)]
    pub balance: Decimal,

    #[schema(value_type = String)]
    pub available_balance: Decimal,

    #[schema(value_type = String)]
    pub locked_balance: Decimal,

    #[schema(value_type = String)]
    pub usd_value: Decimal,

    #[schema(value_type = String)]
    pub percentage: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PerformanceMetrics {
    #[schema(value_type = String)]
    pub pnl_24h: Decimal,

    #[schema(value_type = String)]
    pub pnl_percentage_24h: Decimal,

    #[schema(value_type = String)]
    pub total_volume_24h: Decimal,

    #[schema(value_type = String)]
    pub total_fees_24h: Decimal,
}
