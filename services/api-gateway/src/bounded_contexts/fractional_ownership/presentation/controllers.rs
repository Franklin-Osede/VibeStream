use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    Extension,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::shared::domain::errors::AppError;
use crate::auth::AuthUser;

use crate::bounded_contexts::fractional_ownership::application::{
    FractionalOwnershipApplicationService,
    // Commands
    CreateOwnershipContract, ActivateOwnershipContract, PurchaseShares, TradeShares,
    DistributeRevenue, TerminateOwnershipContract,
    // Queries  
    GetOwnershipContract, GetUserPortfolio, GetContractAnalytics,
    SearchOwnershipContracts, GetContractsByArtist, GetMarketStatistics,
    // DTOs
    ContractSearchFilters,
};
use crate::bounded_contexts::fractional_ownership::domain::repository::OwnershipContractRepository;

/// HTTP Controllers for Fractional Ownership operations
/// 
/// These controllers handle HTTP requests and translate them to application
/// service calls, following REST API conventions and proper error handling.
pub struct FractionalOwnershipController<R: OwnershipContractRepository> {
    service: Arc<FractionalOwnershipApplicationService<R>>,
}

impl<R: OwnershipContractRepository> FractionalOwnershipController<R> {
    pub fn new(service: Arc<FractionalOwnershipApplicationService<R>>) -> Self {
        Self { service }
    }

    /// POST /api/v1/fractional-ownership/contracts
    /// Create a new ownership contract for a song
    pub async fn create_contract(
        State(controller): State<Arc<Self>>,
        Extension(auth_user): Extension<AuthUser>,
        Json(request): Json<CreateContractRequest>,
    ) -> Result<Json<CreateContractResponse>, AppError> {
        let command = CreateOwnershipContract {
            song_id: request.song_id,
            artist_id: auth_user.user_id, // Artist creating the contract
            total_shares: request.total_shares,
            price_per_share: request.price_per_share,
            artist_retained_percentage: request.artist_retained_percentage,
            minimum_investment: request.minimum_investment,
            maximum_ownership_per_user: request.maximum_ownership_per_user,
        };

        let result = controller.service.create_ownership_contract(command).await?;

        Ok(Json(CreateContractResponse {
            contract_id: result.contract_id,
            shares_available_for_sale: result.shares_available_for_sale,
            total_market_cap: result.total_market_cap,
            message: "Ownership contract created successfully".to_string(),
        }))
    }

    /// POST /api/v1/fractional-ownership/contracts/{contract_id}/activate
    /// Activate an ownership contract for public investment
    pub async fn activate_contract(
        State(controller): State<Arc<Self>>,
        Extension(auth_user): Extension<AuthUser>,
        Path(contract_id): Path<Uuid>,
    ) -> Result<Json<ActivateContractResponse>, AppError> {
        // TODO: Add authorization check - only contract owner can activate
        
        let command = ActivateOwnershipContract { contract_id };

        let result = controller.service.activate_ownership_contract(command).await?;

        Ok(Json(ActivateContractResponse {
            contract_id: result.contract_id,
            activated_at: result.activated_at,
            message: "Contract activated successfully".to_string(),
        }))
    }

    /// POST /api/v1/fractional-ownership/contracts/{contract_id}/purchase
    /// Purchase shares in an ownership contract
    pub async fn purchase_shares(
        State(controller): State<Arc<Self>>,
        Extension(auth_user): Extension<AuthUser>,
        Path(contract_id): Path<Uuid>,
        Json(request): Json<PurchaseSharesRequest>,
    ) -> Result<Json<PurchaseSharesResponse>, AppError> {
        let command = PurchaseShares {
            contract_id,
            buyer_id: auth_user.user_id,
            ownership_percentage: request.ownership_percentage,
            vesting_start_date: request.vesting_start_date,
            vesting_end_date: request.vesting_end_date,
        };

        let result = controller.service.purchase_shares(command).await?;

        Ok(Json(PurchaseSharesResponse {
            share_id: result.share_id,
            contract_id: result.contract_id,
            ownership_percentage: result.ownership_percentage,
            investment_amount: result.investment_amount,
            events_triggered: result.events_triggered,
            message: "Shares purchased successfully".to_string(),
        }))
    }

    /// POST /api/v1/fractional-ownership/shares/{share_id}/trade
    /// Trade shares between users
    pub async fn trade_shares(
        State(controller): State<Arc<Self>>,
        Extension(auth_user): Extension<AuthUser>,
        Path(share_id): Path<Uuid>,
        Json(request): Json<TradeSharesRequest>,
    ) -> Result<Json<TradeSharesResponse>, AppError> {
        let command = TradeShares {
            share_id,
            from_user_id: auth_user.user_id,
            to_user_id: request.to_user_id,
            trade_price: request.trade_price,
        };

        let result = controller.service.trade_shares(command).await?;

        Ok(Json(TradeSharesResponse {
            share_id: result.share_id,
            from_user_id: result.from_user_id,
            to_user_id: result.to_user_id,
            trade_price: result.trade_price,
            ownership_percentage: result.ownership_percentage,
            events_triggered: result.events_triggered,
            message: "Shares traded successfully".to_string(),
        }))
    }

