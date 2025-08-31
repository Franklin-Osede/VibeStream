use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use num_traits::ToPrimitive;
use std::collections::HashMap;

use crate::bounded_contexts::listen_reward::domain::value_objects::{
    ListenSessionId, RewardAmount, RewardPoolId, ValidationPeriod
};
use crate::bounded_contexts::listen_reward::domain::entities::{
    ListenSession
};
use crate::shared::domain::events::DomainEvent;
use crate::bounded_contexts::listen_reward::domain::events::{
    RewardDistributed, ArtistRoyaltyPaid, RewardPoolDepleted, RewardDistributionCreated
};
use vibestream_types::RoyaltyPercentage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardPool {
    id: RewardPoolId,
    total_tokens: RewardAmount,
    distributed_tokens: RewardAmount,
    reserved_tokens: RewardAmount,
    validation_period: ValidationPeriod,
    created_at: DateTime<Utc>,
}

impl RewardPool {
    pub fn new(total_tokens: RewardAmount, validation_period: ValidationPeriod) -> Self {
        Self {
            id: RewardPoolId::new(),
            total_tokens: total_tokens.clone(),
            distributed_tokens: RewardAmount::zero(),
            reserved_tokens: RewardAmount::zero(),
            validation_period,
            created_at: Utc::now(),
        }
    }

    pub fn id(&self) -> &RewardPoolId {
        &self.id
    }

    pub fn available_tokens(&self) -> Result<RewardAmount, String> {
        let total = self.total_tokens.tokens();
        let distributed = self.distributed_tokens.tokens();
        let reserved = self.reserved_tokens.tokens();
        
        RewardAmount::new(total - distributed - reserved)
    }

    pub fn can_distribute(&self, amount: &RewardAmount) -> bool {
        if let Ok(available) = self.available_tokens() {
            available.tokens() >= amount.tokens() && self.validation_period.is_active()
        } else {
            false
        }
    }

    pub fn reserve_tokens(&mut self, amount: &RewardAmount) -> Result<(), String> {
        if !self.can_distribute(amount) {
            return Err("Insufficient tokens available for reservation".to_string());
        }

        self.reserved_tokens = self.reserved_tokens.add(amount)?;
        Ok(())
    }

    pub fn distribute_tokens(&mut self, amount: &RewardAmount) -> Result<(), String> {
        let reserved = self.reserved_tokens.tokens();
        if reserved < amount.tokens() {
            return Err("Cannot distribute more than reserved amount".to_string());
        }

        self.distributed_tokens = self.distributed_tokens.add(amount)?;
        self.reserved_tokens = RewardAmount::new(reserved - amount.tokens())?;
        Ok(())
    }

    pub fn is_depleted(&self) -> bool {
        if let Ok(available) = self.available_tokens() {
            available.tokens() < 1.0 // Less than 1 token remaining
        } else {
            true
        }
    }

    pub fn total_tokens(&self) -> &RewardAmount {
        &self.total_tokens
    }

    pub fn distributed_tokens(&self) -> &RewardAmount {
        &self.distributed_tokens
    }

    pub fn reserved_tokens(&self) -> &RewardAmount {
        &self.reserved_tokens
    }

    pub fn validation_period(&self) -> &ValidationPeriod {
        &self.validation_period
    }

    pub fn from_parts(
        id: RewardPoolId,
        total_tokens: RewardAmount,
        distributed_tokens: RewardAmount,
        reserved_tokens: RewardAmount,
        validation_period: ValidationPeriod,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            total_tokens,
            distributed_tokens,
            reserved_tokens,
            validation_period,
            created_at,
        }
    }
}

