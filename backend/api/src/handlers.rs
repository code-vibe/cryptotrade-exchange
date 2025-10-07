use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    Extension,
};
use cryptotrade_core::{Claims, *};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono;

// Import AppState from the parent module (main.rs)
use super::AppState;

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
}

// Auth handlers
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    tag = "Authentication",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "User registered successfully", body = AuthResponse),
        (status = 400, description = "Registration failed", body = ErrorResponse)
    )
)]
pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> std::result::Result<Json<AuthResponse>, (StatusCode, Json<ErrorResponse>)> {
    match state.user_service.register(payload).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(handle_error(e)),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "Authentication",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "User logged in successfully", body = AuthResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse)
    )
)]
pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> std::result::Result<Json<AuthResponse>, (StatusCode, Json<ErrorResponse>)> {
    match state.user_service.login(payload).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(handle_error(e)),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    tag = "Authentication",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = TokenResponse),
        (status = 401, description = "Invalid refresh token", body = ErrorResponse)
    )
)]
pub async fn refresh_token_handler(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> std::result::Result<Json<TokenResponse>, (StatusCode, Json<ErrorResponse>)> {
    match state.auth_service.verify_refresh_token(&payload.refresh_token) {
        Ok(token_data) => {
            match token_data.claims.sub.parse::<Uuid>() {
                Ok(user_id) => {
                    match state.user_service.get_user_by_id(user_id).await {
                        Ok(user) => {
                            match state.auth_service.generate_jwt(&user) {
                                Ok(new_access_token) => {
                                    Ok(Json(TokenResponse {
                                        access_token: new_access_token,
                                        expires_in: 3600,
                                    }))
                                }
                                Err(e) => Err(handle_error(e))
                            }
                        }
                        Err(e) => Err(handle_error(e))
                    }
                }
                Err(_) => Err((StatusCode::BAD_REQUEST, Json(ErrorResponse {
                    error: "Invalid user ID in token".to_string(),
                    code: "INVALID_TOKEN".to_string(),
                })))
            }
        }
        Err(e) => Err(handle_error(e)),
    }
}

// User handlers
#[utoipa::path(
    get,
    path = "/api/v1/user/profile",
    tag = "User Management",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "User profile retrieved", body = UserProfile),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    )
)]
pub async fn get_user_profile_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> std::result::Result<Json<UserProfile>, (StatusCode, Json<ErrorResponse>)> {
    let user_id = claims.sub.parse::<Uuid>()
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(ErrorResponse {
            error: "Invalid user ID".to_string(),
            code: "INVALID_USER_ID".to_string(),
        })))?;

    match state.user_service.get_user_by_id(user_id).await {
        Ok(user) => {
            let profile = UserProfile {
                id: user.id,
                email: user.email,
                username: user.username,
                first_name: user.first_name,
                last_name: user.last_name,
                is_verified: user.is_verified.unwrap_or(false),
                two_fa_enabled: user.two_fa_enabled.unwrap_or(false),
                kyc_status: user.kyc_status.unwrap_or(KycStatus::Pending),
            };
            Ok(Json(profile))
        }
        Err(e) => Err(handle_error(e)),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/user/accounts",
    tag = "User Management",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "User accounts retrieved", body = [Account]),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    )
)]
pub async fn get_user_accounts_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> std::result::Result<Json<Vec<Account>>, (StatusCode, Json<ErrorResponse>)> {
    let user_id = claims.sub.parse::<Uuid>()
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(ErrorResponse {
            error: "Invalid user ID".to_string(),
            code: "INVALID_USER_ID".to_string(),
        })))?;

    match state.user_service.get_user_accounts(user_id).await {
        Ok(accounts) => Ok(Json(accounts)),
        Err(e) => Err(handle_error(e)),
    }
}

