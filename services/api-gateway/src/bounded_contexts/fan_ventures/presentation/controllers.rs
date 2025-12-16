// Fan Ventures Controller Functions
//
// Este módulo contiene funciones handler independientes para Axum que manejan
// todas las operaciones HTTP relacionadas con Fan Ventures (anteriormente Fractional Ownership).

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json as ResponseJson,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::shared::infrastructure::app_state::FanVenturesAppState;
use crate::bounded_contexts::orchestrator::DomainEvent;

// =============================================================================
// REQUEST/RESPONSE DTOs
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateVentureRequest {
    pub artist_id: Uuid,
    pub title: String,
    pub description: String,
    pub funding_goal: f64,
    pub equity_percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct VentureResponse {
    pub venture_id: Uuid,
    pub artist_id: Uuid,
    pub title: String,
    pub description: String,
    pub funding_goal: f64,
    pub current_funding: f64,
    pub equity_percentage: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct InvestRequest {
    pub investor_id: Uuid,
    pub amount: f64,
}

// =============================================================================
// FAN VENTURES CONTROLLER
// =============================================================================

pub struct FanVenturesController;

impl FanVenturesController {
    /// POST /api/v1/fan-ventures/ventures - Create a new venture
    pub async fn create_venture(
        State(state): State<FanVenturesAppState>,
        axum::extract::Json(request): axum::extract::Json<CreateVentureRequest>,
    ) -> Result<ResponseJson<VentureResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
        let venture_id = Uuid::new_v4();
        let now = Utc::now();
        
        // Map request to domain entity
        // Note: Using defaults for fields not present in simple request
        let venture = crate::bounded_contexts::fan_ventures::domain::entities::ArtistVenture {
            id: venture_id,
            artist_id: request.artist_id,
            title: request.title.clone(),
            description: Some(request.description.clone()),
            category: crate::bounded_contexts::fan_ventures::domain::entities::VentureCategory::Music, // Default
            tags: vec![],
            risk_level: crate::bounded_contexts::fan_ventures::domain::entities::RiskLevel::Medium, // Default
            expected_return: 0.0, // Default
            artist_rating: 0.0,
            artist_previous_ventures: 0,
            artist_success_rate: 0.0,
            funding_goal: request.funding_goal,
            current_funding: 0.0,
            min_investment: 10.0, // Default min
            max_investment: None,
            status: crate::bounded_contexts::fan_ventures::domain::entities::VentureStatus::Draft,
            start_date: None,
            end_date: None,
            created_at: now,
            updated_at: now,
            benefits: vec![],
        };

        // Persist to DB
        if let Err(e) = state.venture_repository.create_venture(&venture).await {
            tracing::error!("Failed to create venture: {:?}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({"error": "Database error"}))));
        }
        
        // Publish domain event
        let event = DomainEvent::VentureCreated {
            venture_id,
            artist_id: request.artist_id,
            occurred_at: now,
        };
        
        if let Err(e) = state.app_state.publish_event(event).await {
            tracing::warn!("Failed to publish venture created event: {:?}", e);
        }
        
        let response = VentureResponse {
            venture_id,
            artist_id: request.artist_id,
            title: request.title,
            description: request.description,
            funding_goal: request.funding_goal,
            current_funding: 0.0,
            equity_percentage: request.equity_percentage,
            status: "Draft".to_string(),
            created_at: now,
            updated_at: now,
        };
        
        Ok(ResponseJson(response))
    }
    
    /// GET /api/v1/fan-ventures/ventures - List ventures
    pub async fn get_ventures(
        State(state): State<FanVenturesAppState>,
    ) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
        match state.venture_repository.list_open_ventures(None).await {
            Ok(ventures) => {
                // Map to simpler response if needed, or return as is
                let total = ventures.len();
                Ok(ResponseJson(serde_json::json!({
                    "ventures": ventures,
                    "total": total
                })))
            },
            Err(e) => {
                tracing::error!("Failed to list ventures: {:?}", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({"error": "Database error"}))))
            }
        }
    }
    
    /// GET /api/v1/fan-ventures/ventures/:id - Get venture by ID
    pub async fn get_venture(
        State(state): State<FanVenturesAppState>,
        Path(venture_id): Path<Uuid>,
    ) -> Result<ResponseJson<VentureResponse>, (StatusCode, ResponseJson<serde_json::Value>)> {
        match state.venture_repository.get_venture(venture_id).await {
            Ok(Some(venture)) => {
                let response = VentureResponse {
                    venture_id: venture.id,
                    artist_id: venture.artist_id,
                    title: venture.title,
                    description: venture.description.unwrap_or_default(),
                    funding_goal: venture.funding_goal,
                    current_funding: venture.current_funding,
                    equity_percentage: 0.0, // TODO: Store this in DB or calculate
                    status: venture.status.to_string(),
                    created_at: venture.created_at,
                    updated_at: venture.updated_at,
                };
                Ok(ResponseJson(response))
            },
            Ok(None) => Err((StatusCode::NOT_FOUND, ResponseJson(serde_json::json!({"error": "Venture not found"})))),
            Err(e) => {
                tracing::error!("Failed to get venture: {:?}", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({"error": "Database error"}))))
            }
        }
    }
    
    /// POST /api/v1/fan-ventures/ventures/:id/invest - Invest in a venture
    pub async fn invest_in_venture(
        State(state): State<FanVenturesAppState>,
        Path(venture_id): Path<Uuid>,
        axum::extract::Json(request): axum::extract::Json<InvestRequest>,
    ) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
        let investment_id = Uuid::new_v4();
        let now = Utc::now();

        let investment = crate::bounded_contexts::fan_ventures::domain::entities::FanInvestment::new(
            investment_id,
            request.investor_id,
            venture_id,
            request.amount,
            crate::bounded_contexts::fan_ventures::domain::entities::InvestmentType::RevenueShare, // Default
            crate::bounded_contexts::fan_ventures::domain::entities::InvestmentStatus::Pending,
        );

        if let Err(e) = state.venture_repository.create_fan_investment(&investment).await {
             tracing::error!("Failed to create investment: {:?}", e);
             return Err((StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({"error": "Database error"}))));
        }
        
        // Publish domain event
        let event = DomainEvent::InvestmentMade {
            venture_id,
            investor_id: request.investor_id,
            amount: request.amount,
            occurred_at: now,
        };
        
        if let Err(e) = state.app_state.publish_event(event).await {
            tracing::warn!("Failed to publish investment made event: {:?}", e);
        }
        
        Ok(ResponseJson(serde_json::json!({
            "message": "Investment successful",
            "venture_id": venture_id,
            "investor_id": request.investor_id,
            "amount": request.amount,
            "investment_id": investment_id
        })))
    }
    
    /// GET /api/v1/fan-ventures/ventures/:id/benefits - Get venture benefits
    pub async fn get_venture_benefits(
        State(state): State<FanVenturesAppState>,
        Path(venture_id): Path<Uuid>,
    ) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // Using get_venture_tiers as proxy for benefits for now as method get_venture_benefits is not implemented in repo
        match state.venture_repository.get_venture_tiers(venture_id).await {
             Ok(tiers) => Ok(ResponseJson(serde_json::json!({
                "venture_id": venture_id,
                "tiers": tiers
            }))),
             Err(e) => {
                tracing::error!("Failed to get venture benefits: {:?}", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({"error": "Database error"}))))
            }
        }
    }
    
    /// POST /api/v1/fan-ventures/ventures/:id/benefits/:benefit_id/deliver - Deliver benefit
    pub async fn deliver_benefit(
        State(state): State<FanVenturesAppState>,
        Path((venture_id, benefit_id)): Path<(Uuid, Uuid)>,
    ) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // TODO: Implement actual benefit delivery logic via repository
        // For now just partial implementation
        let investor_id = Uuid::new_v4(); // Placeholder
        
        // Publish domain event
        let event = DomainEvent::BenefitDelivered {
            venture_id,
            investor_id,
            benefit_type: "Exclusive Merchandise".to_string(), // Placeholder
            occurred_at: Utc::now(),
        };
        
        if let Err(e) = state.app_state.publish_event(event).await {
            tracing::warn!("Failed to publish benefit delivered event: {:?}", e);
        }
        
        Ok(ResponseJson(serde_json::json!({
            "message": "Benefit delivery initiated",
            "venture_id": venture_id,
            "benefit_id": benefit_id
        })))
    }
    
    /// GET /api/v1/fan-ventures/investments/user/:user_id - Get user investments
    pub async fn get_user_investments(
        State(state): State<FanVenturesAppState>,
        Path(user_id): Path<Uuid>,
    ) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
        // Since get_fan_investments isn't fully implemented in repo (returns empty), we call it anyway for structure
        match state.venture_repository.get_fan_investments(user_id).await {
             Ok(investments) => Ok(ResponseJson(serde_json::json!({
                "user_id": user_id,
                "investments": investments,
                "count": investments.len()
            }))),
             Err(e) => {
                tracing::error!("Failed to get user investments: {:?}", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({"error": "Database error"}))))
            }
        }
    }
    
    /// GET /api/v1/fan-ventures/analytics/venture/:id - Get venture analytics
    pub async fn get_venture_analytics(
        State(state): State<FanVenturesAppState>,
        Path(venture_id): Path<Uuid>,
    ) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
        match state.venture_repository.get_investment_count().await {
             Ok(count) => Ok(ResponseJson(serde_json::json!({
                "venture_id": venture_id,
                "total_investors_system_wide": count,
                "status": "partial_implementation"
            }))),
             Err(e) => {
                tracing::error!("Failed to get analytics: {:?}", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, ResponseJson(serde_json::json!({"error": "Database error"}))))
            }
        }
    }
}
