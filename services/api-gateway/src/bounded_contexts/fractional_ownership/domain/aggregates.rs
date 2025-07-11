// TODO: Implement fractional ownership aggregates 

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::music::domain::value_objects::{SongId, ArtistId};
use crate::bounded_contexts::user::domain::value_objects::UserId;

use super::value_objects::{
    OwnershipContractId, OwnershipPercentage, SharePrice, RevenueAmount, 
    ShareId, VestingPeriod
};
use super::entities::{FractionalShare, RevenueDistribution, DistributionStatus};
use super::events::{
    OwnershipContractCreated, SharesPurchased, SharesTraded, RevenueDistributed,
    OwnershipContractUpdated, InvestmentThresholdReached, ThresholdType,
    OwnershipContractTerminated, TerminationReason, PaymentRequested, PaymentType,
    PaymentMetadata, UserPortfolioUpdated, PortfolioUpdateType
};

/// Aggregate Root: Manages ownership contracts and fractional shares
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipContractAggregate {
    contract: OwnershipContract,
    shares: HashMap<ShareId, FractionalShare>,
    revenue_distributions: Vec<RevenueDistribution>,
    pending_events: Vec<String>,
    version: u64,
}

/// Main entity within the aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipContract {
    id: OwnershipContractId,
    song_id: SongId,
    artist_id: ArtistId,
    total_shares: u32,
    price_per_share: SharePrice,
    artist_retained_percentage: OwnershipPercentage,
    shares_available_for_sale: u32,
    shares_sold: u32,
    minimum_investment: Option<RevenueAmount>,
    maximum_ownership_per_user: Option<OwnershipPercentage>,
    contract_status: ContractStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContractStatus {
    Draft,
    Active,
    Paused,
    SoldOut,
    Terminated,
}

impl std::fmt::Display for ContractStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContractStatus::Draft => write!(f, "Draft"),
            ContractStatus::Active => write!(f, "Active"),
            ContractStatus::Paused => write!(f, "Paused"),
            ContractStatus::SoldOut => write!(f, "SoldOut"),
            ContractStatus::Terminated => write!(f, "Terminated"),
        }
    }
}

impl OwnershipContract {
    // Getter methods for accessing private fields
    pub fn id(&self) -> &OwnershipContractId { &self.id }
    pub fn song_id(&self) -> &SongId { &self.song_id }
    pub fn artist_id(&self) -> &ArtistId { &self.artist_id }
    pub fn total_shares(&self) -> u32 { self.total_shares }
    pub fn price_per_share(&self) -> &SharePrice { &self.price_per_share }
    pub fn artist_retained_percentage(&self) -> &OwnershipPercentage { &self.artist_retained_percentage }
    pub fn shares_available_for_sale(&self) -> u32 { self.shares_available_for_sale }
    pub fn shares_sold(&self) -> u32 { self.shares_sold }
    pub fn minimum_investment(&self) -> Option<&RevenueAmount> { self.minimum_investment.as_ref() }
    pub fn maximum_ownership_per_user(&self) -> Option<&OwnershipPercentage> { self.maximum_ownership_per_user.as_ref() }
    pub fn contract_status(&self) -> &ContractStatus { &self.contract_status }
    pub fn created_at(&self) -> DateTime<Utc> { self.created_at }
    pub fn updated_at(&self) -> DateTime<Utc> { self.updated_at }
}

impl OwnershipContractAggregate {
    // Additional getter methods
    pub fn contract(&self) -> &OwnershipContract { &self.contract }
    pub fn song_id(&self) -> &SongId { &self.contract.song_id }
    pub fn artist_id(&self) -> &ArtistId { &self.contract.artist_id }
    pub fn is_active(&self) -> bool { 
        matches!(self.contract.contract_status, ContractStatus::Active) 
    }
    pub fn status(&self) -> &ContractStatus { &self.contract.contract_status }
    pub fn total_value(&self) -> f64 {
        self.contract.price_per_share.value() * self.contract.total_shares as f64
    }
}

