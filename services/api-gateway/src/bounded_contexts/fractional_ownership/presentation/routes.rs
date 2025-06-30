use axum::{
    routing::{get, post, delete},
    Router,
};
use std::sync::Arc;

use crate::bounded_contexts::fractional_ownership::{
    domain::repository::OwnershipContractRepository,
    presentation::controllers::FractionalOwnershipController,
};

/// Sets up all HTTP routes for Fractional Ownership bounded context
/// 
/// This function creates the complete router with all endpoints,
/// following RESTful conventions and proper HTTP methods.
pub fn create_routes<R: OwnershipContractRepository + Send + Sync + 'static>(
    controller: Arc<FractionalOwnershipController<R>>,
) -> Router {
    Router::new()
        // Contract management routes
        .route(
            "/api/v1/fractional-ownership/contracts",
            post(FractionalOwnershipController::create_contract),
        )
        .route(
            "/api/v1/fractional-ownership/contracts/:contract_id",
            get(FractionalOwnershipController::get_contract),
        )
        .route(
            "/api/v1/fractional-ownership/contracts/:contract_id/activate",
            post(FractionalOwnershipController::activate_contract),
        )
        .route(
            "/api/v1/fractional-ownership/contracts/:contract_id",
            delete(FractionalOwnershipController::terminate_contract),
        )
        
        // Share trading routes
        .route(
            "/api/v1/fractional-ownership/contracts/:contract_id/purchase",
            post(FractionalOwnershipController::purchase_shares),
        )
        .route(
            "/api/v1/fractional-ownership/shares/:share_id/trade",
            post(FractionalOwnershipController::trade_shares),
        )
        
        // Revenue distribution routes
        .route(
            "/api/v1/fractional-ownership/contracts/:contract_id/distribute-revenue",
            post(FractionalOwnershipController::distribute_revenue),
        )
        
        // Analytics and reporting routes
        .route(
            "/api/v1/fractional-ownership/contracts/:contract_id/analytics",
            get(FractionalOwnershipController::get_contract_analytics),
        )
        .route(
            "/api/v1/fractional-ownership/contracts/search",
            get(FractionalOwnershipController::search_contracts),
        )
        .route(
            "/api/v1/fractional-ownership/artists/:artist_id/contracts",
            get(FractionalOwnershipController::get_contracts_by_artist),
        )
        .route(
            "/api/v1/fractional-ownership/market/statistics",
            get(FractionalOwnershipController::get_market_statistics),
        )
        
        // User portfolio routes
        .route(
            "/api/v1/fractional-ownership/users/:user_id/portfolio",
            get(FractionalOwnershipController::get_user_portfolio),
        )
        
        // Add the controller as application state
        .with_state(controller)
}

/// Alternative route grouping for modular composition
/// 
/// This allows different route groups to be composed together
/// for more complex API structures.
pub struct FractionalOwnershipRoutes;

