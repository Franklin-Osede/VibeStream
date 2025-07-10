use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::listen_reward::domain::value_objects::{
    ListenSessionId, RewardAmount, ListenDuration, QualityScore, ZkProofHash, RewardTier
};
use crate::bounded_contexts::music::domain::value_objects::{SongId, ArtistId};
use crate::shared::domain::events::{DomainEvent, EventMetadata};

// Listen session started
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenSessionStarted {
    pub session_id: ListenSessionId,
    pub user_id: Uuid,
    pub song_id: SongId,
    pub artist_id: ArtistId,
    pub quality_score: QualityScore,
    pub started_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl ListenSessionStarted {
    pub fn new(
        session_id: ListenSessionId,
        user_id: Uuid,
        song_id: SongId,
        artist_id: ArtistId,
        quality_score: QualityScore,
        started_at: DateTime<Utc>,
    ) -> Self {
        Self {
            session_id,
            user_id,
            song_id,
            artist_id,
            quality_score,
            started_at,
            metadata: EventMetadata::new(),
        }
    }
}

impl DomainEvent for ListenSessionStarted {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }

    fn event_type(&self) -> &str {
        "ListenSessionStarted"
    }

    fn aggregate_id(&self) -> Uuid {
        self.session_id.to_uuid()
    }

    fn aggregate_type(&self) -> &str {
        "ListenSession"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.started_at
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}

// Listen session completed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenSessionCompleted {
    pub session_id: ListenSessionId,
    pub user_id: Uuid,
    pub song_id: SongId,
    pub artist_id: ArtistId,
    pub duration: ListenDuration,
    pub quality_score: QualityScore,
    pub completion_percentage: f64,
    pub completed_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl ListenSessionCompleted {
    pub fn new(
        session_id: ListenSessionId,
        user_id: Uuid,
        song_id: SongId,
        artist_id: ArtistId,
        duration: ListenDuration,
        quality_score: QualityScore,
        completion_percentage: f64,
        completed_at: DateTime<Utc>,
    ) -> Self {
        Self {
            session_id,
            user_id,
            song_id,
            artist_id,
            duration,
            quality_score,
            completion_percentage,
            completed_at,
            metadata: EventMetadata::new(),
        }
    }
}

impl DomainEvent for ListenSessionCompleted {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }

    fn event_type(&self) -> &str {
        "ListenSessionCompleted"
    }

    fn aggregate_id(&self) -> Uuid {
        self.session_id.to_uuid()
    }

    fn aggregate_type(&self) -> &str {
        "ListenSession"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.completed_at
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}

// ZK Proof verification failed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZkProofVerificationFailed {
    pub session_id: ListenSessionId,
    pub user_id: Uuid,
    pub song_id: SongId,
    pub artist_id: ArtistId,
    pub proof_hash: ZkProofHash,
    pub failure_reason: String,
    pub failed_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl ZkProofVerificationFailed {
    pub fn new(
        session_id: ListenSessionId,
        user_id: Uuid,
        song_id: SongId,
        artist_id: ArtistId,
        proof_hash: ZkProofHash,
        failure_reason: String,
        failed_at: DateTime<Utc>,
    ) -> Self {
        Self {
            session_id,
            user_id,
            song_id,
            artist_id,
            proof_hash,
            failure_reason,
            failed_at,
            metadata: EventMetadata::new(),
        }
    }
}

impl DomainEvent for ZkProofVerificationFailed {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }

    fn event_type(&self) -> &str {
        "ZkProofVerificationFailed"
    }

    fn aggregate_id(&self) -> Uuid {
        self.session_id.to_uuid()
    }

    fn aggregate_type(&self) -> &str {
        "ListenSession"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.failed_at
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}

// Reward calculated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardCalculated {
    pub session_id: ListenSessionId,
    pub user_id: Uuid,
    pub song_id: SongId,
    pub artist_id: ArtistId,
    pub base_reward: RewardAmount,
    pub final_reward: RewardAmount,
    pub calculated_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl RewardCalculated {
    pub fn new(
        session_id: ListenSessionId,
        user_id: Uuid,
        song_id: SongId,
        artist_id: ArtistId,
        base_reward: RewardAmount,
        final_reward: RewardAmount,
        calculated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            session_id,
            user_id,
            song_id,
            artist_id,
            base_reward,
            final_reward,
            calculated_at,
            metadata: EventMetadata::new(),
        }
    }
}

