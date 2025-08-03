// Fan Ventures Controller Functions
//
// Este módulo contiene funciones handler independientes para Axum que manejan
// todas las operaciones HTTP relacionadas con Fan Ventures (anteriormente Fractional Ownership).

use crate::shared::infrastructure::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Request/Response types
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateVentureRequest {
    pub artist_id: Uuid,
    pub title: String,
    pub description: String,
    pub funding_goal: f64,
    pub benefits: Option<Vec<BenefitRequest>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BenefitRequest {
    pub title: String,
    pub description: String,
    pub benefit_type: String,
    pub min_investment: f64,
}

#[derive(Debug, Serialize)]
pub struct VentureResponse {
    pub venture_id: Uuid,
    pub title: String,
    pub description: String,
    pub funding_goal: f64,
    pub current_funding: f64,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub errors: Option<Vec<String>>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
            errors: None,
        }
    }
    
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            message: Some(message),
            errors: None,
        }
    }
}

// Controller functions using unified AppState
pub async fn create_venture(
    State(app_state): State<AppState>,
    Json(request): Json<CreateVentureRequest>,
) -> Result<Json<ApiResponse<VentureResponse>>, StatusCode> {
    let request_json = serde_json::to_value(request)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    match app_state.fan_ventures_service.create_venture(request_json).await {
        Ok(venture) => {
            let response = VentureResponse {
                venture_id: venture.venture_id,
                title: venture.title,
                description: venture.description,
                funding_goal: venture.funding_goal,
                current_funding: venture.current_funding,
                status: format!("{:?}", venture.status),
            };
            Ok(Json(ApiResponse::success(response)))
        }
        Err(e) => {
            Ok(Json(ApiResponse::error(e.to_string())))
        }
    }
}

pub async fn get_ventures(
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<VentureResponse>>>, StatusCode> {
    match app_state.fan_ventures_service.get_all_ventures().await {
        Ok(ventures) => {
            let responses: Vec<VentureResponse> = ventures.into_iter().map(|venture| {
                VentureResponse {
                    venture_id: venture.venture_id,
                    title: venture.title,
                    description: venture.description,
                    funding_goal: venture.funding_goal,
                    current_funding: venture.current_funding,
                    status: format!("{:?}", venture.status),
                }
            }).collect();
            Ok(Json(ApiResponse::success(responses)))
        }
        Err(e) => {
            Ok(Json(ApiResponse::error(e.to_string())))
        }
    }
}

pub async fn get_venture_by_id(
    State(app_state): State<AppState>,
    Path(venture_id): Path<Uuid>,
) -> Result<Json<ApiResponse<VentureResponse>>, StatusCode> {
    match app_state.fan_ventures_service.get_venture_by_id(&venture_id).await {
        Ok(Some(venture)) => {
            let response = VentureResponse {
                venture_id: venture.venture_id,
                title: venture.title,
                description: venture.description,
                funding_goal: venture.funding_goal,
                current_funding: venture.current_funding,
                status: format!("{:?}", venture.status),
            };
            Ok(Json(ApiResponse::success(response)))
        }
        Ok(None) => {
            Ok(Json(ApiResponse::error("Venture not found".to_string())))
        }
        Err(e) => {
            Ok(Json(ApiResponse::error(e.to_string())))
        }
    }
}

pub async fn get_ventures_by_artist(
    State(app_state): State<AppState>,
    Path(artist_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<VentureResponse>>>, StatusCode> {
    match app_state.fan_ventures_service.get_ventures_by_artist(&artist_id).await {
        Ok(ventures) => {
            let responses: Vec<VentureResponse> = ventures.into_iter().map(|venture| {
                VentureResponse {
                    venture_id: venture.venture_id,
                    title: venture.title,
                    description: venture.description,
                    funding_goal: venture.funding_goal,
                    current_funding: venture.current_funding,
                    status: format!("{:?}", venture.status),
                }
            }).collect();
            Ok(Json(ApiResponse::success(responses)))
        }
        Err(e) => {
            Ok(Json(ApiResponse::error(e.to_string())))
        }
    }
}

// Create routes function
pub fn create_fan_ventures_routes() -> axum::Router<AppState> {
    use axum::routing::{get, post};
    
    axum::Router::new()
        .route("/ventures", post(create_venture))
        .route("/ventures", get(get_ventures))
        .route("/ventures/:venture_id", get(get_venture_by_id))
        .route("/artists/:artist_id/ventures", get(get_ventures_by_artist))
}