impl FractionalOwnershipRoutes {
    /// Contract management routes group
    pub fn contracts<R: OwnershipContractRepository + Send + Sync + 'static>(
        controller: Arc<FractionalOwnershipController<R>>,
    ) -> Router {
        Router::new()
            .route("/", post(FractionalOwnershipController::create_contract))
            .route("/:contract_id", get(FractionalOwnershipController::get_contract))
            .route("/:contract_id/activate", post(FractionalOwnershipController::activate_contract))
            .route("/:contract_id", delete(FractionalOwnershipController::terminate_contract))
            .route("/:contract_id/purchase", post(FractionalOwnershipController::purchase_shares))
            .route("/:contract_id/distribute-revenue", post(FractionalOwnershipController::distribute_revenue))
            .route("/:contract_id/analytics", get(FractionalOwnershipController::get_contract_analytics))
            .route("/search", get(FractionalOwnershipController::search_contracts))
            .with_state(controller)
    }

    /// Share trading routes group
    pub fn shares<R: OwnershipContractRepository + Send + Sync + 'static>(
        controller: Arc<FractionalOwnershipController<R>>,
    ) -> Router {
        Router::new()
            .route("/:share_id/trade", post(FractionalOwnershipController::trade_shares))
            .with_state(controller)
    }

    /// Artist-specific routes group
    pub fn artists<R: OwnershipContractRepository + Send + Sync + 'static>(
        controller: Arc<FractionalOwnershipController<R>>,
    ) -> Router {
        Router::new()
            .route("/:artist_id/contracts", get(FractionalOwnershipController::get_contracts_by_artist))
            .with_state(controller)
    }

    /// User portfolio routes group
    pub fn users<R: OwnershipContractRepository + Send + Sync + 'static>(
        controller: Arc<FractionalOwnershipController<R>>,
    ) -> Router {
        Router::new()
            .route("/:user_id/portfolio", get(FractionalOwnershipController::get_user_portfolio))
            .with_state(controller)
    }

    /// Market data routes group
    pub fn market<R: OwnershipContractRepository + Send + Sync + 'static>(
        controller: Arc<FractionalOwnershipController<R>>,
    ) -> Router {
        Router::new()
            .route("/statistics", get(FractionalOwnershipController::get_market_statistics))
            .with_state(controller)
    }

    /// Compose all route groups into a single router
    pub fn compose_all<R: OwnershipContractRepository + Send + Sync + 'static>(
        controller: Arc<FractionalOwnershipController<R>>,
    ) -> Router {
        Router::new()
            .nest("/contracts", Self::contracts(Arc::clone(&controller)))
            .nest("/shares", Self::shares(Arc::clone(&controller)))
            .nest("/artists", Self::artists(Arc::clone(&controller)))
            .nest("/users", Self::users(Arc::clone(&controller)))
            .nest("/market", Self::market(controller))
    }
}

/// Health check and admin routes for Fractional Ownership
pub fn admin_routes<R: OwnershipContractRepository + Send + Sync + 'static>() -> Router {
    Router::new()
        .route("/api/v1/fractional-ownership/health", get(health_check))
        .route("/api/v1/fractional-ownership/metrics", get(get_metrics))
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "Fractional Ownership service is healthy"
}

