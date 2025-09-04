use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    response::Json as ResponseJson,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::campaign::domain::{
    entities::{Campaign, CampaignStatus},
    value_objects::*,
    repository::CampaignRepository,
};
use vibestream_types::{SongContract, ArtistContract};
use crate::shared::infrastructure::app_state::CampaignAppState;
use crate::bounded_contexts::orchestrator::DomainEvent;
use crate::bounded_contexts::campaign::application::{
    commands::CreateCampaign,
    services::MockCampaignApplicationService,
};

// =============================================================================
// REQUEST/RESPONSE DTOs
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateCampaignRequest {
    pub artist_id: Uuid,
    pub song_id: Uuid,
    pub title: String,
    pub description: String,
    pub target_amount: f64,
    pub nft_price: f64,
    pub max_nfts: u32,
}

#[derive(Debug, Serialize)]
pub struct CreateCampaignResponse {
    pub campaign_id: Uuid,
    pub artist_id: Uuid,
    pub song_id: Uuid,
    pub title: String,
    pub description: String,
    pub target_amount: f64,
    pub nft_price: f64,
    pub max_nfts: u32,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CampaignResponse {
    pub campaign_id: Uuid,
    pub artist_id: Uuid,
    pub song_id: Uuid,
    pub title: String,
    pub description: String,
    pub target_amount: f64,
    pub nft_price: f64,
    pub max_nfts: u32,
    pub sold_nfts: u32,
    pub total_raised: f64,
    pub status: String,
    pub nft_contract_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct PurchaseNFTRequest {
    pub buyer_id: Uuid,
    pub quantity: u32,
}

#[derive(Debug, Serialize)]
pub struct PurchaseNFTResponse {
    pub transaction_hash: String,
    pub nft_ids: Vec<String>,
    pub total_amount: f64,
    pub purchased_at: DateTime<Utc>,
}

// =============================================================================
// CAMPAIGN CONTROLLER
// =============================================================================

pub struct CampaignController;

impl CampaignController {
    /// POST /api/v1/campaigns - Create a new campaign
    pub async fn create_campaign(
        State(state): State<CampaignAppState>,
        Json(request): Json<CreateCampaignRequest>,
    ) -> Result<ResponseJson<CreateCampaignResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // Validate input
        if request.title.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                ResponseJson(serde_json::json!({
                    "error": "Campaign title cannot be empty"
                }))
            ));
        }
        
        if request.target_amount <= 0.0 {
            return Err((
                StatusCode::BAD_REQUEST,
                ResponseJson(serde_json::json!({
                    "error": "Target amount must be greater than 0"
                }))
            ));
        }
        
        if request.nft_price <= 0.0 {
            return Err((
                StatusCode::BAD_REQUEST,
                ResponseJson(serde_json::json!({
                    "error": "NFT price must be greater than 0"
                }))
            ));
        }
        
        // Create campaign entity using domain logic
        let campaign_id = Uuid::new_v4();
        let now = Utc::now();
        
        // Save to repository using real implementation
        let song_contract = SongContract {
            id: request.song_id,
            title: "Unknown Song".to_string(),
            artist_id: request.artist_id,
            artist_name: "Unknown Artist".to_string(),
            duration_seconds: None,
            genre: None,
            ipfs_hash: None,
            metadata_url: None,
            nft_contract_address: None,
            nft_token_id: None,
            royalty_percentage: None,
            is_minted: false,
            created_at: now,
        };
        
        let artist_contract = ArtistContract {
            id: request.artist_id,
            user_id: Uuid::new_v4(),
            stage_name: "Unknown Artist".to_string(),
            bio: None,
            profile_image_url: None,
            verified: false,
            created_at: now,
        };
        
        let date_range = DateRange::new(now, now + chrono::Duration::days(30)).map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                ResponseJson(serde_json::json!({
                    "error": format!("Invalid date range: {}", e)
                }))
            )
        })?;
        
        let (campaign, _event) = Campaign::create(
            song_contract,
            artist_contract,
            request.title.clone(),
            request.description.clone(),
            date_range,
            1.0, // Default multiplier
            request.nft_price,
            request.max_nfts,
            Some(request.target_amount), // Use target_amount as target_revenue
        ).map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                ResponseJson(serde_json::json!({
                    "error": format!("Failed to create campaign: {}", e)
                }))
            )
        })?;
        
        // Save to repository
        if let Err(e) = state.campaign_repository.save(&campaign).await {
            tracing::error!("Failed to save campaign: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(serde_json::json!({
                    "error": "Failed to save campaign to database"
                }))
            ));
        }
        
        // Publish domain event
        let event = DomainEvent::CampaignCreated {
            campaign_id,
            artist_id: request.artist_id,
            song_id: request.song_id,
            occurred_at: now,
        };
        
        if let Err(e) = state.app_state.publish_event(event).await {
            tracing::warn!("Failed to publish campaign created event: {:?}", e);
        }
        
        let response = CreateCampaignResponse {
            campaign_id,
            artist_id: request.artist_id,
            song_id: request.song_id,
            title: request.title,
            description: request.description,
            target_amount: request.target_amount,
            nft_price: request.nft_price,
            max_nfts: request.max_nfts,
            status: "draft".to_string(),
            created_at: now,
        };
        
        Ok(ResponseJson(response))
    }
    
    /// GET /api/v1/campaigns - List campaigns
    pub async fn list_campaigns(
        State(state): State<CampaignAppState>,
    ) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // Get campaigns from repository using real implementation
        let campaigns = match state.campaign_repository.find_all().await {
            Ok(campaigns) => campaigns,
            Err(e) => {
                tracing::error!("Failed to fetch campaigns: {:?}", e);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ResponseJson(serde_json::json!({
                        "error": "Failed to fetch campaigns from database"
                    }))
                ));
            }
        };
        
        let campaign_data: Vec<serde_json::Value> = campaigns.into_iter().map(|campaign| {
            serde_json::json!({
                "campaign_id": campaign.id().value(),
                "artist_id": campaign.artist_contract().id,
                "song_id": campaign.song_contract().id,
                "title": campaign.name(),
                "description": campaign.description(),
                "target_amount": campaign.target().map(|t| t.value()).unwrap_or(0.0),
                "nft_price": campaign.nft_price().value(),
                "max_nfts": campaign.nft_supply().max_nfts(),
                "sold_nfts": campaign.nft_supply().current_sold(),
                "status": match campaign.status() {
                    CampaignStatus::Draft => "draft",
                    CampaignStatus::Active => "active",
                    CampaignStatus::Paused => "paused",
                    CampaignStatus::Completed => "completed",
                    CampaignStatus::Cancelled => "cancelled",
                    CampaignStatus::Failed => "failed",
                },
                "created_at": campaign.created_at()
            })
        }).collect();
        
        Ok(ResponseJson(serde_json::json!({
            "campaigns": campaign_data,
            "total": campaign_data.len()
        })))
    }
    
    /// GET /api/v1/campaigns/:id - Get campaign by ID
    pub async fn get_campaign(
        State(state): State<CampaignAppState>,
        Path(campaign_id): Path<Uuid>,
    ) -> Result<ResponseJson<CampaignResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // Get campaign from repository using real implementation
        let campaign = match state.campaign_repository.find_by_id(&CampaignId::new(campaign_id)).await {
            Ok(Some(campaign)) => campaign,
            Ok(None) => {
                return Err((
                    StatusCode::NOT_FOUND,
                    ResponseJson(serde_json::json!({
                        "error": "Campaign not found"
                    }))
                ));
            }
            Err(e) => {
                tracing::error!("Failed to fetch campaign: {:?}", e);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ResponseJson(serde_json::json!({
                        "error": "Failed to fetch campaign from database"
                    }))
                ));
            }
        };
        
        let response = CampaignResponse {
            campaign_id,
            artist_id: campaign.artist_contract().id,
            song_id: campaign.song_contract().id,
            title: campaign.name(),
            description: campaign.description(),
            target_amount: campaign.target().map(|t| t.value()).unwrap_or(0.0),
            nft_price: campaign.nft_price().value(),
            max_nfts: campaign.nft_supply().max_nfts(),
            sold_nfts: campaign.nft_supply().current_sold(),
            total_raised: campaign.nft_supply().current_sold() as f64 * campaign.nft_price().value(),
            status: match campaign.status() {
                CampaignStatus::Draft => "draft",
                CampaignStatus::Active => "active",
                CampaignStatus::Paused => "paused",
                CampaignStatus::Completed => "completed",
                CampaignStatus::Cancelled => "cancelled",
                CampaignStatus::Failed => "failed",
            },
            nft_contract_address: campaign.nft_contract_address(),
            created_at: campaign.created_at(),
            updated_at: campaign.updated_at(),
        };
        
        Ok(ResponseJson(response))
    }
    
    /// PUT /api/v1/campaigns/:id/activate - Activate a campaign
    pub async fn activate_campaign(
        State(state): State<CampaignAppState>,
        Path(campaign_id): Path<Uuid>,
    ) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // TODO: Implement actual campaign activation logic
        tracing::info!("Activating campaign: {}", campaign_id);
        
        // Publish domain event
        let event = DomainEvent::CampaignActivated {
            campaign_id,
            nft_contract_address: "0x1234567890abcdef".to_string(),
            occurred_at: Utc::now(),
        };
        
        if let Err(e) = state.app_state.publish_event(event).await {
            tracing::warn!("Failed to publish campaign activated event: {:?}", e);
        }
        
        Ok(ResponseJson(serde_json::json!({
            "message": "Campaign activated successfully",
            "campaign_id": campaign_id,
            "nft_contract_address": "0x1234567890abcdef"
        })))
    }
    
    /// PUT /api/v1/campaigns/:id/pause - Pause a campaign
    pub async fn pause_campaign(
        State(_state): State<CampaignAppState>,
        Path(campaign_id): Path<Uuid>,
    ) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // TODO: Implement actual campaign pause logic
        Ok(ResponseJson(serde_json::json!({
            "message": "Campaign paused successfully",
            "campaign_id": campaign_id
        })))
    }
    
    /// PUT /api/v1/campaigns/:id/resume - Resume a campaign
    pub async fn resume_campaign(
        State(_state): State<CampaignAppState>,
        Path(campaign_id): Path<Uuid>,
    ) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // TODO: Implement actual campaign resume logic
        Ok(ResponseJson(serde_json::json!({
            "message": "Campaign resumed successfully",
            "campaign_id": campaign_id
        })))
    }
    
    /// PUT /api/v1/campaigns/:id/end - End a campaign
    pub async fn end_campaign(
        State(_state): State<CampaignAppState>,
        Path(campaign_id): Path<Uuid>,
    ) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // TODO: Implement actual campaign end logic
        Ok(ResponseJson(serde_json::json!({
            "message": "Campaign ended successfully",
            "campaign_id": campaign_id
        })))
    }
    
    /// POST /api/v1/campaigns/:id/purchase - Purchase NFTs from a campaign
    pub async fn purchase_nft(
        State(state): State<CampaignAppState>,
        Path(campaign_id): Path<Uuid>,
        Json(request): Json<PurchaseNFTRequest>,
    ) -> Result<ResponseJson<PurchaseNFTResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // Validate input
        if request.quantity == 0 {
            return Err((
                StatusCode::BAD_REQUEST,
                ResponseJson(serde_json::json!({
                    "error": "Quantity must be greater than 0"
                }))
            ));
        }
        
        // TODO: Implement actual NFT purchase logic
        let transaction_hash = format!("0x{}", hex::encode(&[0u8; 32]));
        let nft_ids: Vec<String> = (0..request.quantity)
            .map(|i| format!("NFT-{}-{}", campaign_id, i))
            .collect();
        
        // Publish domain event
        let event = DomainEvent::NFTPurchased {
            campaign_id,
            buyer_id: request.buyer_id,
            quantity: request.quantity,
            amount: request.quantity as f64 * 100.0, // TODO: Get actual price
            occurred_at: Utc::now(),
        };
        
        if let Err(e) = state.app_state.publish_event(event).await {
            tracing::warn!("Failed to publish NFT purchased event: {:?}", e);
        }
        
        let response = PurchaseNFTResponse {
            transaction_hash,
            nft_ids,
            total_amount: request.quantity as f64 * 100.0,
            purchased_at: Utc::now(),
        };
        
        Ok(ResponseJson(response))
    }
    
    /// GET /api/v1/campaigns/:id/analytics - Get campaign analytics
    pub async fn get_campaign_analytics(
        State(_state): State<CampaignAppState>,
        Path(campaign_id): Path<Uuid>,
    ) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // TODO: Implement actual analytics logic
        let analytics = serde_json::json!({
            "campaign_id": campaign_id,
            "total_raised": 0.0,
            "total_nfts_sold": 0,
            "unique_buyers": 0,
            "average_purchase_size": 0.0,
            "completion_percentage": 0.0,
            "days_remaining": 30,
            "trending_score": 0.0
        });
        
        Ok(ResponseJson(analytics))
    }
    
    /// GET /api/v1/campaigns/trending - Get trending campaigns
    pub async fn get_trending_campaigns(
        State(_state): State<CampaignAppState>,
    ) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // TODO: Implement actual trending logic
        let trending_campaigns = vec![
            serde_json::json!({
                "campaign_id": Uuid::new_v4(),
                "title": "Trending Campaign 1",
                "artist_name": "Demo Artist",
                "trending_score": 95.5,
                "total_raised": 5000.0,
                "completion_percentage": 50.0
            }),
            serde_json::json!({
                "campaign_id": Uuid::new_v4(),
                "title": "Trending Campaign 2",
                "artist_name": "Demo Artist 2",
                "trending_score": 87.2,
                "total_raised": 3000.0,
                "completion_percentage": 30.0
            })
        ];
        
        Ok(ResponseJson(serde_json::json!({
            "trending_campaigns": trending_campaigns
        })))
    }
} 