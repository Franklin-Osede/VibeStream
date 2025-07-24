// TODO: Implement fractional ownership events 

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


/// Event emitted when a new ownership contract is created for a song
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipContractCreated {
    pub aggregate_id: Uuid,
    pub contract_id: Uuid,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub total_shares: u32,
    pub price_per_share: f64,
    pub artist_retained_percentage: f64,
    pub shares_available_for_sale: u32,
    pub created_at: DateTime<Utc>,
    pub occurred_on: DateTime<Utc>,
}

/// Event emitted when shares are purchased by a fan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharesPurchased {
    pub aggregate_id: Uuid,
    pub contract_id: Uuid,
    pub share_id: Uuid,
    pub buyer_id: Uuid,
    pub song_id: Uuid,
    pub ownership_percentage: f64,
    pub purchase_price: f64,
    pub transaction_hash: Option<String>,
    pub purchased_at: DateTime<Utc>,
    pub occurred_on: DateTime<Utc>,
}

/// Event emitted when shares are traded between users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharesTraded {
    pub aggregate_id: Uuid,
    pub contract_id: Uuid,
    pub share_id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub song_id: Uuid,
    pub ownership_percentage: f64,
    pub trade_price: f64,
    pub transaction_hash: Option<String>,
    pub traded_at: DateTime<Utc>,
    pub occurred_on: DateTime<Utc>,
}

/// Event emitted when revenue is distributed to shareholders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueDistributed {
    pub aggregate_id: Uuid,
    pub contract_id: Uuid,
    pub song_id: Uuid,
    pub total_revenue: f64,
    pub distribution_period_start: DateTime<Utc>,
    pub distribution_period_end: DateTime<Utc>,
    pub total_distributed: f64,
    pub artist_share: f64,
    pub platform_fee: f64,
    pub shareholder_distributions: Vec<ShareholderDistribution>,
    pub distributed_at: DateTime<Utc>,
    pub occurred_on: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareholderDistribution {
    pub shareholder_id: Uuid,
    pub ownership_percentage: f64,
    pub revenue_share: f64,
    pub transaction_hash: Option<String>,
}

/// Event emitted when ownership contract terms are updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipContractUpdated {
    pub aggregate_id: Uuid,
    pub contract_id: Uuid,
    pub song_id: Uuid,
    pub updated_fields: Vec<String>,
    pub updated_by: Uuid,
    pub updated_at: DateTime<Utc>,
    pub occurred_on: DateTime<Utc>,
}

/// Event emitted when shares are locked (e.g., during vesting period)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharesLocked {
    pub aggregate_id: Uuid,
    pub share_id: Uuid,
    pub contract_id: Uuid,
    pub owner_id: Uuid,
    pub ownership_percentage: f64,
    pub lock_reason: ShareLockReason,
    pub lock_until: Option<DateTime<Utc>>,
    pub locked_at: DateTime<Utc>,
    pub occurred_on: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ShareLockReason {
    VestingPeriod,
    LegalDispute,
    Maintenance,
    ArtistRequest,
}

/// Event emitted when locked shares are unlocked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharesUnlocked {
    pub aggregate_id: Uuid,
    pub share_id: Uuid,
    pub contract_id: Uuid,
    pub owner_id: Uuid,
    pub ownership_percentage: f64,
    pub unlock_reason: String,
    pub unlocked_at: DateTime<Utc>,
    pub occurred_on: DateTime<Utc>,
}