impl OwnershipContractAggregate {
    /// Create a new ownership contract - Domain Factory
    pub fn create_contract(
        song_id: SongId,
        artist_id: ArtistId,
        total_shares: u32,
        price_per_share: SharePrice,
        artist_retained_percentage: OwnershipPercentage,
        minimum_investment: Option<RevenueAmount>,
        maximum_ownership_per_user: Option<OwnershipPercentage>,
    ) -> Result<Self, AppError> {
        // Domain Rules Validation
        if total_shares == 0 {
            return Err(AppError::DomainRuleViolation(
                "Total shares must be greater than 0".to_string(),
            ));
        }

        if artist_retained_percentage.value() < 1.0 || artist_retained_percentage.value() > 99.0 {
            return Err(AppError::DomainRuleViolation(
                "Artist must retain between 1% and 99% ownership".to_string(),
            ));
        }

        if price_per_share.value() <= 0.0 {
            return Err(AppError::DomainRuleViolation(
                "Price per share must be greater than 0".to_string(),
            ));
        }

        let now = Utc::now();
        let contract_id = OwnershipContractId::new();

        // Calculate shares available for sale
        let shares_for_sale = ((100.0 - artist_retained_percentage.value()) / 100.0 * total_shares as f64) as u32;

        let contract = OwnershipContract {
            id: contract_id.clone(),
            song_id: song_id.clone(),
            artist_id: artist_id.clone(),
            total_shares,
            price_per_share: price_per_share.clone(),
            artist_retained_percentage: artist_retained_percentage.clone(),
            shares_available_for_sale: shares_for_sale,
            shares_sold: 0,
            minimum_investment: minimum_investment.clone(),
            maximum_ownership_per_user: maximum_ownership_per_user.clone(),
            contract_status: ContractStatus::Draft,
            created_at: now,
            updated_at: now,
        };

        let mut aggregate = Self {
            contract,
            shares: HashMap::new(),
            revenue_distributions: Vec::new(),
            pending_events: Vec::new(),
            version: 1,
        };

        aggregate.add_event("OwnershipContractCreated".to_string());
        Ok(aggregate)
    }

    /// Activate the contract for public investment
    pub fn activate_contract(&mut self) -> Result<(), AppError> {
        if !matches!(self.contract.contract_status, ContractStatus::Draft) {
            return Err(AppError::DomainRuleViolation(
                "Only draft contracts can be activated".to_string(),
            ));
        }

        self.contract.contract_status = ContractStatus::Active;
        self.contract.updated_at = Utc::now();
        self.add_event("OwnershipContractActivated".to_string());
        self.increment_version();
        Ok(())
    }

    /// Purchase shares - Core Domain Behavior
    pub fn purchase_shares(
        &mut self,
        buyer_id: UserId,
        ownership_percentage: OwnershipPercentage,
        vesting_period: Option<VestingPeriod>,
    ) -> Result<(FractionalShare, Vec<String>), AppError> {
        // Domain Rules Validation
        if !matches!(self.contract.contract_status, ContractStatus::Active) {
            return Err(AppError::DomainRuleViolation(
                "Contract is not active for investment".to_string(),
            ));
        }

        // Check if enough shares are available
        let requested_shares = ((ownership_percentage.value() / 100.0) * self.contract.total_shares as f64) as u32;
        if requested_shares > self.shares_available() {
            return Err(AppError::DomainRuleViolation(
                format!("Not enough shares available. Requested: {}, Available: {}", 
                    requested_shares, self.shares_available())
            ));
        }

        // Check minimum investment
        let investment_amount = self.contract.price_per_share.multiply_by_percentage(&ownership_percentage);
        if let Some(min_investment) = &self.contract.minimum_investment {
            if investment_amount < min_investment.value() {
                return Err(AppError::DomainRuleViolation(
                    format!("Investment amount {} is below minimum {}", 
                        investment_amount, min_investment.value())
                ));
            }
        }

        // Check maximum ownership per user
        if let Some(max_ownership) = &self.contract.maximum_ownership_per_user {
            let current_user_ownership = self.get_user_total_ownership(&buyer_id);
            let new_total = current_user_ownership.add(&ownership_percentage)?;
            if new_total.value() > max_ownership.value() {
                return Err(AppError::DomainRuleViolation(
                    format!("Purchase would exceed maximum ownership of {}%", max_ownership.value())
                ));
            }
        }

        // Create the fractional share
        let purchase_price = SharePrice::new(investment_amount)?;
        let (share, purchase_event) = FractionalShare::create(
            self.contract.id.clone(),
            buyer_id.clone(),
            self.contract.song_id.clone(),
            ownership_percentage.clone(),
            purchase_price,
            vesting_period,
        )?;

        // Update contract state
        self.contract.shares_sold += requested_shares;
        self.contract.updated_at = Utc::now();

        // Add share to aggregate
        self.shares.insert(share.id().clone(), share.clone());

        // Collect events
        let mut events = vec!["SharesPurchased".to_string()];

        // Check for thresholds
        if let Some(threshold_event) = self.check_investment_thresholds()? {
            events.push("InvestmentThresholdReached".to_string());
        }

        // Check if sold out
        if self.shares_available() == 0 {
            self.contract.contract_status = ContractStatus::SoldOut;
            events.push("ContractSoldOut".to_string());
        }

        // Add payment request event
        events.push("PaymentRequested".to_string());

        // Add portfolio update event
        events.push("UserPortfolioUpdated".to_string());

        for event in &events {
            self.add_event(event.clone());
        }

        self.increment_version();
        Ok((share, events))
    }