#[derive(Debug)]
pub struct RewardDistribution {
    id: Uuid,
    reward_pool: RewardPool,
    pending_distributions: HashMap<ListenSessionId, PendingDistribution>,
    completed_distributions: Vec<CompletedDistribution>,
    artist_royalties: HashMap<Uuid, ArtistRoyaltyInfo>,
    distribution_limits: DistributionLimits,
    created_at: DateTime<Utc>,
    uncommitted_events: Vec<Box<dyn DomainEvent>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingDistribution {
    session_id: ListenSessionId,
    user_id: Uuid,
    artist_id: Uuid,
    song_id: Uuid,
    reward_amount: RewardAmount,
    royalty_percentage: f64,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedDistribution {
    session_id: ListenSessionId,
    user_id: Uuid,
    artist_id: Uuid,
    reward_amount: RewardAmount,
    royalty_amount: RewardAmount,
    user_transaction_hash: String,
    artist_transaction_hash: String,
    completed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistRoyaltyInfo {
    artist_id: Uuid,
    total_earned: RewardAmount,
    pending_amount: RewardAmount,
    last_payout: Option<DateTime<Utc>>,
    payout_threshold: RewardAmount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionLimits {
    max_daily_per_user: RewardAmount,
    max_sessions_per_user_per_day: u32,
    min_payout_threshold: RewardAmount,
    max_pending_distributions: usize,
}

impl Default for DistributionLimits {
    fn default() -> Self {
        Self {
            max_daily_per_user: RewardAmount::new(100.0).unwrap(),
            max_sessions_per_user_per_day: 50,
            min_payout_threshold: RewardAmount::new(1.0).unwrap(),
            max_pending_distributions: 1000,
        }
    }
}

impl RewardDistribution {
    pub fn new(reward_pool: RewardPool) -> Self {
        let mut aggregate = Self {
            id: Uuid::new_v4(),
            reward_pool,
            pending_distributions: HashMap::new(),
            completed_distributions: Vec::new(),
            artist_royalties: HashMap::new(),
            distribution_limits: DistributionLimits::default(),
            created_at: Utc::now(),
            uncommitted_events: Vec::new(),
        };

        aggregate.apply_event(RewardDistributionCreated::new(
            aggregate.id,
            ListenSessionId::new(), // placeholder
            Uuid::new_v4(), // placeholder user_id
            Uuid::new_v4(), // placeholder artist_id
            aggregate.reward_pool.total_tokens().clone(),
            RewardAmount::zero(),
            RewardAmount::zero(),
            RewardAmount::zero(),
            aggregate.created_at,
        ));

        aggregate
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn reward_pool(&self) -> &RewardPool {
        &self.reward_pool
    }

    pub fn pool_id(&self) -> Uuid {
        self.reward_pool.id().value()
    }

    pub fn total_amount(&self) -> f64 {
        self.reward_pool.total_tokens().tokens()
    }

    pub fn distributed_amount(&self) -> f64 {
        self.reward_pool.distributed_tokens().tokens()
    }

    pub fn reserved_amount(&self) -> f64 {
        self.reward_pool.reserved_tokens().tokens()
    }

    pub fn period_start(&self) -> DateTime<Utc> {
        self.reward_pool.validation_period().start_time()
    }

    pub fn period_end(&self) -> DateTime<Utc> {
        self.reward_pool.validation_period().end_time()
    }

    pub fn status(&self) -> String {
        if self.reward_pool.is_depleted() {
            "depleted".to_string()
        } else if self.reward_pool.validation_period().is_active() {
            "active".to_string()
        } else {
            "expired".to_string()
        }
    }

    pub fn take_uncommitted_events(&mut self) -> Vec<Box<dyn DomainEvent>> {
        std::mem::take(&mut self.uncommitted_events)
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn pending_distributions_count(&self) -> usize {
        self.pending_distributions.len()
    }

    pub fn completed_distributions_count(&self) -> usize {
        self.completed_distributions.len()
    }

    pub fn get_events(&self) -> &Vec<Box<dyn DomainEvent>> {
        &self.uncommitted_events
    }

    pub fn queue_reward_distribution(
        &mut self,
        session: &ListenSession,
        royalty_percentage: &RoyaltyPercentage,
    ) -> Result<(), String> {
        if !session.can_be_rewarded() {
            return Err("Session is not eligible for reward distribution".to_string());
        }

        let reward_amount = session.final_reward()
            .ok_or("Session has no calculated reward")?
            .clone();

        // Check pool availability
        if !self.reward_pool.can_distribute(&reward_amount) {
            return Err("Insufficient tokens in reward pool".to_string());
        }

        // Check distribution limits
        self.validate_distribution_limits(session.user_id(), &reward_amount)?;

        // Reserve tokens in the pool
        self.reward_pool.reserve_tokens(&reward_amount)?;

        // Create pending distribution
        let pending = PendingDistribution {
            session_id: session.id().clone(),
            user_id: session.user_id(),
            artist_id: session.artist_id().clone(),
            song_id: session.song_id().clone(),
            reward_amount,
            royalty_percentage: royalty_percentage.percentage().to_f64().unwrap_or(0.0),
            created_at: Utc::now(),
        };

        self.pending_distributions.insert(session.id().clone(), pending);
        Ok(())
    }

    pub fn execute_distribution(
        &mut self,
        session_id: &ListenSessionId,
        user_transaction_hash: String,
        artist_transaction_hash: String,
    ) -> Result<(), String> {
        let pending = self.pending_distributions.remove(session_id)
            .ok_or("No pending distribution found for session")?;

        // Calculate artist royalty
        let royalty_amount = RewardAmount::new(
            pending.reward_amount.tokens() * (pending.royalty_percentage / 100.0)
        )?;

        // Distribute tokens from pool
        self.reward_pool.distribute_tokens(&pending.reward_amount)?;

        // Update artist royalty info
        self.update_artist_royalty(&pending.artist_id, &royalty_amount);

        // Record completed distribution
        let completed = CompletedDistribution {
            session_id: pending.session_id.clone(),
            user_id: pending.user_id,
            artist_id: pending.artist_id.clone(),
            reward_amount: pending.reward_amount.clone(),
            royalty_amount: royalty_amount.clone(),
            user_transaction_hash: user_transaction_hash.clone(),
            artist_transaction_hash: artist_transaction_hash.clone(),
            completed_at: Utc::now(),
        };

        self.completed_distributions.push(completed);

        // Generate events
        self.uncommitted_events.push(Box::new(RewardDistributed::new(
            self.id,
            pending.user_id,
            pending.artist_id.clone(),
            pending.reward_amount.clone(),
            "user".to_string(),
            Utc::now(),
        )));

        self.uncommitted_events.push(Box::new(ArtistRoyaltyPaid::new(
            Uuid::new_v4(),
            pending.artist_id.clone(),
            pending.song_id.clone(),
            royalty_amount,
            Utc::now(),
            Utc::now(),
            Utc::now(),
        )));

        // Check if pool is depleted
        if self.reward_pool.is_depleted() {
            self.uncommitted_events.push(Box::new(RewardPoolDepleted::new(
                self.reward_pool.id().value(),
                self.reward_pool.available_tokens().unwrap_or(RewardAmount::zero()),
                0.1, // 10% threshold
                Utc::now(),
            )));
        }

        Ok(())
    }

    pub fn get_user_daily_rewards(&self, user_id: Uuid) -> RewardAmount {
        let today = Utc::now().date_naive();
        let total_tokens: f64 = self.completed_distributions
            .iter()
            .filter(|d| d.user_id == user_id && d.completed_at.date_naive() == today)
            .map(|d| d.reward_amount.tokens())
            .sum();

        RewardAmount::new(total_tokens).unwrap_or(RewardAmount::zero())
    }

    pub fn get_user_daily_session_count(&self, user_id: Uuid) -> u32 {
        let today = Utc::now().date_naive();
        self.completed_distributions
            .iter()
            .filter(|d| d.user_id == user_id && d.completed_at.date_naive() == today)
            .count() as u32
    }

    pub fn get_artist_pending_royalties(&self, artist_id: &Uuid) -> RewardAmount {
        self.artist_royalties
            .get(artist_id)
            .map(|info| info.pending_amount.clone())
            .unwrap_or(RewardAmount::zero())
    }

    pub fn get_distribution_analytics(&self) -> DistributionAnalytics {
        let total_distributed = self.reward_pool.distributed_tokens.tokens();
        let total_pending = self.pending_distributions.len();
        let total_completed = self.completed_distributions.len();
        
        let unique_users: std::collections::HashSet<_> = self.completed_distributions
            .iter()
            .map(|d| d.user_id)
            .collect();

        let unique_artists: std::collections::HashSet<_> = self.completed_distributions
            .iter()
            .map(|d| &d.artist_id)
            .collect();

        DistributionAnalytics {
            total_tokens_distributed: total_distributed,
            total_pending_distributions: total_pending,
            total_completed_distributions: total_completed,
            unique_users_rewarded: unique_users.len(),
            unique_artists_earning: unique_artists.len(),
            pool_utilization_percentage: (total_distributed / self.reward_pool.total_tokens.tokens()) * 100.0,
            average_reward_per_session: if total_completed > 0 { 
                total_distributed / total_completed as f64 
            } else { 
                0.0 
            },
        }
    }

    fn validate_distribution_limits(&self, user_id: Uuid, reward_amount: &RewardAmount) -> Result<(), String> {
        // Check daily limit per user
        let daily_rewards = self.get_user_daily_rewards(user_id);
        if daily_rewards.add(reward_amount)?.tokens() > self.distribution_limits.max_daily_per_user.tokens() {
            return Err("Daily reward limit exceeded for user".to_string());
        }

        // Check session limit per user per day
        let daily_sessions = self.get_user_daily_session_count(user_id);
        if daily_sessions >= self.distribution_limits.max_sessions_per_user_per_day {
            return Err("Daily session limit exceeded for user".to_string());
        }

        // Check minimum payout threshold
        if reward_amount.tokens() < self.distribution_limits.min_payout_threshold.tokens() {
            return Err("Reward amount below minimum payout threshold".to_string());
        }

        // Check pending distributions limit
        if self.pending_distributions.len() >= self.distribution_limits.max_pending_distributions {
            return Err("Maximum pending distributions limit reached".to_string());
        }

        Ok(())
    }

    fn update_artist_royalty(&mut self, artist_id: &Uuid, royalty_amount: &RewardAmount) {
        let royalty_info = self.artist_royalties.entry(artist_id.clone()).or_insert_with(|| {
            ArtistRoyaltyInfo {
                artist_id: artist_id.clone(),
                total_earned: RewardAmount::zero(),
                pending_amount: RewardAmount::zero(),
                last_payout: None,
                payout_threshold: RewardAmount::new(10.0).unwrap(), // Default threshold
            }
        });

        royalty_info.total_earned = royalty_info.total_earned.add(royalty_amount).unwrap_or(royalty_info.total_earned.clone());
        royalty_info.pending_amount = royalty_info.pending_amount.add(royalty_amount).unwrap_or(royalty_info.pending_amount.clone());
    }

    pub fn version(&self) -> i32 {
        0 // Versión por defecto
    }

    pub fn from_persisted_state(
        id: Uuid,
        reward_pool: RewardPool,
        created_at: DateTime<Utc>,
        uncommitted_events: Vec<Box<dyn DomainEvent>>,
        version: i32,
    ) -> Self {
        Self {
            id,
            reward_pool,
            pending_distributions: HashMap::new(),
            completed_distributions: Vec::new(),
            artist_royalties: HashMap::new(),
            distribution_limits: DistributionLimits::default(),
            created_at,
            uncommitted_events,
        }
    }

    fn apply_event(&mut self, event: impl DomainEvent + 'static) {
        self.uncommitted_events.push(Box::new(event));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionAnalytics {
    pub total_tokens_distributed: f64,
    pub total_pending_distributions: usize,
    pub total_completed_distributions: usize,
    pub unique_users_rewarded: usize,
    pub unique_artists_earning: usize,
    pub pool_utilization_percentage: f64,
    pub average_reward_per_session: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_contexts::music::domain::{SongId, ArtistId};
    use crate::bounded_contexts::listen_reward::domain::value_objects::RewardTier;

    fn create_test_pool() -> RewardPool {
        RewardPool::new(
            RewardAmount::new(1000.0).unwrap(),
            ValidationPeriod::daily(),
        )
    }

    fn create_test_session() -> ListenSession {
        let (mut session, _) = ListenSession::new(
            Uuid::new_v4(),
            SongId::new(),
            ArtistId::new(),
            RewardTier::Basic,
        );
        
        // Complete and verify session
        let duration = crate::bounded_contexts::listen_reward::domain::value_objects::ListenDuration::new(120).unwrap();
        let quality = crate::bounded_contexts::listen_reward::domain::value_objects::QualityScore::new(0.8).unwrap();
        let proof = crate::bounded_contexts::listen_reward::domain::value_objects::ZkProofHash::new("a".repeat(64)).unwrap();
        
        let _ = session.complete_session(duration, quality, proof, 180);
        let _ = session.verify_and_calculate_reward(1.0, true);
        
        session
    }

    #[test]
    fn test_reward_pool_creation() {
        let pool = create_test_pool();
        assert_eq!(pool.total_tokens.tokens(), 1000.0);
        assert_eq!(pool.distributed_tokens.tokens(), 0.0);
        assert!(pool.validation_period.is_active());
    }

    #[test]
    fn test_queue_reward_distribution() {
        let pool = create_test_pool();
        let mut distribution = RewardDistribution::new(pool);
        let session = create_test_session();
        let royalty = RoyaltyPercentage::new(10.0).unwrap();

        let result = distribution.queue_reward_distribution(&session, &royalty);
        assert!(result.is_ok());
        assert_eq!(distribution.pending_distributions.len(), 1);
    }

    #[test]
    fn test_execute_distribution() {
        let pool = create_test_pool();
        let mut distribution = RewardDistribution::new(pool);
        let session = create_test_session();
        let royalty = RoyaltyPercentage::new(10.0).unwrap();

        // Queue distribution
        let _ = distribution.queue_reward_distribution(&session, &royalty);
        
        // Execute distribution
        let result = distribution.execute_distribution(
            session.id(),
            "user_tx_hash".to_string(),
            "artist_tx_hash".to_string(),
        );
        
        assert!(result.is_ok());
        assert_eq!(distribution.pending_distributions.len(), 0);
        assert_eq!(distribution.completed_distributions.len(), 1);
        assert_eq!(distribution.uncommitted_events.len(), 2); // RewardDistributed + ArtistRoyaltyPaid
    }

    #[test]
    fn test_distribution_limits() {
        let pool = create_test_pool();
        let mut distribution = RewardDistribution::new(pool);
        let user_id = Uuid::new_v4();
        
        // Set low daily limit
        distribution.distribution_limits.max_daily_per_user = RewardAmount::new(5.0).unwrap();
        
        let session = create_test_session();
        let royalty = RoyaltyPercentage::new(10.0).unwrap();

        // This should fail due to daily limit (assuming session reward > 5.0)
        let result = distribution.queue_reward_distribution(&session, &royalty);
        // Result depends on the actual reward amount calculated in the test session
        // If reward is > 5.0, it should fail
    }

    #[test]
    fn test_artist_royalty_tracking() {
        let pool = create_test_pool();
        let mut distribution = RewardDistribution::new(pool);
        let session = create_test_session();
        let royalty = RoyaltyPercentage::new(15.0).unwrap();

        // Queue and execute distribution
        let _ = distribution.queue_reward_distribution(&session, &royalty);
        let _ = distribution.execute_distribution(
            session.id(),
            "user_tx_hash".to_string(),
            "artist_tx_hash".to_string(),
        );

        // Check artist royalty tracking
        let pending_royalties = distribution.get_artist_pending_royalties(session.artist_id());
        assert!(pending_royalties.tokens() > 0.0);
    }
} 