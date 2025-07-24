use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::application::command::{Command, CommandHandler};
use crate::shared::domain::errors::AppError;

use crate::bounded_contexts::fractional_ownership::domain::{
    aggregates::OwnershipContractAggregate,
    repository::OwnershipContractRepository,
    value_objects::{OwnershipContractId, OwnershipPercentage, SharePrice, RevenueAmount, ShareId, VestingPeriod},
    events::TerminationReason,
};
use crate::bounded_contexts::music::domain::value_objects::{SongId, ArtistId};
use crate::bounded_contexts::user::domain::value_objects::UserId;

/// Command: Create a new ownership contract for a song
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOwnershipContract {
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub total_shares: u32,
    pub price_per_share: f64,
    pub artist_retained_percentage: f64,
    pub minimum_investment: Option<f64>,
    pub maximum_ownership_per_user: Option<f64>,
}

impl Command for CreateOwnershipContract {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOwnershipContractResult {
    pub contract_id: Uuid,
    pub shares_available_for_sale: u32,
    pub total_market_cap: f64,
}

/// Command: Activate ownership contract for public investment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivateOwnershipContract {
    pub contract_id: Uuid,
}

impl Command for ActivateOwnershipContract {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivateOwnershipContractResult {
    pub contract_id: Uuid,
    pub activated_at: DateTime<Utc>,
}

/// Command: Purchase shares in an ownership contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseShares {
    pub contract_id: Uuid,
    pub buyer_id: Uuid,
    pub ownership_percentage: f64,
    pub vesting_start_date: Option<DateTime<Utc>>,
    pub vesting_end_date: Option<DateTime<Utc>>,
}

impl Command for PurchaseShares {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseSharesResult {
    pub share_id: Uuid,
    pub contract_id: Uuid,
    pub ownership_percentage: f64,
    pub purchase_price: f64,
    pub investment_amount: f64,
    pub events_triggered: Vec<String>,
}

/// Command: Trade shares between users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeShares {
    pub share_id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub trade_price: f64,
}

impl Command for TradeShares {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeSharesResult {
    pub share_id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub trade_price: f64,
    pub ownership_percentage: f64,
    pub events_triggered: Vec<String>,
}

/// Command: Distribute revenue to shareholders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributeRevenue {
    pub contract_id: Uuid,
    pub total_revenue: f64,
    pub distribution_period_start: DateTime<Utc>,
    pub distribution_period_end: DateTime<Utc>,
    pub platform_fee_percentage: f64,
}

impl Command for DistributeRevenue {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributeRevenueResult {
    pub contract_id: Uuid,
    pub total_revenue: f64,
    pub total_distributed: f64,
    pub artist_share: f64,
    pub platform_fee: f64,
    pub shareholder_count: u32,
    pub distribution_id: Uuid,
}

/// Command: Terminate ownership contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminateOwnershipContract {
    pub contract_id: Uuid,
    pub terminated_by: Uuid,
    pub termination_reason: String,
}

impl Command for TerminateOwnershipContract {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminateOwnershipContractResult {
    pub contract_id: Uuid,
    pub terminated_at: DateTime<Utc>,
    pub termination_reason: String,
}

// Command Handlers Implementation

pub struct CreateOwnershipContractHandler<R: OwnershipContractRepository> {
    pub repository: R,
}

#[async_trait]
impl<R: OwnershipContractRepository + Send + Sync> CommandHandler<CreateOwnershipContract> for CreateOwnershipContractHandler<R> {
    type Output = CreateOwnershipContractResult;
    
