use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

/// Domain Event Wrapper for Kafka serialization
/// 
/// This wraps all domain events in a consistent format for cross-context communication.
/// Each bounded context can serialize/deserialize their specific events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainEventWrapper {
    /// Event metadata
    pub metadata: EventMetadata,
    /// The actual event payload
    pub payload: EventPayload,
}

impl DomainEventWrapper {
    pub fn new(
        event_type: String,
        aggregate_type: String,
        aggregate_id: Uuid,
        payload: EventPayload,
        correlation_id: Option<Uuid>,
    ) -> Self {
        Self {
            metadata: EventMetadata {
                event_id: Uuid::new_v4(),
                event_type,
                aggregate_type,
                aggregate_id,
                correlation_id,
                causation_id: None,
                occurred_at: Utc::now(),
                version: 1,
                producer: "vibestream-api-gateway".to_string(),
                headers: HashMap::new(),
            },
            payload,
        }
    }

    pub fn with_correlation(mut self, correlation_id: Uuid) -> Self {
        self.metadata.correlation_id = Some(correlation_id);
        self
    }

    pub fn with_causation(mut self, causation_id: Uuid) -> Self {
        self.metadata.causation_id = Some(causation_id);
        self
    }

    pub fn add_header(mut self, key: String, value: String) -> Self {
        self.metadata.headers.insert(key, value);
        self
    }
}

/// Event metadata for tracking and routing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    /// Unique event identifier
    pub event_id: Uuid,
    /// Type of event (e.g., "ListenSessionCompleted", "SharesPurchased")
    pub event_type: String,
    /// Type of aggregate that produced the event
    pub aggregate_type: String,
    /// ID of the aggregate that produced the event
    pub aggregate_id: Uuid,
    /// Correlation ID for tracking related events
    pub correlation_id: Option<Uuid>,
    /// ID of the event that caused this event
    pub causation_id: Option<Uuid>,
    /// When the event occurred
    pub occurred_at: DateTime<Utc>,
    /// Event schema version
    pub version: u32,
    /// Service that produced the event
    pub producer: String,
    /// Additional headers for routing/filtering
    pub headers: HashMap<String, String>,
}

/// Event payload variants for different bounded contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum EventPayload {
    // Listen Reward Events
    ListenSessionStarted(ListenSessionStartedPayload),
    ListenSessionCompleted(ListenSessionCompletedPayload),
    RewardCalculated(RewardCalculatedPayload),
    RewardDistributed(RewardDistributedPayload),
    ArtistRoyaltyPaid(ArtistRoyaltyPaidPayload),

    // Fractional Ownership Events
    OwnershipContractCreated(OwnershipContractCreatedPayload),
    SharesPurchased(SharesPurchasedPayload),
    SharesTraded(SharesTradedPayload),
    RevenueDistributed(RevenueDistributedPayload),
    OwnershipContractTerminated(OwnershipContractTerminatedPayload),

    // Music Events
    SongUploaded(SongUploadedPayload),
    SongListened(SongListenedPayload),
    AlbumCreated(AlbumCreatedPayload),

    // Campaign Events
    CampaignCreated(CampaignCreatedPayload),
    CampaignActivated(CampaignActivatedPayload),
    NFTPurchased(NFTPurchasedPayload),

    // User Events
    UserRegistered(UserRegisteredPayload),
    UserProfileUpdated(UserProfileUpdatedPayload),

    // System Events
    SystemHealthCheck(SystemHealthCheckPayload),
    Analytics(AnalyticsPayload),
}