// 2FA handlers
#[utoipa::path(
    post,
    path = "/api/v1/user/2fa/enable",
    tag = "Two-Factor Authentication",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "2FA enabled successfully", body = TwoFactorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    )
)]
pub async fn enable_2fa_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> std::result::Result<Json<TwoFactorResponse>, (StatusCode, Json<ErrorResponse>)> {
    let user_id = claims.sub.parse::<Uuid>()
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(ErrorResponse {
            error: "Invalid user ID".to_string(),
            code: "INVALID_USER_ID".to_string(),
        })))?;

    match state.user_service.enable_2fa(user_id).await {
        Ok(response) => {
            Ok(Json(response))
        }
        Err(e) => Err(handle_error(e)),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/user/2fa/confirm",
    tag = "Two-Factor Authentication",
    security(
        ("bearer_auth" = [])
    ),
    request_body = ConfirmTwoFactorRequest,
    responses(
        (status = 200, description = "2FA confirmed successfully", body = SuccessResponse),
        (status = 400, description = "Invalid 2FA code", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    )
)]
pub async fn confirm_2fa_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(payload): Json<ConfirmTwoFactorRequest>,
) -> std::result::Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let user_id = claims.sub.parse::<Uuid>()
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(ErrorResponse {
            error: "Invalid user ID".to_string(),
            code: "INVALID_USER_ID".to_string(),
        })))?;

    match state.user_service.confirm_2fa(user_id, payload).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(handle_error(e)),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/user/2fa/disable",
    tag = "Two-Factor Authentication",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "2FA disabled successfully", body = SuccessResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    )
)]
pub async fn disable_2fa_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> std::result::Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    let user_id = claims.sub.parse::<Uuid>()
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(ErrorResponse {
            error: "Invalid user ID".to_string(),
            code: "INVALID_USER_ID".to_string(),
        })))?;

    match state.user_service.disable_2fa(user_id).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => Err(handle_error(e)),
    }
}

// Order handlers
#[utoipa::path(
    post,
    path = "/api/v1/orders",
    tag = "Trading",
    security(
        ("bearer_auth" = [])
    ),
    request_body = CreateOrderRequest,
    responses(
        (status = 200, description = "Order created successfully", body = Order),
        (status = 400, description = "Invalid order request", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    )
)]
pub async fn create_order_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(payload): Json<CreateOrderRequest>,
) -> std::result::Result<Json<Order>, (StatusCode, Json<ErrorResponse>)> {
    let user_id = claims.sub.parse::<Uuid>()
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(ErrorResponse {
            error: "Invalid user ID".to_string(),
            code: "INVALID_USER_ID".to_string(),
        })))?;

    match state.order_service.create_order(user_id, payload).await {
        Ok(order) => Ok(Json(order)),
        Err(e) => Err(handle_error(e)),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/orders",
    tag = "Trading",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("status" = Option<String>, Query, description = "Filter by order status"),
        ("limit" = Option<i64>, Query, description = "Limit number of results")
    ),
    responses(
        (status = 200, description = "Orders retrieved successfully", body = [Order]),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    )
)]
pub async fn get_user_orders_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Query(params): Query<OrdersQuery>,
) -> std::result::Result<Json<Vec<Order>>, (StatusCode, Json<ErrorResponse>)> {
    let user_id = claims.sub.parse::<Uuid>()
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(ErrorResponse {
            error: "Invalid user ID".to_string(),
            code: "INVALID_USER_ID".to_string(),
        })))?;

    match state.order_service.get_user_orders(user_id, params.status, params.limit).await {
        Ok(orders) => Ok(Json(orders)),
        Err(e) => Err(handle_error(e)),
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/orders/{order_id}",
    tag = "Trading",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("order_id" = Uuid, Path, description = "Order ID to cancel")
    ),
    responses(
        (status = 200, description = "Order cancelled successfully", body = Order),
        (status = 404, description = "Order not found", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    )
)]
pub async fn cancel_order_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Path(order_id): Path<Uuid>,
) -> std::result::Result<Json<Order>, (StatusCode, Json<ErrorResponse>)> {
    let user_id = claims.sub.parse::<Uuid>()
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(ErrorResponse {
            error: "Invalid user ID".to_string(),
            code: "INVALID_USER_ID".to_string(),
        })))?;

    match state.order_service.cancel_order(user_id, order_id).await {
        Ok(order) => Ok(Json(order)),
        Err(e) => Err(handle_error(e)),
    }
}

