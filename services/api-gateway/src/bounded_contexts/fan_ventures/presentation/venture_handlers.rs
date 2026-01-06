use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json as ResponseJson,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::auth::Claims;
use crate::shared::infrastructure::app_state::AppState;
use crate::bounded_contexts::fan_ventures::{
    domain::entities::{ArtistVenture, FanInvestment, VentureStatus, InvestmentType, InvestmentStatus},
    infrastructure::{
        postgres_repository::PostgresFanVenturesRepository,
        payment_integration::FanVenturesPaymentIntegration,
        payment_helper::create_payment_command_handler,
    },
};
use crate::shared::domain::errors::AppError;
use crate::openapi::{ApiResponse, ApiError};

// =============================================================================
// REQUEST/RESPONSE TYPES
// =============================================================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateVentureRequest {
    pub title: String,
    pub description: String,
    pub category: Option<String>,
    pub funding_goal: f64,
    pub min_investment: f64,
    pub max_investment: Option<f64>,
    pub end_date: Option<DateTime<Utc>>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateVentureResponse {
    pub venture_id: Uuid,
    pub title: String,
    pub funding_goal: f64,
    pub min_investment: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InvestInVentureRequest {
    pub investment_amount: f64,
    pub investment_type: Option<String>, // "revenue_share", "equity", etc.
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InvestInVentureResponse {
    pub investment_id: Uuid,
    pub venture_id: Uuid,
    pub investment_amount: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub payment_id: Option<Uuid>, // Payment ID for tracking
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct VentureDetailsResponse {
    pub venture_id: Uuid,
    pub artist_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub category: String,
    pub funding_goal: f64,
    pub current_funding: f64,
    pub funding_percentage: f64,
    pub min_investment: f64,
    pub max_investment: Option<f64>,
    pub status: String,
    pub investor_count: u32,
    pub created_at: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserPortfolioResponse {
    pub user_id: Uuid,
    pub total_invested: f64,
    pub active_investments: u32,
    pub completed_investments: u32,
    pub investments: Vec<PortfolioInvestment>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PortfolioInvestment {
    pub investment_id: Uuid,
    pub venture_id: Uuid,
    pub venture_title: String,
    pub investment_amount: f64,
    pub investment_type: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

// =============================================================================
// HANDLERS
// =============================================================================

/// Create a new venture
/// 
/// Allows artists to create a new venture for fan investment.
/// The venture starts in "Draft" status and must be activated to accept investments.
#[utoipa::path(
    post,
    path = "/api/v1/fan-ventures",
    request_body = CreateVentureRequest,
    responses(
        (status = 201, description = "Venture created successfully", body = ApiResponse<CreateVentureResponse>),
        (status = 400, description = "Invalid request data", body = ApiError),
        (status = 403, description = "Forbidden - Only artists and admins can create ventures", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "fan-ventures",
    security(
        ("bearer" = [])
    )
)]
pub async fn create_venture(
    State(state): State<AppState>,
    claims: Claims,
    axum::extract::Json(request): axum::extract::Json<CreateVentureRequest>,
) -> Result<ResponseJson<CreateVentureResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
    // Verify user is artist or admin
    if claims.role != "artist" && claims.role != "admin" {
        return Err((
            StatusCode::FORBIDDEN,
            ResponseJson(serde_json::json!({"error": "Only artists and admins can create ventures"})),
        ));
    }

    // Get artist_id from claims
    let artist_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                ResponseJson(serde_json::json!({"error": "Invalid user ID"})),
            )
        })?;

    // Get repository from state
    let repository = PostgresFanVenturesRepository::new(
        state.get_db_pool().clone()
    );

    // Create venture entity
    let venture = ArtistVenture {
        id: Uuid::new_v4(),
        artist_id,
        title: request.title.clone(),
        description: Some(request.description),
        category: request.category
            .as_ref()
            .and_then(|c| parse_venture_category(c))
            .unwrap_or(crate::bounded_contexts::fan_ventures::domain::entities::VentureCategory::Other),
        tags: request.tags.unwrap_or_default(),
        risk_level: crate::bounded_contexts::fan_ventures::domain::entities::RiskLevel::Medium,
        expected_return: 0.0,
        artist_rating: 0.0,
        artist_previous_ventures: 0,
        artist_success_rate: 0.0,
        funding_goal: request.funding_goal,
        current_funding: 0.0,
        min_investment: request.min_investment,
        max_investment: request.max_investment,
        status: VentureStatus::Draft,
        start_date: None,
        end_date: request.end_date,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        benefits: Vec::new(),
    };

    // Save to repository
    repository.create_venture(&venture).await
        .map_err(|e| {
            tracing::error!("Failed to create venture: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(serde_json::json!({"error": "Failed to create venture"})),
            )
        })?;

    let response = CreateVentureResponse {
        venture_id: venture.id,
        title: venture.title,
        funding_goal: venture.funding_goal,
        min_investment: venture.min_investment,
        status: venture.status.to_string(),
        created_at: venture.created_at,
    };

    tracing::info!("✅ Created venture {} for artist {}", venture.id, artist_id);
    Ok(ResponseJson(ApiResponse::success(response)))
}

/// Get venture details by ID
/// 
/// Returns detailed information about a specific venture including funding progress,
/// investor count, and current status.
#[utoipa::path(
    get,
    path = "/api/v1/fan-ventures/{id}",
    params(
        ("id" = Uuid, Path, description = "Venture ID")
    ),
    responses(
        (status = 200, description = "Venture details", body = ApiResponse<VentureDetailsResponse>),
        (status = 404, description = "Venture not found", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "fan-ventures",
    security(
        ("bearer" = [])
    )
)]
pub async fn get_venture_details(
    State(state): State<AppState>,
    Path(venture_id): Path<Uuid>,
    _claims: Claims,
) -> Result<ResponseJson<VentureDetailsResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
    let repository = PostgresFanVenturesRepository::new(
        state.get_db_pool().clone()
    );

    let venture = repository.get_venture(venture_id).await
        .map_err(|e| {
            tracing::error!("Failed to get venture: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(serde_json::json!({"error": "Failed to get venture"})),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                ResponseJson(serde_json::json!({"error": "Venture not found"})),
            )
        })?;

    // Count investors (simplified - could be optimized)
    let investments = repository.get_fan_investments_by_venture(venture_id).await
        .unwrap_or_default();
    let investor_count = investments.len() as u32;

    let funding_percentage = if venture.funding_goal > 0.0 {
        (venture.current_funding / venture.funding_goal) * 100.0
    } else {
        0.0
    };

    let response = VentureDetailsResponse {
        venture_id: venture.id,
        artist_id: venture.artist_id,
        title: venture.title,
        description: venture.description,
        category: venture.category.to_string(),
        funding_goal: venture.funding_goal,
        current_funding: venture.current_funding,
        funding_percentage,
        min_investment: venture.min_investment,
        max_investment: venture.max_investment,
        status: venture.status.to_string(),
        investor_count,
        created_at: venture.created_at,
        end_date: venture.end_date,
    };

    Ok(ResponseJson(ApiResponse::success(response)))
}

/// Invest in a venture
/// 
/// Allows fans to invest in an open venture. The investment is created with "Pending" status
/// and the venture's current funding is updated immediately.
/// 
/// Note: Full payment integration is pending. Currently funding is updated directly.
#[utoipa::path(
    post,
    path = "/api/v1/fan-ventures/{id}/invest",
    params(
        ("id" = Uuid, Path, description = "Venture ID")
    ),
    request_body = InvestInVentureRequest,
    responses(
        (status = 200, description = "Investment created successfully", body = ApiResponse<InvestInVentureResponse>),
        (status = 400, description = "Invalid investment amount or venture not open", body = ApiError),
        (status = 404, description = "Venture not found", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "fan-ventures",
    security(
        ("bearer" = [])
    )
)]
pub async fn invest_in_venture(
    State(state): State<AppState>,
    Path(venture_id): Path<Uuid>,
    claims: Claims,
    axum::extract::Json(request): axum::extract::Json<InvestInVentureRequest>,
) -> Result<ResponseJson<InvestInVentureResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
    let fan_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                ResponseJson(serde_json::json!({"error": "Invalid user ID"})),
            )
        })?;

    let repository = PostgresFanVenturesRepository::new(
        state.get_db_pool().clone()
    );

    // Get venture to validate
    let venture = repository.get_venture(venture_id).await
        .map_err(|e| {
            tracing::error!("Failed to get venture: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(serde_json::json!({"error": "Failed to get venture"})),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                ResponseJson(serde_json::json!({"error": "Venture not found"})),
            )
        })?;

    // Validate investment
    if venture.status != VentureStatus::Open {
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(serde_json::json!({"error": "Venture is not open for investments"})),
        ));
    }

    if request.investment_amount < venture.min_investment {
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(serde_json::json!({
                "error": format!("Investment amount must be at least ${}", venture.min_investment)
            })),
        ));
    }

    if let Some(max_inv) = venture.max_investment {
        if request.investment_amount > max_inv {
            return Err((
                StatusCode::BAD_REQUEST,
                ResponseJson(serde_json::json!({
                    "error": format!("Investment amount must be at most ${}", max_inv)
                })),
            ));
        }
    }

    // Parse investment type
    let investment_type = request.investment_type
        .as_ref()
        .and_then(|s| parse_investment_type(s))
        .unwrap_or(InvestmentType::RevenueShare);

    // Create investment
    let investment = FanInvestment {
        id: Uuid::new_v4(),
        fan_id,
        venture_id,
        investment_amount: request.investment_amount,
        investment_type,
        status: InvestmentStatus::Pending,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    repository.create_fan_investment(&investment).await
        .map_err(|e| {
            tracing::error!("Failed to create investment: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(serde_json::json!({"error": "Failed to create investment"})),
            )
        })?;

    // Create payment automatically
    let payment_id = {
        let payment_handler = create_payment_command_handler(state.get_db_pool().clone());
        let venture_repo = Arc::new(repository);
        let payment_integration = FanVenturesPaymentIntegration::new(
            payment_handler,
            venture_repo.clone(),
        );

        // Create payment for this investment
        payment_integration.create_investment_payment(
            &investment,
            venture_id,
            venture.artist_id,
        ).await.map_err(|e| {
            tracing::error!("Failed to create payment: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(serde_json::json!({
                    "error": "Failed to create payment",
                    "details": format!("{}", e)
                })),
            )
        })?
    };

    // Note: Funding will be updated automatically when payment is confirmed
    // via the event listener (to be implemented)
    // For now, we keep the investment in "Pending" status until payment confirms

    let response = InvestInVentureResponse {
        investment_id: investment.id,
        venture_id: investment.venture_id,
        investment_amount: investment.investment_amount,
        status: investment.status.to_string(),
        created_at: investment.created_at,
        payment_id: Some(payment_id), // Include payment ID in response
    };

    tracing::info!("✅ User {} invested ${} in venture {}", fan_id, request.investment_amount, venture_id);
    Ok(ResponseJson(ApiResponse::success(response)))
}

