// TODO: Implement fractional ownership entities 

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::music::domain::value_objects::SongId;
use crate::bounded_contexts::user::domain::value_objects::UserId;

use super::value_objects::{
    OwnershipContractId, OwnershipPercentage, SharePrice, RevenueAmount, 
    ShareId, VestingPeriod
};
use super::events::{
    SharesPurchased, SharesTraded, SharesLocked, SharesUnlocked, 
    RevenueDistributed, ShareholderDistribution, ShareLockReason
};

/// Rich Entity: Represents a fractional share of a song with business behaviors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FractionalShare {
    id: ShareId,
    contract_id: OwnershipContractId,
    owner_id: UserId,
    song_id: SongId,
    ownership_percentage: OwnershipPercentage,
    purchase_price: SharePrice,
    purchased_at: DateTime<Utc>,
    vesting_period: Option<VestingPeriod>,
    is_locked: bool,
    lock_reason: Option<ShareLockReason>,
    locked_until: Option<DateTime<Utc>>,
    total_revenue_received: RevenueAmount,
    is_tradeable: bool,
    current_market_value: SharePrice,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl FractionalShare {
    /// Create a new fractional share - Domain Factory Method
    pub fn create(
        contract_id: OwnershipContractId,
        owner_id: UserId,
        song_id: SongId,
        ownership_percentage: OwnershipPercentage,
        purchase_price: SharePrice,
        vesting_period: Option<VestingPeriod>,
    ) -> Result<(Self, SharesPurchased), AppError> {
        let now = Utc::now();
        let share_id = ShareId::new();

        // Domain Rules Validation
        if ownership_percentage.value() <= 0.0 {
            return Err(AppError::DomainRuleViolation(
                "Ownership percentage must be greater than 0".to_string(),
            ));
        }

        if purchase_price.value() <= 0.0 {
            return Err(AppError::DomainRuleViolation(
                "Purchase price must be greater than 0".to_string(),
            ));
        }

        let share = Self {
            id: share_id.clone(),
            contract_id: contract_id.clone(),
            owner_id: owner_id.clone(),
            song_id: song_id.clone(),
            ownership_percentage: ownership_percentage.clone(),
            purchase_price: purchase_price.clone(),
            purchased_at: now,
            vesting_period: vesting_period.clone(),
            is_locked: vesting_period.is_some(),
            lock_reason: if vesting_period.is_some() { 
                Some(ShareLockReason::VestingPeriod) 
            } else { 
                None 
            },
            locked_until: vesting_period.as_ref().map(|v| v.end_date()),
            total_revenue_received: RevenueAmount::new(0.0).unwrap(),
            is_tradeable: vesting_period.is_none(),
            current_market_value: purchase_price.clone(),
            created_at: now,
            updated_at: now,
        };

        let event = SharesPurchased {
            aggregate_id: contract_id.value(),
            contract_id: contract_id.value(),
            share_id: share_id.value(),
            buyer_id: owner_id.value(),
            song_id: *song_id.value(),
            ownership_percentage: ownership_percentage.value(),
            purchase_price: purchase_price.value(),
            transaction_hash: None, // Will be set by blockchain service
            purchased_at: now,
            occurred_on: now,
        };

        Ok((share, event))
    }

    /// Transfer share to another owner - Rich Domain Behavior
    pub fn transfer_to(
        &mut self, 
        new_owner: UserId, 
        trade_price: SharePrice
    ) -> Result<SharesTraded, AppError> {
        // Domain Rules
        if !self.can_be_traded() {
            return Err(AppError::DomainRuleViolation(
                format!("Share cannot be traded. Locked: {}, Tradeable: {}", 
                    self.is_locked, self.is_tradeable)
            ));
        }

        if new_owner.value() == self.owner_id.value() {
            return Err(AppError::DomainRuleViolation(
                "Cannot transfer share to the same owner".to_string(),
            ));
        }

        let old_owner = self.owner_id.clone();
        self.owner_id = new_owner.clone();
        self.current_market_value = trade_price.clone();
        self.updated_at = Utc::now();

        Ok(SharesTraded {
            aggregate_id: self.contract_id.value(),
            contract_id: self.contract_id.value(),
            share_id: self.id.value(),
            from_user_id: old_owner.value(),
            to_user_id: new_owner.value(),
            song_id: *self.song_id.value(),
            ownership_percentage: self.ownership_percentage.value(),
            trade_price: trade_price.value(),
            transaction_hash: None,
            traded_at: Utc::now(),
            occurred_on: Utc::now(),
        })
    }

    /// Lock share for various reasons - Rich Domain Behavior
    pub fn lock(&mut self, reason: ShareLockReason, lock_until: Option<DateTime<Utc>>) -> Result<SharesLocked, AppError> {
        if self.is_locked {
            return Err(AppError::DomainRuleViolation(
                "Share is already locked".to_string(),
            ));
        }

        self.is_locked = true;
        self.is_tradeable = false;
        self.lock_reason = Some(reason.clone());
        self.locked_until = lock_until;
        self.updated_at = Utc::now();

        Ok(SharesLocked {
            aggregate_id: self.contract_id.value(),
            share_id: self.id.value(),
            contract_id: self.contract_id.value(),
            owner_id: self.owner_id.value(),
            ownership_percentage: self.ownership_percentage.value(),
            lock_reason: reason,
            lock_until,
            locked_at: Utc::now(),
            occurred_on: Utc::now(),
        })
    }

    /// Unlock share - Rich Domain Behavior
    pub fn unlock(&mut self, unlock_reason: String) -> Result<SharesUnlocked, AppError> {
        if !self.is_locked {
            return Err(AppError::DomainRuleViolation(
                "Share is not locked".to_string(),
            ));
        }

        // Check if vesting period has ended
        if let Some(vesting) = &self.vesting_period {
            if !vesting.is_fully_vested() {
                return Err(AppError::DomainRuleViolation(
                    "Cannot unlock share before vesting period ends".to_string(),
                ));
            }
        }

        self.is_locked = false;
        self.is_tradeable = true;
        self.lock_reason = None;
        self.locked_until = None;
        self.updated_at = Utc::now();

        Ok(SharesUnlocked {
            aggregate_id: self.contract_id.value(),
            share_id: self.id.value(),
            contract_id: self.contract_id.value(),
            owner_id: self.owner_id.value(),
            ownership_percentage: self.ownership_percentage.value(),
            unlock_reason,
            unlocked_at: Utc::now(),
            occurred_on: Utc::now(),
        })
    }

    /// Receive revenue distribution - Rich Domain Behavior
    pub fn receive_revenue(&mut self, amount: RevenueAmount) -> Result<(), AppError> {
        self.total_revenue_received = self.total_revenue_received.add(&amount);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Update market value based on trading activity
    pub fn update_market_value(&mut self, new_value: SharePrice) {
        self.current_market_value = new_value;
        self.updated_at = Utc::now();
    }

    // Domain Queries
    pub fn can_be_traded(&self) -> bool {
        !self.is_locked && self.is_tradeable
    }

    pub fn is_vested(&self) -> bool {
        match &self.vesting_period {
            Some(vesting) => vesting.is_fully_vested(),
            None => true,
        }
    }

    pub fn vesting_progress(&self) -> f64 {
        match &self.vesting_period {
            Some(vesting) => vesting.vesting_progress(),
            None => 1.0,
        }
    }

    pub fn calculate_current_value(&self) -> f64 {
        self.current_market_value.value()
    }

    pub fn calculate_roi(&self) -> f64 {
        let current_value = self.current_market_value.value();
        let purchase_price = self.purchase_price.value();
        let revenue_received = self.total_revenue_received.value();
        
        ((current_value + revenue_received - purchase_price) / purchase_price) * 100.0
    }

    pub fn days_until_unlocked(&self) -> Option<i64> {
        self.locked_until.map(|until| {
            let now = Utc::now();
            if until <= now {
                0
            } else {
                (until.date_naive() - now.date_naive()).num_days()
            }
        })
    }

    // Getters
    pub fn id(&self) -> &ShareId { &self.id }
    pub fn contract_id(&self) -> &OwnershipContractId { &self.contract_id }
    pub fn owner_id(&self) -> &UserId { &self.owner_id }
    pub fn song_id(&self) -> &SongId { &self.song_id }
    pub fn ownership_percentage(&self) -> &OwnershipPercentage { &self.ownership_percentage }
    pub fn purchase_price(&self) -> &SharePrice { &self.purchase_price }
    pub fn purchased_at(&self) -> DateTime<Utc> { self.purchased_at }
    pub fn vesting_period(&self) -> Option<&VestingPeriod> { self.vesting_period.as_ref() }
    pub fn is_locked(&self) -> bool { self.is_locked }
    pub fn lock_reason(&self) -> Option<&ShareLockReason> { self.lock_reason.as_ref() }
    pub fn locked_until(&self) -> Option<DateTime<Utc>> { self.locked_until }
    pub fn total_revenue_received(&self) -> &RevenueAmount { &self.total_revenue_received }
    pub fn is_tradeable(&self) -> bool { self.is_tradeable }
    pub fn current_market_value(&self) -> &SharePrice { &self.current_market_value }
    pub fn created_at(&self) -> DateTime<Utc> { self.created_at }
    pub fn updated_at(&self) -> DateTime<Utc> { self.updated_at }
}

/// Rich Entity: Represents revenue distribution with business logic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueDistribution {
    id: Uuid,
    contract_id: OwnershipContractId,
    song_id: SongId,
    distribution_period_start: DateTime<Utc>,
    distribution_period_end: DateTime<Utc>,
    total_revenue: RevenueAmount,
    platform_fee_percentage: f64,
    artist_retained_percentage: f64,
    distributed_amount: RevenueAmount,
    platform_fee: RevenueAmount,
    artist_share: RevenueAmount,
    shareholder_distributions: Vec<ShareholderDistribution>,
    status: DistributionStatus,
    processed_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistributionStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

impl RevenueDistribution {
    /// Create new revenue distribution - Domain Factory
    pub fn create(
        contract_id: OwnershipContractId,
        song_id: SongId,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
        total_revenue: RevenueAmount,
        platform_fee_percentage: f64,
        artist_retained_percentage: f64,
    ) -> Result<Self, AppError> {
        // Domain Rules Validation
        if period_start >= period_end {
            return Err(AppError::DomainRuleViolation(
                "Distribution period start must be before end".to_string(),
            ));
        }

        if platform_fee_percentage < 0.0 || platform_fee_percentage > 50.0 {
            return Err(AppError::DomainRuleViolation(
                "Platform fee must be between 0% and 50%".to_string(),
            ));
        }

        if artist_retained_percentage < 0.0 || artist_retained_percentage > 100.0 {
            return Err(AppError::DomainRuleViolation(
                "Artist retained percentage must be between 0% and 100%".to_string(),
            ));
        }

        let now = Utc::now();

        Ok(Self {
            id: Uuid::new_v4(),
            contract_id,
            song_id,
            distribution_period_start: period_start,
            distribution_period_end: period_end,
            total_revenue,
            platform_fee_percentage,
            artist_retained_percentage,
            distributed_amount: RevenueAmount::new(0.0).unwrap(),
            platform_fee: RevenueAmount::new(0.0).unwrap(),
            artist_share: RevenueAmount::new(0.0).unwrap(),
            shareholder_distributions: Vec::new(),
            status: DistributionStatus::Pending,
            processed_at: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// Calculate and process distribution - Rich Domain Behavior
    pub fn process_distribution(&mut self, shares: &[FractionalShare]) -> Result<RevenueDistributed, AppError> {
        if !matches!(self.status, DistributionStatus::Pending) {
            return Err(AppError::DomainRuleViolation(
                "Distribution has already been processed".to_string(),
            ));
        }

        self.status = DistributionStatus::Processing;

        // Calculate platform fee
        let platform_fee_amount = self.total_revenue.value() * (self.platform_fee_percentage / 100.0);
        self.platform_fee = RevenueAmount::new(platform_fee_amount).unwrap();

        // Calculate distributable amount (after platform fee)
        let distributable = self.total_revenue.subtract(&self.platform_fee)?;

        // Calculate artist share
        let artist_share_amount = distributable.value() * (self.artist_retained_percentage / 100.0);
        self.artist_share = RevenueAmount::new(artist_share_amount).unwrap();

        // Calculate shareholder distributions
        let shareholder_pool = distributable.subtract(&self.artist_share)?;
        let total_shareholder_percentage: f64 = shares.iter()
            .map(|s| s.ownership_percentage().value())
            .sum();

        if total_shareholder_percentage > 0.0 {
            for share in shares {
                let share_ratio = share.ownership_percentage().value() / total_shareholder_percentage;
                let distribution_amount = shareholder_pool.value() * share_ratio;

                self.shareholder_distributions.push(ShareholderDistribution {
                    shareholder_id: share.owner_id().value(),
                    ownership_percentage: share.ownership_percentage().value(),
                    revenue_share: distribution_amount,
                    transaction_hash: None,
                });
            }
        }

        self.distributed_amount = RevenueAmount::new(
            self.platform_fee.value() + self.artist_share.value() + 
            self.shareholder_distributions.iter().map(|d| d.revenue_share).sum::<f64>()
        ).unwrap();

        self.status = DistributionStatus::Completed;
        self.processed_at = Some(Utc::now());
        self.updated_at = Utc::now();

        Ok(RevenueDistributed {
            aggregate_id: self.contract_id.value(),
            contract_id: self.contract_id.value(),
            song_id: *self.song_id.value(),
            total_revenue: self.total_revenue.value(),
            distribution_period_start: self.distribution_period_start,
            distribution_period_end: self.distribution_period_end,
            total_distributed: self.distributed_amount.value(),
            artist_share: self.artist_share.value(),
            platform_fee: self.platform_fee.value(),
            shareholder_distributions: self.shareholder_distributions.clone(),
            distributed_at: Utc::now(),
            occurred_on: Utc::now(),
        })
    }

    /// Cancel distribution - Rich Domain Behavior
    pub fn cancel(&mut self, reason: String) -> Result<(), AppError> {
        if matches!(self.status, DistributionStatus::Completed) {
            return Err(AppError::DomainRuleViolation(
                "Cannot cancel completed distribution".to_string(),
            ));
        }

        self.status = DistributionStatus::Cancelled;
        self.updated_at = Utc::now();
        Ok(())
    }

    // Domain Queries
    pub fn is_processed(&self) -> bool {
        matches!(self.status, DistributionStatus::Completed)
    }

    pub fn total_shareholder_amount(&self) -> f64 {
        self.shareholder_distributions.iter().map(|d| d.revenue_share).sum()
    }

    pub fn get_distribution_for_user(&self, user_id: Uuid) -> Option<&ShareholderDistribution> {
        self.shareholder_distributions.iter().find(|d| d.shareholder_id == user_id)
    }

    // Getters
    pub fn id(&self) -> Uuid { self.id }
    pub fn contract_id(&self) -> &OwnershipContractId { &self.contract_id }
    pub fn song_id(&self) -> &SongId { &self.song_id }
    pub fn total_revenue(&self) -> &RevenueAmount { &self.total_revenue }
    pub fn status(&self) -> &DistributionStatus { &self.status }
    pub fn shareholder_distributions(&self) -> &[ShareholderDistribution] { &self.shareholder_distributions }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    fn create_test_share() -> Result<FractionalShare, AppError> {
        let (share, _) = FractionalShare::create(
            OwnershipContractId::new(),
            UserId::new(),
            SongId::new(),
            OwnershipPercentage::new(10.0)?,
            SharePrice::new(100.0)?,
            None,
        )?;
        Ok(share)
    }

    #[test]
    fn test_fractional_share_creation() {
        let share = create_test_share().unwrap();
        
        assert_eq!(share.ownership_percentage().value(), 10.0);
        assert_eq!(share.purchase_price().value(), 100.0);
        assert!(!share.is_locked());
        assert!(share.is_tradeable());
    }

    #[test]
    fn test_share_creation_with_vesting() {
        let future_start = Utc::now() + Duration::days(1);
        let future_end = future_start + Duration::days(365);
        let vesting = VestingPeriod::new(future_start, future_end).unwrap();

        let (share, _) = FractionalShare::create(
            OwnershipContractId::new(),
            UserId::new(),
            SongId::new(),
            OwnershipPercentage::new(10.0).unwrap(),
            SharePrice::new(100.0).unwrap(),
            Some(vesting),
        ).unwrap();

        assert!(share.is_locked());
        assert!(!share.is_tradeable());
        assert!(share.vesting_period().is_some());
    }

    #[test]
    fn test_share_transfer() {
        let mut share = create_test_share().unwrap();
        let new_owner = UserId::new();
        let trade_price = SharePrice::new(150.0).unwrap();

        let event = share.transfer_to(new_owner.clone(), trade_price.clone()).unwrap();

        assert_eq!(share.owner_id().value(), new_owner.value());
        assert_eq!(share.current_market_value().value(), 150.0);
        assert_eq!(event.trade_price, 150.0);
    }

    #[test]
    fn test_share_cannot_transfer_to_same_owner() {
        let mut share = create_test_share().unwrap();
        let same_owner = share.owner_id().clone();
        let trade_price = SharePrice::new(150.0).unwrap();

        let result = share.transfer_to(same_owner, trade_price);
        assert!(result.is_err());
    }

    #[test]
    fn test_share_lock_unlock() {
        let mut share = create_test_share().unwrap();
        
        // Lock share
        let lock_event = share.lock(ShareLockReason::LegalDispute, None).unwrap();
        assert!(share.is_locked());
        assert!(!share.can_be_traded());
        assert_eq!(lock_event.lock_reason, ShareLockReason::LegalDispute);

        // Unlock share
        let unlock_event = share.unlock("Legal dispute resolved".to_string()).unwrap();
        assert!(!share.is_locked());
        assert!(share.can_be_traded());
        assert_eq!(unlock_event.unlock_reason, "Legal dispute resolved");
    }

    #[test]
    fn test_share_roi_calculation() {
        let mut share = create_test_share().unwrap();
        
        // Receive revenue
        let revenue = RevenueAmount::new(50.0).unwrap();
        share.receive_revenue(revenue).unwrap();
        
        // Update market value
        share.update_market_value(SharePrice::new(120.0).unwrap());
        
        let roi = share.calculate_roi();
        // ROI = ((120 + 50 - 100) / 100) * 100 = 70%
        assert_eq!(roi, 70.0);
    }

    #[test]
    fn test_revenue_distribution_creation() {
        let distribution = RevenueDistribution::create(
            OwnershipContractId::new(),
            SongId::new(),
            Utc::now() - Duration::days(30),
            Utc::now(),
            RevenueAmount::new(1000.0).unwrap(),
            5.0, // 5% platform fee
            51.0, // 51% artist retained
        ).unwrap();

        assert_eq!(distribution.total_revenue().value(), 1000.0);
        assert_eq!(distribution.platform_fee_percentage, 5.0);
        assert_eq!(distribution.artist_retained_percentage, 51.0);
        assert!(matches!(distribution.status(), DistributionStatus::Pending));
    }

    #[test]
    fn test_revenue_distribution_processing() {
        let mut distribution = RevenueDistribution::create(
            OwnershipContractId::new(),
            SongId::new(),
            Utc::now() - Duration::days(30),
            Utc::now(),
            RevenueAmount::new(1000.0).unwrap(),
            5.0, // 5% platform fee
            51.0, // 51% artist retained
        ).unwrap();

        // Create test shares
        let share1 = create_test_share().unwrap(); // 10%
        let (share2, _) = FractionalShare::create(
            OwnershipContractId::new(),
            UserId::new(),
            SongId::new(),
            OwnershipPercentage::new(20.0).unwrap(),
            SharePrice::new(200.0).unwrap(),
            None,
        ).unwrap();

        let shares = vec![share1, share2];
        let event = distribution.process_distribution(&shares).unwrap();

        // Verify calculations
        // Platform fee: 1000 * 5% = 50
        // Distributable: 1000 - 50 = 950
        // Artist share: 950 * 51% = 484.5
        // Shareholder pool: 950 - 484.5 = 465.5
        // Total shareholder percentage: 10% + 20% = 30%
        // Share1 (10%): 465.5 * (10/30) = 155.17
        // Share2 (20%): 465.5 * (20/30) = 310.33

        assert_eq!(distribution.platform_fee.value(), 50.0);
        assert_eq!(distribution.artist_share.value(), 484.5);
        assert_eq!(distribution.shareholder_distributions.len(), 2);
        assert!(distribution.is_processed());
        assert_eq!(event.total_revenue, 1000.0);
    }

    #[test]
    fn test_revenue_distribution_validation() {
        // Invalid period
        let result = RevenueDistribution::create(
            OwnershipContractId::new(),
            SongId::new(),
            Utc::now(),
            Utc::now() - Duration::days(1), // End before start
            RevenueAmount::new(1000.0).unwrap(),
            5.0,
            51.0,
        );
        assert!(result.is_err());

        // Invalid platform fee
        let result = RevenueDistribution::create(
            OwnershipContractId::new(),
            SongId::new(),
            Utc::now() - Duration::days(30),
            Utc::now(),
            RevenueAmount::new(1000.0).unwrap(),
            55.0, // Too high
            51.0,
        );
        assert!(result.is_err());
    }
} 