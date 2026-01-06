use axum::{
    routing::{get, post, put, delete},
    Router,
    middleware,
};
use crate::shared::infrastructure::app_state::AppState;
use crate::auth::Claims;
use super::venture_handlers::{
    create_venture,
    get_venture_details,
    invest_in_venture,
    get_user_portfolio,
    list_ventures,
    update_venture,
    delete_venture,
    get_artist_ventures,
};

/// Create all routes for Fan Ventures API
pub fn create_venture_routes() -> Router<AppState> {
    Router::new()
        // Venture management
        .route("/", get(list_ventures).post(create_venture))
        .route("/:id", get(get_venture_details).put(update_venture).delete(delete_venture))
        .route("/:id/invest", post(invest_in_venture))
        
        // Artist ventures
        .route("/artists/:id/ventures", get(get_artist_ventures))
        
        // User portfolio
        .route("/users/:id/portfolio", get(get_user_portfolio))
        
        // Add authentication middleware to all routes
        .layer(middleware::from_fn(auth_middleware))
}

/// Authentication middleware to ensure all routes require valid JWT
async fn auth_middleware(
    mut req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
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
    req.extensions_mut().insert(claims.clone());

    let response = next.run(req).await;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_routes_exist() {
        let app = create_venture_routes();
        
        // Test that routes are properly registered (will fail auth but route exists)
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/")
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