/// Get user investment portfolio
/// 
/// Returns all investments made by a specific user across all ventures.
/// Users can only view their own portfolio unless they are admins.
#[utoipa::path(
    get,
    path = "/api/v1/fan-ventures/users/{id}/portfolio",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User portfolio", body = ApiResponse<UserPortfolioResponse>),
        (status = 403, description = "Forbidden - Cannot access other user's portfolio", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "fan-ventures",
    security(
        ("bearer" = [])
    )
)]
pub async fn get_user_portfolio(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    claims: Claims,
) -> Result<ResponseJson<UserPortfolioResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
    // Verify user can access this portfolio
    if claims.sub != user_id.to_string() && claims.role != "admin" {
        return Err((
            StatusCode::FORBIDDEN,
            ResponseJson(serde_json::json!({"error": "Forbidden"})),
        ));
    }

    let repository = PostgresFanVenturesRepository::new(
        state.get_db_pool().clone()
    );

    let investments = repository.get_fan_investments(user_id).await
        .map_err(|e| {
            tracing::error!("Failed to get investments: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(serde_json::json!({"error": "Failed to get investments"})),
            )
        })?;

    let mut portfolio_investments = Vec::new();
    let mut total_invested = 0.0;
    let mut active_count = 0;
    let mut completed_count = 0;

    for investment in investments {
        total_invested += investment.investment_amount;

        if investment.status == InvestmentStatus::Active {
            active_count += 1;
        } else if investment.status == InvestmentStatus::Completed {
            completed_count += 1;
        }

        // Get venture details
        let venture = repository.get_venture(investment.venture_id).await
            .ok()
            .flatten();

        portfolio_investments.push(PortfolioInvestment {
            investment_id: investment.id,
            venture_id: investment.venture_id,
            venture_title: venture.map(|v| v.title).unwrap_or_else(|| "Unknown Venture".to_string()),
            investment_amount: investment.investment_amount,
            investment_type: format!("{:?}", investment.investment_type),
            status: investment.status.to_string(),
            created_at: investment.created_at,
        });
    }

    let response = UserPortfolioResponse {
        user_id,
        total_invested,
        active_investments: active_count,
        completed_investments: completed_count,
        investments: portfolio_investments,
    };

    tracing::info!("📈 Portfolio requested for user {}", user_id);
    Ok(ResponseJson(ApiResponse::success(response)))
}

