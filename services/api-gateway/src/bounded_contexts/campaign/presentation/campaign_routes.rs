use axum::{
    routing::{get, post},
    Router,
    middleware,
};

use crate::services::AppState;
use crate::auth::Claims;
use super::handlers::{
    create_campaign,
    activate_campaign,
    purchase_nft,
    get_campaign_details,
    list_campaigns,
    get_campaign_analytics,
};

/// Create all routes for Campaign API
pub fn create_campaign_routes() -> Router<AppState> {
    Router::new()
        // Campaign management
        .route("/campaigns", get(list_campaigns).post(create_campaign))
        .route("/campaigns/:id", get(get_campaign_details))
        .route("/campaigns/:id/activate", post(activate_campaign))
        .route("/campaigns/:id/purchase", post(purchase_nft))
        .route("/campaigns/:id/analytics", get(get_campaign_analytics))
        
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
    async fn test_campaign_routes_exist() {
        let app = create_campaign_routes();
        
        // Test that routes are properly registered
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/campaigns")
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