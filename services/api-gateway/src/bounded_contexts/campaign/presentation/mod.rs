#![allow(unused_imports)]

use axum::{Router, routing::post, extract::State, Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::services::AppState;

use super::application::commands::{CreateCampaign, CreateCampaignResult};

pub mod controllers;
pub mod routes;

// Re-export main components
pub use controllers::*;
pub use routes::*;

// Re-export for convenience
pub use controllers::CampaignController;
pub use routes::{create_campaign_routes, create_versioned_campaign_routes};

#[derive(Deserialize)]
struct CreateCampaignRequest {
    artist_id: String,
    nft_contract: String,
    start: String, // ISO-8601 datetime
    end: String,   // ISO-8601 datetime
    multiplier: f64,
}

#[derive(Serialize)]
struct CreateCampaignResponse {
    campaign_id: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/campaigns", post(create_campaign))
}

async fn create_campaign(State(state): State<AppState>, Json(req): Json<CreateCampaignRequest>) -> Result<(StatusCode, Json<CreateCampaignResponse>), StatusCode> {
    // Parse UUID and datetimes
    let artist_id = Uuid::parse_str(&req.artist_id).map_err(|_| StatusCode::BAD_REQUEST)?;
    let start_dt = DateTime::parse_from_rfc3339(&req.start)
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .with_timezone(&Utc);
    let end_dt = DateTime::parse_from_rfc3339(&req.end)
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .with_timezone(&Utc);

    let cmd = CreateCampaign {
        song_id: Uuid::new_v4(), // Mock song ID
        artist_id,
        name: "Campaign from API".to_string(),
        description: Some("Created via API".to_string()),
        nft_contract: req.nft_contract,
        start: start_dt,
        end: end_dt,
        multiplier: req.multiplier,
        nft_price: 10.0, // Default price
        max_nfts: 1000, // Default max
        target_revenue: Some(10000.0), // Default target
    };

    // Dispatch command via Command Bus
    // TODO: Implement command bus integration
    // let boxed_res = state
    //     .command_bus
    //     .dispatch(cmd)
    //     .await
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // let result = *boxed_res.downcast::<CreateCampaignResult>().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Temporary mock response
    let result = CreateCampaignResult {
        campaign_id: Uuid::new_v4(),
    };

    Ok((StatusCode::CREATED, Json(CreateCampaignResponse { campaign_id: result.campaign_id.to_string() })))
} 