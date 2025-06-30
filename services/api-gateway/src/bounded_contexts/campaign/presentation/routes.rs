use axum::{
    routing::{get, post},
    Router,
};

use super::controllers::*;

pub fn create_campaign_routes() -> Router<crate::services::AppState> {
    Router::new()
        .route("/campaigns", post(create_campaign))
        .route("/campaigns", get(get_active_campaigns))
        .route("/campaigns/health", get(campaign_health_check))
        .route("/campaigns/:id", get(get_campaign_by_id))
        .route("/campaigns/:id/activate", post(activate_campaign))
        .route("/campaigns/:id/purchase", post(purchase_nft))
        .route("/campaigns/:id/analytics", get(get_campaign_analytics))
        .route("/campaigns/artist/:artist_id", get(get_campaigns_by_artist))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_campaign_routes_health() {
        let app = create_campaign_routes();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/campaigns/health")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_active_campaigns_route() {
        let app = create_campaign_routes();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/campaigns")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
} 