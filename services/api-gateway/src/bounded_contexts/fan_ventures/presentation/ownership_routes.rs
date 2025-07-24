use axum::{
    routing::{get, post},
    Router,
    middleware,
};

use crate::services::AppState;
use crate::auth::Claims;
use super::handlers::{
    create_ownership_contract,
    purchase_shares,
    get_contract_details,
    get_user_portfolio,
    list_contracts,
    distribute_revenue,
};

/// Create all routes for Fractional Ownership API
pub fn create_ownership_routes() -> Router<AppState> {
    Router::new()
        // Contract management
        .route("/contracts", get(list_contracts).post(create_ownership_contract))
        .route("/contracts/:id", get(get_contract_details))
        .route("/contracts/:id/purchase", post(purchase_shares))
        .route("/contracts/:id/distribute", post(distribute_revenue))
        
        // User portfolio
        .route("/users/:id/portfolio", get(get_user_portfolio))
        
        // Add authentication middleware to all routes
        .layer(middleware::from_fn(auth_middleware))
}

/// Authentication middleware to ensure all routes require valid JWT
async fn auth_middleware<B>(
    mut req: axum::http::Request<B>,
    next: axum::middleware::Next<B>,
) -> Result<axum::response::Response, axum::http::StatusCode> {
    // Extract and validate JWT claims
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;

    let claims = Claims::from_jwt(auth_header)
        .map_err(|_| axum::http::StatusCode::UNAUTHORIZED)?;

    // Insert claims into request extensions for handlers to use
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_routes_exist() {
        let app = create_ownership_routes();
        
        // Test that routes are properly registered (will fail auth but route exists)
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/contracts")
                    .method("GET")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should be 401 Unauthorized (not 404 Not Found)
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
} 