use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoTradeError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Authentication error: {message}")]
    Authentication { message: String },

    #[error("Authorization error: {message}")]
    Authorization { message: String },

    #[error("Validation error: {message}")]
    Validation { message: String },

    #[error("Not found: {message}")]
    NotFound { message: String },

    #[error("User not found")]
    UserNotFound,

    #[error("Order not found")]
    OrderNotFound,

    #[error("Order cannot be cancelled")]
    OrderNotCancellable,

    #[error("Trading pair not found")]
    TradingPairNotFound,

    #[error("Insufficient balance")]
    InsufficientBalance,

    #[error("Invalid order type")]
    InvalidOrderType,

    #[error("Invalid price")]
    InvalidPrice,

    #[error("Invalid quantity")]
    InvalidQuantity,

    #[error("Trading pair not active")]
    TradingPairNotActive,

    #[error("KYC verification required")]
    KycRequired,

    #[error("Two-factor authentication required")]
    TwoFactorRequired,

    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("BCrypt error: {0}")]
    BCrypt(#[from] bcrypt::BcryptError),

    #[error("TOTP error: {0}")]
    Totp(#[from] totp_rs::TotpUrlError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Internal server error")]
    Internal,
}

impl CryptoTradeError {
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::Database(_) => "DATABASE_ERROR",
            Self::Migration(_) => "MIGRATION_ERROR",
            Self::Redis(_) => "REDIS_ERROR",
            Self::Authentication { .. } => "AUTHENTICATION_ERROR",
            Self::Authorization { .. } => "AUTHORIZATION_ERROR",
            Self::Validation { .. } => "VALIDATION_ERROR",
            Self::NotFound { .. } => "NOT_FOUND",
            Self::UserNotFound => "USER_NOT_FOUND",
            Self::OrderNotFound => "ORDER_NOT_FOUND",
            Self::OrderNotCancellable => "ORDER_NOT_CANCELLABLE",
            Self::TradingPairNotFound => "TRADING_PAIR_NOT_FOUND",
            Self::InsufficientBalance => "INSUFFICIENT_BALANCE",
            Self::InvalidOrderType => "INVALID_ORDER_TYPE",
            Self::InvalidPrice => "INVALID_PRICE",
            Self::InvalidQuantity => "INVALID_QUANTITY",
            Self::TradingPairNotActive => "TRADING_PAIR_NOT_ACTIVE",
            Self::KycRequired => "KYC_REQUIRED",
            Self::TwoFactorRequired => "TWO_FACTOR_REQUIRED",
            Self::Config(_) => "CONFIGURATION_ERROR",
            Self::Jwt(_) => "JWT_ERROR",
            Self::BCrypt(_) => "BCRYPT_ERROR",
            Self::Totp(_) => "TOTP_ERROR",
            Self::Io(_) => "IO_ERROR",
            Self::Internal => "INTERNAL_ERROR",
        }
    }

    pub fn status_code(&self) -> u16 {
        match self {
            Self::Database(_) | Self::Migration(_) | Self::Redis(_) | Self::Internal => 500,
            Self::Authentication { .. } => 401,
            Self::Authorization { .. } => 403,
            Self::Validation { .. } => 400,
            Self::NotFound { .. } => 404,
            Self::UserNotFound | Self::OrderNotFound | Self::TradingPairNotFound => 404,
            Self::OrderNotCancellable => 400,
            Self::InsufficientBalance | Self::InvalidOrderType | Self::InvalidPrice | Self::InvalidQuantity => 400,
            Self::TradingPairNotActive | Self::KycRequired | Self::TwoFactorRequired => 403,
            Self::Config(_) => 500,
            Self::Jwt(_) | Self::BCrypt(_) | Self::Totp(_) | Self::Io(_) => 500,
        }
    }
}

pub type Result<T> = std::result::Result<T, CryptoTradeError>;
