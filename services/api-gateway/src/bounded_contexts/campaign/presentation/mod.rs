#![allow(unused_imports)]

use axum::{Router, routing::post, extract::State, Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::services::AppState;

use super::application::commands::{CreateCampaign, CreateCampaignResult};

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
        artist_id,
        nft_contract: req.nft_contract,
        start: start_dt,
        end: end_dt,
        multiplier: req.multiplier,
    };

    // Dispatch command via Command Bus
    let boxed_res = state
        .command_bus
        .dispatch(cmd)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result = *boxed_res.downcast::<CreateCampaignResult>().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(CreateCampaignResponse { campaign_id: result.campaign_id.to_string() })))
} 