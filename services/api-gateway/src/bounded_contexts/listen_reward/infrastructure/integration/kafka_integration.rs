/// Kafka Integration for Listen Reward Context
/// 
/// Handles real-time event publishing to other bounded contexts:
/// - Listen sessions → Fractional Ownership (for revenue distribution)
/// - Reward calculations → Analytics (for real-time metrics)
/// - Fraud detection → Security (for alerts)

use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use tracing::{info, error, warn};

use crate::shared::infrastructure::event_bus::{
    KafkaEventBus, DomainEventWrapper, EventPayload, EventTopics,
    event_schema::{
        ListenSessionCompletedPayload, RewardCalculatedPayload, 
        RewardDistributedPayload, ArtistRoyaltyPaidPayload
    }
};
use crate::bounded_contexts::listen_reward::domain::{
    events::{ListenSessionCompleted, RewardCalculated, RewardDistributed},
    value_objects::{ListenSessionId, UserId, RewardAmount},
};
use crate::shared::domain::errors::AppError;

/// Listen Reward Kafka Event Publisher
/// 
/// Publishes listen reward events to Kafka for consumption by:
/// - Fractional Ownership (revenue distribution)
/// - Analytics Service (real-time metrics)
/// - Fraud Detection Service (behavior analysis)
/// - Artist Dashboard (earnings tracking)
pub struct ListenRewardKafkaPublisher {
    event_bus: Arc<KafkaEventBus>,
}

impl ListenRewardKafkaPublisher {
    pub fn new(event_bus: Arc<KafkaEventBus>) -> Self {
        Self { event_bus }
    }

    /// Publish listen session completion for revenue distribution
    pub async fn publish_listen_session_completed(
        &self,
        event: &ListenSessionCompleted,
    ) -> Result<(), AppError> {
        let payload = EventPayload::ListenSessionCompleted(ListenSessionCompletedPayload {
            session_id: event.session_id.value(),
            user_id: event.user_id.value(),
            song_id: event.song_id.value(),
            listen_duration_seconds: event.listen_duration.seconds(),
            quality_score: event.quality_score.score(),
            zk_proof_hash: event.zk_proof_hash.hash().clone(),
            completed_at: event.completed_at,
        });

        let domain_event = DomainEventWrapper::new(
            "ListenSessionCompleted".to_string(),
            "ListenSession".to_string(),
            event.session_id.value(),
            payload,
            event.correlation_id,
        );

        self.event_bus.publish_event(domain_event).await?;

        info!("✅ Published ListenSessionCompleted for session {}", event.session_id.value());
        Ok(())
    }

    /// Publish reward calculation for analytics
    pub async fn publish_reward_calculated(
        &self,
        event: &RewardCalculated,
    ) -> Result<(), AppError> {
        let payload = EventPayload::RewardCalculated(RewardCalculatedPayload {
            session_id: event.session_id.value(),
            user_id: event.user_id.value(),
            song_id: event.song_id.value(),
            artist_id: event.artist_id.value(),
            base_reward: event.base_reward.amount(),
            final_reward: event.final_reward.amount(),
            user_tier: event.user_tier.as_string(),
            quality_multiplier: event.quality_multiplier,
            calculated_at: event.calculated_at,
        });

        let domain_event = DomainEventWrapper::new(
            "RewardCalculated".to_string(),
            "RewardCalculation".to_string(),
            event.session_id.value(),
            payload,
            event.correlation_id,
        );

        self.event_bus.publish_event(domain_event).await?;

        info!("📊 Published RewardCalculated for session {}", event.session_id.value());
        Ok(())
    }

