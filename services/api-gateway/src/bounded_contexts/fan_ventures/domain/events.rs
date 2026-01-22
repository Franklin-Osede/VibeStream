use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::bounded_contexts::fan_ventures::domain::entities::{InvestmentType, DeliveryStatus, DeliveryMethod, VentureStatus};

// ====== VENTURE EVENTS ======

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentureCreated {
    pub venture_id: Uuid,
    pub artist_id: Uuid,
    pub title: String,
    pub funding_goal: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentureStatusChanged {
    pub venture_id: Uuid,
    pub old_status: VentureStatus,
    pub new_status: VentureStatus,
    pub changed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanInvested {
    pub investment_id: Uuid,
    pub venture_id: Uuid,
    pub fan_id: Uuid,
    pub amount: f64,
    pub investment_type: InvestmentType,
    pub invested_at: DateTime<Utc>,
}

// ====== REVENUE EVENTS ======

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueDistributed {
    pub distribution_id: Uuid,
    pub venture_id: Uuid,
    pub total_revenue: f64,
    pub artist_share: f64,
    pub fan_share: f64,
    pub platform_fee: f64,
    pub distributed_at: DateTime<Utc>,
}

// ====== BENEFIT EVENTS ======

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenefitDelivered {
    pub delivery_id: Uuid,
    pub venture_id: Uuid,
    pub fan_id: Uuid,
    pub benefit_id: Uuid,
    pub method: DeliveryMethod,
    pub delivered_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryStatusUpdated {
    pub delivery_id: Uuid,
    pub old_status: DeliveryStatus,
    pub new_status: DeliveryStatus,
    pub updated_at: DateTime<Utc>,
}

// ====== TRAIT IMPLS ======

impl crate::shared::domain::events::DomainEvent for VentureCreated {
    fn event_type(&self) -> &str { "VentureCreated" }
    fn aggregate_id(&self) -> Uuid { self.venture_id }
    fn aggregate_type(&self) -> &str { "ArtistVenture" }
    fn occurred_at(&self) -> DateTime<Utc> { self.created_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
    fn metadata(&self) -> &crate::shared::domain::events::EventMetadata { unimplemented!() }
}

impl crate::shared::domain::events::DomainEvent for FanInvested {
    fn event_type(&self) -> &str { "FanInvested" }
    fn aggregate_id(&self) -> Uuid { self.venture_id }
    fn aggregate_type(&self) -> &str { "ArtistVenture" }
    fn occurred_at(&self) -> DateTime<Utc> { self.invested_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
    fn metadata(&self) -> &crate::shared::domain::events::EventMetadata { unimplemented!() }
}

impl crate::shared::domain::events::DomainEvent for RevenueDistributed {
    fn event_type(&self) -> &str { "RevenueDistributed" }
    fn aggregate_id(&self) -> Uuid { self.venture_id }
    fn aggregate_type(&self) -> &str { "ArtistVenture" }
    fn occurred_at(&self) -> DateTime<Utc> { self.distributed_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
    fn metadata(&self) -> &crate::shared::domain::events::EventMetadata { unimplemented!() }
}

impl crate::shared::domain::events::DomainEvent for BenefitDelivered {
    fn event_type(&self) -> &str { "BenefitDelivered" }
    fn aggregate_id(&self) -> Uuid { self.venture_id }
    fn aggregate_type(&self) -> &str { "ArtistVenture" }
    fn occurred_at(&self) -> DateTime<Utc> { self.delivered_at }
    fn event_data(&self) -> serde_json::Value { serde_json::to_value(self).unwrap_or_default() }
    fn metadata(&self) -> &crate::shared::domain::events::EventMetadata { unimplemented!() }
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