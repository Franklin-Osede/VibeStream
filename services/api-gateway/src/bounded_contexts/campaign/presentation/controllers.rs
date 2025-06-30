use axum::{
    extract::{Path, State, Query},
    response::Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::services::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCampaignRequest {
    pub song_id: String,
    pub artist_id: String,
    pub name: String,
    pub description: String,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub boost_multiplier: f64,
    pub nft_price: f64,
    pub max_nfts: u32,
    pub target_revenue: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivateCampaignRequest {
    pub nft_contract_address: String,
    pub blockchain_network: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseNFTRequest {
    pub user_id: String,
    pub payment_method: String,
    pub payment_token: String,
    pub wallet_address: String,
    pub quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndCampaignRequest {
    pub reason: String,
    pub force_end: Option<bool>,
}

pub async fn create_campaign(
    State(_state): State<AppState>,
    Json(request): Json<CreateCampaignRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let campaign_id = Uuid::new_v4().to_string();
    
    let response = serde_json::json!({
        "success": true,
        "campaign_id": campaign_id,
        "message": "Campaign created successfully",
        "details": {
            "name": request.name,
            "artist_id": request.artist_id,
            "song_id": request.song_id,
            "nft_price": request.nft_price,
            "max_nfts": request.max_nfts
        }
    });

    Ok(Json(response))
}

pub async fn activate_campaign(
    State(_state): State<AppState>,
    Path(campaign_id): Path<String>,
    Json(request): Json<ActivateCampaignRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let response = serde_json::json!({
        "success": true,
        "campaign_id": campaign_id,
        "message": "Campaign activated successfully",
        "nft_contract_address": request.nft_contract_address,
        "blockchain_network": request.blockchain_network
    });

    Ok(Json(response))
}

pub async fn purchase_nft(
    State(_state): State<AppState>,
    Path(campaign_id): Path<String>,
    Json(request): Json<PurchaseNFTRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let nft_ids: Vec<String> = (0..request.quantity)
        .map(|_| Uuid::new_v4().to_string())
        .collect();

    let response = serde_json::json!({
        "success": true,
        "campaign_id": campaign_id,
        "message": format!("Successfully purchased {} NFT(s)", request.quantity),
        "nft_ids": nft_ids,
        "user_id": request.user_id,
        "total_amount": request.quantity as f64 * 10.0,
        "payment_method": request.payment_method
    });

    Ok(Json(response))
}

pub async fn get_campaign_analytics(
    State(_state): State<AppState>,
    Path(campaign_id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let include_predictions = params.get("include_predictions")
        .and_then(|v| v.parse::<bool>().ok())
        .unwrap_or(false);
    
    let include_optimization_suggestions = params.get("include_optimization_suggestions")
        .and_then(|v| v.parse::<bool>().ok())
        .unwrap_or(false);

    let response = serde_json::json!({
        "success": true,
        "campaign_id": campaign_id,
        "analytics": {
            "total_nfts_sold": 50,
            "total_revenue": 500.0,
            "completion_percentage": 25.0,
            "unique_buyers": 45,
            "average_purchase_amount": 11.11,
            "sales_velocity": 5.5,
            "days_remaining": 15
        },
        "include_predictions": include_predictions,
        "include_optimization_suggestions": include_optimization_suggestions,
        "message": "Analytics retrieved successfully"
    });

    Ok(Json(response))
}

pub async fn end_campaign(
    State(_state): State<AppState>,
    Path(campaign_id): Path<String>,
    Json(request): Json<EndCampaignRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let response = serde_json::json!({
        "success": true,
        "campaign_id": campaign_id,
        "message": format!("Campaign ended with reason: {}", request.reason),
        "force_end": request.force_end.unwrap_or(false)
    });

    Ok(Json(response))
}

pub async fn get_campaign_by_id(
    State(_state): State<AppState>,
    Path(campaign_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let response = serde_json::json!({
        "success": true,
        "campaign_id": campaign_id,
        "name": "Mock Campaign",
        "status": "Active",
        "artist_id": "artist-123",
        "song_id": "song-456",
        "nft_price": 10.0,
        "max_nfts": 200,
        "current_sold": 50,
        "message": "Campaign retrieved successfully"
    });

    Ok(Json(response))
}

pub async fn get_campaigns_by_artist(
    State(_state): State<AppState>,
    Path(artist_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let response = serde_json::json!({
        "success": true,
        "artist_id": artist_id,
        "campaigns": [
            {
                "campaign_id": "campaign-1",
                "name": "Artist Campaign 1",
                "status": "Active"
            },
            {
                "campaign_id": "campaign-2", 
                "name": "Artist Campaign 2",
                "status": "Completed"
            }
        ],
        "total": 2,
        "message": "Campaigns retrieved successfully"
    });

    Ok(Json(response))
}

pub async fn get_active_campaigns(
    State(_state): State<AppState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let response = serde_json::json!({
        "success": true,
        "campaigns": [
            {
                "campaign_id": "campaign-1",
                "name": "Active Campaign 1",
                "artist_id": "artist-123",
                "status": "Active",
                "nft_price": 10.0,
                "completion_percentage": 25.0
            },
            {
                "campaign_id": "campaign-2",
                "name": "Active Campaign 2", 
                "artist_id": "artist-456",
                "status": "Active",
                "nft_price": 15.0,
                "completion_percentage": 60.0
            }
        ],
        "total": 2,
        "message": "Active campaigns retrieved successfully"
    });

    Ok(Json(response))
}

pub async fn campaign_health_check() -> Result<Json<serde_json::Value>, StatusCode> {
    let response = serde_json::json!({
        "status": "healthy",
        "service": "campaign-service",
        "timestamp": chrono::Utc::now(),
        "version": "1.0.0"
    });

    Ok(Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let result = campaign_health_check().await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_create_campaign_request_serialization() {
        let request = CreateCampaignRequest {
            song_id: "song-123".to_string(),
            artist_id: "artist-456".to_string(),
            name: "Test Campaign".to_string(),
            description: "Test Description".to_string(),
            start_date: chrono::Utc::now(),
            end_date: chrono::Utc::now() + chrono::Duration::days(30),
            boost_multiplier: 1.5,
            nft_price: 10.0,
            max_nfts: 100,
            target_revenue: Some(1000.0),
        };

        let serialized = serde_json::to_string(&request);
        assert!(serialized.is_ok());
    }
} 