    /// Publish reward distribution for financial tracking
    pub async fn publish_reward_distributed(
        &self,
        event: &RewardDistributed,
    ) -> Result<(), AppError> {
        let payload = EventPayload::RewardDistributed(RewardDistributedPayload {
            session_id: event.session_id.value(),
            user_id: event.user_id.value(),
            reward_amount: event.reward_amount.amount(),
            transaction_hash: event.transaction_hash.clone(),
            distributed_at: event.distributed_at,
        });

        let domain_event = DomainEventWrapper::new(
            "RewardDistributed".to_string(),
            "RewardDistribution".to_string(),
            event.session_id.value(),
            payload,
            event.correlation_id,
        );

        self.event_bus.publish_event(domain_event).await?;

        info!("💰 Published RewardDistributed for user {}", event.user_id.value());
        Ok(())
    }

    /// Publish artist royalty payment
    pub async fn publish_artist_royalty_paid(
        &self,
        session_id: ListenSessionId,
        artist_id: Uuid,
        song_id: Uuid,
        royalty_amount: RewardAmount,
        royalty_percentage: f64,
        transaction_hash: String,
        correlation_id: Option<Uuid>,
    ) -> Result<(), AppError> {
        let payload = EventPayload::ArtistRoyaltyPaid(ArtistRoyaltyPaidPayload {
            session_id: session_id.value(),
            artist_id,
            song_id,
            royalty_amount: royalty_amount.amount(),
            royalty_percentage,
            transaction_hash,
            paid_at: chrono::Utc::now(),
        });

        let domain_event = DomainEventWrapper::new(
            "ArtistRoyaltyPaid".to_string(),
            "ArtistRoyalty".to_string(),
            artist_id.value(),
            payload,
            correlation_id,
        );

        self.event_bus.publish_event(domain_event).await?;

        info!("🎵 Published ArtistRoyaltyPaid for artist {}", artist_id.value());
        Ok(())
    }
}

/// Listen Reward Kafka Event Consumer
/// 
/// Consumes events from other contexts that affect listen rewards:
/// - Song ownership changes (affects revenue split)
/// - User tier upgrades (affects reward multipliers)
/// - Campaign activations (affects reward calculations)
pub struct ListenRewardKafkaConsumer {
    event_bus: Arc<KafkaEventBus>,
    integration_service: Arc<crate::bounded_contexts::listen_reward::infrastructure::integration::ListenRewardIntegrationService>,
}

impl ListenRewardKafkaConsumer {
    pub fn new(
        event_bus: Arc<KafkaEventBus>,
        integration_service: Arc<crate::bounded_contexts::listen_reward::infrastructure::integration::ListenRewardIntegrationService>,
    ) -> Self {
        Self {
            event_bus,
            integration_service,
        }
    }

    /// Start consuming fractional ownership events
    pub async fn start_fractional_ownership_consumer(&self) -> Result<(), AppError> {
        use crate::shared::infrastructure::event_bus::EventSubscription;

        let subscription = EventSubscription::new(
            "listen-reward-fractional-ownership".to_string(),
            vec![EventTopics::FRACTIONAL_OWNERSHIP.to_string()],
        );

        let integration_service = Arc::clone(&self.integration_service);

        self.event_bus.subscribe(subscription, move |event| {
            let integration_service = Arc::clone(&integration_service);
            
            // Handle in async context
            tokio::spawn(async move {
                if let Err(e) = Self::handle_fractional_ownership_event(integration_service, event).await {
                    error!("Failed to handle fractional ownership event: {}", e);
                }
            });
            
            Ok(())
        }).await?;

        info!("🎧 Started listening to fractional ownership events");
        Ok(())
    }

    /// Start consuming user events (tier changes, etc.)
    pub async fn start_user_consumer(&self) -> Result<(), AppError> {
        use crate::shared::infrastructure::event_bus::EventSubscription;

        let subscription = EventSubscription::new(
            "listen-reward-users".to_string(),
            vec![EventTopics::USERS.to_string()],
        );

        self.event_bus.subscribe(subscription, move |event| {
            match &event.payload {
                EventPayload::UserProfileUpdated(payload) => {
                    info!("📝 User {} profile updated: {:?}", payload.user_id, payload.updated_fields);
                    
                    // Handle tier changes that affect reward calculations
                    if payload.updated_fields.contains(&"tier".to_string()) {
                        // Invalidate user tier cache
                        // Update reward calculation parameters
                    }
                }
                _ => {}
            }
            
            Ok(())
        }).await?;

        info!("👥 Started listening to user events");
        Ok(())
    }

