use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json as ResponseJson,
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::shared::domain::errors::AppError;
use super::super::domain::{
    aggregates::CampaignAggregate,
    entities::{Campaign, CampaignStatus},
    value_objects::{CampaignId, DateRange},
    events::CampaignEndReason,
};
use crate::bounded_contexts::music::domain::value_objects::{SongId, ArtistId};

// ============================================================================
// REQUEST/RESPONSE DTOs
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateCampaignRequest {
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub name: String,
    pub description: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub boost_multiplier: f64,
    pub nft_price: f64,
    pub max_nfts: u32,
    pub target_revenue: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct CreateCampaignResponse {
    pub campaign_id: Uuid,
    pub name: String,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub status: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub boost_multiplier: f64,
    pub nft_price: f64,
    pub max_nfts: u32,
    pub target_revenue: Option<f64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ActivateCampaignRequest {
    pub nft_contract_address: String,
}

#[derive(Debug, Serialize)]
pub struct ActivateCampaignResponse {
    pub success: bool,
    pub message: String,
    pub campaign_id: Uuid,
    pub nft_contract_address: String,
    pub activated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct PurchaseNFTRequest {
    pub buyer_id: Uuid,
    pub quantity: u32,
}

#[derive(Debug, Serialize)]
pub struct PurchaseNFTResponse {
    pub success: bool,
    pub message: String,
    pub transaction_id: Uuid,
    pub nft_id: Uuid,
    pub quantity: u32,
    pub total_amount: f64,
    pub boost_multiplier: f64,
}

#[derive(Debug, Serialize)]
pub struct CampaignDetailsResponse {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub status: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub boost_multiplier: f64,
    pub nft_price: f64,
    pub max_nfts: u32,
    pub nfts_sold: u32,
    pub nfts_remaining: u32,
    pub completion_percentage: f64,
    pub current_revenue: f64,
    pub target_revenue: Option<f64>,
    pub days_remaining: i64,
    pub sales_velocity: f64,
    pub is_successful: bool,
    pub nft_contract_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CampaignListResponse {
    pub campaigns: Vec<CampaignSummary>,
    pub total_count: usize,
    pub page: u32,
    pub page_size: u32,
    pub has_next_page: bool,
}

#[derive(Debug, Serialize)]
pub struct CampaignSummary {
    pub id: Uuid,
    pub name: String,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub status: String,
    pub nft_price: f64,
    pub nfts_sold: u32,
    pub max_nfts: u32,
    pub completion_percentage: f64,
    pub current_revenue: f64,
    pub boost_multiplier: f64,
    pub days_remaining: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CampaignAnalyticsResponse {
    pub campaign_id: Uuid,
    pub total_nfts_sold: u32,
    pub total_revenue: f64,
    pub completion_percentage: f64,
    pub unique_buyers: u32,
    pub average_purchase_amount: f64,
    pub sales_velocity: f64,
    pub days_remaining: i64,
    pub is_successful: bool,
    pub boost_efficiency: f64,
    pub daily_sales: Vec<DailySalesData>,
    pub top_buyers: Vec<TopBuyerData>,
    pub performance_metrics: PerformanceMetrics,
}

#[derive(Debug, Serialize)]
pub struct DailySalesData {
    pub date: DateTime<Utc>,
    pub nfts_sold: u32,
    pub revenue: f64,
}

#[derive(Debug, Serialize)]
pub struct TopBuyerData {
    pub buyer_id: Uuid,
    pub nfts_purchased: u32,
    pub total_spent: f64,
}

#[derive(Debug, Serialize)]
pub struct PerformanceMetrics {
    pub roi_percentage: f64,
    pub engagement_score: f64,
    pub viral_coefficient: f64,
    pub predicted_final_sales: u32,
}

#[derive(Debug, Deserialize)]
pub struct ListCampaignsQuery {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub status: Option<String>,
    pub artist_id: Option<Uuid>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub sort_by: Option<String>, // "completion", "revenue", "ending_soon", "newest"
    pub search: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EndCampaignRequest {
    pub reason: String, // "time_expired", "sold_out", "artist_terminated", "admin_terminated"
}

// ============================================================================
// ERROR RESPONSE
// ============================================================================

#[derive(Debug, Serialize)]
pub struct ApiErrorResponse {
    pub error: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

// ============================================================================
// CAMPAIGN CONTROLLER
// ============================================================================

pub struct CampaignController;

impl CampaignController {
    /// POST /api/v1/campaigns - Create new campaign
    pub async fn create_campaign(
        Json(request): Json<CreateCampaignRequest>,
    ) -> Result<ResponseJson<CreateCampaignResponse>, (StatusCode, ResponseJson<ApiErrorResponse>)> {
        // Validate request
        Self::validate_create_campaign_request(&request)?;

        // Create campaign aggregate
        let campaign_aggregate = CampaignAggregate::create_campaign(
            SongId::from_uuid(request.song_id),
            ArtistId::from_uuid(request.artist_id),
            request.name.clone(),
            request.description.clone(),
            request.start_date,
            request.end_date,
            request.boost_multiplier,
            request.nft_price,
            request.max_nfts,
            request.target_revenue,
        ).map_err(|e| Self::map_domain_error(e))?;

        let campaign = campaign_aggregate.campaign();

        // TODO: Save to repository
        // self.campaign_repository.save(&campaign_aggregate).await?;

        // TODO: Publish domain events
        // self.event_publisher.publish_events(campaign_aggregate.pending_events()).await?;

        let response = CreateCampaignResponse {
            campaign_id: campaign.id().value().clone(),
            name: campaign.name().to_string(),
            song_id: campaign.song_id().value().clone(),
            artist_id: campaign.artist_id().value().clone(),
            status: format!("{:?}", campaign.status()),
            start_date: campaign.date_range().start(),
            end_date: campaign.date_range().end(),
            boost_multiplier: campaign.boost_multiplier().value(),
            nft_price: campaign.nft_price().value(),
            max_nfts: campaign.nft_supply().max_supply(),
            target_revenue: campaign.target().map(|t| t.value()),
            created_at: campaign.created_at(),
        };

        Ok(ResponseJson(response))
    }

    /// GET /api/v1/campaigns/{id} - Get campaign details
    pub async fn get_campaign(
        Path(campaign_id): Path<Uuid>,
    ) -> Result<ResponseJson<CampaignDetailsResponse>, (StatusCode, ResponseJson<ApiErrorResponse>)> {
        // TODO: Load from repository
        // let campaign_aggregate = self.campaign_repository.find_by_id(&CampaignId::from_uuid(campaign_id)).await?;

        // Mock response for now
        let response = CampaignDetailsResponse {
            id: campaign_id,
            name: "Sample Campaign".to_string(),
            description: "A sample campaign for testing".to_string(),
            song_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            status: "Active".to_string(),
            start_date: Utc::now(),
            end_date: Utc::now() + chrono::Duration::days(30),
            boost_multiplier: 2.0,
            nft_price: 10.0,
            max_nfts: 1000,
            nfts_sold: 150,
            nfts_remaining: 850,
            completion_percentage: 15.0,
            current_revenue: 1500.0,
            target_revenue: Some(10000.0),
            days_remaining: 25,
            sales_velocity: 6.0,
            is_successful: false,
            nft_contract_address: Some("0x123456789...".to_string()),
            created_at: Utc::now() - chrono::Duration::days(5),
            updated_at: Utc::now(),
        };

        Ok(ResponseJson(response))
    }

    /// GET /api/v1/campaigns - List campaigns with filters
    pub async fn list_campaigns(
        Query(query): Query<ListCampaignsQuery>,
    ) -> Result<ResponseJson<CampaignListResponse>, (StatusCode, ResponseJson<ApiErrorResponse>)> {
        let page = query.page.unwrap_or(1);
        let page_size = query.page_size.unwrap_or(20).min(100); // Max 100 per page

        // TODO: Implement repository query with filters
        // let campaigns = self.campaign_repository.find_with_filters(&query).await?;

        // Mock response for now
        let mock_campaign = CampaignSummary {
            id: Uuid::new_v4(),
            name: "Sample Campaign 1".to_string(),
            song_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            status: "Active".to_string(),
            nft_price: 10.0,
            nfts_sold: 150,
            max_nfts: 1000,
            completion_percentage: 15.0,
            current_revenue: 1500.0,
            boost_multiplier: 2.0,
            days_remaining: 25,
            created_at: Utc::now() - chrono::Duration::days(5),
        };

        let response = CampaignListResponse {
            campaigns: vec![mock_campaign],
            total_count: 1,
            page,
            page_size,
            has_next_page: false,
        };

        Ok(ResponseJson(response))
    }

    /// PUT /api/v1/campaigns/{id}/activate - Activate campaign
    pub async fn activate_campaign(
        Path(campaign_id): Path<Uuid>,
        Json(request): Json<ActivateCampaignRequest>,
    ) -> Result<ResponseJson<ActivateCampaignResponse>, (StatusCode, ResponseJson<ApiErrorResponse>)> {
        // Validate NFT contract address
        if request.nft_contract_address.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                ResponseJson(ApiErrorResponse {
                    error: "ValidationError".to_string(),
                    message: "NFT contract address is required".to_string(),
                    details: None,
                }),
            ));
        }

        // TODO: Load campaign, activate, and save
        // let mut campaign_aggregate = self.campaign_repository.find_by_id(&CampaignId::from_uuid(campaign_id)).await?;
        // campaign_aggregate.activate_campaign(request.nft_contract_address.clone())?;
        // self.campaign_repository.save(&campaign_aggregate).await?;

        let response = ActivateCampaignResponse {
            success: true,
            message: "Campaign activated successfully".to_string(),
            campaign_id,
            nft_contract_address: request.nft_contract_address,
            activated_at: Utc::now(),
        };

        Ok(ResponseJson(response))
    }

    /// POST /api/v1/campaigns/{id}/purchase - Purchase NFT
    pub async fn purchase_nft(
        Path(campaign_id): Path<Uuid>,
        Json(request): Json<PurchaseNFTRequest>,
    ) -> Result<ResponseJson<PurchaseNFTResponse>, (StatusCode, ResponseJson<ApiErrorResponse>)> {
        // Validate request
        if request.quantity == 0 {
            return Err((
                StatusCode::BAD_REQUEST,
                ResponseJson(ApiErrorResponse {
                    error: "ValidationError".to_string(),
                    message: "Quantity must be greater than 0".to_string(),
                    details: None,
                }),
            ));
        }

        // TODO: Load campaign, process purchase, and save
        // let mut campaign_aggregate = self.campaign_repository.find_by_id(&CampaignId::from_uuid(campaign_id)).await?;
        // let purchase_event = campaign_aggregate.purchase_nft(request.buyer_id, request.quantity)?;
        // self.campaign_repository.save(&campaign_aggregate).await?;

        let response = PurchaseNFTResponse {
            success: true,
            message: "NFT purchase successful".to_string(),
            transaction_id: Uuid::new_v4(),
            nft_id: Uuid::new_v4(),
            quantity: request.quantity,
            total_amount: 10.0 * request.quantity as f64, // Mock calculation
            boost_multiplier: 2.0,
        };

        Ok(ResponseJson(response))
    }

    /// PUT /api/v1/campaigns/{id}/pause - Pause campaign
    pub async fn pause_campaign(
        Path(campaign_id): Path<Uuid>,
    ) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<ApiErrorResponse>)> {
        // TODO: Load campaign, pause, and save
        // let mut campaign_aggregate = self.campaign_repository.find_by_id(&CampaignId::from_uuid(campaign_id)).await?;
        // campaign_aggregate.pause_campaign()?;
        // self.campaign_repository.save(&campaign_aggregate).await?;

        Ok(ResponseJson(serde_json::json!({
            "success": true,
            "message": "Campaign paused successfully",
            "campaign_id": campaign_id
        })))
    }

    /// PUT /api/v1/campaigns/{id}/resume - Resume campaign
    pub async fn resume_campaign(
        Path(campaign_id): Path<Uuid>,
    ) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<ApiErrorResponse>)> {
        // TODO: Load campaign, resume, and save

        Ok(ResponseJson(serde_json::json!({
            "success": true,
            "message": "Campaign resumed successfully",
            "campaign_id": campaign_id
        })))
    }

    /// PUT /api/v1/campaigns/{id}/end - End campaign
    pub async fn end_campaign(
        Path(campaign_id): Path<Uuid>,
        Json(request): Json<EndCampaignRequest>,
    ) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<ApiErrorResponse>)> {
        let reason = match request.reason.as_str() {
            "time_expired" => CampaignEndReason::TimeExpired,
            "sold_out" => CampaignEndReason::SoldOut,
            "artist_terminated" => CampaignEndReason::ArtistTerminated,
            "admin_terminated" => CampaignEndReason::AdminTerminated,
            _ => return Err((
                StatusCode::BAD_REQUEST,
                ResponseJson(ApiErrorResponse {
                    error: "ValidationError".to_string(),
                    message: "Invalid end reason".to_string(),
                    details: None,
                }),
            )),
        };

        // TODO: Load campaign, end, and save

        Ok(ResponseJson(serde_json::json!({
            "success": true,
            "message": "Campaign ended successfully",
            "campaign_id": campaign_id,
            "end_reason": request.reason
        })))
    }

    /// GET /api/v1/campaigns/{id}/analytics - Get campaign analytics
    pub async fn get_campaign_analytics(
        Path(campaign_id): Path<Uuid>,
    ) -> Result<ResponseJson<CampaignAnalyticsResponse>, (StatusCode, ResponseJson<ApiErrorResponse>)> {
        // TODO: Load campaign and calculate analytics

        // Mock analytics data
        let response = CampaignAnalyticsResponse {
            campaign_id,
            total_nfts_sold: 150,
            total_revenue: 1500.0,
            completion_percentage: 15.0,
            unique_buyers: 75,
            average_purchase_amount: 20.0,
            sales_velocity: 6.0,
            days_remaining: 25,
            is_successful: false,
            boost_efficiency: 85.5,
            daily_sales: vec![
                DailySalesData {
                    date: Utc::now() - chrono::Duration::days(1),
                    nfts_sold: 8,
                    revenue: 80.0,
                },
                DailySalesData {
                    date: Utc::now(),
                    nfts_sold: 12,
                    revenue: 120.0,
                },
            ],
            top_buyers: vec![
                TopBuyerData {
                    buyer_id: Uuid::new_v4(),
                    nfts_purchased: 25,
                    total_spent: 250.0,
                },
            ],
            performance_metrics: PerformanceMetrics {
                roi_percentage: 12.5,
                engagement_score: 78.2,
                viral_coefficient: 1.3,
                predicted_final_sales: 800,
            },
        };

        Ok(ResponseJson(response))
    }

    /// GET /api/v1/campaigns/trending - Get trending campaigns
    pub async fn get_trending_campaigns() -> Result<ResponseJson<CampaignListResponse>, (StatusCode, ResponseJson<ApiErrorResponse>)> {
        // TODO: Implement trending algorithm

        // Mock trending campaigns
        let mock_campaign = CampaignSummary {
            id: Uuid::new_v4(),
            name: "Trending Campaign".to_string(),
            song_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            status: "Active".to_string(),
            nft_price: 15.0,
            nfts_sold: 450,
            max_nfts: 1000,
            completion_percentage: 45.0,
            current_revenue: 6750.0,
            boost_multiplier: 3.0,
            days_remaining: 15,
            created_at: Utc::now() - chrono::Duration::days(10),
        };

        let response = CampaignListResponse {
            campaigns: vec![mock_campaign],
            total_count: 1,
            page: 1,
            page_size: 20,
            has_next_page: false,
        };

        Ok(ResponseJson(response))
    }

    // ============================================================================
    // VALIDATION HELPERS
    // ============================================================================

    fn validate_create_campaign_request(request: &CreateCampaignRequest) -> Result<(), (StatusCode, ResponseJson<ApiErrorResponse>)> {
        let mut errors = Vec::new();

        // Name validation
        if request.name.trim().is_empty() {
            errors.push("Campaign name cannot be empty");
        }
        if request.name.len() > 100 {
            errors.push("Campaign name cannot exceed 100 characters");
        }

        // Date validation
        if request.start_date >= request.end_date {
            errors.push("Start date must be before end date");
        }
        if request.start_date <= Utc::now() {
            errors.push("Start date must be in the future");
        }

        // Price validation
        if request.nft_price <= 0.0 {
            errors.push("NFT price must be greater than 0");
        }
        if request.nft_price > 10000.0 {
            errors.push("NFT price cannot exceed 10,000");
        }

        // Multiplier validation
        if request.boost_multiplier < 1.0 {
            errors.push("Boost multiplier must be at least 1.0");
        }
        if request.boost_multiplier > 10.0 {
            errors.push("Boost multiplier cannot exceed 10.0");
        }

        // Max NFTs validation
        if request.max_nfts == 0 {
            errors.push("Max NFTs must be greater than 0");
        }
        if request.max_nfts > 100000 {
            errors.push("Max NFTs cannot exceed 100,000");
        }

        if !errors.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                ResponseJson(ApiErrorResponse {
                    error: "ValidationError".to_string(),
                    message: errors.join(", "),
                    details: Some(serde_json::json!({ "validation_errors": errors })),
                }),
            ));
        }

        Ok(())
    }

    fn map_domain_error(error: AppError) -> (StatusCode, ResponseJson<ApiErrorResponse>) {
        match error {
            AppError::DomainRuleViolation(msg) => (
                StatusCode::BAD_REQUEST,
                ResponseJson(ApiErrorResponse {
                    error: "DomainRuleViolation".to_string(),
                    message: msg,
                    details: None,
                }),
            ),
            AppError::ValidationError(msg) => (
                StatusCode::BAD_REQUEST,
                ResponseJson(ApiErrorResponse {
                    error: "ValidationError".to_string(),
                    message: msg,
                    details: None,
                }),
            ),
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                ResponseJson(ApiErrorResponse {
                    error: "NotFound".to_string(),
                    message: msg,
                    details: None,
                }),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(ApiErrorResponse {
                    error: "InternalServerError".to_string(),
                    message: "An unexpected error occurred".to_string(),
                    details: None,
                }),
            ),
        }
    }
} 