pub mod commands;
pub mod queries;
pub mod services;

// Re-export the main service and facade for easy access
pub use services::{FractionalOwnershipApplicationService, FractionalOwnershipFacade};

// Re-export key command types
pub use commands::{
    CreateOwnershipContract, CreateOwnershipContractResult,
    ActivateOwnershipContract, ActivateOwnershipContractResult,
    PurchaseShares, PurchaseSharesResult,
    TradeShares, TradeSharesResult,
    DistributeRevenue, DistributeRevenueResult,
    TerminateOwnershipContract, TerminateOwnershipContractResult,
};

// Re-export key query types
pub use queries::{
    GetOwnershipContract, GetOwnershipContractResult,
    GetUserPortfolio, GetUserPortfolioResult,
    GetContractAnalytics, GetContractAnalyticsResult,
    SearchOwnershipContracts, SearchOwnershipContractsResult,
    GetContractsByArtist, GetContractsByArtistResult,
    GetMarketStatistics, GetMarketStatisticsResult,
};

// Re-export supporting DTOs
pub use queries::{
    SharePortfolioItem, PortfolioPerformance, ContractActivity,
    ShareholderBreakdown, RevenueDistributionSummary, TrendingContract,
    TopArtist, RecentDistribution, ContractSearchFilters, ContractSearchItem,
    ContractSummary,
};

pub use services::{InvestmentOverview, InvestmentRecommendation, MarketTrends}; 