// =============================================================================
// ADDITIONAL HANDLERS
// =============================================================================

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListVenturesResponse {
    pub ventures: Vec<VentureSummary>,
    pub total: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct VentureSummary {
    pub venture_id: Uuid,
    pub artist_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub category: String,
    pub funding_goal: f64,
    pub current_funding: f64,
    pub funding_percentage: f64,
    pub status: String,
    pub investor_count: u32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateVentureRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub funding_goal: Option<f64>,
    pub min_investment: Option<f64>,
    pub max_investment: Option<f64>,
    pub end_date: Option<DateTime<Utc>>,
    pub tags: Option<Vec<String>>,
    pub status: Option<String>,
}

/// List all open ventures
/// 
/// Returns a paginated list of open ventures available for investment.
#[utoipa::path(
    get,
    path = "/api/v1/fan-ventures",
    params(
        ("limit" = Option<i32>, Query, description = "Maximum number of ventures to return (default: 50)"),
        ("category" = Option<String>, Query, description = "Filter by category"),
        ("status" = Option<String>, Query, description = "Filter by status")
    ),
    responses(
        (status = 200, description = "List of ventures", body = ApiResponse<ListVenturesResponse>),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "fan-ventures",
    security(
        ("bearer" = [])
    )
)]
pub async fn list_ventures(
    State(state): State<AppState>,
    Query(params): Query<serde_json::Value>,
    _claims: Claims,
) -> Result<ResponseJson<ListVenturesResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
    let repository = PostgresFanVenturesRepository::new(
        state.get_db_pool().clone()
    );

    let limit = params.get("limit")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32);

    let ventures = if let Some(category) = params.get("category")
        .and_then(|v| v.as_str()) {
        repository.get_ventures_by_category(category).await
    } else if let Some(status) = params.get("status")
        .and_then(|v| v.as_str()) {
        repository.get_ventures_by_status(status).await
    } else {
        repository.list_open_ventures(limit).await
    }
    .map_err(|e| {
        tracing::error!("Failed to list ventures: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            ResponseJson(serde_json::json!({"error": "Failed to list ventures"})),
        )
    })?;

    let mut venture_summaries = Vec::new();
    for venture in &ventures {
        let investments = repository.get_fan_investments_by_venture(venture.id).await
            .unwrap_or_default();
        let investor_count = investments.len() as u32;

        let funding_percentage = if venture.funding_goal > 0.0 {
            (venture.current_funding / venture.funding_goal) * 100.0
        } else {
            0.0
        };

        venture_summaries.push(VentureSummary {
            venture_id: venture.id,
            artist_id: venture.artist_id,
            title: venture.title.clone(),
            description: venture.description.clone(),
            category: venture.category.to_string(),
            funding_goal: venture.funding_goal,
            current_funding: venture.current_funding,
            funding_percentage,
            status: venture.status.to_string(),
            investor_count,
            created_at: venture.created_at,
        });
    }

    let response = ListVenturesResponse {
        ventures: venture_summaries,
        total: ventures.len() as u32,
    };

    Ok(ResponseJson(ApiResponse::success(response)))
}

