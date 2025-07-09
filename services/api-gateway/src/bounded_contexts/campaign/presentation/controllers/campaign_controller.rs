use axum::{
    extract::{Query, Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router, Extension,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Arc;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::campaign::application::{
    // Commands
    CreateCampaignCommand, CreateCampaignCommandHandler, CreateCampaignResult,
    ActivateCampaignCommand, ActivateCampaignCommandHandler,
    UpdateCampaignCommand, UpdateCampaignCommandHandler,
    ParticipateCampaignCommand, ParticipateCampaignCommandHandler,
    BoostCampaignCommand, BoostCampaignCommandHandler,
    MintCampaignNFTCommand, MintCampaignNFTCommandHandler,
    // Queries
    GetCampaignQuery, GetCampaignQueryHandler, CampaignDetailDTO,
    SearchCampaignsQuery, SearchCampaignsQueryHandler, SearchCampaignsResult,
    GetCampaignAnalyticsQuery, GetCampaignAnalyticsQueryHandler,
    GetTrendingCampaignsQuery, GetUserCampaignsQuery,
};

use crate::bounded_contexts::campaign::infrastructure::repositories::{
    PostgresCampaignRepository, PostgresCampaignParticipationRepository,
};

use crate::shared::domain::errors::AppError;

// =============================================================================
// REQUEST/RESPONSE DTOs
// =============================================================================

// Campaign DTOs
#[derive(Debug, Deserialize)]
pub struct CreateCampaignRequest {
    pub name: String,
    pub description: String,
    pub campaign_type: String, // "nft_boost", "promotion", "contest", "airdrop"
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub target_audience: TargetAudience,
    pub budget: f64,
    pub currency: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub campaign_parameters: CampaignParameters,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TargetAudience {
    pub age_range: Option<AgeRange>,
    pub locations: Vec<String>,
    pub genres: Vec<String>,
    pub fan_level: Option<String>, // "new", "casual", "dedicated", "superfan"
    pub platform_activity: Option<String>, // "high", "medium", "low"
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AgeRange {
    pub min_age: u8,
    pub max_age: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CampaignParameters {
    pub boost_multiplier: Option<f64>,
    pub max_participants: Option<u32>,
    pub reward_per_action: Option<f64>,
    pub required_actions: Vec<String>, // "listen", "share", "follow", "playlist_add"
    pub nft_collection_size: Option<u32>,
    pub nft_metadata_url: Option<String>,
    pub minimum_listen_duration: Option<u32>, // seconds
}

#[derive(Debug, Serialize)]
pub struct CreateCampaignResponse {
    pub campaign_id: Uuid,
    pub name: String,
    pub campaign_type: String,
    pub status: String,
    pub budget: f64,
    pub estimated_reach: u32,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCampaignRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub budget: Option<f64>,
    pub end_date: Option<DateTime<Utc>>,
    pub target_audience: Option<TargetAudience>,
    pub campaign_parameters: Option<CampaignParameters>,
}

#[derive(Debug, Deserialize)]
pub struct ParticipateCampaignRequest {
    pub action_type: String, // "listen", "share", "follow", "playlist_add"
    pub action_data: Option<serde_json::Value>,
    pub proof_of_action: Option<String>, // ZK proof or verification data
}

#[derive(Debug, Serialize)]
pub struct ParticipateCampaignResponse {
    pub participation_id: Uuid,
    pub campaign_id: Uuid,
    pub user_id: Uuid,
    pub action_type: String,
    pub reward_earned: f64,
    pub is_eligible_for_nft: bool,
    pub total_actions: u32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct BoostCampaignRequest {
    pub boost_amount: f64,
    pub boost_duration_hours: u32,
    pub target_metrics: Vec<String>, // "reach", "engagement", "listens"
}

#[derive(Debug, Serialize)]
pub struct BoostCampaignResponse {
    pub boost_id: Uuid,
    pub campaign_id: Uuid,
    pub boost_amount: f64,
    pub boost_multiplier: f64,
    pub estimated_additional_reach: u32,
    pub boost_start: DateTime<Utc>,
    pub boost_end: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct MintNFTRequest {
    pub recipient_id: Option<Uuid>, // If None, mint to top participants
    pub nft_count: u32,
    pub metadata_override: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct MintNFTResponse {
    pub mint_batch_id: Uuid,
    pub campaign_id: Uuid,
    pub nft_count: u32,
    pub recipients: Vec<NFTRecipient>,
    pub blockchain: String,
    pub transaction_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct NFTRecipient {
    pub user_id: Uuid,
    pub nft_token_id: String,
    pub metadata_url: String,
    pub mint_status: String,
}

// Search DTOs
#[derive(Debug, Deserialize)]
pub struct SearchCampaignsRequest {
    pub search_text: Option<String>,
    pub campaign_type: Option<String>,
    pub artist_id: Option<Uuid>,
    pub status: Option<String>,
    pub min_budget: Option<f64>,
    pub max_budget: Option<f64>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub is_active: Option<bool>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub sort_by: Option<String>, // "created_at", "budget", "participants", "performance"
    pub sort_order: Option<String>, // "asc", "desc"
}

// Analytics DTOs
#[derive(Debug, Serialize)]
pub struct CampaignAnalytics {
    pub campaign_id: Uuid,
    pub performance_metrics: PerformanceMetrics,
    pub audience_insights: AudienceInsights,
    pub engagement_data: EngagementData,
    pub conversion_funnel: ConversionFunnel,
    pub roi_analysis: ROIAnalysis,
    pub time_series_data: Vec<TimeSeriesDataPoint>,
}

#[derive(Debug, Serialize)]
pub struct PerformanceMetrics {
    pub total_reach: u32,
    pub unique_participants: u32,
    pub total_actions: u32,
    pub completion_rate: f64,
    pub engagement_rate: f64,
    pub viral_coefficient: f64,
    pub cost_per_action: f64,
    pub budget_utilization: f64,
}

#[derive(Debug, Serialize)]
pub struct AudienceInsights {
    pub age_distribution: std::collections::HashMap<String, u32>,
    pub location_distribution: std::collections::HashMap<String, u32>,
    pub genre_preferences: std::collections::HashMap<String, u32>,
    pub platform_activity: std::collections::HashMap<String, u32>,
    pub new_vs_returning: NewVsReturning,
}

#[derive(Debug, Serialize)]
pub struct NewVsReturning {
    pub new_users: u32,
    pub returning_users: u32,
    pub percentage_new: f64,
}

#[derive(Debug, Serialize)]
pub struct EngagementData {
    pub actions_breakdown: std::collections::HashMap<String, u32>,
    pub average_session_duration: f64,
    pub repeat_action_rate: f64,
    pub social_sharing_rate: f64,
    pub playlist_addition_rate: f64,
}

#[derive(Debug, Serialize)]
pub struct ConversionFunnel {
    pub impressions: u32,
    pub clicks: u32,
    pub participations: u32,
    pub completions: u32,
    pub conversions: u32,
    pub click_through_rate: f64,
    pub participation_rate: f64,
    pub completion_rate: f64,
    pub conversion_rate: f64,
}

#[derive(Debug, Serialize)]
pub struct ROIAnalysis {
    pub total_spend: f64,
    pub revenue_generated: f64,
    pub roi_percentage: f64,
    pub cost_per_acquisition: f64,
    pub lifetime_value_increase: f64,
    pub break_even_point: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct TimeSeriesDataPoint {
    pub timestamp: DateTime<Utc>,
    pub reach: u32,
    pub participants: u32,
    pub actions: u32,
    pub spend: f64,
    pub engagement_score: f64,
}

// API Response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: Utc::now(),
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            timestamp: Utc::now(),
        }
    }
}

// =============================================================================
// CAMPAIGN CONTROLLER
// =============================================================================

pub struct CampaignController {
    campaign_repository: Arc<PostgresCampaignRepository>,
    participation_repository: Arc<PostgresCampaignParticipationRepository>,
}

impl CampaignController {
    pub fn new(
        campaign_repository: Arc<PostgresCampaignRepository>,
        participation_repository: Arc<PostgresCampaignParticipationRepository>,
    ) -> Self {
        Self {
            campaign_repository,
            participation_repository,
        }
    }

    pub fn routes(controller: Arc<Self>) -> Router {
        Router::new()
            // Campaign CRUD
            .route("/campaigns", get(Self::search_campaigns).post(Self::create_campaign))
            .route("/campaigns/:campaign_id", get(Self::get_campaign).put(Self::update_campaign).delete(Self::delete_campaign))
            
            // Campaign operations
            .route("/campaigns/:campaign_id/activate", post(Self::activate_campaign))
            .route("/campaigns/:campaign_id/participate", post(Self::participate_campaign))
            .route("/campaigns/:campaign_id/boost", post(Self::boost_campaign))
            .route("/campaigns/:campaign_id/nft/mint", post(Self::mint_campaign_nft))
            
            // Campaign analytics
            .route("/campaigns/:campaign_id/analytics", get(Self::get_campaign_analytics))
            .route("/campaigns/:campaign_id/participants", get(Self::get_campaign_participants))
            .route("/campaigns/:campaign_id/leaderboard", get(Self::get_campaign_leaderboard))
            
            // Discovery
            .route("/campaigns/trending", get(Self::get_trending_campaigns))
            .route("/campaigns/featured", get(Self::get_featured_campaigns))
            .route("/campaigns/user/:user_id", get(Self::get_user_campaigns))
            
            // NFT collections
            .route("/campaigns/nft-collections", get(Self::list_nft_collections))
            .route("/campaigns/nft-collections/:collection_id", get(Self::get_nft_collection))
            
            .with_state(controller)
    }

    // =============================================================================
    // CAMPAIGN CRUD
    // =============================================================================

    async fn create_campaign(
        State(controller): State<Arc<Self>>,
        Extension(current_user_id): Extension<Uuid>,
        Json(request): Json<CreateCampaignRequest>,
    ) -> Result<Json<ApiResponse<CreateCampaignResponse>>, StatusCode> {
        let command = CreateCampaignCommand {
            name: request.name,
            description: request.description,
            campaign_type: request.campaign_type,
            song_id: request.song_id,
            artist_id: request.artist_id,
            target_audience: request.target_audience,
            budget: request.budget,
            currency: request.currency,
            start_date: request.start_date,
            end_date: request.end_date,
            campaign_parameters: request.campaign_parameters,
            metadata: request.metadata,
            created_by: current_user_id,
        };

        let handler = CreateCampaignCommandHandler::new(controller.campaign_repository.clone());

        match handler.handle(command).await {
            Ok(result) => {
                let response = CreateCampaignResponse {
                    campaign_id: result.campaign_id,
                    name: result.name,
                    campaign_type: result.campaign_type,
                    status: result.status,
                    budget: result.budget,
                    estimated_reach: result.estimated_reach,
                    start_date: result.start_date,
                    end_date: result.end_date,
                    created_at: result.created_at,
                };
                Ok(Json(ApiResponse::success(response)))
            }
            Err(err) => {
                eprintln!("Create campaign error: {:?}", err);
                match err {
                    AppError::ValidationError(_) => Err(StatusCode::BAD_REQUEST),
                    AppError::InsufficientFundsError(_) => Err(StatusCode::PAYMENT_REQUIRED),
                    AppError::UnauthorizedError(_) => Err(StatusCode::FORBIDDEN),
                    _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            }
        }
    }

    async fn get_campaign(
        State(controller): State<Arc<Self>>,
        Path(campaign_id): Path<Uuid>,
    ) -> Result<Json<ApiResponse<CampaignDetailDTO>>, StatusCode> {
        let query = GetCampaignQuery { campaign_id };
        let handler = GetCampaignQueryHandler::new(controller.campaign_repository.clone());

        match handler.handle(query).await {
            Ok(Some(campaign)) => Ok(Json(ApiResponse::success(campaign))),
            Ok(None) => Err(StatusCode::NOT_FOUND),
            Err(err) => {
                eprintln!("Get campaign error: {:?}", err);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    async fn search_campaigns(
        State(controller): State<Arc<Self>>,
        Query(params): Query<SearchCampaignsRequest>,
    ) -> Result<Json<ApiResponse<SearchCampaignsResult>>, StatusCode> {
        let query = SearchCampaignsQuery {
            search_text: params.search_text,
            campaign_type: params.campaign_type,
            artist_id: params.artist_id,
            status: params.status,
            min_budget: params.min_budget,
            max_budget: params.max_budget,
            date_from: params.date_from,
            date_to: params.date_to,
            is_active: params.is_active,
            limit: params.limit,
            offset: params.offset,
            sort_by: params.sort_by,
            sort_order: params.sort_order,
        };

        let handler = SearchCampaignsQueryHandler::new(controller.campaign_repository.clone());

        match handler.handle(query).await {
            Ok(result) => Ok(Json(ApiResponse::success(result))),
            Err(err) => {
                eprintln!("Search campaigns error: {:?}", err);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    async fn update_campaign(
        State(controller): State<Arc<Self>>,
        Path(campaign_id): Path<Uuid>,
        Extension(current_user_id): Extension<Uuid>,
        Json(request): Json<UpdateCampaignRequest>,
    ) -> Result<Json<ApiResponse<CampaignDetailDTO>>, StatusCode> {
        let command = UpdateCampaignCommand {
            campaign_id,
            name: request.name,
            description: request.description,
            budget: request.budget,
            end_date: request.end_date,
            target_audience: request.target_audience,
            campaign_parameters: request.campaign_parameters,
            updated_by: current_user_id,
        };

        let handler = UpdateCampaignCommandHandler::new(controller.campaign_repository.clone());

        match handler.handle(command).await {
            Ok(campaign) => Ok(Json(ApiResponse::success(campaign))),
            Err(err) => {
                eprintln!("Update campaign error: {:?}", err);
                match err {
                    AppError::NotFoundError(_) => Err(StatusCode::NOT_FOUND),
                    AppError::ValidationError(_) => Err(StatusCode::BAD_REQUEST),
                    AppError::UnauthorizedError(_) => Err(StatusCode::FORBIDDEN),
                    _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            }
        }
    }

    async fn delete_campaign(
        State(_controller): State<Arc<Self>>,
        Path(campaign_id): Path<Uuid>,
        Extension(current_user_id): Extension<Uuid>,
    ) -> Result<Json<ApiResponse<()>>, StatusCode> {
        // Delete campaign logic would be implemented here
        Ok(Json(ApiResponse::success(())))
    }

    // =============================================================================
    // CAMPAIGN OPERATIONS
    // =============================================================================

    async fn activate_campaign(
        State(controller): State<Arc<Self>>,
        Path(campaign_id): Path<Uuid>,
        Extension(current_user_id): Extension<Uuid>,
    ) -> Result<Json<ApiResponse<CampaignDetailDTO>>, StatusCode> {
        let command = ActivateCampaignCommand {
            campaign_id,
            activated_by: current_user_id,
        };

        let handler = ActivateCampaignCommandHandler::new(controller.campaign_repository.clone());

        match handler.handle(command).await {
            Ok(campaign) => Ok(Json(ApiResponse::success(campaign))),
            Err(err) => {
                eprintln!("Activate campaign error: {:?}", err);
                match err {
                    AppError::NotFoundError(_) => Err(StatusCode::NOT_FOUND),
                    AppError::ValidationError(_) => Err(StatusCode::BAD_REQUEST),
                    AppError::UnauthorizedError(_) => Err(StatusCode::FORBIDDEN),
                    _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            }
        }
    }

    async fn participate_campaign(
        State(controller): State<Arc<Self>>,
        Path(campaign_id): Path<Uuid>,
        Extension(current_user_id): Extension<Uuid>,
        Json(request): Json<ParticipateCampaignRequest>,
    ) -> Result<Json<ApiResponse<ParticipateCampaignResponse>>, StatusCode> {
        let command = ParticipateCampaignCommand {
            campaign_id,
            user_id: current_user_id,
            action_type: request.action_type,
            action_data: request.action_data,
            proof_of_action: request.proof_of_action,
        };

        let handler = ParticipateCampaignCommandHandler::new(controller.participation_repository.clone());

        match handler.handle(command).await {
            Ok(result) => {
                let response = ParticipateCampaignResponse {
                    participation_id: result.participation_id,
                    campaign_id: result.campaign_id,
                    user_id: result.user_id,
                    action_type: result.action_type,
                    reward_earned: result.reward_earned,
                    is_eligible_for_nft: result.is_eligible_for_nft,
                    total_actions: result.total_actions,
                    created_at: result.created_at,
                };
                Ok(Json(ApiResponse::success(response)))
            }
            Err(err) => {
                eprintln!("Participate campaign error: {:?}", err);
                match err {
                    AppError::NotFoundError(_) => Err(StatusCode::NOT_FOUND),
                    AppError::ValidationError(_) => Err(StatusCode::BAD_REQUEST),
                    AppError::ConflictError(_) => Err(StatusCode::CONFLICT),
                    _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            }
        }
    }

    async fn boost_campaign(
        State(controller): State<Arc<Self>>,
        Path(campaign_id): Path<Uuid>,
        Extension(current_user_id): Extension<Uuid>,
        Json(request): Json<BoostCampaignRequest>,
    ) -> Result<Json<ApiResponse<BoostCampaignResponse>>, StatusCode> {
        let command = BoostCampaignCommand {
            campaign_id,
            boost_amount: request.boost_amount,
            boost_duration_hours: request.boost_duration_hours,
            target_metrics: request.target_metrics,
            boosted_by: current_user_id,
        };

        let handler = BoostCampaignCommandHandler::new(controller.campaign_repository.clone());

        match handler.handle(command).await {
            Ok(result) => {
                let response = BoostCampaignResponse {
                    boost_id: result.boost_id,
                    campaign_id: result.campaign_id,
                    boost_amount: result.boost_amount,
                    boost_multiplier: result.boost_multiplier,
                    estimated_additional_reach: result.estimated_additional_reach,
                    boost_start: result.boost_start,
                    boost_end: result.boost_end,
                };
                Ok(Json(ApiResponse::success(response)))
            }
            Err(err) => {
                eprintln!("Boost campaign error: {:?}", err);
                match err {
                    AppError::InsufficientFundsError(_) => Err(StatusCode::PAYMENT_REQUIRED),
                    AppError::ValidationError(_) => Err(StatusCode::BAD_REQUEST),
                    _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            }
        }
    }

    async fn mint_campaign_nft(
        State(controller): State<Arc<Self>>,
        Path(campaign_id): Path<Uuid>,
        Extension(current_user_id): Extension<Uuid>,
        Json(request): Json<MintNFTRequest>,
    ) -> Result<Json<ApiResponse<MintNFTResponse>>, StatusCode> {
        let command = MintCampaignNFTCommand {
            campaign_id,
            recipient_id: request.recipient_id,
            nft_count: request.nft_count,
            metadata_override: request.metadata_override,
            minted_by: current_user_id,
        };

        let handler = MintCampaignNFTCommandHandler::new(controller.campaign_repository.clone());

        match handler.handle(command).await {
            Ok(result) => {
                let response = MintNFTResponse {
                    mint_batch_id: result.mint_batch_id,
                    campaign_id: result.campaign_id,
                    nft_count: result.nft_count,
                    recipients: result.recipients,
                    blockchain: result.blockchain,
                    transaction_hash: result.transaction_hash,
                    created_at: result.created_at,
                };
                Ok(Json(ApiResponse::success(response)))
            }
            Err(err) => {
                eprintln!("Mint campaign NFT error: {:?}", err);
                match err {
                    AppError::BlockchainError(_) => Err(StatusCode::BAD_GATEWAY),
                    AppError::ValidationError(_) => Err(StatusCode::BAD_REQUEST),
                    _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
                }
            }
        }
    }

    // =============================================================================
    // CAMPAIGN ANALYTICS
    // =============================================================================

    async fn get_campaign_analytics(
        State(controller): State<Arc<Self>>,
        Path(campaign_id): Path<Uuid>,
        Query(params): Query<std::collections::HashMap<String, String>>,
    ) -> Result<Json<ApiResponse<CampaignAnalytics>>, StatusCode> {
        let query = GetCampaignAnalyticsQuery {
            campaign_id,
            time_range: params.get("time_range").cloned(),
            metrics: params.get("metrics").map(|m| m.split(',').map(|s| s.to_string()).collect()),
        };

        let handler = GetCampaignAnalyticsQueryHandler::new(controller.campaign_repository.clone());

        match handler.handle(query).await {
            Ok(analytics) => Ok(Json(ApiResponse::success(analytics))),
            Err(err) => {
                eprintln!("Get campaign analytics error: {:?}", err);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    async fn get_campaign_participants(
        State(_controller): State<Arc<Self>>,
        Path(campaign_id): Path<Uuid>,
        Query(params): Query<std::collections::HashMap<String, String>>,
    ) -> Result<Json<ApiResponse<Vec<ParticipateCampaignResponse>>>, StatusCode> {
        // Get campaign participants with pagination
        Ok(Json(ApiResponse::success(vec![])))
    }

    async fn get_campaign_leaderboard(
        State(_controller): State<Arc<Self>>,
        Path(campaign_id): Path<Uuid>,
        Query(params): Query<std::collections::HashMap<String, String>>,
    ) -> Result<Json<ApiResponse<Vec<ParticipateCampaignResponse>>>, StatusCode> {
        // Get campaign leaderboard
        Ok(Json(ApiResponse::success(vec![])))
    }

    // =============================================================================
    // DISCOVERY
    // =============================================================================

    async fn get_trending_campaigns(
        State(controller): State<Arc<Self>>,
        Query(params): Query<std::collections::HashMap<String, String>>,
    ) -> Result<Json<ApiResponse<SearchCampaignsResult>>, StatusCode> {
        let limit = params.get("limit")
            .and_then(|s| s.parse().ok())
            .unwrap_or(20);

        let query = GetTrendingCampaignsQuery {
            limit: Some(limit),
            campaign_type: params.get("campaign_type").cloned(),
            time_range: params.get("time_range").cloned(),
        };

        // Handler would be implemented
        let result = SearchCampaignsResult {
            campaigns: vec![],
            total_count: 0,
            has_more: false,
        };

        Ok(Json(ApiResponse::success(result)))
    }

    async fn get_featured_campaigns(
        State(_controller): State<Arc<Self>>,
        Query(params): Query<std::collections::HashMap<String, String>>,
    ) -> Result<Json<ApiResponse<SearchCampaignsResult>>, StatusCode> {
        let result = SearchCampaignsResult {
            campaigns: vec![],
            total_count: 0,
            has_more: false,
        };

        Ok(Json(ApiResponse::success(result)))
    }

    async fn get_user_campaigns(
        State(controller): State<Arc<Self>>,
        Path(user_id): Path<Uuid>,
        Query(params): Query<std::collections::HashMap<String, String>>,
    ) -> Result<Json<ApiResponse<SearchCampaignsResult>>, StatusCode> {
        let query = GetUserCampaignsQuery {
            user_id,
            campaign_type: params.get("campaign_type").cloned(),
            status: params.get("status").cloned(),
            limit: params.get("limit").and_then(|s| s.parse().ok()),
            offset: params.get("offset").and_then(|s| s.parse().ok()),
        };

        // Handler would be implemented
        let result = SearchCampaignsResult {
            campaigns: vec![],
            total_count: 0,
            has_more: false,
        };

        Ok(Json(ApiResponse::success(result)))
    }

    // =============================================================================
    // NFT COLLECTIONS
    // =============================================================================

    async fn list_nft_collections(
        State(_controller): State<Arc<Self>>,
        Query(params): Query<std::collections::HashMap<String, String>>,
    ) -> Result<Json<ApiResponse<Vec<String>>>, StatusCode> {
        // List NFT collections from campaigns
        let collections = vec![
            "VibeStream Genesis Collection".to_string(),
            "Artist Spotlight Series".to_string(),
            "Fan Rewards Collection".to_string(),
        ];

        Ok(Json(ApiResponse::success(collections)))
    }

    async fn get_nft_collection(
        State(_controller): State<Arc<Self>>,
        Path(collection_id): Path<String>,
    ) -> Result<Json<ApiResponse<()>>, StatusCode> {
        // Get NFT collection details
        Ok(Json(ApiResponse::success(())))
    }
}

// Factory functions
pub fn create_campaign_controller(
    campaign_repository: Arc<PostgresCampaignRepository>,
    participation_repository: Arc<PostgresCampaignParticipationRepository>,
) -> Arc<CampaignController> {
    Arc::new(CampaignController::new(
        campaign_repository,
        participation_repository,
    ))
}

pub fn create_campaign_routes(
    campaign_repository: Arc<PostgresCampaignRepository>,
    participation_repository: Arc<PostgresCampaignParticipationRepository>,
) -> Router {
    let controller = create_campaign_controller(
        campaign_repository,
        participation_repository,
    );
    
    CampaignController::routes(controller)
} 