    /// POST /api/v1/fractional-ownership/contracts/{contract_id}/distribute-revenue
    /// Distribute revenue to shareholders
    pub async fn distribute_revenue(
        State(controller): State<Arc<Self>>,
        Extension(auth_user): Extension<AuthUser>,
        Path(contract_id): Path<Uuid>,
        Json(request): Json<DistributeRevenueRequest>,
    ) -> Result<Json<DistributeRevenueResponse>, AppError> {
        // TODO: Add authorization check - only artist or admin can distribute revenue

        let command = DistributeRevenue {
            contract_id,
            total_revenue: request.total_revenue,
            distribution_period_start: request.distribution_period_start,
            distribution_period_end: request.distribution_period_end,
            platform_fee_percentage: request.platform_fee_percentage,
        };

        let result = controller.service.distribute_revenue(command).await?;

        Ok(Json(DistributeRevenueResponse {
            contract_id: result.contract_id,
            total_revenue: result.total_revenue,
            total_distributed: result.total_distributed,
            artist_share: result.artist_share,
            platform_fee: result.platform_fee,
            shareholder_count: result.shareholder_count,
            distribution_id: result.distribution_id,
            message: "Revenue distributed successfully".to_string(),
        }))
    }

    /// DELETE /api/v1/fractional-ownership/contracts/{contract_id}
    /// Terminate an ownership contract
    pub async fn terminate_contract(
        State(controller): State<Arc<Self>>,
        Extension(auth_user): Extension<AuthUser>,
        Path(contract_id): Path<Uuid>,
        Json(request): Json<TerminateContractRequest>,
    ) -> Result<Json<TerminateContractResponse>, AppError> {
        // TODO: Add authorization check - only artist or admin can terminate

        let command = TerminateOwnershipContract {
            contract_id,
            terminated_by: auth_user.user_id,
            termination_reason: request.termination_reason,
        };

        let result = controller.service.terminate_ownership_contract(command).await?;

        Ok(Json(TerminateContractResponse {
            contract_id: result.contract_id,
            terminated_at: result.terminated_at,
            termination_reason: result.termination_reason,
            message: "Contract terminated successfully".to_string(),
        }))
    }

    /// GET /api/v1/fractional-ownership/contracts/{contract_id}
    /// Get ownership contract details
    pub async fn get_contract(
        State(controller): State<Arc<Self>>,
        Path(contract_id): Path<Uuid>,
    ) -> Result<Json<ContractDetailsResponse>, AppError> {
        let query = GetOwnershipContract { contract_id };

        let result = controller.service.get_ownership_contract(query).await?;

        Ok(Json(ContractDetailsResponse {
            contract_id: result.contract_id,
            song_id: result.song_id,
            artist_id: result.artist_id,
            total_shares: result.total_shares,
            price_per_share: result.price_per_share,
            artist_retained_percentage: result.artist_retained_percentage,
            shares_available_for_sale: result.shares_available_for_sale,
            shares_sold: result.shares_sold,
            completion_percentage: result.completion_percentage,
            total_investment_value: result.total_investment_value,
            contract_status: result.contract_status,
            minimum_investment: result.minimum_investment,
            maximum_ownership_per_user: result.maximum_ownership_per_user,
            unique_shareholders: result.unique_shareholders,
            can_accept_investment: result.can_accept_investment,
            created_at: result.created_at,
            updated_at: result.updated_at,
        }))
    }

    /// GET /api/v1/fractional-ownership/users/{user_id}/portfolio
    /// Get user's investment portfolio
    pub async fn get_user_portfolio(
        State(controller): State<Arc<Self>>,
        Extension(auth_user): Extension<AuthUser>,
        Path(user_id): Path<Uuid>,
    ) -> Result<Json<UserPortfolioResponse>, AppError> {
        // TODO: Add authorization check - only user themselves or admin can view portfolio
        
        let query = GetUserPortfolio { user_id };

        let result = controller.service.get_user_portfolio(query).await?;

        Ok(Json(UserPortfolioResponse {
            user_id: result.user_id,
            total_portfolio_value: result.total_portfolio_value,
            total_ownership_percentage: result.total_ownership_percentage,
            total_revenue_received: result.total_revenue_received,
            contracts_invested: result.contracts_invested,
            shares: result.shares,
            portfolio_performance: result.portfolio_performance,
        }))
    }