    /// Trade shares between users
    pub fn trade_shares(
        &mut self,
        share_id: ShareId,
        new_owner: UserId,
        trade_price: SharePrice,
    ) -> Result<Vec<String>, AppError> {
        // First, check ownership limits before mutable borrowing
        if let Some(max_ownership) = &self.contract.maximum_ownership_per_user {
            let current_ownership = self.get_user_total_ownership(&new_owner);
            
            // Get the share ownership percentage separately to avoid borrowing conflicts
            let share_ownership = self.shares.get(&share_id)
                .ok_or_else(|| AppError::NotFound("Share not found".to_string()))?
                .ownership_percentage();
                
            let new_total = current_ownership.add(share_ownership)?;
            if new_total.value() > max_ownership.value() {
                return Err(AppError::DomainRuleViolation(
                    "Trade would exceed maximum ownership for buyer".to_string(),
                ));
            }
        }

        // Now safely get mutable reference
        let share = self.shares.get_mut(&share_id)
            .ok_or_else(|| AppError::NotFound("Share not found".to_string()))?;

        let _trade_event = share.transfer_to(new_owner.clone(), trade_price)?;
        
        let events = vec![
            "SharesTraded".to_string(),
            "PaymentRequested".to_string(),
            "UserPortfolioUpdated".to_string(),
        ];

        for event in &events {
            self.add_event(event.clone());
        }

        self.increment_version();
        Ok(events)
    }

    /// Distribute revenue to shareholders
    pub fn distribute_revenue(
        &mut self,
        total_revenue: RevenueAmount,
        distribution_period_start: DateTime<Utc>,
        distribution_period_end: DateTime<Utc>,
        platform_fee_percentage: f64,
    ) -> Result<RevenueDistributed, AppError> {
        // Create revenue distribution
        let mut distribution = RevenueDistribution::create(
            self.contract.id.clone(),
            self.contract.song_id.clone(),
            distribution_period_start,
            distribution_period_end,
            total_revenue,
            platform_fee_percentage,
            self.contract.artist_retained_percentage.value(),
        )?;

        // Get all active shares
        let active_shares: Vec<FractionalShare> = self.shares.values().cloned().collect();
        
        // Process distribution
        let distribution_event = distribution.process_distribution(&active_shares)?;

        // Update shares with revenue received
        for shareholder_dist in distribution.shareholder_distributions() {
            for share in self.shares.values_mut() {
                if share.owner_id().value() == shareholder_dist.shareholder_id {
                    let revenue = RevenueAmount::new(shareholder_dist.revenue_share)?;
                    share.receive_revenue(revenue)?;
                }
            }
        }

        // Store distribution
        self.revenue_distributions.push(distribution);

        self.add_event("RevenueDistributed".to_string());
        self.add_event("PaymentRequested".to_string());
        self.add_event("UserPortfolioUpdated".to_string());
        self.increment_version();

        Ok(distribution_event)
    }