/// Update an existing venture
/// 
/// Allows artists to update their venture details. Only the venture owner or admin can update.
#[utoipa::path(
    put,
    path = "/api/v1/fan-ventures/{id}",
    params(
        ("id" = Uuid, Path, description = "Venture ID")
    ),
    request_body = UpdateVentureRequest,
    responses(
        (status = 200, description = "Venture updated successfully", body = ApiResponse<VentureDetailsResponse>),
        (status = 403, description = "Forbidden - Not the venture owner", body = ApiError),
        (status = 404, description = "Venture not found", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "fan-ventures",
    security(
        ("bearer" = [])
    )
)]
pub async fn update_venture(
    State(state): State<AppState>,
    Path(venture_id): Path<Uuid>,
    claims: Claims,
    axum::extract::Json(request): axum::extract::Json<UpdateVentureRequest>,
) -> Result<ResponseJson<VentureDetailsResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
    let repository = PostgresFanVenturesRepository::new(
        state.get_db_pool().clone()
    );

    // Get existing venture
    let mut venture = repository.get_venture(venture_id).await
        .map_err(|e| {
            tracing::error!("Failed to get venture: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(serde_json::json!({"error": "Failed to get venture"})),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                ResponseJson(serde_json::json!({"error": "Venture not found"})),
            )
        })?;

    // Verify ownership (artist or admin)
    if claims.sub != venture.artist_id.to_string() && claims.role != "admin" {
        return Err((
            StatusCode::FORBIDDEN,
            ResponseJson(serde_json::json!({"error": "Forbidden - Not the venture owner"})),
        ));
    }

    // Update fields if provided
    if let Some(title) = request.title {
        venture.title = title;
    }
    if let Some(description) = request.description {
        venture.description = Some(description);
    }
    if let Some(category) = request.category {
        venture.category = category.parse().unwrap_or(venture.category);
    }
    if let Some(funding_goal) = request.funding_goal {
        venture.funding_goal = funding_goal;
    }
    if let Some(min_investment) = request.min_investment {
        venture.min_investment = min_investment;
    }
    if let Some(max_investment) = request.max_investment {
        venture.max_investment = Some(max_investment);
    }
    if let Some(end_date) = request.end_date {
        venture.end_date = Some(end_date);
    }
    if let Some(tags) = request.tags {
        venture.tags = tags;
    }
    if let Some(status) = request.status {
        venture.status = status.parse().unwrap_or(venture.status);
    }

    venture.updated_at = Utc::now();

    // Save updated venture
    repository.update_venture(&venture).await
        .map_err(|e| {
            tracing::error!("Failed to update venture: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(serde_json::json!({"error": "Failed to update venture"})),
            )
        })?;

    // Get updated venture with all details
    let updated_venture = repository.get_venture(venture_id).await
        .map_err(|e| {
            tracing::error!("Failed to get updated venture: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(serde_json::json!({"error": "Failed to get updated venture"})),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                ResponseJson(serde_json::json!({"error": "Venture not found"})),
            )
        })?;

    let investments = repository.get_fan_investments_by_venture(venture_id).await
        .unwrap_or_default();
    let investor_count = investments.len() as u32;

    let funding_percentage = if updated_venture.funding_goal > 0.0 {
        (updated_venture.current_funding / updated_venture.funding_goal) * 100.0
    } else {
        0.0
    };

    let response = VentureDetailsResponse {
        venture_id: updated_venture.id,
        artist_id: updated_venture.artist_id,
        title: updated_venture.title,
        description: updated_venture.description,
        category: updated_venture.category.to_string(),
        funding_goal: updated_venture.funding_goal,
        current_funding: updated_venture.current_funding,
        funding_percentage,
        min_investment: updated_venture.min_investment,
        max_investment: updated_venture.max_investment,
        status: updated_venture.status.to_string(),
        investor_count,
        created_at: updated_venture.created_at,
        end_date: updated_venture.end_date,
    };

    tracing::info!("✅ Updated venture {}", venture_id);
    Ok(ResponseJson(ApiResponse::success(response)))
}

