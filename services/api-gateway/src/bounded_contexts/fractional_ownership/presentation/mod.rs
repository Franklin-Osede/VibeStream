pub mod controllers;
pub mod routes;

// Re-export the main components for easy access
pub use controllers::FractionalOwnershipController;
pub use routes::{create_routes, admin_routes, FractionalOwnershipRoutes, ApiDocumentation};

// Re-export key DTOs for external usage
pub use controllers::{
    CreateContractRequest, CreateContractResponse,
    ActivateContractResponse, PurchaseSharesRequest, PurchaseSharesResponse,
    TradeSharesRequest, TradeSharesResponse, DistributeRevenueRequest, DistributeRevenueResponse,
    TerminateContractRequest, TerminateContractResponse, ContractDetailsResponse,
    UserPortfolioResponse, ContractAnalyticsResponse, SearchContractsResponse,
    ArtistContractsResponse, MarketStatisticsResponse, SearchContractsQuery,
};

pub use routes::{EndpointInfo}; 