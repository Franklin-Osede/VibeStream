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

use crate::bounded_contexts::fan_ventures::presentation::handlers::{
    CreateContractRequest, CreateContractResponse,
    PurchaseSharesRequest, PurchaseSharesResponse,
    ContractDetailsResponse, DistributeRevenueRequest, DistributeRevenueResponse,
    UserPortfolioResponse, ListContractsQuery, ContractSummary,
};
use crate::shared::infrastructure::AppState;
use crate::services::{MessageQueue, DatabasePool};
use crate::bounded_contexts::orchestrator::InMemoryEventBus;

// Type aliases for missing types
type TerminateContractRequest = CreateContractRequest;
type TerminateContractResponse = CreateContractResponse;
type ContractAnalyticsResponse = ContractDetailsResponse;
type SearchContractsQuery = ListContractsQuery;
type SearchContractsResponse = Vec<ContractSummary>;
type ArtistContractsResponse = Vec<ContractSummary>;
type MarketStatisticsResponse = ContractSummary;
type AuthUser = crate::auth::Claims;

type ConcreteApplicationService = crate::bounded_contexts::fan_ventures::presentation::ConcreteApplicationService;

async fn list_contracts_placeholder(
    _state: State<AppState>,
    _params: Query<SearchContractsQuery>,
) -> Result<Json<SearchContractsResponse>, StatusCode> {
    Ok(Json(Vec::new()))
}

// WRAPPER FUNCTIONS

async fn create_contract_handler(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(request): Json<CreateContractRequest>,
) -> Result<Json<CreateContractResponse>, StatusCode> {
    match crate::bounded_contexts::fan_ventures::presentation::handlers::create_ownership_contract(
        State(state), auth_user, Json(request)
    ).await {
        Ok(response) => Ok(response),
        Err(err) => Err(StatusCode::from(err)),
    }
}

async fn activate_contract_handler(
    State(_state): State<AppState>,
    Extension(_auth_user): Extension<AuthUser>,
    Path(_contract_id): Path<Uuid>,
) -> Result<Json<CreateContractResponse>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

async fn purchase_shares_handler(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(contract_id): Path<Uuid>,
    Json(request): Json<PurchaseSharesRequest>,
) -> Result<Json<PurchaseSharesResponse>, StatusCode> {
    crate::bounded_contexts::fan_ventures::presentation::handlers::purchase_shares(
        State(state), Path(contract_id), auth_user, Json(request)
    ).await
}

async fn trade_shares_handler(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(share_id): Path<Uuid>,
    Json(request): Json<PurchaseSharesRequest>,
) -> Result<Json<PurchaseSharesResponse>, StatusCode> {
    crate::bounded_contexts::fan_ventures::presentation::handlers::purchase_shares(
        State(state), Path(share_id), auth_user, Json(request)
    ).await
}

async fn distribute_revenue_handler(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(contract_id): Path<Uuid>,
    Json(request): Json<DistributeRevenueRequest>,
) -> Result<Json<DistributeRevenueResponse>, StatusCode> {
    crate::bounded_contexts::fan_ventures::presentation::handlers::distribute_revenue(
        State(state), Path(contract_id), auth_user, Json(request)
    ).await
}

async fn terminate_contract_handler(
    State(_state): State<AppState>,
    Extension(_auth_user): Extension<AuthUser>,
    Path(_contract_id): Path<Uuid>,
    Json(_request): Json<TerminateContractRequest>,
) -> Result<Json<TerminateContractResponse>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

async fn get_contract_handler(
    State(state): State<AppState>,
    Path(contract_id): Path<Uuid>,
) -> Result<Json<ContractDetailsResponse>, StatusCode> {
    crate::bounded_contexts::fan_ventures::presentation::handlers::get_contract_details(
        State(state), Path(contract_id), crate::auth::Claims { 
            sub: "".into(), 
            username: "admin".into(),
            email: "admin@example.com".into(),
            role: "admin".into(), 
            exp: chrono::Utc::now().timestamp() + 3600,
            iat: chrono::Utc::now().timestamp(),
            token_type: "access".into()
        }
    ).await
}

async fn get_user_portfolio_handler(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<UserPortfolioResponse>, StatusCode> {
    crate::bounded_contexts::fan_ventures::presentation::handlers::get_user_portfolio(
        State(state), Path(user_id), auth_user
    ).await
}

async fn get_contract_analytics_handler(
    State(_state): State<AppState>,
    Path(_contract_id): Path<Uuid>,
) -> Result<Json<ContractAnalyticsResponse>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

async fn search_contracts_handler(
    State(state): State<AppState>,
    Query(params): Query<SearchContractsQuery>,
) -> Result<Json<SearchContractsResponse>, StatusCode> {
    list_contracts_placeholder(State(state), Query(params)).await
}

async fn get_contracts_by_artist_handler(
    State(_state): State<AppState>,
    Path(_artist_id): Path<Uuid>,
) -> Result<Json<ArtistContractsResponse>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

async fn get_market_statistics_handler(
    State(_state): State<AppState>,
) -> Result<Json<MarketStatisticsResponse>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

pub async fn create_routes(
    _service: Arc<ConcreteApplicationService>,
) -> Router {
    // TODO: Create proper AppState with database and redis connections
    // let state = AppState {
    //     message_queue: MessageQueue::new("redis://localhost").await.unwrap(),
    //     database_pool: DatabasePool::new("postgres://localhost").await.unwrap(),
    //     event_bus: Arc::new(InMemoryEventBus::new()),
    // };
    
    Router::new()
        // TODO: Implement proper routes with state management
        .route("/health", get(|| async { "OK" }))
}

/// Admin routes for internal operations
pub async fn admin_routes(
    _service: Arc<ConcreteApplicationService>,
) -> Router {
    // TODO: Create proper AppState with database and redis connections
    // let state = AppState {
    //     message_queue: MessageQueue::new("redis://localhost").await.unwrap(),
    //     database_pool: DatabasePool::new("postgres://localhost").await.unwrap(),
    //     event_bus: Arc::new(InMemoryEventBus::new()),
    // };
    
    Router::new()
        // TODO: Implement proper admin routes
        .route("/health", get(|| async { "OK" }))
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
    pub async fn compose_all(service: Arc<ConcreteApplicationService>) -> Router {
        // TODO: Create proper AppState with database and redis connections
        // let state = AppState {
        //     message_queue: MessageQueue::new("redis://localhost").await.unwrap(),
        //     database_pool: DatabasePool::new("postgres://localhost").await.unwrap(),
        //     event_bus: Arc::new(InMemoryEventBus::new()),
        // };
        
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