/// Delete (cancel) a venture
/// 
/// Soft deletes a venture by setting its status to 'Cancelled'.
/// Only the venture owner or admin can delete.
#[utoipa::path(
    delete,
    path = "/api/v1/fan-ventures/{id}",
    params(
        ("id" = Uuid, Path, description = "Venture ID")
    ),
    responses(
        (status = 200, description = "Venture deleted successfully", body = ApiResponse<serde_json::Value>),
        (status = 403, description = "Forbidden - Not the venture owner", body = ApiError),
        (status = 404, description = "Venture not found", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "fan-ventures",
    security(
        ("bearer" = [])
    )
)]
pub async fn delete_venture(
    State(state): State<AppState>,
    Path(venture_id): Path<Uuid>,
    claims: Claims,
) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
    let repository = PostgresFanVenturesRepository::new(
        state.get_db_pool().clone()
    );

    // Get venture to verify ownership
    let venture = repository.get_venture(venture_id).await
        .map_err(|e| {
            tracing::error!("Failed to get venture: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(serde_json::json!({"error": "Failed to get venture"})),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                ResponseJson(serde_json::json!({"error": "Venture not found"})),
            )
        })?;

    // Verify ownership (artist or admin)
    if claims.sub != venture.artist_id.to_string() && claims.role != "admin" {
        return Err((
            StatusCode::FORBIDDEN,
            ResponseJson(serde_json::json!({"error": "Forbidden - Not the venture owner"})),
        ));
    }

    // Soft delete (set status to cancelled)
    repository.delete_venture(venture_id).await
        .map_err(|e| {
            tracing::error!("Failed to delete venture: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(serde_json::json!({"error": "Failed to delete venture"})),
            )
        })?;

    tracing::info!("🗑️ Deleted venture {}", venture_id);
    Ok(ResponseJson(ApiResponse::success(serde_json::json!({
        "message": "Venture deleted successfully",
        "venture_id": venture_id
    }))))
}