    /// Start consuming campaign events
    pub async fn start_campaign_consumer(&self) -> Result<(), AppError> {
        use crate::shared::infrastructure::event_bus::EventSubscription;

        let subscription = EventSubscription::new(
            "listen-reward-campaigns".to_string(),
            vec![EventTopics::CAMPAIGNS.to_string()],
        );

        self.event_bus.subscribe(subscription, move |event| {
            match &event.payload {
                EventPayload::CampaignActivated(payload) => {
                    info!("🚀 Campaign {} activated", payload.campaign_id);
                    
                    // Update reward parameters for campaign songs
                    // Apply special reward multipliers
                }
                EventPayload::NFTPurchased(payload) => {
                    info!("🖼️ NFT purchased for campaign {}", payload.campaign_id);
                    
                    // Update supporter tier for user
                    // Apply NFT holder bonuses
                }
                _ => {}
            }
            
            Ok(())
        }).await?;

        info!("🎪 Started listening to campaign events");
        Ok(())
    }

    async fn handle_fractional_ownership_event(
        integration_service: Arc<crate::bounded_contexts::listen_reward::infrastructure::integration::ListenRewardIntegrationService>,
        event: DomainEventWrapper,
    ) -> Result<(), AppError> {
        match &event.payload {
            EventPayload::OwnershipContractCreated(payload) => {
                info!("🏗️ New ownership contract created for song {}", payload.song_id);
                
                // Update song revenue split configuration
                // This affects how future listen rewards are calculated
                // integration_service.update_song_revenue_split(payload.song_id, payload.artist_retained_percentage).await?;
            }
            
            EventPayload::SharesPurchased(payload) => {
                info!("💰 Shares purchased for song {} by user {}", payload.song_id, payload.buyer_id);
                
                // Update ownership distribution for revenue calculations
                // This affects how artist royalties vs investor dividends are split
                // integration_service.update_ownership_distribution(payload.song_id).await?;
            }
            
            EventPayload::RevenueDistributed(payload) => {
                info!("📈 Revenue distributed for contract {}: ${:.2}", payload.contract_id, payload.total_distributed);
                
                // Track successful revenue distribution
                // Update metrics for artist dashboard
            }
            
            _ => {
                // Other fractional ownership events we don't need to handle
            }
        }
        
        Ok(())
    }
}

/// Kafka Integration Factory for Listen Reward Context
pub struct ListenRewardKafkaIntegration {
    publisher: Arc<ListenRewardKafkaPublisher>,
    consumer: Arc<ListenRewardKafkaConsumer>,
}

impl ListenRewardKafkaIntegration {
    pub async fn new(
        event_bus: Arc<KafkaEventBus>,
        integration_service: Arc<crate::bounded_contexts::listen_reward::infrastructure::integration::ListenRewardIntegrationService>,
    ) -> Self {
        let publisher = Arc::new(ListenRewardKafkaPublisher::new(Arc::clone(&event_bus)));
        let consumer = Arc::new(ListenRewardKafkaConsumer::new(event_bus, integration_service));
        
        Self {
            publisher,
            consumer,
        }
    }

    pub fn get_publisher(&self) -> Arc<ListenRewardKafkaPublisher> {
        Arc::clone(&self.publisher)
    }

    pub fn get_consumer(&self) -> Arc<ListenRewardKafkaConsumer> {
        Arc::clone(&self.consumer)
    }

    /// Start all consumers
    pub async fn start_consumers(&self) -> Result<(), AppError> {
        info!("🚀 Starting Listen Reward Kafka consumers...");
        
        // Start all event consumers
        self.consumer.start_fractional_ownership_consumer().await?;
        self.consumer.start_user_consumer().await?;
        self.consumer.start_campaign_consumer().await?;
        
        info!("✅ All Listen Reward Kafka consumers started successfully");
        Ok(())
    }

