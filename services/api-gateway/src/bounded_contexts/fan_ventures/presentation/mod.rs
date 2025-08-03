pub mod controllers;
pub mod routes;
pub mod handlers;
pub mod ownership_routes;

// Type alias for compatibility
pub type ConcreteApplicationService = crate::bounded_contexts::fan_ventures::application::services::MockFanVenturesApplicationService;

// Re-export principal: AppState y rutas básicas
pub use controllers::{
    create_fan_ventures_routes, 
};
pub use routes::{
    create_routes,
    admin_routes,
};
pub use handlers::{
    CreateContractRequest,
    CreateContractResponse,
    PurchaseSharesRequest,
    PurchaseSharesResponse,
    ContractDetailsResponse,
}; 