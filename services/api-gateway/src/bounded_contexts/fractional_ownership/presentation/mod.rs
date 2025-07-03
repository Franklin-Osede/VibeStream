pub mod controllers;
pub mod routes;

// Re-export the main components for easy access
pub use controllers::{AppState, ConcreteApplicationService};
pub use routes::{create_routes, admin_routes, FractionalOwnershipRoutes, ApiDocumentation};

// Re-export key DTOs for external usage
pub use controllers::{
    CreateContractRequest, CreateContractResponse,
    ActivateContractResponse, PurchaseSharesRequest, PurchaseSharesResponse,
    TradeSharesRequest, TradeSharesResponse, DistributeRevenueRequest, DistributeRevenueResponse,
    TerminateContractRequest, TerminateContractResponse, ContractDetailsResponse,
    UserPortfolioResponse, ContractAnalyticsResponse, SearchContractsResponse,
    ArtistContractsResponse, MarketStatisticsResponse, SearchContractsQuery,
    AuthUser,
};

// Re-export handler functions for direct use if needed
pub use controllers::{
    create_contract, activate_contract, purchase_shares, trade_shares,
    distribute_revenue, terminate_contract, get_contract, get_user_portfolio,
    get_contract_analytics, search_contracts, get_contracts_by_artist, get_market_statistics,
};

pub use routes::{EndpointInfo}; 