    async fn handle(&self, command: CreateOwnershipContract) -> Result<Self::Output, AppError> {
        // Validate inputs
        let song_id = SongId::from_uuid(command.song_id);
        let artist_id = ArtistId::from_uuid(command.artist_id);
        let price_per_share = SharePrice::new(command.price_per_share)?;
        let artist_retained = OwnershipPercentage::new(command.artist_retained_percentage)?;
        
        let minimum_investment = command.minimum_investment
            .map(|amount| RevenueAmount::new(amount))
            .transpose()?;
            
        let max_ownership = command.maximum_ownership_per_user
            .map(|percentage| OwnershipPercentage::new(percentage))
            .transpose()?;

        // Check if contract already exists for this song
        if self.repository.exists_for_song(&song_id).await? {
            return Err(AppError::DomainRuleViolation(
                "Ownership contract already exists for this song".to_string(),
            ));
        }

        // Create aggregate
        let aggregate = OwnershipContractAggregate::create_contract(
            song_id,
            artist_id,
            command.total_shares,
            price_per_share.clone(),
            artist_retained,
            minimum_investment,
            max_ownership,
        )?;

        let contract_id = aggregate.id().value();
        let shares_available = aggregate.shares_available();
        let market_cap = price_per_share.calculate_market_cap(command.total_shares);

        // Save to repository
        self.repository.save(&aggregate).await?;

        Ok(CreateOwnershipContractResult {
            contract_id,
            shares_available_for_sale: shares_available,
            total_market_cap: market_cap,
        })
    }
}

pub struct ActivateOwnershipContractHandler<R: OwnershipContractRepository> {
    pub repository: R,
}

#[async_trait]
impl<R: OwnershipContractRepository + Send + Sync> CommandHandler<ActivateOwnershipContract> for ActivateOwnershipContractHandler<R> {
    type Output = ActivateOwnershipContractResult;

    async fn handle(&self, command: ActivateOwnershipContract) -> Result<Self::Output, AppError> {
        let contract_id = OwnershipContractId::from_uuid(command.contract_id);
        
        let mut aggregate = self.repository.find_by_id(&contract_id).await?
            .ok_or_else(|| AppError::NotFound("Ownership contract not found".to_string()))?;

        aggregate.activate_contract()?;
        let activated_at = Utc::now();

        self.repository.update(&aggregate).await?;

        Ok(ActivateOwnershipContractResult {
            contract_id: command.contract_id,
            activated_at,
        })
    }
}

pub struct PurchaseSharesHandler<R: OwnershipContractRepository> {
    pub repository: R,
}

#[async_trait]
impl<R: OwnershipContractRepository + Send + Sync> CommandHandler<PurchaseShares> for PurchaseSharesHandler<R> {
    type Output = PurchaseSharesResult;

    async fn handle(&self, command: PurchaseShares) -> Result<Self::Output, AppError> {
        let contract_id = OwnershipContractId::from_uuid(command.contract_id);
        let buyer_id = UserId::from_uuid(command.buyer_id);
        let ownership_percentage = OwnershipPercentage::new(command.ownership_percentage)?;

        // Create vesting period if provided
        let vesting_period = if let (Some(start), Some(end)) = (command.vesting_start_date, command.vesting_end_date) {
            Some(VestingPeriod::new(start, end)?)
        } else {
            None
        };

        let mut aggregate = self.repository.find_by_id(&contract_id).await?
            .ok_or_else(|| AppError::NotFound("Ownership contract not found".to_string()))?;

        let (share, events) = aggregate.purchase_shares(buyer_id, ownership_percentage.clone(), vesting_period)?;

        self.repository.update(&aggregate).await?;

        Ok(PurchaseSharesResult {
            share_id: share.id().value(),
            contract_id: command.contract_id,
            ownership_percentage: ownership_percentage.value(),
            purchase_price: share.purchase_price().value(),
            investment_amount: share.purchase_price().value(),
            events_triggered: events,
        })
    }
}

pub struct TradeSharesHandler<R: OwnershipContractRepository> {
    pub repository: R,
}

#[async_trait]
impl<R: OwnershipContractRepository + Send + Sync> CommandHandler<TradeShares> for TradeSharesHandler<R> {
    type Output = TradeSharesResult;