/// Get all ventures for a specific artist
/// 
/// Returns all ventures created by a specific artist, regardless of status.
#[utoipa::path(
    get,
    path = "/api/v1/fan-ventures/artists/{id}/ventures",
    params(
        ("id" = Uuid, Path, description = "Artist ID")
    ),
    responses(
        (status = 200, description = "List of artist ventures", body = ApiResponse<ListVenturesResponse>),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "fan-ventures",
    security(
        ("bearer" = [])
    )
)]
pub async fn get_artist_ventures(
    State(state): State<AppState>,
    Path(artist_id): Path<Uuid>,
    _claims: Claims,
) -> Result<ResponseJson<ListVenturesResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
    let repository = PostgresFanVenturesRepository::new(
        state.get_db_pool().clone()
    );

    let ventures = repository.get_ventures_by_artist(artist_id).await
        .map_err(|e| {
            tracing::error!("Failed to get artist ventures: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(serde_json::json!({"error": "Failed to get artist ventures"})),
            )
        })?;

    let mut venture_summaries = Vec::new();
    for venture in &ventures {
        let investments = repository.get_fan_investments_by_venture(venture.id).await
            .unwrap_or_default();
        let investor_count = investments.len() as u32;

        let funding_percentage = if venture.funding_goal > 0.0 {
            (venture.current_funding / venture.funding_goal) * 100.0
        } else {
            0.0
        };

        venture_summaries.push(VentureSummary {
            venture_id: venture.id,
            artist_id: venture.artist_id,
            title: venture.title.clone(),
            description: venture.description.clone(),
            category: venture.category.to_string(),
            funding_goal: venture.funding_goal,
            current_funding: venture.current_funding,
            funding_percentage,
            status: venture.status.to_string(),
            investor_count,
            created_at: venture.created_at,
        });
    }

    let response = ListVenturesResponse {
        ventures: venture_summaries,
        total: ventures.len() as u32,
    };

    tracing::info!("📊 Retrieved {} ventures for artist {}", ventures.len(), artist_id);
    Ok(ResponseJson(ApiResponse::success(response)))
}

// =============================================================================
// HELPER FUNCTIONS
// =============================================================================

fn parse_venture_category(s: &str) -> Option<crate::bounded_contexts::fan_ventures::domain::entities::VentureCategory> {
    use crate::bounded_contexts::fan_ventures::domain::entities::VentureCategory;
    match s.to_lowercase().as_str() {
        "music" => Some(VentureCategory::Music),
        "visual_arts" => Some(VentureCategory::VisualArts),
        "film" => Some(VentureCategory::Film),
        "gaming" => Some(VentureCategory::Gaming),
        "technology" => Some(VentureCategory::Technology),
        "fashion" => Some(VentureCategory::Fashion),
        "food" => Some(VentureCategory::Food),
        "travel" => Some(VentureCategory::Travel),
        "education" => Some(VentureCategory::Education),
        "health" => Some(VentureCategory::Health),
        _ => Some(VentureCategory::Other),
    }
}

fn parse_investment_type(s: &str) -> Option<InvestmentType> {
    match s.to_lowercase().as_str() {
        "early_access" => Some(InvestmentType::EarlyAccess),
        "exclusive_content" => Some(InvestmentType::ExclusiveContent),
        "merchandise" => Some(InvestmentType::Merchandise),
        "concert_tickets" => Some(InvestmentType::ConcertTickets),
        "meet_and_greet" => Some(InvestmentType::MeetAndGreet),
        "revenue_share" => Some(InvestmentType::RevenueShare),
        _ => None,
    }
}

