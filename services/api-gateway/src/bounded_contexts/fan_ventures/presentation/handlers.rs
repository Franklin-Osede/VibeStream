use axum::{
    extract::{Path, Query, State, Json},
    http::StatusCode,
    response::Json as ResponseJson,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;

use crate::auth::Claims;
use crate::bounded_contexts::fan_ventures::infrastructure::postgres_repository::PostgresFanVenturesRepository;
use crate::bounded_contexts::fan_ventures::domain::entities::{
    ArtistVenture, FanInvestment, VentureStatus, RevenueDistribution, 
    InvestmentType, VentureCategory, RiskLevel,
    CreateVentureRequest, BenefitDelivery, DeliveryStatus, DeliveryMethod
};

// ====== REQUEST/RESPONSE TYPES ======
// Re-using domain entities where possible or specific DTOs

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVentureResponse {
    pub venture_id: Uuid,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvestRequest {
    pub amount: f64,
    pub investment_type: InvestmentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvestResponse {
    pub investment_id: Uuid,
    pub status: String,
    pub transactions_hash: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DistributeRevenueRequest {
    pub total_revenue: f64,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub artist_share: f64,
    pub fan_share: f64,
    pub platform_fee: f64,
}

#[derive(Debug, Serialize)]
pub struct DistributeRevenueResponse {
    pub success: bool,
    pub distribution_id: Uuid,
    pub distributed_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDeliveryRequest {
    pub status: DeliveryStatus,
    pub tracking_number: Option<String>,
    pub carrier: Option<String>,
    pub notes: Option<String>,
}

// ====== STATE TYPE ======

pub type AppState = crate::shared::infrastructure::app_state::AppState;

// ====== HANDLERS ======

/// POST /api/v1/ventures - Create new venture
pub async fn create_venture_handler(
    State(state): State<AppState>,
    claims: Claims,
    Json(request): Json<CreateVentureRequest>,
) -> Result<ResponseJson<CreateVentureResponse>, StatusCode> {
    // Only artists can create ventures
    if claims.role != "artist" && claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let repo = PostgresFanVenturesRepository::new(state.get_db_pool());
    
    let venture_id = Uuid::new_v4();
    let artist_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::BAD_REQUEST)?;

    let venture = ArtistVenture {
        id: venture_id,
        artist_id,
        title: request.title,
        description: Some(request.description),
        category: VentureCategory::Music, // Default, should come from request
        tags: vec![],
        risk_level: RiskLevel::Medium,
        expected_return: 0.0,
        artist_rating: 0.0,
        artist_previous_ventures: 0,
        artist_success_rate: 0.0,
        funding_goal: request.funding_goal,
        current_funding: 0.0,
        min_investment: request.min_investment,
        max_investment: Some(request.max_investment),
        status: VentureStatus::Draft,
        start_date: None,
        end_date: request.expires_at,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        benefits: vec![],
    };

    repo.create_venture(&venture).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ResponseJson(CreateVentureResponse {
        venture_id,
        status: "draft".to_string(),
        created_at: Utc::now(),
    }))
}

/// POST /api/v1/ventures/{id}/invest - Invest in venture
pub async fn invest_in_venture_handler(
    State(state): State<AppState>,
    Path(venture_id): Path<Uuid>,
    claims: Claims,
    Json(request): Json<InvestRequest>,
) -> Result<ResponseJson<InvestResponse>, StatusCode> {
    let repo = PostgresFanVenturesRepository::new(state.get_db_pool());
    let fan_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::BAD_REQUEST)?;

    // 1. Check if venture exists and is open
    let venture = repo.get_venture(venture_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if venture.status != VentureStatus::Open {
        return Err(StatusCode::BAD_REQUEST);
    }

    // 2. Create Investment Record
    let investment_id = Uuid::new_v4();
    let investment = FanInvestment::new(
        investment_id,
        fan_id,
        venture_id,
        request.amount,
        request.investment_type,
        crate::bounded_contexts::fan_ventures::domain::entities::InvestmentStatus::Pending, // Pending payment confirmation
    );

    repo.create_fan_investment(&investment).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // TODO: Trigger Payment Intent via Payment Service here (or return necessary info for frontend to do it)

    Ok(ResponseJson(InvestResponse {
        investment_id,
        status: "pending".to_string(),
        transactions_hash: None,
    }))
}

/// POST /api/v1/ventures/{id}/revenue - Distribute Revenue
pub async fn distribute_revenue_handler(
    State(state): State<AppState>,
    Path(venture_id): Path<Uuid>,
    claims: Claims,
    Json(request): Json<DistributeRevenueRequest>,
) -> Result<ResponseJson<DistributeRevenueResponse>, StatusCode> {
    // Only admin or artist
    if claims.role != "admin" && claims.role != "artist" {
        return Err(StatusCode::FORBIDDEN);
    }

    let repo = PostgresFanVenturesRepository::new(state.get_db_pool());

    let distribution_id = Uuid::new_v4();
    let distribution = RevenueDistribution {
        id: distribution_id,
        venture_id,
        total_revenue: request.total_revenue,
        artist_share: request.artist_share,
        fan_share: request.fan_share,
        platform_fee: request.platform_fee,
        distributed_at: Utc::now(),
        period_start: request.period_start,
        period_end: request.period_end,
    };

    repo.create_revenue_distribution(&distribution).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // TODO: Trigger Async Job to actually transfer funds/tokens to all investors
    
    Ok(ResponseJson(DistributeRevenueResponse {
        success: true,
        distribution_id,
        distributed_at: Utc::now(),
    }))
}

/// GET /api/v1/ventures/{id}/deliveries - Get deliveries for a venture (Artist view)
pub async fn get_venture_deliveries_handler(
    State(state): State<AppState>,
    Path(venture_id): Path<Uuid>,
    claims: Claims,
) -> Result<ResponseJson<Vec<BenefitDelivery>>, StatusCode> {
    let repo = PostgresFanVenturesRepository::new(state.get_db_pool());
    
    let deliveries = repo.get_venture_deliveries(venture_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
    Ok(ResponseJson(deliveries))
}

/// PUT /api/v1/deliveries/{id} - Update delivery status
pub async fn update_delivery_handler(
    State(state): State<AppState>,
    Path(delivery_id): Path<Uuid>,
    claims: Claims,
    Json(request): Json<UpdateDeliveryRequest>,
) -> Result<StatusCode, StatusCode> {
    let repo = PostgresFanVenturesRepository::new(state.get_db_pool());

    // Fetch existing
    let mut delivery = repo.get_benefit_delivery(delivery_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Update fields
    delivery.delivery_status = request.status;
    if let Some(notes) = request.notes {
        delivery.notes = Some(notes);
    }
    
    // Update tracking info if provided
    if request.tracking_number.is_some() || request.carrier.is_some() {
        let mut tracking = delivery.tracking_info.unwrap_or(crate::bounded_contexts::fan_ventures::domain::entities::TrackingInfo {
            tracking_number: None,
            carrier: None,
            estimated_delivery: None,
            actual_delivery: None,
            delivery_notes: None,
        });
        
        if let Some(tn) = request.tracking_number { tracking.tracking_number = Some(tn); }
        if let Some(c) = request.carrier { tracking.carrier = Some(c); }
        
        delivery.tracking_info = Some(tracking);
    }

    repo.update_benefit_delivery(delivery_id, &delivery).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
 