    /// GET /api/v1/fractional-ownership/contracts/{contract_id}/analytics
    /// Get contract analytics
    pub async fn get_contract_analytics(
        State(controller): State<Arc<Self>>,
        Path(contract_id): Path<Uuid>,
    ) -> Result<Json<ContractAnalyticsResponse>, AppError> {
        let query = GetContractAnalytics { contract_id };

        let result = controller.service.get_contract_analytics(query).await?;

        Ok(Json(ContractAnalyticsResponse {
            contract_id: result.contract_id,
            song_id: result.song_id,
            analytics: result.analytics,
            recent_activity: result.recent_activity,
            shareholder_breakdown: result.shareholder_breakdown,
            revenue_history: result.revenue_history,
        }))
    }

    /// GET /api/v1/fractional-ownership/contracts/search
    /// Search ownership contracts
    pub async fn search_contracts(
        State(controller): State<Arc<Self>>,
        Query(params): Query<SearchContractsQuery>,
    ) -> Result<Json<SearchContractsResponse>, AppError> {
        let query = SearchOwnershipContracts {
            query: params.q.unwrap_or_default(),
            filters: ContractSearchFilters {
                artist_id: params.artist_id,
                min_completion: params.min_completion,
                max_completion: params.max_completion,
                min_investment: params.min_investment,
                max_investment: params.max_investment,
                status: params.status,
                has_available_shares: params.has_available_shares,
                sort_by: params.sort_by,
                sort_order: params.sort_order,
            },
            page: params.page.unwrap_or(1),
            page_size: params.page_size.unwrap_or(20),
        };

        let result = controller.service.search_ownership_contracts(query).await?;

        Ok(Json(SearchContractsResponse {
            contracts: result.contracts,
            total_count: result.total_count,
            page: result.page,
            page_size: result.page_size,
            total_pages: result.total_pages,
        }))
    }

    /// GET /api/v1/fractional-ownership/artists/{artist_id}/contracts
    /// Get contracts by artist
    pub async fn get_contracts_by_artist(
        State(controller): State<Arc<Self>>,
        Path(artist_id): Path<Uuid>,
    ) -> Result<Json<ArtistContractsResponse>, AppError> {
        let query = GetContractsByArtist { artist_id };

        let result = controller.service.get_contracts_by_artist(query).await?;

        Ok(Json(ArtistContractsResponse {
            artist_id: result.artist_id,
            contracts: result.contracts,
            total_market_cap: result.total_market_cap,
            total_revenue_distributed: result.total_revenue_distributed,
            average_completion_rate: result.average_completion_rate,
        }))
    }

    /// GET /api/v1/fractional-ownership/market/statistics
    /// Get market statistics
    pub async fn get_market_statistics(
        State(controller): State<Arc<Self>>,
    ) -> Result<Json<MarketStatisticsResponse>, AppError> {
        let query = GetMarketStatistics {};

        let result = controller.service.get_market_statistics(query).await?;

        Ok(Json(MarketStatisticsResponse {
            market_stats: result.market_stats,
            trending_contracts: result.trending_contracts,
            top_artists: result.top_artists,
            recent_distributions: result.recent_distributions,
        }))
    }
}

// Request/Response DTOs