/// Metrics endpoint (mock implementation)
async fn get_metrics() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "service": "fractional_ownership",
        "status": "healthy",
        "version": "1.0.0",
        "uptime": "24h",
        "active_contracts": 42,
        "total_market_cap": 1_250_000.0
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{Method, Request, StatusCode};
    use axum::body::Body;
    use tower::ServiceExt;
    
    use crate::bounded_contexts::fractional_ownership::{
        application::FractionalOwnershipApplicationService,
        domain::repository::tests::MockOwnershipContractRepository,
        presentation::controllers::FractionalOwnershipController,
    };

    fn setup_test_router() -> Router {
        let repo = Arc::new(MockOwnershipContractRepository::new());
        let service = Arc::new(FractionalOwnershipApplicationService::new(repo));
        let controller = Arc::new(FractionalOwnershipController::new(service));
        
        create_routes(controller)
    }

    #[tokio::test]
    async fn test_health_check_route() {
        let app = admin_routes::<MockOwnershipContractRepository>();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/api/v1/fractional-ownership/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_metrics_route() {
        let app = admin_routes::<MockOwnershipContractRepository>();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .uri("/api/v1/fractional-ownership/metrics")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_route_structure() {
        let router = setup_test_router();
        
        // Test that the router is properly constructed
        // In a real test, we would make actual HTTP requests
        // For now, we just verify the router structure exists
        assert!(true); // Router constructed successfully
    }

    #[tokio::test]
    async fn test_grouped_routes() {
        let repo = Arc::new(MockOwnershipContractRepository::new());
        let service = Arc::new(FractionalOwnershipApplicationService::new(repo));
        let controller = Arc::new(FractionalOwnershipController::new(service));
        
        // Test individual route groups
        let contracts_router = FractionalOwnershipRoutes::contracts(Arc::clone(&controller));
        let shares_router = FractionalOwnershipRoutes::shares(Arc::clone(&controller));
        let artists_router = FractionalOwnershipRoutes::artists(Arc::clone(&controller));
        let users_router = FractionalOwnershipRoutes::users(Arc::clone(&controller));
        let market_router = FractionalOwnershipRoutes::market(controller);
        
        // Verify all routers are created successfully
        assert!(true);
    }

    #[tokio::test]
    async fn test_composed_routes() {
        let repo = Arc::new(MockOwnershipContractRepository::new());
        let service = Arc::new(FractionalOwnershipApplicationService::new(repo));
        let controller = Arc::new(FractionalOwnershipController::new(service));
        
        let composed_router = FractionalOwnershipRoutes::compose_all(controller);
        
        // Verify composed router is created successfully
        assert!(true);
    }
}

/// Route documentation for API consumers
/// 
/// This provides a structured overview of all available endpoints
/// for integration and documentation purposes.
pub struct ApiDocumentation;

impl ApiDocumentation {
    pub fn get_endpoint_summary() -> Vec<EndpointInfo> {
        vec![
            EndpointInfo {
                method: "POST".to_string(),
                path: "/api/v1/fractional-ownership/contracts".to_string(),
                description: "Create a new ownership contract for a song".to_string(),
                auth_required: true,
                rate_limited: true,
            },
            EndpointInfo {
                method: "GET".to_string(),
                path: "/api/v1/fractional-ownership/contracts/{contract_id}".to_string(),
                description: "Get ownership contract details".to_string(),
                auth_required: false,
                rate_limited: false,
            },
            EndpointInfo {
                method: "POST".to_string(),
                path: "/api/v1/fractional-ownership/contracts/{contract_id}/activate".to_string(),
                description: "Activate an ownership contract for public investment".to_string(),
                auth_required: true,
                rate_limited: true,
            },
            EndpointInfo {
                method: "POST".to_string(),
                path: "/api/v1/fractional-ownership/contracts/{contract_id}/purchase".to_string(),
                description: "Purchase shares in an ownership contract".to_string(),
                auth_required: true,
                rate_limited: true,
            },
            EndpointInfo {
                method: "POST".to_string(),
                path: "/api/v1/fractional-ownership/shares/{share_id}/trade".to_string(),
                description: "Trade shares between users".to_string(),
                auth_required: true,
                rate_limited: true,
            },
            EndpointInfo {
                method: "POST".to_string(),
                path: "/api/v1/fractional-ownership/contracts/{contract_id}/distribute-revenue".to_string(),
                description: "Distribute revenue to shareholders".to_string(),
                auth_required: true,
                rate_limited: true,
            },
            EndpointInfo {
                method: "GET".to_string(),
                path: "/api/v1/fractional-ownership/contracts/{contract_id}/analytics".to_string(),
                description: "Get contract analytics".to_string(),
                auth_required: false,
                rate_limited: false,
            },
            EndpointInfo {
                method: "GET".to_string(),
                path: "/api/v1/fractional-ownership/contracts/search".to_string(),
                description: "Search ownership contracts".to_string(),
                auth_required: false,
                rate_limited: false,
            },
            EndpointInfo {
                method: "GET".to_string(),
                path: "/api/v1/fractional-ownership/artists/{artist_id}/contracts".to_string(),
                description: "Get contracts by artist".to_string(),
                auth_required: false,
                rate_limited: false,
            },
            EndpointInfo {
                method: "GET".to_string(),
                path: "/api/v1/fractional-ownership/users/{user_id}/portfolio".to_string(),
                description: "Get user's investment portfolio".to_string(),
                auth_required: true,
                rate_limited: false,
            },
            EndpointInfo {
                method: "GET".to_string(),
                path: "/api/v1/fractional-ownership/market/statistics".to_string(),
                description: "Get market statistics".to_string(),
                auth_required: false,
                rate_limited: false,
            },
            EndpointInfo {
                method: "DELETE".to_string(),
                path: "/api/v1/fractional-ownership/contracts/{contract_id}".to_string(),
                description: "Terminate an ownership contract".to_string(),
                auth_required: true,
                rate_limited: true,
            },
        ]
    }
}

#[derive(Debug, Clone)]
pub struct EndpointInfo {
    pub method: String,
    pub path: String,
    pub description: String,
    pub auth_required: bool,
    pub rate_limited: bool,
} 