    /// Terminate the contract
    pub fn terminate_contract(
        &mut self,
        reason: TerminationReason,
        terminated_by: UserId,
    ) -> Result<OwnershipContractTerminated, AppError> {
        if matches!(self.contract.contract_status, ContractStatus::Terminated) {
            return Err(AppError::DomainRuleViolation(
                "Contract is already terminated".to_string(),
            ));
        }

        self.contract.contract_status = ContractStatus::Terminated;
        self.contract.updated_at = Utc::now();

        // Calculate final distributions (simplified)
        let final_distributions = Vec::new(); // Would calculate final payouts

        let event = OwnershipContractTerminated {
            aggregate_id: self.contract.id.value(),
            contract_id: self.contract.id.value(),
            song_id: *self.contract.song_id.value(),
            termination_reason: reason,
            final_distributions,
            terminated_by: terminated_by.value(),
            terminated_at: Utc::now(),
            occurred_on: Utc::now(),
        };

        self.add_event("OwnershipContractTerminated".to_string());
        self.increment_version();

        Ok(event)
    }

    // Domain Queries
    pub fn shares_available(&self) -> u32 {
        self.contract.shares_available_for_sale - self.contract.shares_sold
    }

    pub fn total_investment_value(&self) -> f64 {
        self.contract.price_per_share.value() * self.contract.shares_sold as f64
    }

    pub fn completion_percentage(&self) -> f64 {
        if self.contract.shares_available_for_sale == 0 {
            return 100.0;
        }
        (self.contract.shares_sold as f64 / self.contract.shares_available_for_sale as f64) * 100.0
    }

    pub fn get_user_total_ownership(&self, user_id: &UserId) -> OwnershipPercentage {
        let total: f64 = self.shares.values()
            .filter(|s| s.owner_id().value() == user_id.value())
            .map(|s| s.ownership_percentage().value())
            .sum();
        
        OwnershipPercentage::new(total).unwrap_or_else(|_| OwnershipPercentage::new(0.0).unwrap())
    }

    pub fn get_user_shares(&self, user_id: &UserId) -> Vec<&FractionalShare> {
        self.shares.values()
            .filter(|s| s.owner_id().value() == user_id.value())
            .collect()
    }

    pub fn get_unique_shareholders(&self) -> Vec<UserId> {
        let mut shareholders: Vec<UserId> = self.shares.values()
            .map(|s| s.owner_id().clone())
            .collect();
        shareholders.sort_by_key(|id| id.value());
        shareholders.dedup_by_key(|id| id.value());
        shareholders
    }

    pub fn is_fully_funded(&self) -> bool {
        self.shares_available() == 0
    }

    pub fn can_accept_investment(&self) -> bool {
        matches!(self.contract.contract_status, ContractStatus::Active) && !self.is_fully_funded()
    }

    // Private helper methods
    fn check_investment_thresholds(&self) -> Result<Option<InvestmentThresholdReached>, AppError> {
        // Check for milestones (e.g., 25%, 50%, 75% funded)
        let completion = self.completion_percentage();
        
        if completion >= 25.0 && completion < 50.0 {
            return Ok(Some(InvestmentThresholdReached {
                aggregate_id: self.contract.id.value(),
                contract_id: self.contract.id.value(),
                song_id: *self.contract.song_id.value(),
                threshold_type: ThresholdType::MinimumInvestment,
                threshold_value: 25.0,
                current_value: completion,
                reached_at: Utc::now(),
                occurred_on: Utc::now(),
            }));
        }

        Ok(None)
    }

    fn add_event(&mut self, event_name: String) {
        self.pending_events.push(event_name);
    }

    fn increment_version(&mut self) {
        self.version += 1;
    }

    // Getters
    pub fn shares(&self) -> &HashMap<ShareId, FractionalShare> { &self.shares }
    pub fn revenue_distributions(&self) -> &[RevenueDistribution] { &self.revenue_distributions }
    pub fn pending_events(&self) -> &[String] { &self.pending_events }
    pub fn clear_events(&mut self) { self.pending_events.clear(); }
    pub fn version(&self) -> u64 { self.version }
    pub fn id(&self) -> &OwnershipContractId { &self.contract.id }
}