#[derive(Debug, Deserialize)]
pub struct CreateContractRequest {
    pub song_id: Uuid,
    pub total_shares: u32,
    pub price_per_share: f64,
    pub artist_retained_percentage: f64,
    pub minimum_investment: Option<f64>,
    pub maximum_ownership_per_user: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct CreateContractResponse {
    pub contract_id: Uuid,
    pub shares_available_for_sale: u32,
    pub total_market_cap: f64,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ActivateContractResponse {
    pub contract_id: Uuid,
    pub activated_at: chrono::DateTime<chrono::Utc>,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct PurchaseSharesRequest {
    pub ownership_percentage: f64,
    pub vesting_start_date: Option<chrono::DateTime<chrono::Utc>>,
    pub vesting_end_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize)]
pub struct PurchaseSharesResponse {
    pub share_id: Uuid,
    pub contract_id: Uuid,
    pub ownership_percentage: f64,
    pub investment_amount: f64,
    pub events_triggered: Vec<String>,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct TradeSharesRequest {
    pub to_user_id: Uuid,
    pub trade_price: f64,
}

#[derive(Debug, Serialize)]
pub struct TradeSharesResponse {
    pub share_id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub trade_price: f64,
    pub ownership_percentage: f64,
    pub events_triggered: Vec<String>,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct DistributeRevenueRequest {
    pub total_revenue: f64,
    pub distribution_period_start: chrono::DateTime<chrono::Utc>,
    pub distribution_period_end: chrono::DateTime<chrono::Utc>,
    pub platform_fee_percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct DistributeRevenueResponse {
    pub contract_id: Uuid,
    pub total_revenue: f64,
    pub total_distributed: f64,
    pub artist_share: f64,
    pub platform_fee: f64,
    pub shareholder_count: u32,
    pub distribution_id: Uuid,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct TerminateContractRequest {
    pub termination_reason: String,
}

#[derive(Debug, Serialize)]
pub struct TerminateContractResponse {
    pub contract_id: Uuid,
    pub terminated_at: chrono::DateTime<chrono::Utc>,
    pub termination_reason: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ContractDetailsResponse {
    pub contract_id: Uuid,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub total_shares: u32,
    pub price_per_share: f64,
    pub artist_retained_percentage: f64,
    pub shares_available_for_sale: u32,
    pub shares_sold: u32,
    pub completion_percentage: f64,
    pub total_investment_value: f64,
    pub contract_status: String,
    pub minimum_investment: Option<f64>,
    pub maximum_ownership_per_user: Option<f64>,
    pub unique_shareholders: u32,
    pub can_accept_investment: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// Re-export types from application layer
pub use crate::bounded_contexts::fractional_ownership::application::{
    GetUserPortfolioResult as UserPortfolioResponse,
    GetContractAnalyticsResult as ContractAnalyticsResponse,
    SearchOwnershipContractsResult as SearchContractsResponse,
    GetContractsByArtistResult as ArtistContractsResponse,
    GetMarketStatisticsResult as MarketStatisticsResponse,
};

#[derive(Debug, Deserialize)]
pub struct SearchContractsQuery {
    pub q: Option<String>,
    pub artist_id: Option<Uuid>,
    pub min_completion: Option<f64>,
    pub max_completion: Option<f64>,
    pub min_investment: Option<f64>,
    pub max_investment: Option<f64>,
    pub status: Option<String>,
    pub has_available_shares: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

// Error handling helper
impl From<AppError> for StatusCode {
    fn from(error: AppError) -> Self {
        match error {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            AppError::DomainRuleViolation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::ConcurrencyConflict(_) => StatusCode::CONFLICT,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::SerializationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NetworkError(_) => StatusCode::BAD_GATEWAY,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_contexts::fractional_ownership::domain::repository::tests::MockOwnershipContractRepository;
    use std::sync::Arc;

    fn setup_controller() -> FractionalOwnershipController<MockOwnershipContractRepository> {
        let repo = Arc::new(MockOwnershipContractRepository::new());
        let service = Arc::new(FractionalOwnershipApplicationService::new(repo));
        FractionalOwnershipController::new(service)
    }

    #[tokio::test]
    async fn test_create_contract_request_validation() {
        let controller = Arc::new(setup_controller());
        
        let request = CreateContractRequest {
            song_id: Uuid::new_v4(),
            total_shares: 1000,
            price_per_share: 10.0,
            artist_retained_percentage: 51.0,
            minimum_investment: Some(100.0),
            maximum_ownership_per_user: Some(20.0),
        };

        // This would normally test the actual HTTP request handling
        // For now, we're just testing the structure
        assert_eq!(request.total_shares, 1000);
        assert_eq!(request.price_per_share, 10.0);
    }

    #[tokio::test]
    async fn test_purchase_shares_request_validation() {
        let request = PurchaseSharesRequest {
            ownership_percentage: 15.0,
            vesting_start_date: None,
            vesting_end_date: None,
        };

        assert_eq!(request.ownership_percentage, 15.0);
        assert!(request.vesting_start_date.is_none());
    }

    #[tokio::test]
    async fn test_search_query_parameters() {
        let query = SearchContractsQuery {
            q: Some("artist name".to_string()),
            artist_id: Some(Uuid::new_v4()),
            min_completion: Some(50.0),
            max_completion: Some(100.0),
            min_investment: None,
            max_investment: None,
            status: Some("Active".to_string()),
            has_available_shares: Some(true),
            sort_by: Some("completion".to_string()),
            sort_order: Some("desc".to_string()),
            page: Some(1),
            page_size: Some(20),
        };

        assert!(query.q.is_some());
        assert!(query.artist_id.is_some());
        assert_eq!(query.min_completion, Some(50.0));
        assert_eq!(query.status, Some("Active".to_string()));
    }

    #[test]
    fn test_error_status_code_mapping() {
        assert_eq!(StatusCode::from(AppError::NotFound("test".to_string())), StatusCode::NOT_FOUND);
        assert_eq!(StatusCode::from(AppError::InvalidInput("test".to_string())), StatusCode::BAD_REQUEST);
        assert_eq!(StatusCode::from(AppError::DomainRuleViolation("test".to_string())), StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(StatusCode::from(AppError::ConcurrencyConflict("test".to_string())), StatusCode::CONFLICT);
        assert_eq!(StatusCode::from(AppError::Unauthorized("test".to_string())), StatusCode::UNAUTHORIZED);
    }
} 