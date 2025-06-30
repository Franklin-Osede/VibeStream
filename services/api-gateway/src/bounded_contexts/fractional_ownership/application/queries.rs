use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::application::query::{Query, QueryHandler};
use crate::shared::domain::errors::AppError;

use crate::bounded_contexts::fractional_ownership::domain::{
    aggregates::{OwnershipContractAggregate, OwnershipAnalytics},
    repository::{OwnershipContractRepository, OwnershipContractSpecification, MarketStatistics},
    value_objects::OwnershipContractId,
    entities::FractionalShare,
};
use crate::bounded_contexts::music::domain::value_objects::{SongId, ArtistId};
use crate::bounded_contexts::user::domain::value_objects::UserId;

/// Query: Get ownership contract by ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOwnershipContract {
    pub contract_id: Uuid,
}

impl Query for GetOwnershipContract {
    type Result = GetOwnershipContractResult;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOwnershipContractResult {
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Query: Get user's portfolio across all contracts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserPortfolio {
    pub user_id: Uuid,
}

impl Query for GetUserPortfolio {
    type Result = GetUserPortfolioResult;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserPortfolioResult {
    pub user_id: Uuid,
    pub total_portfolio_value: f64,
    pub total_ownership_percentage: f64,
    pub total_revenue_received: f64,
    pub contracts_invested: u32,
    pub shares: Vec<SharePortfolioItem>,
    pub portfolio_performance: PortfolioPerformance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharePortfolioItem {
    pub share_id: Uuid,
    pub contract_id: Uuid,
    pub song_id: Uuid,
    pub ownership_percentage: f64,
    pub purchase_price: f64,
    pub current_market_value: f64,
    pub total_revenue_received: f64,
    pub roi_percentage: f64,
    pub is_locked: bool,
    pub is_tradeable: bool,
    pub vesting_progress: f64,
    pub purchased_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioPerformance {
    pub total_invested: f64,
    pub current_value: f64,
    pub total_roi_percentage: f64,
    pub best_performing_share: Option<SharePortfolioItem>,
    pub worst_performing_share: Option<SharePortfolioItem>,
}

/// Query: Get contract analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetContractAnalytics {
    pub contract_id: Uuid,
}

impl Query for GetContractAnalytics {
    type Result = GetContractAnalyticsResult;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetContractAnalyticsResult {
    pub contract_id: Uuid,
    pub song_id: Uuid,
    pub analytics: OwnershipAnalytics,
    pub recent_activity: Vec<ContractActivity>,
    pub shareholder_breakdown: Vec<ShareholderBreakdown>,
    pub revenue_history: Vec<RevenueDistributionSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractActivity {
    pub activity_type: String,
    pub user_id: Option<Uuid>,
    pub amount: Option<f64>,
    pub ownership_percentage: Option<f64>,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareholderBreakdown {
    pub user_id: Uuid,
    pub total_ownership_percentage: f64,
    pub total_investment: f64,
    pub number_of_shares: u32,
    pub total_revenue_received: f64,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueDistributionSummary {
    pub distribution_id: Uuid,
    pub total_revenue: f64,
    pub artist_share: f64,
    pub platform_fee: f64,
    pub shareholder_total: f64,
    pub distribution_date: DateTime<Utc>,
}

/// Query: Get market statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMarketStatistics {
    // Empty - gets overall market stats
}

impl Query for GetMarketStatistics {
    type Result = GetMarketStatisticsResult;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMarketStatisticsResult {
    pub market_stats: MarketStatistics,
    pub trending_contracts: Vec<TrendingContract>,
    pub top_artists: Vec<TopArtist>,
    pub recent_distributions: Vec<RecentDistribution>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendingContract {
    pub contract_id: Uuid,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub completion_percentage: f64,
    pub total_investment: f64,
    pub price_performance: f64,
    pub trading_volume_24h: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopArtist {
    pub artist_id: Uuid,
    pub total_contracts: u32,
    pub total_market_cap: f64,
    pub total_revenue_distributed: f64,
    pub average_completion_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentDistribution {
    pub contract_id: Uuid,
    pub song_id: Uuid,
    pub total_amount: f64,
    pub shareholder_count: u32,
    pub distributed_at: DateTime<Utc>,
}

/// Query: Search ownership contracts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOwnershipContracts {
    pub query: String,
    pub filters: ContractSearchFilters,
    pub page: u32,
    pub page_size: u32,
}

impl Query for SearchOwnershipContracts {
    type Result = SearchOwnershipContractsResult;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractSearchFilters {
    pub artist_id: Option<Uuid>,
    pub min_completion: Option<f64>,
    pub max_completion: Option<f64>,
    pub min_investment: Option<f64>,
    pub max_investment: Option<f64>,
    pub status: Option<String>,
    pub has_available_shares: Option<bool>,
    pub sort_by: Option<String>, // "completion", "investment", "created_at"
    pub sort_order: Option<String>, // "asc", "desc"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOwnershipContractsResult {
    pub contracts: Vec<ContractSearchItem>,
    pub total_count: u64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractSearchItem {
    pub contract_id: Uuid,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub price_per_share: f64,
    pub completion_percentage: f64,
    pub total_investment: f64,
    pub shares_available: u32,
    pub unique_shareholders: u32,
    pub can_accept_investment: bool,
    pub created_at: DateTime<Utc>,
}

/// Query: Get contracts by artist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetContractsByArtist {
    pub artist_id: Uuid,
}

impl Query for GetContractsByArtist {
    type Result = GetContractsByArtistResult;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetContractsByArtistResult {
    pub artist_id: Uuid,
    pub contracts: Vec<ContractSummary>,
    pub total_market_cap: f64,
    pub total_revenue_distributed: f64,
    pub average_completion_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractSummary {
    pub contract_id: Uuid,
    pub song_id: Uuid,
    pub completion_percentage: f64,
    pub total_investment: f64,
    pub unique_shareholders: u32,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

// Query Handlers Implementation

pub struct GetOwnershipContractHandler<R: OwnershipContractRepository> {
    pub repository: R,
}

#[async_trait]
impl<R: OwnershipContractRepository> QueryHandler<GetOwnershipContract> for GetOwnershipContractHandler<R> {
    async fn handle(&self, query: GetOwnershipContract) -> Result<GetOwnershipContractResult, AppError> {
        let contract_id = OwnershipContractId::from_uuid(query.contract_id);
        
        let aggregate = self.repository.find_by_id(&contract_id).await?
            .ok_or_else(|| AppError::NotFound("Ownership contract not found".to_string()))?;

        let contract = aggregate.contract();
        let unique_shareholders = aggregate.get_unique_shareholders().len() as u32;

        Ok(GetOwnershipContractResult {
            contract_id: query.contract_id,
            song_id: contract.song_id.value(),
            artist_id: contract.artist_id.value(),
            total_shares: contract.total_shares,
            price_per_share: contract.price_per_share.value(),
            artist_retained_percentage: contract.artist_retained_percentage.value(),
            shares_available_for_sale: contract.shares_available_for_sale,
            shares_sold: contract.shares_sold,
            completion_percentage: aggregate.completion_percentage(),
            total_investment_value: aggregate.total_investment_value(),
            contract_status: format!("{:?}", contract.contract_status),
            minimum_investment: contract.minimum_investment.as_ref().map(|mi| mi.value()),
            maximum_ownership_per_user: contract.maximum_ownership_per_user.as_ref().map(|mo| mo.value()),
            unique_shareholders,
            can_accept_investment: aggregate.can_accept_investment(),
            created_at: contract.created_at,
            updated_at: contract.updated_at,
        })
    }
}

pub struct GetUserPortfolioHandler<R: OwnershipContractRepository> {
    pub repository: R,
}

#[async_trait]
impl<R: OwnershipContractRepository> QueryHandler<GetUserPortfolio> for GetUserPortfolioHandler<R> {
    async fn handle(&self, query: GetUserPortfolio) -> Result<GetUserPortfolioResult, AppError> {
        let user_id = UserId::from_uuid(query.user_id);
        
        let contracts = self.repository.find_contracts_with_user_shares(&user_id).await?;
        
        let mut total_portfolio_value = 0.0;
        let mut total_ownership_percentage = 0.0;
        let mut total_revenue_received = 0.0;
        let mut total_invested = 0.0;
        let mut shares = Vec::new();

        for contract in &contracts {
            let user_shares = contract.get_user_shares(&user_id);
            
            for share in user_shares {
                let share_item = SharePortfolioItem {
                    share_id: share.id().value(),
                    contract_id: share.contract_id().value(),
                    song_id: share.song_id().value(),
                    ownership_percentage: share.ownership_percentage().value(),
                    purchase_price: share.purchase_price().value(),
                    current_market_value: share.current_market_value().value(),
                    total_revenue_received: share.total_revenue_received().value(),
                    roi_percentage: share.calculate_roi(),
                    is_locked: share.is_locked(),
                    is_tradeable: share.is_tradeable(),
                    vesting_progress: share.vesting_progress(),
                    purchased_at: share.purchased_at(),
                };

                total_portfolio_value += share_item.current_market_value;
                total_ownership_percentage += share_item.ownership_percentage;
                total_revenue_received += share_item.total_revenue_received;
                total_invested += share_item.purchase_price;
                
                shares.push(share_item);
            }
        }

        // Calculate performance
        let best_performing = shares.iter()
            .max_by(|a, b| a.roi_percentage.partial_cmp(&b.roi_percentage).unwrap())
            .cloned();
            
        let worst_performing = shares.iter()
            .min_by(|a, b| a.roi_percentage.partial_cmp(&b.roi_percentage).unwrap())
            .cloned();

        let total_roi = if total_invested > 0.0 {
            ((total_portfolio_value + total_revenue_received - total_invested) / total_invested) * 100.0
        } else {
            0.0
        };

        let performance = PortfolioPerformance {
            total_invested,
            current_value: total_portfolio_value,
            total_roi_percentage: total_roi,
            best_performing_share: best_performing,
            worst_performing_share: worst_performing,
        };

        Ok(GetUserPortfolioResult {
            user_id: query.user_id,
            total_portfolio_value,
            total_ownership_percentage,
            total_revenue_received,
            contracts_invested: contracts.len() as u32,
            shares,
            portfolio_performance: performance,
        })
    }
}

pub struct GetContractAnalyticsHandler<R: OwnershipContractRepository> {
    pub repository: R,
}

#[async_trait]
impl<R: OwnershipContractRepository> QueryHandler<GetContractAnalytics> for GetContractAnalyticsHandler<R> {
    async fn handle(&self, query: GetContractAnalytics) -> Result<GetContractAnalyticsResult, AppError> {
        let contract_id = OwnershipContractId::from_uuid(query.contract_id);
        
        let aggregate = self.repository.find_by_id(&contract_id).await?
            .ok_or_else(|| AppError::NotFound("Ownership contract not found".to_string()))?;

        let analytics = aggregate.get_analytics();
        
        // Build shareholder breakdown
        let mut shareholder_breakdown = Vec::new();
        let unique_shareholders = aggregate.get_unique_shareholders();
        
        for shareholder in unique_shareholders {
            let user_shares = aggregate.get_user_shares(&shareholder);
            let total_ownership: f64 = user_shares.iter().map(|s| s.ownership_percentage().value()).sum();
            let total_investment: f64 = user_shares.iter().map(|s| s.purchase_price().value()).sum();
            let total_revenue: f64 = user_shares.iter().map(|s| s.total_revenue_received().value()).sum();
            let earliest_purchase = user_shares.iter().map(|s| s.purchased_at()).min().unwrap_or(Utc::now());

            shareholder_breakdown.push(ShareholderBreakdown {
                user_id: shareholder.value(),
                total_ownership_percentage: total_ownership,
                total_investment,
                number_of_shares: user_shares.len() as u32,
                total_revenue_received: total_revenue,
                joined_at: earliest_purchase,
            });
        }

        // Mock recent activity and revenue history (in real implementation, would come from event store)
        let recent_activity = Vec::new();
        let revenue_history = Vec::new();

        Ok(GetContractAnalyticsResult {
            contract_id: query.contract_id,
            song_id: analytics.song_id,
            analytics,
            recent_activity,
            shareholder_breakdown,
            revenue_history,
        })
    }
}

// Additional handler implementations would follow similar patterns...

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_contexts::fractional_ownership::domain::repository::tests::MockOwnershipContractRepository;
    use crate::bounded_contexts::fractional_ownership::domain::{
        aggregates::OwnershipContractAggregate,
        value_objects::{SharePrice, OwnershipPercentage, RevenueAmount},
    };

    async fn setup_test_contract() -> (MockOwnershipContractRepository, OwnershipContractAggregate) {
        let repo = MockOwnershipContractRepository::new();
        
        let aggregate = OwnershipContractAggregate::create_contract(
            SongId::new(),
            ArtistId::new(),
            1000,
            SharePrice::new(10.0).unwrap(),
            OwnershipPercentage::new(51.0).unwrap(),
            Some(RevenueAmount::new(100.0).unwrap()),
            Some(OwnershipPercentage::new(20.0).unwrap()),
        ).unwrap();

        repo.save(&aggregate).await.unwrap();
        (repo, aggregate)
    }

    #[tokio::test]
    async fn test_get_ownership_contract_query() {
        let (repo, aggregate) = setup_test_contract().await;
        let handler = GetOwnershipContractHandler { repository: repo };

        let query = GetOwnershipContract {
            contract_id: aggregate.id().value(),
        };

        let result = handler.handle(query).await.unwrap();
        
        assert_eq!(result.total_shares, 1000);
        assert_eq!(result.price_per_share, 10.0);
        assert_eq!(result.artist_retained_percentage, 51.0);
        assert_eq!(result.shares_available_for_sale, 490);
        assert_eq!(result.completion_percentage, 0.0);
        assert!(result.can_accept_investment);
    }

    #[tokio::test]
    async fn test_get_user_portfolio_query() {
        let (repo, mut aggregate) = setup_test_contract().await;
        
        // Activate contract and add shares
        aggregate.activate_contract().unwrap();
        
        let user_id = UserId::new();
        let ownership = OwnershipPercentage::new(10.0).unwrap();
        
        let (_, _) = aggregate.purchase_shares(user_id.clone(), ownership, None).unwrap();
        repo.update(&aggregate).await.unwrap();

        let handler = GetUserPortfolioHandler { repository: repo };

        let query = GetUserPortfolio {
            user_id: user_id.value(),
        };

        let result = handler.handle(query).await.unwrap();
        
        assert_eq!(result.contracts_invested, 1);
        assert_eq!(result.shares.len(), 1);
        assert_eq!(result.shares[0].ownership_percentage, 10.0);
        assert_eq!(result.total_portfolio_value, 1000.0); // 10% * $10 * 1000 shares
        assert!(result.portfolio_performance.total_invested > 0.0);
    }

    #[tokio::test]
    async fn test_get_contract_analytics_query() {
        let (repo, mut aggregate) = setup_test_contract().await;
        
        // Activate and add some activity
        aggregate.activate_contract().unwrap();
        
        let user1 = UserId::new();
        let user2 = UserId::new();
        
        aggregate.purchase_shares(user1, OwnershipPercentage::new(10.0).unwrap(), None).unwrap();
        aggregate.purchase_shares(user2, OwnershipPercentage::new(15.0).unwrap(), None).unwrap();
        
        repo.update(&aggregate).await.unwrap();

        let handler = GetContractAnalyticsHandler { repository: repo };

        let query = GetContractAnalytics {
            contract_id: aggregate.id().value(),
        };

        let result = handler.handle(query).await.unwrap();
        
        assert_eq!(result.analytics.total_shares, 1000);
        assert_eq!(result.analytics.shares_sold, 250); // 10% + 15% of 1000
        assert_eq!(result.analytics.unique_shareholders, 2);
        assert_eq!(result.shareholder_breakdown.len(), 2);
        
        // Check shareholder breakdown
        let shareholder1 = &result.shareholder_breakdown[0];
        assert!(shareholder1.total_ownership_percentage == 10.0 || shareholder1.total_ownership_percentage == 15.0);
        assert!(shareholder1.total_investment > 0.0);
    }

    #[tokio::test]
    async fn test_get_nonexistent_contract_returns_error() {
        let repo = MockOwnershipContractRepository::new();
        let handler = GetOwnershipContractHandler { repository: repo };

        let query = GetOwnershipContract {
            contract_id: Uuid::new_v4(),
        };

        let result = handler.handle(query).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_empty_user_portfolio() {
        let repo = MockOwnershipContractRepository::new();
        let handler = GetUserPortfolioHandler { repository: repo };

        let query = GetUserPortfolio {
            user_id: Uuid::new_v4(),
        };

        let result = handler.handle(query).await.unwrap();
        
        assert_eq!(result.contracts_invested, 0);
        assert_eq!(result.shares.len(), 0);
        assert_eq!(result.total_portfolio_value, 0.0);
        assert_eq!(result.total_ownership_percentage, 0.0);
    }
} 