impl DomainEvent for RewardCalculated {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }

    fn event_type(&self) -> &str {
        "RewardCalculated"
    }

    fn aggregate_id(&self) -> Uuid {
        self.session_id.to_uuid()
    }

    fn aggregate_type(&self) -> &str {
        "ListenSession"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.calculated_at
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}

// Reward distribution created
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardDistributionCreated {
    pub distribution_id: Uuid,
    pub session_id: ListenSessionId,
    pub user_id: Uuid,
    pub artist_id: ArtistId,
    pub total_amount: RewardAmount,
    pub user_share: RewardAmount,
    pub artist_share: RewardAmount,
    pub platform_fee: RewardAmount,
    pub created_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl RewardDistributionCreated {
    pub fn new(
        distribution_id: Uuid,
        session_id: ListenSessionId,
        user_id: Uuid,
        artist_id: ArtistId,
        total_amount: RewardAmount,
        user_share: RewardAmount,
        artist_share: RewardAmount,
        platform_fee: RewardAmount,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            distribution_id,
            session_id,
            user_id,
            artist_id,
            total_amount,
            user_share,
            artist_share,
            platform_fee,
            created_at,
            metadata: EventMetadata::new(),
        }
    }
}

impl DomainEvent for RewardDistributionCreated {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }

    fn event_type(&self) -> &str {
        "RewardDistributionCreated"
    }

    fn aggregate_id(&self) -> Uuid {
        self.distribution_id
    }

    fn aggregate_type(&self) -> &str {
        "RewardDistribution"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}

// Reward distributed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardDistributed {
    pub distribution_id: Uuid,
    pub user_id: Uuid,
    pub artist_id: ArtistId,
    pub amount: RewardAmount,
    pub recipient_type: String, // "user", "artist", "platform"
    pub distributed_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl RewardDistributed {
    pub fn new(
        distribution_id: Uuid,
        user_id: Uuid,
        artist_id: ArtistId,
        amount: RewardAmount,
        recipient_type: String,
        distributed_at: DateTime<Utc>,
    ) -> Self {
        Self {
            distribution_id,
            user_id,
            artist_id,
            amount,
            recipient_type,
            distributed_at,
            metadata: EventMetadata::new(),
        }
    }
}

impl DomainEvent for RewardDistributed {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }

    fn event_type(&self) -> &str {
        "RewardDistributed"
    }

    fn aggregate_id(&self) -> Uuid {
        self.distribution_id
    }

    fn aggregate_type(&self) -> &str {
        "RewardDistribution"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.distributed_at
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}

// Artist royalty paid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistRoyaltyPaid {
    pub payment_id: Uuid,
    pub artist_id: ArtistId,
    pub song_id: SongId,
    pub amount: RewardAmount,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub paid_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl ArtistRoyaltyPaid {
    pub fn new(
        payment_id: Uuid,
        artist_id: ArtistId,
        song_id: SongId,
        amount: RewardAmount,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
        paid_at: DateTime<Utc>,
    ) -> Self {
        Self {
            payment_id,
            artist_id,
            song_id,
            amount,
            period_start,
            period_end,
            paid_at,
            metadata: EventMetadata::new(),
        }
    }
}

impl DomainEvent for ArtistRoyaltyPaid {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }

    fn event_type(&self) -> &str {
        "ArtistRoyaltyPaid"
    }

    fn aggregate_id(&self) -> Uuid {
        self.payment_id
    }

    fn aggregate_type(&self) -> &str {
        "ArtistPayment"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.paid_at
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}

// Reward pool depleted
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardPoolDepleted {
    pub pool_id: Uuid,
    pub remaining_amount: RewardAmount,
    pub threshold_percentage: f64,
    pub depleted_at: DateTime<Utc>,
    pub metadata: EventMetadata,
}

impl RewardPoolDepleted {
    pub fn new(
        pool_id: Uuid,
        remaining_amount: RewardAmount,
        threshold_percentage: f64,
        depleted_at: DateTime<Utc>,
    ) -> Self {
        Self {
            pool_id,
            remaining_amount,
            threshold_percentage,
            depleted_at,
            metadata: EventMetadata::new(),
        }
    }
}

impl DomainEvent for RewardPoolDepleted {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }

    fn event_type(&self) -> &str {
        "RewardPoolDepleted"
    }

    fn aggregate_id(&self) -> Uuid {
        self.pool_id
    }

    fn aggregate_type(&self) -> &str {
        "RewardPool"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.depleted_at
    }

    fn event_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
} 