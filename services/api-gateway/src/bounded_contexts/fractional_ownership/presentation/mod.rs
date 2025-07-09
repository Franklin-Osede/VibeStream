pub mod controllers;

// Re-export principal: AppState y rutas básicas
pub use controllers::{
    AppState, 
    create_fractional_ownership_routes, 
    create_routes,
    admin_routes,
    ConcreteApplicationService,
    CreateContractRequest,
    CreateContractResponse,
    PurchaseSharesRequest,
    PurchaseSharesResponse,
    ContractDetailsResponse,
    AuthUser,
}; 