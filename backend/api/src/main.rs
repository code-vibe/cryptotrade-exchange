use axum::{
    response::Json,
    routing::{delete, get, post},
    Router,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use cryptotrade_api::handlers::*;
use cryptotrade_api::middleware::auth_middleware;
use cryptotrade_api::websocket;
use cryptotrade_api::AppState;
use cryptotrade_core::{
    database, AuthService, Config, MarketDataService, OrderService,
    PortfolioService, TradingService, UserService,
};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod openapi;
mod handlers;
use openapi::ApiDoc;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter("cryptotrade_api=debug,tower_http=debug")
        .init();

    let config = Config::from_env()?;
    tracing::info!("Loaded configuration for environment: {}", config.app.environment);

    let db = database::connect(&config.database).await?;
    tracing::info!("Connected to database");

    let auth_service = AuthService::new(
        config.jwt.secret.clone(),
        config.jwt.expiration_seconds,
    );

    let app_state = AppState {
        user_service: UserService::new(db.clone(), auth_service.clone()),
        order_service: OrderService::new(db.clone()),
        trading_service: TradingService::new(db.clone()),
        market_data_service: MarketDataService::new(db.clone()),
        portfolio_service: PortfolioService::new(db.clone()),
        auth_service,
    };

    let app = create_router(app_state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn create_router(state: AppState) -> Router {
    // Public routes (no auth middleware)
    let public = Router::new()
        .route("/api/v1/health", get(health_handler))
        .route("/api/v1/auth/register", post(register_handler))
        .route("/api/v1/auth/login", post(login_handler))
        .route("/api/v1/auth/refresh", post(refresh_token_handler))
        .merge(
            SwaggerUi::new("/docs")
                .url("/api-doc/openapi.json", openapi::ApiDoc::openapi()),
        );

    // Protected routes (with auth middleware)
    let protected = Router::new()
        .route("/api/v1/market-data", get(get_all_market_data_handler))
        .route("/api/v1/market-data/:pair_id", get(get_market_data_handler))
        .route("/api/v1/order-book/:pair_id", get(get_order_book_handler))
        .route("/api/v1/trades/:pair_id", get(get_recent_trades_handler))
        .route("/api/v1/candlesticks/:pair_id", get(get_candlestick_data_handler))
        .route("/api/v1/user/profile", get(get_user_profile_handler))
        .route("/api/v1/user/accounts", get(get_user_accounts_handler))
        .route("/api/v1/user/2fa/enable", post(enable_2fa_handler))
        .route("/api/v1/user/2fa/confirm", post(confirm_2fa_handler))
        .route("/api/v1/user/2fa/disable", post(disable_2fa_handler))
        .route("/api/v1/orders", post(create_order_handler))
        .route("/api/v1/orders", get(get_user_orders_handler))
        .route("/api/v1/orders/:order_id", delete(cancel_order_handler))
        .route("/api/v1/portfolio", get(get_portfolio_handler))
        .route("/api/v1/portfolio/history", get(get_portfolio_history_handler))
        .route("/api/v1/trades", get(get_user_trades_handler))
        .route("/ws", get(crate::websocket::websocket_handler))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    public
        .merge(protected)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now(),
        "version": "1.0.0"
    }))
}
