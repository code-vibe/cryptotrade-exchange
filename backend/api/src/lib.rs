pub mod auth;
pub mod handlers;
pub mod middleware;
pub mod websocket;
pub mod openapi;

use cryptotrade_core::{
    UserService, OrderService, TradingService,
    MarketDataService, PortfolioService, AuthService,
};

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
    pub order_service: OrderService,
    pub trading_service: TradingService,
    pub market_data_service: MarketDataService,
    pub portfolio_service: PortfolioService,
    pub auth_service: AuthService,
}