/// Event emitted when minimum investment threshold is reached
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentThresholdReached {
    pub aggregate_id: Uuid,
    pub contract_id: Uuid,
    pub song_id: Uuid,
    pub threshold_type: ThresholdType,
    pub threshold_value: f64,
    pub current_value: f64,
    pub reached_at: DateTime<Utc>,
    pub occurred_on: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThresholdType {
    MinimumInvestment,
    MaximumOwnership,
    RevenueMilestone,
}

/// Event emitted when ownership contract is terminated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipContractTerminated {
    pub aggregate_id: Uuid,
    pub contract_id: Uuid,
    pub song_id: Uuid,
    pub termination_reason: TerminationReason,
    pub final_distributions: Vec<ShareholderDistribution>,
    pub terminated_by: Uuid,
    pub terminated_at: DateTime<Utc>,
    pub occurred_on: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TerminationReason {
    ArtistRequest,
    LegalIssues,
    InsufficientFunding,
    ContractExpired,
    MutualAgreement,
}

// Integration Events - for cross-context communication

/// Event emitted to trigger payment processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRequested {
    pub aggregate_id: Uuid,
    pub payment_type: PaymentType,
    pub from_user_id: Uuid,
    pub to_user_id: Option<Uuid>,
    pub amount: f64,
    pub currency: String,
    pub metadata: PaymentMetadata,
    pub requested_at: DateTime<Utc>,
    pub occurred_on: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentType {
    SharePurchase,
    ShareTrade,
    RevenueDistribution,
    Refund,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMetadata {
    pub contract_id: Uuid,
    pub share_id: Option<Uuid>,
    pub song_id: Uuid,
    pub transaction_type: String,
    pub additional_data: serde_json::Value,
}

/// Event emitted to update user portfolio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPortfolioUpdated {
    pub aggregate_id: Uuid,
    pub user_id: Uuid,
    pub contract_id: Uuid,
    pub song_id: Uuid,
    pub update_type: PortfolioUpdateType,
    pub ownership_percentage_change: f64,
    pub value_change: f64,
    pub updated_at: DateTime<Utc>,
    pub occurred_on: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PortfolioUpdateType {
    SharesAdded,
    SharesRemoved,
    SharesTraded,
    ValueUpdated,
    RevenueReceived,
}

// Implement common event trait for type safety
impl crate::shared::domain::events::DomainEvent for OwnershipContractCreated {
    fn metadata(&self) -> &crate::shared::domain::events::EventMetadata {
        unimplemented!("EventMetadata not implemented for OwnershipContractCreated")
    }
    
    fn event_type(&self) -> &str {
        "OwnershipContractCreated"
    }
    
    fn aggregate_id(&self) -> Uuid {
        self.aggregate_id
    }
    
    fn aggregate_type(&self) -> &str {
        "OwnershipContract"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_on
    }
    
    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

impl crate::shared::domain::events::DomainEvent for SharesPurchased {
    fn metadata(&self) -> &crate::shared::domain::events::EventMetadata {
        unimplemented!("EventMetadata not implemented for SharesPurchased")
    }
    
    fn event_type(&self) -> &str {
        "SharesPurchased"
    }
    
    fn aggregate_id(&self) -> Uuid {
        self.aggregate_id
    }
    
    fn aggregate_type(&self) -> &str {
        "OwnershipContract"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_on
    }
    
    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

impl crate::shared::domain::events::DomainEvent for SharesTraded {
    fn metadata(&self) -> &crate::shared::domain::events::EventMetadata {
        unimplemented!("EventMetadata not implemented for SharesTraded")
    }
    
    fn event_type(&self) -> &str {
        "SharesTraded"
    }
    
    fn aggregate_id(&self) -> Uuid {
        self.aggregate_id
    }
    
    fn aggregate_type(&self) -> &str {
        "OwnershipContract"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_on
    }
    
    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

impl crate::shared::domain::events::DomainEvent for RevenueDistributed {
    fn metadata(&self) -> &crate::shared::domain::events::EventMetadata {
        unimplemented!("EventMetadata not implemented for RevenueDistributed")
    }
    
    fn event_type(&self) -> &str {
        "RevenueDistributed"
    }
    
    fn aggregate_id(&self) -> Uuid {
        self.aggregate_id
    }
    
    fn aggregate_type(&self) -> &str {
        "OwnershipContract"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_on
    }
    
    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

impl crate::shared::domain::events::DomainEvent for OwnershipContractUpdated {
    fn metadata(&self) -> &crate::shared::domain::events::EventMetadata {
        unimplemented!("EventMetadata not implemented for OwnershipContractUpdated")
    }
    
    fn event_type(&self) -> &str {
        "OwnershipContractUpdated"
    }
    
    fn aggregate_id(&self) -> Uuid {
        self.aggregate_id
    }
    
    fn aggregate_type(&self) -> &str {
        "OwnershipContract"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_on
    }
    
    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

impl crate::shared::domain::events::DomainEvent for SharesLocked {
    fn metadata(&self) -> &crate::shared::domain::events::EventMetadata {
        unimplemented!("EventMetadata not implemented for SharesLocked")
    }
    
    fn event_type(&self) -> &str {
        "SharesLocked"
    }
    
    fn aggregate_id(&self) -> Uuid {
        self.aggregate_id
    }
    
    fn aggregate_type(&self) -> &str {
        "OwnershipContract"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_on
    }
    
    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

impl crate::shared::domain::events::DomainEvent for SharesUnlocked {
    fn metadata(&self) -> &crate::shared::domain::events::EventMetadata {
        unimplemented!("EventMetadata not implemented for SharesUnlocked")
    }
    
    fn event_type(&self) -> &str {
        "SharesUnlocked"
    }
    
    fn aggregate_id(&self) -> Uuid {
        self.aggregate_id
    }
    
    fn aggregate_type(&self) -> &str {
        "OwnershipContract"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_on
    }
    
    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

impl crate::shared::domain::events::DomainEvent for InvestmentThresholdReached {
    fn metadata(&self) -> &crate::shared::domain::events::EventMetadata {
        unimplemented!("EventMetadata not implemented for InvestmentThresholdReached")
    }
    
    fn event_type(&self) -> &str {
        "InvestmentThresholdReached"
    }
    
    fn aggregate_id(&self) -> Uuid {
        self.aggregate_id
    }
    
    fn aggregate_type(&self) -> &str {
        "OwnershipContract"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_on
    }
    
    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

impl crate::shared::domain::events::DomainEvent for OwnershipContractTerminated {
    fn metadata(&self) -> &crate::shared::domain::events::EventMetadata {
        unimplemented!("EventMetadata not implemented for OwnershipContractTerminated")
    }
    
    fn event_type(&self) -> &str {
        "OwnershipContractTerminated"
    }
    
    fn aggregate_id(&self) -> Uuid {
        self.aggregate_id
    }
    
    fn aggregate_type(&self) -> &str {
        "OwnershipContract"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_on
    }
    
    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

impl crate::shared::domain::events::DomainEvent for PaymentRequested {
    fn metadata(&self) -> &crate::shared::domain::events::EventMetadata {
        unimplemented!("EventMetadata not implemented for PaymentRequested")
    }
    
    fn event_type(&self) -> &str {
        "PaymentRequested"
    }
    
    fn aggregate_id(&self) -> Uuid {
        self.aggregate_id
    }
    
    fn aggregate_type(&self) -> &str {
        "OwnershipContract"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_on
    }
    
    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

impl crate::shared::domain::events::DomainEvent for UserPortfolioUpdated {
    fn metadata(&self) -> &crate::shared::domain::events::EventMetadata {
        unimplemented!("EventMetadata not implemented for UserPortfolioUpdated")
    }
    
    fn event_type(&self) -> &str {
        "UserPortfolioUpdated"
    }
    
    fn aggregate_id(&self) -> Uuid {
        self.aggregate_id
    }
    
    fn aggregate_type(&self) -> &str {
        "OwnershipContract"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_on
    }
    
    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership_contract_created_event() {
        let event = OwnershipContractCreated {
            aggregate_id: Uuid::new_v4(),
            contract_id: Uuid::new_v4(),
            song_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            total_shares: 1000,
            price_per_share: 10.0,
            artist_retained_percentage: 51.0,
            shares_available_for_sale: 490,
            created_at: Utc::now(),
            occurred_on: Utc::now(),
        };

        assert_eq!(event.total_shares, 1000);
        assert_eq!(event.price_per_share, 10.0);
        assert_eq!(event.artist_retained_percentage, 51.0);
    }

    #[test]
    fn test_shares_purchased_event() {
        let event = SharesPurchased {
            aggregate_id: Uuid::new_v4(),
            contract_id: Uuid::new_v4(),
            share_id: Uuid::new_v4(),
            buyer_id: Uuid::new_v4(),
            song_id: Uuid::new_v4(),
            ownership_percentage: 5.0,
            purchase_price: 500.0,
            transaction_hash: Some("0x123...".to_string()),
            purchased_at: Utc::now(),
            occurred_on: Utc::now(),
        };

        assert_eq!(event.ownership_percentage, 5.0);
        assert_eq!(event.purchase_price, 500.0);
        assert!(event.transaction_hash.is_some());
    }

    #[test]
    fn test_revenue_distributed_event() {
        let distributions = vec![
            ShareholderDistribution {
                shareholder_id: Uuid::new_v4(),
                ownership_percentage: 10.0,
                revenue_share: 100.0,
                transaction_hash: Some("0x456...".to_string()),
            },
            ShareholderDistribution {
                shareholder_id: Uuid::new_v4(),
                ownership_percentage: 5.0,
                revenue_share: 50.0,
                transaction_hash: Some("0x789...".to_string()),
            },
        ];

        let event = RevenueDistributed {
            aggregate_id: Uuid::new_v4(),
            contract_id: Uuid::new_v4(),
            song_id: Uuid::new_v4(),
            total_revenue: 1000.0,
            distribution_period_start: Utc::now() - chrono::Duration::days(30),
            distribution_period_end: Utc::now(),
            total_distributed: 850.0,
            artist_share: 510.0,
            platform_fee: 50.0,
            shareholder_distributions: distributions.clone(),
            distributed_at: Utc::now(),
            occurred_on: Utc::now(),
        };

        assert_eq!(event.total_revenue, 1000.0);
        assert_eq!(event.shareholder_distributions.len(), 2);
        assert_eq!(event.artist_share, 510.0);
        assert_eq!(event.platform_fee, 50.0);
    }

    #[test]
    fn test_payment_requested_integration_event() {
        let metadata = PaymentMetadata {
            contract_id: Uuid::new_v4(),
            share_id: Some(Uuid::new_v4()),
            song_id: Uuid::new_v4(),
            transaction_type: "share_purchase".to_string(),
            additional_data: serde_json::json!({"boost_multiplier": 1.5}),
        };

        let event = PaymentRequested {
            aggregate_id: Uuid::new_v4(),
            payment_type: PaymentType::SharePurchase,
            from_user_id: Uuid::new_v4(),
            to_user_id: Some(Uuid::new_v4()),
            amount: 500.0,
            currency: "USD".to_string(),
            metadata,
            requested_at: Utc::now(),
            occurred_on: Utc::now(),
        };

        assert_eq!(event.amount, 500.0);
        assert_eq!(event.currency, "USD");
        assert!(matches!(event.payment_type, PaymentType::SharePurchase));
    }

    #[test]
    fn test_domain_event_trait_implementation() {
        let event = OwnershipContractCreated {
            aggregate_id: Uuid::new_v4(),
            contract_id: Uuid::new_v4(),
            song_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            total_shares: 1000,
            price_per_share: 10.0,
            artist_retained_percentage: 51.0,
            shares_available_for_sale: 490,
            created_at: Utc::now(),
            occurred_on: Utc::now(),
        };

        // Test that it implements DomainEvent trait
        let domain_event: &dyn crate::shared::domain::events::DomainEvent = &event;
        assert!(domain_event.occurred_at() <= Utc::now());
    }
} 