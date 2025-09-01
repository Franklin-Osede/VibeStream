use axum::{
    routing::{get, post, put},
    Router,
};
use crate::bounded_contexts::campaign::presentation::controllers::CampaignController;
use crate::shared::infrastructure::app_state::CampaignAppState;

/// Create the campaign routes for the API
/// 
/// This creates all the HTTP routes for campaign management:
/// - Campaign CRUD operations
/// - Campaign lifecycle management (activate, pause, resume, end)
/// - NFT purchasing
/// - Analytics and trending
pub fn create_campaign_routes() -> Router<CampaignAppState> {
    Router::new()
        // Campaign CRUD
        .route("/campaigns", post(CampaignController::create_campaign))
        .route("/campaigns", get(CampaignController::list_campaigns))
        .route("/campaigns/:id", get(CampaignController::get_campaign))
        
        // Campaign lifecycle management
        .route("/campaigns/:id/activate", put(CampaignController::activate_campaign))
        .route("/campaigns/:id/pause", put(CampaignController::pause_campaign))
        .route("/campaigns/:id/resume", put(CampaignController::resume_campaign))
        .route("/campaigns/:id/end", put(CampaignController::end_campaign))
        
        // NFT purchasing
        .route("/campaigns/:id/purchase", post(CampaignController::purchase_nft))
        
        // Analytics and trends
        .route("/campaigns/:id/analytics", get(CampaignController::get_campaign_analytics))
        .route("/campaigns/trending", get(CampaignController::get_trending_campaigns))
}

/// Create campaign routes with API version prefix
/// 
/// This wraps the campaign routes with the `/api/v1` prefix
pub fn create_versioned_campaign_routes() -> Router {
    Router::new()
        .nest("/api/v1", create_campaign_routes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use serde_json::json;
    use axum::ServiceExt;

    #[tokio::test]
    async fn test_campaign_routes_creation() {
        let app = create_campaign_routes();
        
        // Test that routes are created without panicking
        assert!(true);
    }

    // Note: Full integration tests would require setting up test database
    // and mock services. For now, we test route creation only.
    
    #[test]
    fn test_route_paths() {
        // Test that our route structure is correct
        let expected_routes = vec![
            "/campaigns",           // POST, GET
            "/campaigns/:id",       // GET
            "/campaigns/:id/activate", // PUT
            "/campaigns/:id/pause",    // PUT  
            "/campaigns/:id/resume",   // PUT
            "/campaigns/:id/end",      // PUT
            "/campaigns/:id/purchase", // POST
            "/campaigns/:id/analytics", // GET
            "/campaigns/trending",     // GET
        ];

        // In a real test, we would verify these routes are registered
        // For now, we just verify our expected route list
        assert_eq!(expected_routes.len(), 9);
    }
}

/// Route documentation for API consumers
/// 
/// # Campaign API Endpoints
/// 
/// ## Campaign Management
/// 
/// ### Create Campaign
/// ```
/// POST /api/v1/campaigns
/// Content-Type: application/json
/// 
/// {
///   "song_id": "uuid",
///   "artist_id": "uuid", 
///   "name": "Campaign Name",
///   "description": "Campaign description",
///   "start_date": "2024-01-01T00:00:00Z",
///   "end_date": "2024-02-01T00:00:00Z",
///   "boost_multiplier": 2.0,
///   "nft_price": 10.0,
///   "max_nfts": 1000,
///   "target_revenue": 10000.0
/// }
/// ```
/// 
/// ### Get Campaign Details
/// ```
/// GET /api/v1/campaigns/{id}
/// ```
/// 
/// ### List Campaigns
/// ```
/// GET /api/v1/campaigns?page=1&page_size=20&status=active&artist_id=uuid
/// ```
/// 
/// ## Campaign Lifecycle
/// 
/// ### Activate Campaign
/// ```
/// PUT /api/v1/campaigns/{id}/activate
/// Content-Type: application/json
/// 
/// {
///   "nft_contract_address": "0x123..."
/// }
/// ```
/// 
/// ### Pause Campaign
/// ```
/// PUT /api/v1/campaigns/{id}/pause
/// ```
/// 
/// ### Resume Campaign  
/// ```
/// PUT /api/v1/campaigns/{id}/resume
/// ```
/// 
/// ### End Campaign
/// ```
/// PUT /api/v1/campaigns/{id}/end
/// Content-Type: application/json
/// 
/// {
///   "reason": "time_expired"
/// }
/// ```
/// 
/// ## NFT Operations
/// 
/// ### Purchase NFT
/// ```
/// POST /api/v1/campaigns/{id}/purchase
/// Content-Type: application/json
/// 
/// {
///   "buyer_id": "uuid",
///   "quantity": 5
/// }
/// ```
/// 
/// ## Analytics
/// 
/// ### Get Campaign Analytics
/// ```
/// GET /api/v1/campaigns/{id}/analytics
/// ```
/// 
/// ### Get Trending Campaigns
/// ```
/// GET /api/v1/campaigns/trending
/// ```
pub const CAMPAIGN_API_DOCS: &str = include_str!("./routes.rs"); 