    async fn handle(&self, command: TradeShares) -> Result<Self::Output, AppError> {
        let share_id = ShareId::from_uuid(command.share_id);
        let to_user_id = UserId::from_uuid(command.to_user_id);
        let trade_price = SharePrice::new(command.trade_price)?;

        // Find contract containing the share (simplified - in real implementation might need a share-to-contract index)
        let contracts = self.repository.find_active_contracts().await?;
        let target_aggregate: Option<OwnershipContractAggregate> = None;
        
        for mut aggregate in contracts {
            if aggregate.shares().contains_key(&share_id) {
                let events = aggregate.trade_shares(share_id.clone(), to_user_id.clone(), trade_price.clone())?;
                
                let share = aggregate.shares().get(&share_id).unwrap();
                let result = TradeSharesResult {
                    share_id: command.share_id,
                    from_user_id: command.from_user_id,
                    to_user_id: command.to_user_id,
                    trade_price: command.trade_price,
                    ownership_percentage: share.ownership_percentage().value(),
                    events_triggered: events,
                };

                self.repository.update(&aggregate).await?;
                return Ok(result);
            }
        }

        Err(AppError::NotFound("Share not found in any active contract".to_string()))
    }
}

pub struct DistributeRevenueHandler<R: OwnershipContractRepository> {
    pub repository: R,
}

#[async_trait]
impl<R: OwnershipContractRepository + Send + Sync> CommandHandler<DistributeRevenue> for DistributeRevenueHandler<R> {
    type Output = DistributeRevenueResult;

    async fn handle(&self, command: DistributeRevenue) -> Result<Self::Output, AppError> {
        let contract_id = OwnershipContractId::from_uuid(command.contract_id);
        let total_revenue = RevenueAmount::new(command.total_revenue)?;

        let mut aggregate = self.repository.find_by_id(&contract_id).await?
            .ok_or_else(|| AppError::NotFound("Ownership contract not found".to_string()))?;

        let distribution_event = aggregate.distribute_revenue(
            total_revenue,
            command.distribution_period_start,
            command.distribution_period_end,
            command.platform_fee_percentage,
        )?;

        self.repository.update(&aggregate).await?;

        Ok(DistributeRevenueResult {
            contract_id: command.contract_id,
            total_revenue: distribution_event.total_revenue,
            total_distributed: distribution_event.total_distributed,
            artist_share: distribution_event.artist_share,
            platform_fee: distribution_event.platform_fee,
            shareholder_count: distribution_event.shareholder_distributions.len() as u32,
            distribution_id: Uuid::new_v4(), // Would be from the actual distribution entity
        })
    }
}

pub struct TerminateOwnershipContractHandler<R: OwnershipContractRepository> {
    pub repository: R,
}

#[async_trait]
impl<R: OwnershipContractRepository + Send + Sync> CommandHandler<TerminateOwnershipContract> for TerminateOwnershipContractHandler<R> {
    type Output = TerminateOwnershipContractResult;