// Listen Reward Event Payloads
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenSessionStartedPayload {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub user_tier: String,
    pub started_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenSessionCompletedPayload {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub song_id: Uuid,
    pub listen_duration_seconds: u32,
    pub quality_score: f64,
    pub zk_proof_hash: String,
    pub completed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardCalculatedPayload {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub base_reward: f64,
    pub final_reward: f64,
    pub user_tier: String,
    pub quality_multiplier: f64,
    pub calculated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardDistributedPayload {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub reward_amount: f64,
    pub transaction_hash: String,
    pub distributed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistRoyaltyPaidPayload {
    pub session_id: Uuid,
    pub artist_id: Uuid,
    pub song_id: Uuid,
    pub royalty_amount: f64,
    pub royalty_percentage: f64,
    pub transaction_hash: String,
    pub paid_at: DateTime<Utc>,
}

// Fractional Ownership Event Payloads
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipContractCreatedPayload {
    pub contract_id: Uuid,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub total_shares: u32,
    pub price_per_share: f64,
    pub artist_retained_percentage: f64,
    pub shares_available_for_sale: u32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharesPurchasedPayload {
    pub contract_id: Uuid,
    pub share_id: Uuid,
    pub buyer_id: Uuid,
    pub song_id: Uuid,
    pub ownership_percentage: f64,
    pub purchase_price: f64,
    pub transaction_hash: Option<String>,
    pub purchased_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharesTradedPayload {
    pub contract_id: Uuid,
    pub share_id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub song_id: Uuid,
    pub ownership_percentage: f64,
    pub trade_price: f64,
    pub transaction_hash: Option<String>,
    pub traded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueDistributedPayload {
    pub contract_id: Uuid,
    pub song_id: Uuid,
    pub total_revenue: f64,
    pub distribution_period_start: DateTime<Utc>,
    pub distribution_period_end: DateTime<Utc>,
    pub total_distributed: f64,
    pub artist_share: f64,
    pub platform_fee: f64,
    pub shareholder_count: u32,
    pub distributed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipContractTerminatedPayload {
    pub contract_id: Uuid,
    pub song_id: Uuid,
    pub termination_reason: String,
    pub terminated_by: Uuid,
    pub terminated_at: DateTime<Utc>,
}

// Music Event Payloads
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongUploadedPayload {
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub title: String,
    pub genre: String,
    pub duration_seconds: u32,
    pub uploaded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongListenedPayload {
    pub song_id: Uuid,
    pub listener_id: Uuid,
    pub listen_count: u64,
    pub listen_duration_seconds: u32,
    pub listened_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumCreatedPayload {
    pub album_id: Uuid,
    pub artist_id: Uuid,
    pub title: String,
    pub song_ids: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
}

// Campaign Event Payloads
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignCreatedPayload {
    pub campaign_id: Uuid,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub target_revenue: f64,
    pub nft_price: f64,
    pub max_nfts: u32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampaignActivatedPayload {
    pub campaign_id: Uuid,
    pub activated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NFTPurchasedPayload {
    pub campaign_id: Uuid,
    pub buyer_id: Uuid,
    pub nft_id: Uuid,
    pub purchase_price: f64,
    pub transaction_hash: String,
    pub purchased_at: DateTime<Utc>,
}

// User Event Payloads
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegisteredPayload {
    pub user_id: Uuid,
    pub email: String,
    pub user_type: String, // "artist", "fan", "investor"
    pub registered_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileUpdatedPayload {
    pub user_id: Uuid,
    pub updated_fields: Vec<String>,
    pub updated_at: DateTime<Utc>,
}

// System Event Payloads
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealthCheckPayload {
    pub service: String,
    pub status: String,
    pub response_time_ms: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsPayload {
    pub event_type: String,
    pub entity_id: Uuid,
    pub metrics: HashMap<String, serde_json::Value>,
    pub timestamp: DateTime<Utc>,
}

/// Kafka topics for VibeStream events
pub struct EventTopics;

impl EventTopics {
    // Core business events
    pub const LISTEN_SESSIONS: &'static str = "vibestream.listen-sessions";
    pub const REWARDS: &'static str = "vibestream.rewards";
    pub const FRACTIONAL_OWNERSHIP: &'static str = "vibestream.fractional-ownership";
    pub const MUSIC_CATALOG: &'static str = "vibestream.music-catalog";
    pub const CAMPAIGNS: &'static str = "vibestream.campaigns";
    pub const USERS: &'static str = "vibestream.users";
    
    // Analytics and monitoring
    pub const ANALYTICS: &'static str = "vibestream.analytics";
    pub const SYSTEM_EVENTS: &'static str = "vibestream.system";
    pub const AUDIT_LOG: &'static str = "vibestream.audit";
    
    // Dead letter queues
    pub const DLQ: &'static str = "vibestream.dlq";
    
    /// Get topic for event type
    pub fn get_topic_for_event(event_type: &str) -> &'static str {
        match event_type {
            "ListenSessionStarted" | "ListenSessionCompleted" => Self::LISTEN_SESSIONS,
            "RewardCalculated" | "RewardDistributed" | "ArtistRoyaltyPaid" => Self::REWARDS,
            "OwnershipContractCreated" | "SharesPurchased" | "SharesTraded" | "RevenueDistributed" => Self::FRACTIONAL_OWNERSHIP,
            "SongUploaded" | "SongListened" | "AlbumCreated" => Self::MUSIC_CATALOG,
            "CampaignCreated" | "CampaignActivated" | "NFTPurchased" => Self::CAMPAIGNS,
            "UserRegistered" | "UserProfileUpdated" => Self::USERS,
            "Analytics" => Self::ANALYTICS,
            "SystemHealthCheck" => Self::SYSTEM_EVENTS,
            _ => Self::SYSTEM_EVENTS,
        }
    }
}

/// Event routing key for partitioning with guaranteed ordering
/// 
/// CRITICAL: Events for the same entity MUST go to the same partition
/// to guarantee ordering. This is essential for financial transactions.
pub fn get_partition_key(event: &DomainEventWrapper) -> String {
    match &event.payload {
        // FINANCIAL EVENTS - Must maintain strict ordering
        EventPayload::SharesPurchased(payload) => {
            // All events for same contract go to same partition
            format!("contract:{}", payload.contract_id)
        }
        EventPayload::SharesTraded(payload) => {
            // Same contract ordering
            format!("contract:{}", payload.contract_id)
        }
        EventPayload::RevenueDistributed(payload) => {
            // Revenue for same contract must be ordered
            format!("contract:{}", payload.contract_id)
        }
        EventPayload::OwnershipContractCreated(payload) => {
            format!("contract:{}", payload.contract_id)
        }
        EventPayload::OwnershipContractTerminated(payload) => {
            format!("contract:{}", payload.contract_id)
        }

        // USER EVENTS - Per user ordering
        EventPayload::ListenSessionStarted(payload) => {
            // All listen events for same user stay ordered
            format!("user:{}", payload.user_id)
        }
        EventPayload::ListenSessionCompleted(payload) => {
            format!("user:{}", payload.user_id)
        }
        EventPayload::RewardCalculated(payload) => {
            format!("user:{}", payload.user_id)
        }
        EventPayload::RewardDistributed(payload) => {
            format!("user:{}", payload.user_id)
        }
        EventPayload::UserRegistered(payload) => {
            format!("user:{}", payload.user_id)
        }
        EventPayload::UserProfileUpdated(payload) => {
            format!("user:{}", payload.user_id)
        }

        // SONG EVENTS - Per song ordering  
        EventPayload::SongUploaded(payload) => {
            format!("song:{}", payload.song_id)
        }
        EventPayload::SongListened(payload) => {
            format!("song:{}", payload.song_id)
        }
        EventPayload::ArtistRoyaltyPaid(payload) => {
            // Artist payments must be ordered
            format!("artist:{}", payload.artist_id)
        }

        // CAMPAIGN EVENTS - Per campaign ordering
        EventPayload::CampaignCreated(payload) => {
            format!("campaign:{}", payload.campaign_id)
        }
        EventPayload::CampaignActivated(payload) => {
            format!("campaign:{}", payload.campaign_id)
        }
        EventPayload::NFTPurchased(payload) => {
            format!("campaign:{}", payload.campaign_id)
        }

        // ALBUM EVENTS - Per album ordering
        EventPayload::AlbumCreated(payload) => {
            format!("album:{}", payload.album_id)
        }

        // SYSTEM EVENTS - Service-based partitioning
        EventPayload::SystemHealthCheck(payload) => {
            format!("service:{}", payload.service)
        }
        EventPayload::Analytics(payload) => {
            format!("entity:{}", payload.entity_id)
        }
    }
}

/// Specialized partition key for high-frequency events
/// 
/// For events that happen millions of times per second,
/// we need more granular partitioning while maintaining order
pub fn get_high_frequency_partition_key(event: &DomainEventWrapper) -> String {
    match &event.payload {
        EventPayload::ListenSessionCompleted(payload) => {
            // For listen sessions, we want to distribute load but maintain
            // user ordering. Use user_id with time bucket for better distribution
            let time_bucket = payload.completed_at.timestamp() / 3600; // Hour buckets
            format!("user:{}:hour:{}", payload.user_id, time_bucket)
        }
        EventPayload::Analytics(payload) => {
            // Analytics can be distributed more aggressively
            let time_bucket = payload.timestamp.timestamp() / 300; // 5-minute buckets
            format!("analytics:{}:bucket:{}", payload.entity_id, time_bucket)
        }
        _ => get_partition_key(event), // Use standard partitioning
    }
}

/// Financial event ordering validator
/// 
/// Ensures that financial events maintain strict ordering requirements
pub struct EventOrderingValidator;

impl EventOrderingValidator {
    /// Validate that event can be safely processed in order
    pub fn validate_financial_ordering(event: &DomainEventWrapper) -> Result<(), String> {
        match &event.payload {
            EventPayload::SharesPurchased(payload) => {
                if payload.ownership_percentage <= 0.0 || payload.ownership_percentage > 100.0 {
                    return Err("Invalid ownership percentage".to_string());
                }
                if payload.purchase_price <= 0.0 {
                    return Err("Invalid purchase price".to_string());
                }
            }
            EventPayload::RevenueDistributed(payload) => {
                if payload.total_distributed > payload.total_revenue {
                    return Err("Cannot distribute more than total revenue".to_string());
                }
                if payload.shareholder_count == 0 {
                    return Err("Cannot distribute to zero shareholders".to_string());
                }
            }
            EventPayload::SharesTraded(payload) => {
                if payload.ownership_percentage <= 0.0 {
                    return Err("Cannot trade zero ownership".to_string());
                }
                if payload.from_user_id == payload.to_user_id {
                    return Err("Cannot trade with self".to_string());
                }
            }
            _ => {} // Non-financial events don't need strict validation
        }
        Ok(())
    }

    /// Check if event requires strict ordering
    pub fn requires_strict_ordering(event: &DomainEventWrapper) -> bool {
        matches!(event.payload,
            EventPayload::SharesPurchased(_) |
            EventPayload::SharesTraded(_) |
            EventPayload::RevenueDistributed(_) |
            EventPayload::OwnershipContractCreated(_) |
            EventPayload::OwnershipContractTerminated(_) |
            EventPayload::ArtistRoyaltyPaid(_) |
            EventPayload::RewardDistributed(_)
        )
    }

    /// Get ordering priority (higher = more critical)
    pub fn get_ordering_priority(event: &DomainEventWrapper) -> u8 {
        match &event.payload {
            // Critical financial events
            EventPayload::RevenueDistributed(_) => 10,
            EventPayload::SharesTraded(_) => 9,
            EventPayload::SharesPurchased(_) => 8,
            EventPayload::ArtistRoyaltyPaid(_) => 7,
            EventPayload::RewardDistributed(_) => 6,
            
            // Contract lifecycle events
            EventPayload::OwnershipContractCreated(_) => 5,
            EventPayload::OwnershipContractTerminated(_) => 5,
            
            // User events
            EventPayload::UserRegistered(_) => 4,
            EventPayload::UserProfileUpdated(_) => 3,
            
            // Content events
            EventPayload::SongUploaded(_) => 3,
            EventPayload::AlbumCreated(_) => 3,
            
            // Analytics and monitoring
            EventPayload::ListenSessionCompleted(_) => 2,
            EventPayload::RewardCalculated(_) => 2,
            EventPayload::Analytics(_) => 1,
            EventPayload::SystemHealthCheck(_) => 1,
            
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_wrapper_creation() {
        let payload = EventPayload::ListenSessionCompleted(ListenSessionCompletedPayload {
            session_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            song_id: Uuid::new_v4(),
            listen_duration_seconds: 180,
            quality_score: 0.95,
            zk_proof_hash: "proof123".to_string(),
            completed_at: Utc::now(),
        });

        let event = DomainEventWrapper::new(
            "ListenSessionCompleted".to_string(),
            "ListenSession".to_string(),
            Uuid::new_v4(),
            payload,
            None,
        );

        assert_eq!(event.metadata.event_type, "ListenSessionCompleted");
        assert_eq!(event.metadata.producer, "vibestream-api-gateway");
    }

    #[test]
    fn test_topic_routing() {
        assert_eq!(EventTopics::get_topic_for_event("ListenSessionCompleted"), EventTopics::LISTEN_SESSIONS);
        assert_eq!(EventTopics::get_topic_for_event("SharesPurchased"), EventTopics::FRACTIONAL_OWNERSHIP);
        assert_eq!(EventTopics::get_topic_for_event("SongUploaded"), EventTopics::MUSIC_CATALOG);
    }
} 