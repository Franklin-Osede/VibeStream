use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::listen_reward::domain::value_objects::{
    ListenSessionId, RewardAmount, ListenDuration, QualityScore, ZkProofHash, RewardTier
};
use crate::bounded_contexts::music::domain::value_objects::{SongId, ArtistId};

// Base trait for all domain events
pub trait DomainEvent: Send + Sync + std::fmt::Debug {
    fn event_type(&self) -> &'static str;
    fn aggregate_id(&self) -> Uuid;
    fn timestamp(&self) -> DateTime<Utc>;
    fn data(&self) -> serde_json::Value;
}

// Listen session started
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenSessionStarted {
    pub session_id: ListenSessionId,
    pub user_id: Uuid,
    pub song_id: SongId,
    pub artist_id: ArtistId,
    pub user_tier: RewardTier,
    pub started_at: DateTime<Utc>,
}

impl ListenSessionStarted {
    pub fn new(
        session_id: ListenSessionId,
        user_id: Uuid,
        song_id: SongId,
        artist_id: ArtistId,
        user_tier: RewardTier,
    ) -> Self {
        Self {
            session_id,
            user_id,
            song_id,
            artist_id,
            user_tier,
            started_at: Utc::now(),
        }
    }
}

impl DomainEvent for ListenSessionStarted {
    fn event_type(&self) -> &'static str {
        "ListenSessionStarted"
    }

    fn aggregate_id(&self) -> Uuid {
        self.session_id.value()
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.started_at
    }

    fn data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// Listen session completed with ZK proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenSessionCompleted {
    pub session_id: ListenSessionId,
    pub user_id: Uuid,
    pub song_id: SongId,
    pub listen_duration: ListenDuration,
    pub quality_score: QualityScore,
    pub zk_proof: ZkProofHash,
    pub completed_at: DateTime<Utc>,
}

impl ListenSessionCompleted {
    pub fn new(
        session_id: ListenSessionId,
        user_id: Uuid,
        song_id: SongId,
        listen_duration: ListenDuration,
        quality_score: QualityScore,
        zk_proof: ZkProofHash,
    ) -> Self {
        Self {
            session_id,
            user_id,
            song_id,
            listen_duration,
            quality_score,
            zk_proof,
            completed_at: Utc::now(),
        }
    }
}

impl DomainEvent for ListenSessionCompleted {
    fn event_type(&self) -> &'static str {
        "ListenSessionCompleted"
    }

    fn aggregate_id(&self) -> Uuid {
        self.session_id.value()
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.completed_at
    }

    fn data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// Reward calculated and ready for distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardCalculated {
    pub session_id: ListenSessionId,
    pub user_id: Uuid,
    pub song_id: SongId,
    pub artist_id: ArtistId,
    pub base_reward: RewardAmount,
    pub final_reward: RewardAmount,
    pub user_tier: RewardTier,
    pub quality_multiplier: f64,
    pub calculated_at: DateTime<Utc>,
}

impl RewardCalculated {
    pub fn new(
        session_id: ListenSessionId,
        user_id: Uuid,
        song_id: SongId,
        artist_id: ArtistId,
        base_reward: RewardAmount,
        final_reward: RewardAmount,
        user_tier: RewardTier,
        quality_multiplier: f64,
    ) -> Self {
        Self {
            session_id,
            user_id,
            song_id,
            artist_id,
            base_reward,
            final_reward,
            user_tier,
            quality_multiplier,
            calculated_at: Utc::now(),
        }
    }
}

impl DomainEvent for RewardCalculated {
    fn event_type(&self) -> &'static str {
        "RewardCalculated"
    }

    fn aggregate_id(&self) -> Uuid {
        self.session_id.value()
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.calculated_at
    }

    fn data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// Reward distributed to user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardDistributed {
    pub session_id: ListenSessionId,
    pub user_id: Uuid,
    pub reward_amount: RewardAmount,
    pub transaction_hash: String,
    pub distributed_at: DateTime<Utc>,
}

impl RewardDistributed {
    pub fn new(
        session_id: ListenSessionId,
        user_id: Uuid,
        reward_amount: RewardAmount,
        transaction_hash: String,
    ) -> Self {
        Self {
            session_id,
            user_id,
            reward_amount,
            transaction_hash,
            distributed_at: Utc::now(),
        }
    }
}

impl DomainEvent for RewardDistributed {
    fn event_type(&self) -> &'static str {
        "RewardDistributed"
    }

    fn aggregate_id(&self) -> Uuid {
        self.session_id.value()
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.distributed_at
    }

    fn data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// Artist royalty paid from listen rewards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistRoyaltyPaid {
    pub session_id: ListenSessionId,
    pub artist_id: ArtistId,
    pub song_id: SongId,
    pub royalty_amount: RewardAmount,
    pub royalty_percentage: f64,
    pub transaction_hash: String,
    pub paid_at: DateTime<Utc>,
}

impl ArtistRoyaltyPaid {
    pub fn new(
        session_id: ListenSessionId,
        artist_id: ArtistId,
        song_id: SongId,
        royalty_amount: RewardAmount,
        royalty_percentage: f64,
        transaction_hash: String,
    ) -> Self {
        Self {
            session_id,
            artist_id,
            song_id,
            royalty_amount,
            royalty_percentage,
            transaction_hash,
            paid_at: Utc::now(),
        }
    }
}

impl DomainEvent for ArtistRoyaltyPaid {
    fn event_type(&self) -> &'static str {
        "ArtistRoyaltyPaid"
    }

    fn aggregate_id(&self) -> Uuid {
        self.session_id.value()
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.paid_at
    }

    fn data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ZK proof verification failed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProofVerificationFailed {
    pub session_id: ListenSessionId,
    pub user_id: Uuid,
    pub song_id: SongId,
    pub proof_hash: ZkProofHash,
    pub failure_reason: String,
    pub failed_at: DateTime<Utc>,
}

impl ZkProofVerificationFailed {
    pub fn new(
        session_id: ListenSessionId,
        user_id: Uuid,
        song_id: SongId,
        proof_hash: ZkProofHash,
        failure_reason: String,
    ) -> Self {
        Self {
            session_id,
            user_id,
            song_id,
            proof_hash,
            failure_reason,
            failed_at: Utc::now(),
        }
    }
}

impl DomainEvent for ZkProofVerificationFailed {
    fn event_type(&self) -> &'static str {
        "ZkProofVerificationFailed"
    }

    fn aggregate_id(&self) -> Uuid {
        self.session_id.value()
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.failed_at
    }

    fn data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// Reward pool depleted
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardPoolDepleted {
    pub pool_id: Uuid,
    pub remaining_tokens: RewardAmount,
    pub total_distributed: RewardAmount,
    pub depleted_at: DateTime<Utc>,
}

impl RewardPoolDepleted {
    pub fn new(
        pool_id: Uuid,
        remaining_tokens: RewardAmount,
        total_distributed: RewardAmount,
    ) -> Self {
        Self {
            pool_id,
            remaining_tokens,
            total_distributed,
            depleted_at: Utc::now(),
        }
    }
}

impl DomainEvent for RewardPoolDepleted {
    fn event_type(&self) -> &'static str {
        "RewardPoolDepleted"
    }

    fn aggregate_id(&self) -> Uuid {
        self.pool_id
    }

    fn timestamp(&self) -> DateTime<Utc> {
        self.depleted_at
    }

    fn data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
} 