// Portfolio handlers
#[utoipa::path(
    get,
    path = "/api/v1/portfolio",
    tag = "Portfolio",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Portfolio retrieved successfully", body = Portfolio),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    )
)]
pub async fn get_portfolio_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> std::result::Result<Json<Portfolio>, (StatusCode, Json<ErrorResponse>)> {
    let user_id = claims.sub.parse::<Uuid>()
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(ErrorResponse {
            error: "Invalid user ID".to_string(),
            code: "INVALID_USER_ID".to_string(),
        })))?;

    match state.portfolio_service.get_portfolio(user_id).await {
        Ok(portfolio) => Ok(Json(portfolio)),
        Err(e) => Err(handle_error(e)),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/portfolio/history",
    tag = "Portfolio",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("days" = Option<i32>, Query, description = "Number of days to retrieve history for (default: 30)")
    ),
    responses(
        (status = 200, description = "Portfolio history retrieved successfully", body = [PortfolioSnapshot]),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    )
)]
pub async fn get_portfolio_history_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Query(params): Query<HistoryQuery>,
) -> std::result::Result<Json<Vec<PortfolioSnapshot>>, (StatusCode, Json<ErrorResponse>)> {
    let user_id = claims.sub.parse::<Uuid>()
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(ErrorResponse {
            error: "Invalid user ID".to_string(),
            code: "INVALID_USER_ID".to_string(),
        })))?;

    match state.portfolio_service.get_portfolio_history(user_id, params.days.unwrap_or(30)).await {
        Ok(history) => Ok(Json(history)),
        Err(e) => Err(handle_error(e)),
    }
}

// Trading handlers
#[utoipa::path(
    get,
    path = "/api/v1/trades",
    tag = "Trading",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("limit" = Option<i64>, Query, description = "Limit number of results")
    ),
    responses(
        (status = 200, description = "Trades retrieved successfully", body = [Trade]),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    )
)]
pub async fn get_user_trades_handler(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Query(params): Query<TradesQuery>,
) -> std::result::Result<Json<Vec<Trade>>, (StatusCode, Json<ErrorResponse>)> {
    let user_id = claims.sub.parse::<Uuid>()
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(ErrorResponse {
            error: "Invalid user ID".to_string(),
            code: "INVALID_USER_ID".to_string(),
        })))?;

    match state.trading_service.get_user_trades(user_id, params.limit).await {
        Ok(trades) => Ok(Json(trades)),
        Err(e) => Err(handle_error(e)),
    }
}

