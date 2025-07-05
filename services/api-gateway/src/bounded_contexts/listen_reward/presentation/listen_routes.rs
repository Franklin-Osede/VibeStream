use axum::{
    routing::{get, post, put},
    Router,
    middleware,
};

use crate::services::AppState;
use crate::auth::Claims;
use super::handlers::{
    start_listen_session,
    complete_listen_session,
    get_user_rewards,
    distribute_rewards,
    get_listen_analytics,
};

/// Create all routes for Listen Reward API
pub fn create_listen_routes() -> Router<AppState> {
    Router::new()
        // Listen session management
        .route("/sessions", post(start_listen_session))
        .route("/sessions/:id/complete", put(complete_listen_session))
        
        // User rewards and analytics
        .route("/users/:id/rewards", get(get_user_rewards))
        .route("/analytics", get(get_listen_analytics))
        
        // Admin reward distribution
        .route("/rewards/distribute", post(distribute_rewards))
        
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
    async fn test_listen_routes_exist() {
        let app = create_listen_routes();
        
        // Test that routes are properly registered
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/sessions")
                    .method("POST")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should be 401 Unauthorized (not 404 Not Found)
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
} 