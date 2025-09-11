use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::bounded_contexts::listen_reward::domain::{
    entities::ListenSession,
    aggregates::RewardDistribution, 
    value_objects::{RewardAmount, ValidationPeriod}
};
use crate::shared::domain::events::DomainEvent;
use vibestream_types::RoyaltyPercentage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessRewardDistributionCommand {
    pub session_id: String,
    pub user_transaction_hash: String,
    pub artist_transaction_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessRewardDistributionResponse {
    pub session_id: String,
    pub user_id: Uuid,
    pub reward_amount: f64,
    pub artist_royalty_amount: f64,
    pub user_transaction_hash: String,
    pub artist_transaction_hash: String,
    pub processed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueRewardDistributionCommand {
    pub session_id: String,
    pub royalty_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueRewardDistributionResponse {
    pub session_id: String,
    pub reward_amount: f64,
    pub royalty_percentage: f64,
    pub queued_at: String,
}

pub struct ProcessRewardDistributionUseCase;

impl ProcessRewardDistributionUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn queue_distribution(
        &self,
        mut distribution: RewardDistribution,
        session: &ListenSession,
        command: QueueRewardDistributionCommand,
    ) -> Result<(RewardDistribution, QueueRewardDistributionResponse), String> {
        // Validate command
        self.validate_queue_command(&command)?;

        // Parse royalty percentage
        let royalty_percentage = RoyaltyPercentage::new(
            rust_decimal::Decimal::try_from(command.royalty_percentage).unwrap_or_default(),
            "USD".to_string()
        );

        // Queue the distribution
        distribution.queue_reward_distribution(session, &royalty_percentage)?;

        // Build response
        let reward_amount = session.final_reward()
            .ok_or("Session has no calculated reward")?
            .tokens();

        let response = QueueRewardDistributionResponse {
            session_id: command.session_id,
            reward_amount,
            royalty_percentage: command.royalty_percentage,
            queued_at: chrono::Utc::now().to_rfc3339(),
        };

        Ok((distribution, response))
    }

    pub fn execute_distribution(
        &self,
        mut distribution: RewardDistribution,
        mut session: ListenSession,
        command: ProcessRewardDistributionCommand,
    ) -> Result<(RewardDistribution, ListenSession, ProcessRewardDistributionResponse, Vec<Box<dyn DomainEvent>>), String> {
        // Validate command
        self.validate_execute_command(&command)?;

        // Parse session ID
        let session_id_uuid = Uuid::parse_str(&command.session_id)
            .map_err(|_| "Invalid session ID format")?;
        let session_id = crate::bounded_contexts::listen_reward::domain::ListenSessionId::from_uuid(session_id_uuid);

        // Execute the distribution
        distribution.execute_distribution(
            &session_id,
            command.user_transaction_hash.clone(),
            command.artist_transaction_hash.clone(),
        )?;

        // Mark session as rewarded
        session.mark_rewarded()?;

        // Get events
        let events = distribution.take_uncommitted_events();

        // Calculate royalty amount
        let reward_amount = session.final_reward()
            .ok_or("Session has no calculated reward")?
            .tokens();

        // Assuming 10% royalty for this example - in real implementation this would come from song metadata
        let royalty_amount = reward_amount * 0.10;

        // Build response
        let response = ProcessRewardDistributionResponse {
            session_id: command.session_id,
            user_id: session.user_id(),
            reward_amount,
            artist_royalty_amount: royalty_amount,
            user_transaction_hash: command.user_transaction_hash,
            artist_transaction_hash: command.artist_transaction_hash,
            processed_at: chrono::Utc::now().to_rfc3339(),
        };

        Ok((distribution, session, response, events))
    }

    pub fn create_reward_pool(
        &self,
        total_tokens: f64,
        validation_period_hours: u64,
    ) -> Result<RewardDistribution, String> {
        let reward_amount = RewardAmount::new(total_tokens)
            .map_err(|e| format!("Invalid reward amount: {}", e))?;

        let pool_id = crate::bounded_contexts::listen_reward::domain::value_objects::RewardPoolId::from_uuid(uuid::Uuid::new_v4());
        
        // Crear RewardPool para el agregado
        let aggregate_pool = crate::bounded_contexts::listen_reward::domain::aggregates::reward_distribution::RewardPool::new(
            reward_amount,
            ValidationPeriod::daily(), // Simplificado por ahora
        );
        
        // Crear la distribución con el RewardPool del agregado
        let distribution = RewardDistribution::new(aggregate_pool);

        Ok(distribution)
    }

    fn validate_queue_command(&self, command: &QueueRewardDistributionCommand) -> Result<(), String> {
        if command.session_id.is_empty() {
            return Err("Session ID cannot be empty".to_string());
        }

        if !(0.0..=100.0).contains(&command.royalty_percentage) {
            return Err("Royalty percentage must be between 0.0 and 100.0".to_string());
        }

        Ok(())
    }

    fn validate_execute_command(&self, command: &ProcessRewardDistributionCommand) -> Result<(), String> {
        if command.session_id.is_empty() {
            return Err("Session ID cannot be empty".to_string());
        }

        if command.user_transaction_hash.is_empty() {
            return Err("User transaction hash cannot be empty".to_string());
        }

        if command.artist_transaction_hash.is_empty() {
            return Err("Artist transaction hash cannot be empty".to_string());
        }

        // Validate UUID format
        Uuid::parse_str(&command.session_id)
            .map_err(|_| "Invalid session ID format")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_contexts::music::domain::{SongId, ArtistId};
    use crate::bounded_contexts::listen_reward::domain::{RewardTier, ListenDuration, QualityScore, ZkProofHash};
    use vibestream_types::{SongContract, ArtistContract};

    fn create_test_session() -> ListenSession {
        let song_contract = SongContract::new(
            Uuid::new_v4(),
            "Test Song".to_string(),
            Uuid::new_v4(),
            "Test Artist".to_string(),
        );
        
        let artist_contract = ArtistContract::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "Test Artist".to_string(),
        );
        
        let (mut session, _) = ListenSession::new(
            Uuid::new_v4(),
            song_contract,
            artist_contract,
            RewardTier::Basic,
        );

        // Complete and verify session
        let duration = ListenDuration::new(120).unwrap();
        let quality = QualityScore::new(0.8).unwrap();
        let proof = ZkProofHash::new("a".repeat(64)).unwrap();
        
        let _ = session.complete_session(duration, quality, proof, 180);
        let _ = session.verify_and_calculate_reward(1.0, true);
        
        session
    }

    fn create_test_distribution() -> RewardDistribution {
        let use_case = ProcessRewardDistributionUseCase::new();
        use_case.create_reward_pool(1000.0, 24).unwrap()
    }

    #[test]
    fn test_create_reward_pool() {
        let use_case = ProcessRewardDistributionUseCase::new();
        
        let result = use_case.create_reward_pool(1000.0, 24);
        
        assert!(result.is_ok());
        let distribution = result.unwrap();
        assert_eq!(distribution.reward_pool().total_tokens().tokens(), 1000.0);
    }

    #[test]
    fn test_queue_distribution_success() {
        let use_case = ProcessRewardDistributionUseCase::new();
        let distribution = create_test_distribution();
        let session = create_test_session();
        
        let command = QueueRewardDistributionCommand {
            session_id: session.id().to_string(),
            royalty_percentage: 10.0,
        };

        let result = use_case.queue_distribution(distribution, &session, command);
        
        assert!(result.is_ok());
        let (updated_distribution, response) = result.unwrap();
        assert_eq!(response.royalty_percentage, 10.0);
        assert!(response.reward_amount > 0.0);
    }

    #[test]
    fn test_queue_distribution_invalid_royalty() {
        let use_case = ProcessRewardDistributionUseCase::new();
        let distribution = create_test_distribution();
        let session = create_test_session();
        
        let command = QueueRewardDistributionCommand {
            session_id: session.id().to_string(),
            royalty_percentage: 150.0, // Invalid > 100%
        };

        let result = use_case.queue_distribution(distribution, &session, command);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Royalty percentage must be between 0.0 and 100.0"));
    }

    #[test]
    fn test_execute_distribution_success() {
        let use_case = ProcessRewardDistributionUseCase::new();
        let mut distribution = create_test_distribution();
        let session = create_test_session();
        
        // First queue the distribution
        let queue_command = QueueRewardDistributionCommand {
            session_id: session.id().to_string(),
            royalty_percentage: 15.0,
        };
        let (updated_distribution, _) = use_case.queue_distribution(distribution, &session, queue_command).unwrap();
        
        // Then execute it
        let execute_command = ProcessRewardDistributionCommand {
            session_id: session.id().to_string(),
            user_transaction_hash: "user_tx_123".to_string(),
            artist_transaction_hash: "artist_tx_456".to_string(),
        };

        let result = use_case.execute_distribution(updated_distribution, session, execute_command);
        
        assert!(result.is_ok());
        let (_, updated_session, response, events) = result.unwrap();
        
        assert_eq!(response.user_transaction_hash, "user_tx_123");
        assert_eq!(response.artist_transaction_hash, "artist_tx_456");
        assert!(response.reward_amount > 0.0);
        assert!(response.artist_royalty_amount > 0.0);
        assert_eq!(events.len(), 2); // RewardDistributed + ArtistRoyaltyPaid
    }

    #[test]
    fn test_execute_distribution_empty_transaction_hash() {
        let use_case = ProcessRewardDistributionUseCase::new();
        let distribution = create_test_distribution();
        let session = create_test_session();
        
        let command = ProcessRewardDistributionCommand {
            session_id: session.id().to_string(),
            user_transaction_hash: String::new(), // Empty
            artist_transaction_hash: "artist_tx_456".to_string(),
        };

        let result = use_case.execute_distribution(distribution, session, command);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("User transaction hash cannot be empty"));
    }

    #[test]
    fn test_execute_distribution_invalid_session_id() {
        let use_case = ProcessRewardDistributionUseCase::new();
        let distribution = create_test_distribution();
        let session = create_test_session();
        
        let command = ProcessRewardDistributionCommand {
            session_id: "invalid-uuid".to_string(),
            user_transaction_hash: "user_tx_123".to_string(),
            artist_transaction_hash: "artist_tx_456".to_string(),
        };

        let result = use_case.execute_distribution(distribution, session, command);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid session ID format"));
    }
} 