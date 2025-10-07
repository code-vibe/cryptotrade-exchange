// backend/api/src/openapi.rs
use crate::handlers;
use utoipa::OpenApi;


/// Use fully-qualified paths so the derive can resolve them unambiguously at expansion time.
/// - handlers live in this crate: `crate::handlers::...`
/// - models live in the core crate: `cryptotrade_core::...`
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::register_handler,
        crate::handlers::login_handler,
        crate::handlers::refresh_token_handler,
        crate::handlers::get_user_profile_handler,
        crate::handlers::get_user_accounts_handler,
        crate::handlers::enable_2fa_handler,
        crate::handlers::confirm_2fa_handler,
        crate::handlers::disable_2fa_handler,
        crate::handlers::create_order_handler,
        crate::handlers::get_user_orders_handler,
        crate::handlers::cancel_order_handler,
        crate::handlers::get_portfolio_handler,
        crate::handlers::get_portfolio_history_handler,
        crate::handlers::get_user_trades_handler,
        crate::handlers::get_all_market_data_handler,
        crate::handlers::get_market_data_handler,
        crate::handlers::get_order_book_handler,
        crate::handlers::get_recent_trades_handler,
        crate::handlers::get_candlestick_data_handler
    ),
    components(
        schemas(
            cryptotrade_core::RegisterRequest,
            cryptotrade_core::LoginRequest,
            cryptotrade_core::AuthResponse,
            cryptotrade_core::KycStatus,
            cryptotrade_core::UserProfile,
            cryptotrade_core::RefreshTokenRequest,
            cryptotrade_core::TokenResponse,
            cryptotrade_core::Account,
            cryptotrade_core::Order,
            cryptotrade_core::OrderType,
            cryptotrade_core::OrderSide,
            cryptotrade_core::OrderStatus,
            cryptotrade_core::TimeInForce,
            cryptotrade_core::CreateOrderRequest,
            cryptotrade_core::Trade,
            cryptotrade_core::MarketData,
            cryptotrade_core::OrderBook,
            cryptotrade_core::OrderBookLevel,
            cryptotrade_core::Candlestick,
            cryptotrade_core::Portfolio,
            cryptotrade_core::AccountBalance,
            cryptotrade_core::PerformanceMetrics,
            cryptotrade_core::PortfolioSnapshot,
            cryptotrade_core::TwoFactorResponse,
            cryptotrade_core::ConfirmTwoFactorRequest,
            cryptotrade_core::SuccessResponse
        )
    ),
    tags(
        (name = "Authentication", description = "User authentication and authorization"),
        (name = "User Management", description = "User profile and account management"),
        (name = "Two-Factor Authentication", description = "2FA setup and management"),
        (name = "Trading", description = "Order management and trade execution"),
        (name = "Portfolio", description = "Portfolio tracking and history"),
        (name = "Market Data", description = "Real-time and historical market data")
    )
)]
pub struct ApiDoc;