// Market data handlers
#[utoipa::path(
    get,
    path = "/api/v1/market-data",
    tag = "Market Data",
    responses(
        (status = 200, description = "Market data retrieved successfully", body = [MarketData])
    )
)]
pub async fn get_all_market_data_handler(
    State(state): State<AppState>,
) -> std::result::Result<Json<Vec<MarketData>>, (StatusCode, Json<ErrorResponse>)> {
    match state.market_data_service.get_all_market_data().await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(handle_error(e)),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/market-data/{pair_id}",
    tag = "Market Data",
    params(
        ("pair_id" = Uuid, Path, description = "Trading pair ID")
    ),
    responses(
        (status = 200, description = "Market data retrieved successfully", body = MarketData),
        (status = 404, description = "Trading pair not found", body = ErrorResponse)
    )
)]
pub async fn get_market_data_handler(
    State(state): State<AppState>,
    Path(pair_id): Path<Uuid>,
) -> std::result::Result<Json<MarketData>, (StatusCode, Json<ErrorResponse>)> {
    match state.market_data_service.get_market_data(pair_id).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err(handle_error(e)),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/order-book/{pair_id}",
    tag = "Market Data",
    params(
        ("pair_id" = Uuid, Path, description = "Trading pair ID")
    ),
    responses(
        (status = 200, description = "Order book retrieved successfully", body = OrderBook),
        (status = 404, description = "Trading pair not found", body = ErrorResponse)
    )
)]
pub async fn get_order_book_handler(
    State(state): State<AppState>,
    Path(pair_id): Path<Uuid>,
) -> std::result::Result<Json<OrderBook>, (StatusCode, Json<ErrorResponse>)> {
    match state.order_service.get_order_book(pair_id, Some(20)).await {
        Ok(order_book) => Ok(Json(order_book)),
        Err(e) => Err(handle_error(e)),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/trades/{pair_id}",
    tag = "Market Data",
    params(
        ("pair_id" = Uuid, Path, description = "Trading pair ID")
    ),
    responses(
        (status = 200, description = "Recent trades retrieved successfully", body = [Trade]),
        (status = 404, description = "Trading pair not found", body = ErrorResponse)
    )
)]
pub async fn get_recent_trades_handler(
    State(state): State<AppState>,
    Path(pair_id): Path<Uuid>,
) -> std::result::Result<Json<Vec<Trade>>, (StatusCode, Json<ErrorResponse>)> {
    match state.trading_service.get_recent_trades(pair_id, Some(100)).await {
        Ok(trades) => Ok(Json(trades)),
        Err(e) => Err(handle_error(e)),
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/candlesticks/{pair_id}",
    tag = "Market Data",
    params(
        ("pair_id" = Uuid, Path, description = "Trading pair ID"),
        ("interval" = Option<String>, Query, description = "Candlestick interval (e.g., 1m, 5m, 1h, 1d)"),
        ("start_time" = Option<String>, Query, description = "Start time (ISO 8601)"),
        ("end_time" = Option<String>, Query, description = "End time (ISO 8601)"),
        ("limit" = Option<i32>, Query, description = "Limit number of results")
    ),
    responses(
        (status = 200, description = "Candlestick data retrieved successfully", body = [Candlestick]),
        (status = 404, description = "Trading pair not found", body = ErrorResponse)
    )
)]
pub async fn get_candlestick_data_handler(
    State(state): State<AppState>,
    Path(pair_id): Path<Uuid>,
    Query(params): Query<CandlestickQuery>,
) -> std::result::Result<Json<Vec<Candlestick>>, (StatusCode, Json<ErrorResponse>)> {
    match state.market_data_service.get_candlestick_data(
        pair_id,
        params.interval.unwrap_or_else(|| "1h".to_string()),
        params.start_time,
        params.end_time,
        params.limit,
    ).await {
        Ok(candlesticks) => Ok(Json(candlesticks)),
        Err(e) => Err(handle_error(e)),
    }
}

// Query parameter structs
#[derive(Deserialize)]
pub struct OrdersQuery {
    pub status: Option<OrderStatus>,
    pub limit: Option<i64>,
}

#[derive(Deserialize)]
pub struct HistoryQuery {
    pub days: Option<i32>,
}

#[derive(Deserialize)]
pub struct TradesQuery {
    pub limit: Option<i64>,
}

#[derive(Deserialize)]
pub struct CandlestickQuery {
    pub interval: Option<String>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub limit: Option<i32>,
}

// Error handling
fn handle_error(error: CryptoTradeError) -> (StatusCode, Json<ErrorResponse>) {
    let status_code = StatusCode::from_u16(error.status_code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    let error_response = ErrorResponse {
        error: error.to_string(),
        code: error.error_code().to_string(),
    };
    (status_code, Json(error_response))
}