    /// Health check for Kafka integration
    pub async fn health_check(&self) -> Result<(), AppError> {
        // Test publish a health check event
        let test_payload = EventPayload::SystemHealthCheck(
            crate::shared::infrastructure::event_bus::event_schema::SystemHealthCheckPayload {
                service: "listen-reward-kafka-integration".to_string(),
                status: "healthy".to_string(),
                response_time_ms: 0,
                timestamp: chrono::Utc::now(),
            }
        );

        let test_event = DomainEventWrapper::new(
            "HealthCheck".to_string(),
            "ListenRewardIntegration".to_string(),
            uuid::Uuid::new_v4(),
            test_payload,
            None,
        );

        match self.publisher.event_bus.publish_event(test_event).await {
            Ok(_) => {
                info!("✅ Listen Reward Kafka integration health check passed");
                Ok(())
            }
            Err(e) => {
                error!("❌ Listen Reward Kafka integration health check failed: {}", e);
                Err(e)
            }
        }
    }
}

/// Example: Complete Listen → Revenue Distribution Flow via Kafka
/// 
/// This demonstrates the end-to-end flow:
/// 1. User completes listen session
/// 2. Listen Reward calculates rewards
/// 3. Events published to Kafka
/// 4. Fractional Ownership consumes events
/// 5. Revenue distribution triggered
/// 6. Results published back to Kafka
/// 7. Analytics updated in real-time
pub struct CompleteFlowExample;

impl CompleteFlowExample {
    /// Simulate complete listen-to-revenue-distribution flow
    pub async fn simulate_complete_flow(
        kafka_integration: &ListenRewardKafkaIntegration,
    ) -> Result<(), AppError> {
        info!("🎬 Starting complete listen-to-revenue flow simulation...");

        // 1. Simulate listen session completion
        let session_completed = ListenSessionCompleted {
            session_id: crate::bounded_contexts::listen_reward::domain::value_objects::ListenSessionId::new(),
            user_id: crate::bounded_contexts::listen_reward::domain::value_objects::UserId::new(),
            song_id: uuid::Uuid::new_v4(),
            artist_id: uuid::Uuid::new_v4(),
            listen_duration: crate::bounded_contexts::listen_reward::domain::value_objects::ListenDuration::from_seconds(180),
            quality_score: crate::bounded_contexts::listen_reward::domain::value_objects::QualityScore::new(0.95),
            zk_proof_hash: crate::bounded_contexts::listen_reward::domain::value_objects::ZkProofHash::new("proof_hash_example".to_string()),
            completed_at: chrono::Utc::now(),
            correlation_id: Some(uuid::Uuid::new_v4()),
        };

        // 2. Publish listen session completed
        kafka_integration.publisher.publish_listen_session_completed(&session_completed).await?;

        // 3. This triggers:
        //    - Fractional Ownership service reads the event
        //    - Calculates revenue split based on ownership
        //    - Distributes rewards to shareholders
        //    - Publishes RevenueDistributed event back to Kafka

        // 4. Analytics service processes all events for real-time metrics
        //    - Listen count incremented
        //    - Revenue forecasts updated
        //    - User engagement metrics updated
        //    - Artist earnings tracked

        info!("✅ Complete flow simulation initiated successfully");
        info!("📊 Check analytics dashboard for real-time updates");
        info!("💰 Revenue distribution will be processed asynchronously");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_kafka_integration_creation() {
        // Mock test - in real integration tests we'd have actual Kafka
        // This demonstrates the integration structure
        
        // let event_bus = Arc::new(KafkaEventBus::new(Default::default()).await.unwrap());
        // let integration_service = Arc::new(mock_integration_service());
        // let kafka_integration = ListenRewardKafkaIntegration::new(event_bus, integration_service).await;
        
        // assert!(kafka_integration.health_check().await.is_ok());
    }
} 