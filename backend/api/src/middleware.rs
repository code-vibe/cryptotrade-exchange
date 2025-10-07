use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

// Import AppState from the parent module (main.rs)
use super::AppState;

pub async fn auth_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Skip auth for public routes
    let path = request.uri().path();
    if is_public_route(path) {
        return Ok(next.run(request).await);
    }

    // Extract Authorization header
    let auth_header = headers
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "));

    let token = match auth_header {
        Some(token) => token,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // Verify JWT token
    let claims = match state.auth_service.verify_jwt(token) {
        Ok(token_data) => token_data.claims,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    // Add user claims to request extensions
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

fn is_public_route(path: &str) -> bool {
    let public_routes = [
        "/api/v1/auth/register",
        "/api/v1/auth/login",
        "/api/v1/auth/refresh",
        "/api/v1/health",
        "/api/v1/market-data",
        "/api/v1/order-book",
        "/api/v1/trades",
        "/api/v1/candlesticks",
    ];

    public_routes.iter().any(|&route| path.starts_with(route))
}