/// Analytics data for ownership contracts
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OwnershipAnalytics {
    pub contract_id: OwnershipContractId,
    pub total_investment_value: f64,
    pub completion_percentage: f64,
    pub number_of_shareholders: u32,
    pub average_share_size: f64,
    pub revenue_distributed_to_date: f64,
    pub projected_annual_return: f64,
    pub liquidity_score: f64,
    pub performance_metrics: PerformanceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceMetrics {
    pub roi_percentage: f64,
    pub volatility_score: f64,
    pub trading_volume: f64,
    pub market_sentiment: f64,
}

impl OwnershipContractAggregate {
    pub fn get_analytics(&self) -> OwnershipAnalytics {
        let unique_shareholders = self.get_unique_shareholders();
        let total_investment = self.total_investment_value();
        
        let average_investment = if unique_shareholders.len() > 0 {
            total_investment / unique_shareholders.len() as f64
        } else {
            0.0
        };

        let largest_ownership = self.shares.values()
            .map(|s| s.ownership_percentage().value())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);

        let total_revenue_distributed: f64 = self.revenue_distributions.iter()
            .filter(|d| d.is_processed())
            .map(|d| d.total_revenue().value())
            .sum();

        let artist_total_revenue: f64 = self.revenue_distributions.iter()
            .filter(|d| d.is_processed())
            .map(|d| d.total_revenue().value() * self.contract.artist_retained_percentage.value() / 100.0)
            .sum();

        let platform_total_fees: f64 = self.revenue_distributions.iter()
            .filter(|d| d.is_processed())
            .map(|d| d.total_shareholder_amount())
            .sum();

        OwnershipAnalytics {
            contract_id: self.contract.id.clone(),
            total_investment_value: total_investment,
            completion_percentage: self.completion_percentage(),
            number_of_shareholders: unique_shareholders.len() as u32,
            average_share_size: average_investment,
            revenue_distributed_to_date: total_revenue_distributed,
            projected_annual_return: 0.0, // Placeholder
            liquidity_score: 0.0, // Placeholder
            performance_metrics: PerformanceMetrics::default(), // Placeholder
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn create_test_aggregate() -> Result<OwnershipContractAggregate, AppError> {
        OwnershipContractAggregate::create_contract(
            SongId::new(),
            ArtistId::new(),
            1000, // total shares
            SharePrice::new(10.0)?, // $10 per share
            OwnershipPercentage::new(51.0)?, // Artist retains 51%
            Some(RevenueAmount::new(100.0)?), // Min investment $100
            Some(OwnershipPercentage::new(20.0)?), // Max 20% per user
        )
    }

    #[test]
    fn test_contract_creation() {
        let aggregate = create_test_aggregate().unwrap();
        
        assert_eq!(aggregate.contract().total_shares, 1000);
        assert_eq!(aggregate.contract().price_per_share.value(), 10.0);
        assert_eq!(aggregate.contract().artist_retained_percentage.value(), 51.0);
        assert_eq!(aggregate.shares_available(), 490); // 49% of 1000 shares
        assert!(matches!(aggregate.contract().contract_status, ContractStatus::Draft));
    }

    #[test]
    fn test_contract_activation() {
        let mut aggregate = create_test_aggregate().unwrap();
        
        aggregate.activate_contract().unwrap();
        assert!(matches!(aggregate.contract().contract_status, ContractStatus::Active));
        assert!(aggregate.can_accept_investment());
    }

    #[test]
    fn test_share_purchase() {
        let mut aggregate = create_test_aggregate().unwrap();
        aggregate.activate_contract().unwrap();

        let buyer = UserId::new();
        let ownership = OwnershipPercentage::new(10.0).unwrap(); // 10% = 100 shares = $1000

        let (share, events) = aggregate.purchase_shares(buyer.clone(), ownership, None).unwrap();
        
        assert_eq!(share.ownership_percentage().value(), 10.0);
        assert_eq!(share.purchase_price().value(), 1000.0);
        assert_eq!(aggregate.shares_available(), 390); // 490 - 100
        assert!(!events.is_empty());
    }

    #[test]
    fn test_minimum_investment_validation() {
        let mut aggregate = create_test_aggregate().unwrap();
        aggregate.activate_contract().unwrap();

        let buyer = UserId::new();
        let small_ownership = OwnershipPercentage::new(0.5).unwrap(); // 0.5% = $50, below $100 minimum

        let result = aggregate.purchase_shares(buyer, small_ownership, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_maximum_ownership_validation() {
        let mut aggregate = create_test_aggregate().unwrap();
        aggregate.activate_contract().unwrap();

        let buyer = UserId::new();
        let large_ownership = OwnershipPercentage::new(25.0).unwrap(); // Above 20% maximum

        let result = aggregate.purchase_shares(buyer, large_ownership, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_share_trading() {
        let mut aggregate = create_test_aggregate().unwrap();
        aggregate.activate_contract().unwrap();

        // First user buys shares
        let buyer1 = UserId::new();
        let ownership = OwnershipPercentage::new(10.0).unwrap();
        let (share, _) = aggregate.purchase_shares(buyer1.clone(), ownership, None).unwrap();

        // Trade to second user
        let buyer2 = UserId::new();
        let trade_price = SharePrice::new(1200.0).unwrap(); // 20% profit
        
        let events = aggregate.trade_shares(share.id().clone(), buyer2.clone(), trade_price).unwrap();
        
        let traded_share = aggregate.shares.get(share.id()).unwrap();
        assert_eq!(traded_share.owner_id().value(), buyer2.value());
        assert_eq!(traded_share.current_market_value().value(), 1200.0);
        assert!(!events.is_empty());
    }

    #[test]
    fn test_revenue_distribution() {
        let mut aggregate = create_test_aggregate().unwrap();
        aggregate.activate_contract().unwrap();

        // Add some shareholders
        let buyer1 = UserId::new();
        let buyer2 = UserId::new();
        
        aggregate.purchase_shares(buyer1, OwnershipPercentage::new(10.0).unwrap(), None).unwrap();
        aggregate.purchase_shares(buyer2, OwnershipPercentage::new(15.0).unwrap(), None).unwrap();

        // Distribute revenue
        let total_revenue = RevenueAmount::new(1000.0).unwrap();
        let event = aggregate.distribute_revenue(
            total_revenue,
            Utc::now() - Duration::days(30),
            Utc::now(),
            5.0, // 5% platform fee
        ).unwrap();

        assert_eq!(event.total_revenue, 1000.0);
        assert!(event.shareholder_distributions.len() > 0);
        assert_eq!(aggregate.revenue_distributions.len(), 1);
    }

    #[test]
    fn test_analytics() {
        let mut aggregate = create_test_aggregate().unwrap();
        aggregate.activate_contract().unwrap();

        // Add shareholders
        let buyer1 = UserId::new();
        let buyer2 = UserId::new();
        
        aggregate.purchase_shares(buyer1, OwnershipPercentage::new(10.0).unwrap(), None).unwrap();
        aggregate.purchase_shares(buyer2, OwnershipPercentage::new(15.0).unwrap(), None).unwrap();

        let analytics = aggregate.get_analytics();
        
        // Use the correct field names that exist in OwnershipAnalytics
        assert_eq!(analytics.total_investment_value, 2500.0); // $1000 + $1500
        assert_eq!(analytics.number_of_shareholders, 2);
        assert_eq!(analytics.average_share_size, 1250.0);
        assert_eq!(analytics.completion_percentage, 25.0); // 25% of shares sold
    }

    #[test]
    fn test_contract_sold_out() {
        let mut aggregate = create_test_aggregate().unwrap();
        aggregate.activate_contract().unwrap();

        // Buy all available shares (49% = 490 shares)
        let buyer = UserId::new();
        let all_shares = OwnershipPercentage::new(49.0).unwrap();
        
        let (_, events) = aggregate.purchase_shares(buyer, all_shares, None).unwrap();
        
        assert!(matches!(aggregate.contract().contract_status, ContractStatus::SoldOut));
        assert!(!aggregate.can_accept_investment());
        assert!(events.contains(&"ContractSoldOut".to_string()));
    }

    #[test]
    fn test_contract_termination() {
        let mut aggregate = create_test_aggregate().unwrap();
        
        let terminator = UserId::new();
        let event = aggregate.terminate_contract(
            TerminationReason::ArtistRequest,
            terminator.clone(),
        ).unwrap();

        assert!(matches!(aggregate.contract().contract_status, ContractStatus::Terminated));
        assert_eq!(event.terminated_by, terminator.value());
        assert_eq!(event.termination_reason, TerminationReason::ArtistRequest);
    }
} 