    async fn handle(&self, command: TerminateOwnershipContract) -> Result<Self::Output, AppError> {
        let contract_id = OwnershipContractId::from_uuid(command.contract_id);
        let terminated_by = UserId::from_uuid(command.terminated_by);

        let termination_reason = match command.termination_reason.as_str() {
            "artist_request" => TerminationReason::ArtistRequest,
            "legal_issues" => TerminationReason::LegalIssues,
            "insufficient_funding" => TerminationReason::InsufficientFunding,
            "contract_expired" => TerminationReason::ContractExpired,
            "mutual_agreement" => TerminationReason::MutualAgreement,
            _ => return Err(AppError::InvalidInput("Invalid termination reason".to_string())),
        };

        let mut aggregate = self.repository.find_by_id(&contract_id).await?
            .ok_or_else(|| AppError::NotFound("Ownership contract not found".to_string()))?;

        let termination_event = aggregate.terminate_contract(termination_reason, terminated_by)?;
        
        self.repository.update(&aggregate).await?;

        Ok(TerminateOwnershipContractResult {
            contract_id: command.contract_id,
            terminated_at: termination_event.terminated_at,
            termination_reason: command.termination_reason,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_contexts::fractional_ownership::domain::repository::tests::MockOwnershipContractRepository;
    use chrono::Duration;

    #[tokio::test]
    async fn test_create_ownership_contract_command() {
        let repo = MockOwnershipContractRepository::new();
        let handler = CreateOwnershipContractHandler { repository: repo };

        let command = CreateOwnershipContract {
            song_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            total_shares: 1000,
            price_per_share: 10.0,
            artist_retained_percentage: 51.0,
            minimum_investment: Some(100.0),
            maximum_ownership_per_user: Some(20.0),
        };

        let result = handler.handle(command).await.unwrap();
        
        assert_eq!(result.shares_available_for_sale, 490); // 49% of 1000
        assert_eq!(result.total_market_cap, 10000.0); // 1000 * $10
        assert!(result.contract_id != Uuid::nil());
    }

    #[tokio::test]
    async fn test_create_duplicate_contract_fails() {
        let repo = MockOwnershipContractRepository::new();
        let handler = CreateOwnershipContractHandler { repository: repo };

        let song_id = Uuid::new_v4();
        let command = CreateOwnershipContract {
            song_id,
            artist_id: Uuid::new_v4(),
            total_shares: 1000,
            price_per_share: 10.0,
            artist_retained_percentage: 51.0,
            minimum_investment: Some(100.0),
            maximum_ownership_per_user: Some(20.0),
        };

        // First contract should succeed
        handler.handle(command.clone()).await.unwrap();

        // Second contract for same song should fail
        let result = handler.handle(command).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_activate_contract_command() {
        let repo = MockOwnershipContractRepository::new();
        
        // Create contract first
        let create_handler = CreateOwnershipContractHandler { repository: repo.clone() };
        let create_command = CreateOwnershipContract {
            song_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            total_shares: 1000,
            price_per_share: 10.0,
            artist_retained_percentage: 51.0,
            minimum_investment: Some(100.0),
            maximum_ownership_per_user: Some(20.0),
        };
        
        let create_result = create_handler.handle(create_command).await.unwrap();

        // Activate contract
        let activate_handler = ActivateOwnershipContractHandler { repository: repo };
        let activate_command = ActivateOwnershipContract {
            contract_id: create_result.contract_id,
        };

        let result = activate_handler.handle(activate_command).await.unwrap();
        assert_eq!(result.contract_id, create_result.contract_id);
    }

    #[tokio::test]
    async fn test_purchase_shares_command() {
        let repo = MockOwnershipContractRepository::new();
        
        // Create and activate contract
        let create_handler = CreateOwnershipContractHandler { repository: repo.clone() };
        let activate_handler = ActivateOwnershipContractHandler { repository: repo.clone() };
        let purchase_handler = PurchaseSharesHandler { repository: repo };

        let create_command = CreateOwnershipContract {
            song_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            total_shares: 1000,
            price_per_share: 10.0,
            artist_retained_percentage: 51.0,
            minimum_investment: Some(100.0),
            maximum_ownership_per_user: Some(20.0),
        };
        
        let create_result = create_handler.handle(create_command).await.unwrap();
        
        activate_handler.handle(ActivateOwnershipContract {
            contract_id: create_result.contract_id,
        }).await.unwrap();

        // Purchase shares
        let purchase_command = PurchaseShares {
            contract_id: create_result.contract_id,
            buyer_id: Uuid::new_v4(),
            ownership_percentage: 10.0,
            vesting_start_date: None,
            vesting_end_date: None,
        };

        let result = purchase_handler.handle(purchase_command).await.unwrap();
        
        assert_eq!(result.ownership_percentage, 10.0);
        assert_eq!(result.investment_amount, 1000.0); // 10% * $10 * 1000 shares
        assert!(!result.events_triggered.is_empty());
        assert!(result.share_id != Uuid::nil());
    }

    #[tokio::test]
    async fn test_purchase_shares_with_vesting() {
        let repo = MockOwnershipContractRepository::new();
        
        // Setup contract
        let create_handler = CreateOwnershipContractHandler { repository: repo.clone() };
        let activate_handler = ActivateOwnershipContractHandler { repository: repo.clone() };
        let purchase_handler = PurchaseSharesHandler { repository: repo };

        let create_command = CreateOwnershipContract {
            song_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            total_shares: 1000,
            price_per_share: 10.0,
            artist_retained_percentage: 51.0,
            minimum_investment: Some(100.0),
            maximum_ownership_per_user: Some(20.0),
        };
        
        let create_result = create_handler.handle(create_command).await.unwrap();
        activate_handler.handle(ActivateOwnershipContract {
            contract_id: create_result.contract_id,
        }).await.unwrap();

        // Purchase with vesting
        let vesting_start = Utc::now() + Duration::days(1);
        let vesting_end = vesting_start + Duration::days(365);
        
        let purchase_command = PurchaseShares {
            contract_id: create_result.contract_id,
            buyer_id: Uuid::new_v4(),
            ownership_percentage: 15.0,
            vesting_start_date: Some(vesting_start),
            vesting_end_date: Some(vesting_end),
        };

        let result = purchase_handler.handle(purchase_command).await.unwrap();
        
        assert_eq!(result.ownership_percentage, 15.0);
        assert!(result.events_triggered.contains(&"SharesPurchased".to_string()));
    }

    #[tokio::test]
    async fn test_distribute_revenue_command() {
        let repo = MockOwnershipContractRepository::new();
        
        // Setup contract with shareholders
        let create_handler = CreateOwnershipContractHandler { repository: repo.clone() };
        let activate_handler = ActivateOwnershipContractHandler { repository: repo.clone() };
        let purchase_handler = PurchaseSharesHandler { repository: repo.clone() };
        let distribute_handler = DistributeRevenueHandler { repository: repo };

        // Create and activate contract
        let create_command = CreateOwnershipContract {
            song_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            total_shares: 1000,
            price_per_share: 10.0,
            artist_retained_percentage: 51.0,
            minimum_investment: Some(100.0),
            maximum_ownership_per_user: Some(20.0),
        };
        
        let create_result = create_handler.handle(create_command).await.unwrap();
        activate_handler.handle(ActivateOwnershipContract {
            contract_id: create_result.contract_id,
        }).await.unwrap();

        // Add shareholders
        purchase_handler.handle(PurchaseShares {
            contract_id: create_result.contract_id,
            buyer_id: Uuid::new_v4(),
            ownership_percentage: 10.0,
            vesting_start_date: None,
            vesting_end_date: None,
        }).await.unwrap();

        purchase_handler.handle(PurchaseShares {
            contract_id: create_result.contract_id,
            buyer_id: Uuid::new_v4(),
            ownership_percentage: 15.0,
            vesting_start_date: None,
            vesting_end_date: None,
        }).await.unwrap();

        // Distribute revenue
        let distribute_command = DistributeRevenue {
            contract_id: create_result.contract_id,
            total_revenue: 1000.0,
            distribution_period_start: Utc::now() - Duration::days(30),
            distribution_period_end: Utc::now(),
            platform_fee_percentage: 5.0,
        };

        let result = distribute_handler.handle(distribute_command).await.unwrap();
        
        assert_eq!(result.total_revenue, 1000.0);
        assert!(result.total_distributed > 0.0);
        assert!(result.artist_share > 0.0);
        assert!(result.platform_fee > 0.0);
        assert_eq!(result.shareholder_count, 2);
    }
} 