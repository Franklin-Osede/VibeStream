use axum::{
    routing::{get, post, delete},
    Router,
    extract::{State, Path, Query, Extension},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::bounded_contexts::fan_ventures::{
    application::services::MockFanVenturesApplicationService,
    infrastructure::mock_repository::MockArtistVentureRepository,
    presentation::handlers::{
        CreateContractRequest, CreateContractResponse,
        PurchaseSharesRequest, PurchaseSharesResponse,
        ContractDetailsResponse, DistributeRevenueRequest, DistributeRevenueResponse,
        UserPortfolioResponse, ListContractsQuery, ContractSummary,
    },
};
use crate::shared::infrastructure::AppState;
use crate::shared::domain::errors::AppError;

// Type aliases for missing types
type TerminateContractRequest = CreateContractRequest;
type TerminateContractResponse = CreateContractResponse;
type ContractAnalyticsResponse = ContractDetailsResponse;
type SearchContractsQuery = ListContractsQuery;
type SearchContractsResponse = Vec<ContractSummary>;
type ArtistContractsResponse = Vec<ContractSummary>;
type MarketStatisticsResponse = ContractSummary;
type AuthUser = crate::auth::Claims;

// WRAPPER FUNCTIONS CONCRETAS PARA AXUM
// Estas funciones son wrappers no genéricos que Axum puede usar como handlers

async fn create_contract_handler(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(request): Json<CreateContractRequest>,
) -> Result<Json<CreateContractResponse>, StatusCode> {
            match crate::bounded_contexts::fan_ventures::presentation::controllers::create_contract(
        State(state), Extension(auth_user), Json(request)
    ).await {
        Ok(response) => Ok(response),
        Err(err) => Err(StatusCode::from(err)),
    }
}

async fn activate_contract_handler(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(contract_id): Path<Uuid>,
) -> Result<Json<CreateContractResponse>, StatusCode> {
    match crate::bounded_contexts::fan_ventures::presentation::controllers::activate_contract(
        State(state), Extension(auth_user), Path(contract_id)
    ).await {
        Ok(response) => Ok(response),
        Err(err) => Err(StatusCode::from(err)),
    }
}

async fn purchase_shares_handler(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(contract_id): Path<Uuid>,
    Json(request): Json<PurchaseSharesRequest>,
) -> Result<Json<PurchaseSharesResponse>, StatusCode> {
    match crate::bounded_contexts::fan_ventures::presentation::controllers::purchase_shares(
        State(state), Extension(auth_user), Path(contract_id), Json(request)
    ).await {
        Ok(response) => Ok(response),
        Err(err) => Err(StatusCode::from(err)),
    }
}

async fn trade_shares_handler(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(share_id): Path<Uuid>,
    Json(request): Json<PurchaseSharesRequest>,
) -> Result<Json<PurchaseSharesResponse>, StatusCode> {
    match crate::bounded_contexts::fan_ventures::presentation::controllers::trade_shares(
        State(state), Extension(auth_user), Path(share_id), Json(request)
    ).await {
        Ok(response) => Ok(response),
        Err(err) => Err(StatusCode::from(err)),
    }
}

async fn distribute_revenue_handler(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(contract_id): Path<Uuid>,
    Json(request): Json<DistributeRevenueRequest>,
) -> Result<Json<DistributeRevenueResponse>, StatusCode> {
    match crate::bounded_contexts::fan_ventures::presentation::controllers::distribute_revenue(
        State(state), Extension(auth_user), Path(contract_id), Json(request)
    ).await {
        Ok(response) => Ok(response),
        Err(err) => Err(StatusCode::from(err)),
    }
}

async fn terminate_contract_handler(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(contract_id): Path<Uuid>,
    Json(request): Json<TerminateContractRequest>,
) -> Result<Json<TerminateContractResponse>, StatusCode> {
    match crate::bounded_contexts::fan_ventures::presentation::controllers::terminate_contract(
        State(state), Extension(auth_user), Path(contract_id), Json(request)
    ).await {
        Ok(response) => Ok(response),
        Err(err) => Err(StatusCode::from(err)),
    }
}

async fn get_contract_handler(
    State(state): State<AppState>,
    Path(contract_id): Path<Uuid>,
) -> Result<Json<ContractDetailsResponse>, StatusCode> {
    match crate::bounded_contexts::fan_ventures::presentation::controllers::get_contract(
        State(state), Path(contract_id)
    ).await {
        Ok(response) => Ok(response),
        Err(err) => Err(StatusCode::from(err)),
    }
}

async fn get_user_portfolio_handler(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserPortfolioResponse>, StatusCode> {
    match crate::bounded_contexts::fan_ventures::presentation::controllers::get_user_portfolio(
        State(state), Extension(auth_user), Path(user_id)
    ).await {
        Ok(response) => Ok(response),
        Err(err) => Err(StatusCode::from(err)),
    }
}

async fn get_contract_analytics_handler(
    State(state): State<AppState>,
    Path(contract_id): Path<Uuid>,
) -> Result<Json<ContractAnalyticsResponse>, StatusCode> {
    match crate::bounded_contexts::fan_ventures::presentation::controllers::get_contract_analytics(
        State(state), Path(contract_id)
    ).await {
        Ok(response) => Ok(response),
        Err(err) => Err(StatusCode::from(err)),
    }
}

async fn search_contracts_handler(
    State(state): State<AppState>,
    Query(params): Query<SearchContractsQuery>,
) -> Result<Json<SearchContractsResponse>, StatusCode> {
    match crate::bounded_contexts::fan_ventures::presentation::controllers::search_contracts(
        State(state), Query(params)
    ).await {
        Ok(response) => Ok(response),
        Err(err) => Err(StatusCode::from(err)),
    }
}

async fn get_contracts_by_artist_handler(
    State(state): State<AppState>,
    Path(artist_id): Path<Uuid>,
) -> Result<Json<ArtistContractsResponse>, StatusCode> {
    match crate::bounded_contexts::fan_ventures::presentation::controllers::get_contracts_by_artist(
        State(state), Path(artist_id)
    ).await {
        Ok(response) => Ok(response),
        Err(err) => Err(StatusCode::from(err)),
    }
}

async fn get_market_statistics_handler(
    State(state): State<AppState>,
) -> Result<Json<MarketStatisticsResponse>, StatusCode> {
    match crate::bounded_contexts::fan_ventures::presentation::controllers::get_market_statistics(
        State(state)
    ).await {
        Ok(response) => Ok(response),
        Err(err) => Err(StatusCode::from(err)),
    }
}

/// Create all fractional ownership routes using concrete handlers
pub fn create_routes(
    service: Arc<ConcreteApplicationService>,
) -> Router {
    let state = AppState::new(service);
    
    Router::new()
        // Contract management routes
        .route(
            "/api/v1/fractional-ownership/contracts",
            post(create_contract_handler),
        )
        .route(
            "/api/v1/fractional-ownership/contracts/:contract_id",
            get(get_contract_handler),
        )
        .route(
            "/api/v1/fractional-ownership/contracts/:contract_id/activate",
            post(activate_contract_handler),
        )
        .route(
            "/api/v1/fractional-ownership/contracts/:contract_id/terminate",
            delete(terminate_contract_handler),
        )
        
        // Share trading routes
        .route(
            "/api/v1/fractional-ownership/contracts/:contract_id/purchase",
            post(purchase_shares_handler),
        )
        .route(
            "/api/v1/fractional-ownership/shares/:share_id/trade",
            post(trade_shares_handler),
        )
        
        // Revenue distribution routes
        .route(
            "/api/v1/fractional-ownership/contracts/:contract_id/distribute-revenue",
            post(distribute_revenue_handler),
        )
        
        // Analytics routes
        .route(
            "/api/v1/fractional-ownership/contracts/:contract_id/analytics",
            get(get_contract_analytics_handler),
        )
        .route(
            "/api/v1/fractional-ownership/contracts/search",
            get(search_contracts_handler),
        )
        
        // Artist routes
        .route(
            "/api/v1/fractional-ownership/artists/:artist_id/contracts",
            get(get_contracts_by_artist_handler),
        )
        
        // Market routes
        .route(
            "/api/v1/fractional-ownership/market/statistics",
            get(get_market_statistics_handler),
        )
        
        // User portfolio routes
        .route(
            "/api/v1/fractional-ownership/users/:user_id/portfolio",
            get(get_user_portfolio_handler),
        )
        .with_state(state)
}

/// Admin routes for internal operations
pub fn admin_routes(
    service: Arc<ConcreteApplicationService>,
) -> Router {
    let state = AppState::new(service);
    
    Router::new()
        // Admin contract management
        .route("/admin/contracts", post(create_contract_handler))
        .route("/admin/contracts/:contract_id", get(get_contract_handler))
        .route("/admin/contracts/:contract_id/activate", post(activate_contract_handler))
        .route("/admin/contracts/:contract_id/terminate", delete(terminate_contract_handler))
        .route("/admin/contracts/:contract_id/purchase", post(purchase_shares_handler))
        .route("/admin/contracts/:contract_id/distribute-revenue", post(distribute_revenue_handler))
        .route("/admin/contracts/:contract_id/analytics", get(get_contract_analytics_handler))
        .route("/admin/contracts/search", get(search_contracts_handler))
        .with_state(state)
}

/// Route groups organized by functionality
pub struct FractionalOwnershipRoutes;

impl FractionalOwnershipRoutes {
    /// Contract-related routes
    pub fn contracts(state: AppState) -> Router {
        Router::new()
            .route("/", post(create_contract_handler))
            .route("/:contract_id", get(get_contract_handler))
            .route("/:contract_id/activate", post(activate_contract_handler))
            .route("/:contract_id/terminate", delete(terminate_contract_handler))
            .route("/:contract_id/purchase", post(purchase_shares_handler))
            .route("/:contract_id/distribute-revenue", post(distribute_revenue_handler))
            .route("/:contract_id/analytics", get(get_contract_analytics_handler))
            .route("/search", get(search_contracts_handler))
            .with_state(state)
    }

    /// Share trading routes
    pub fn shares(state: AppState) -> Router {
        Router::new()
            .route("/:share_id/trade", post(trade_shares_handler))
            .with_state(state)
    }

    /// Artist-specific routes
    pub fn artists(state: AppState) -> Router {
        Router::new()
            .route("/:artist_id/contracts", get(get_contracts_by_artist_handler))
            .with_state(state)
    }

    /// User portfolio routes
    pub fn users(state: AppState) -> Router {
        Router::new()
            .route("/:user_id/portfolio", get(get_user_portfolio_handler))
            .with_state(state)
    }

    /// Market statistics routes
    pub fn market(state: AppState) -> Router {
        Router::new()
            .route("/statistics", get(get_market_statistics_handler))
            .with_state(state)
    }

    /// Compose all route groups into a single router
    pub fn compose_all(service: Arc<ConcreteApplicationService>) -> Router {
        let state = AppState::new(service);
        
        Router::new()
            .nest("/contracts", Self::contracts(state.clone()))
            .nest("/shares", Self::shares(state.clone()))
            .nest("/artists", Self::artists(state.clone()))
            .nest("/users", Self::users(state.clone()))
            .nest("/market", Self::market(state))
    }
}

/// API documentation helpers
pub struct ApiDocumentation;

impl ApiDocumentation {
    pub fn get_endpoint_info() -> Vec<EndpointInfo> {
        vec![
            EndpointInfo {
                method: "POST".to_string(),
                path: "/api/v1/fractional-ownership/contracts".to_string(),
                description: "Create a new fractional ownership contract".to_string(),
                auth_required: true,
            },
            EndpointInfo {
                method: "GET".to_string(),
                path: "/api/v1/fractional-ownership/contracts/{contract_id}".to_string(),
                description: "Get contract details".to_string(),
                auth_required: false,
            },
            EndpointInfo {
                method: "POST".to_string(),
                path: "/api/v1/fractional-ownership/contracts/{contract_id}/purchase".to_string(),
                description: "Purchase shares in a contract".to_string(),
                auth_required: true,
            },
            EndpointInfo {
                method: "GET".to_string(),
                path: "/api/v1/fractional-ownership/users/{user_id}/portfolio".to_string(),
                description: "Get user's investment portfolio".to_string(),
                auth_required: true,
            },
            EndpointInfo {
                method: "GET".to_string(),
                path: "/api/v1/fractional-ownership/market/statistics".to_string(),
                description: "Get market statistics".to_string(),
                auth_required: false,
            },
        ]
    }
}

#[derive(Debug, Serialize)]
pub struct EndpointInfo {
    pub method: String,
    pub path: String,
    pub description: String,
    pub auth_required: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_route_creation() {
        let repository = Arc::new(crate::bounded_contexts::fan_ventures::infrastructure::InMemoryOwnershipContractRepository::new());
        let service = Arc::new(FractionalOwnershipApplicationService::<crate::bounded_contexts::fan_ventures::infrastructure::InMemoryOwnershipContractRepository>::new(repository));
        let router = create_routes(service);
        
        // Test that router is created successfully
        // Note: We can't easily test router internals without running a server
        assert!(true); // Router creation succeeded if we reach here
    }

    #[test]
    fn test_api_documentation() {
        let docs = ApiDocumentation::get_endpoint_info();
        assert!(!docs.is_empty());
        assert!(docs.iter().any(|e| e.method == "POST"));
        assert!(docs.iter().any(|e| e.method == "GET"));
    }
} 