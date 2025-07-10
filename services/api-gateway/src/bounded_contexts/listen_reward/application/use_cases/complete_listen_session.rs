use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::listen_reward::domain::entities::ListenSession;
use crate::bounded_contexts::listen_reward::domain::value_objects::{
    ListenSessionId, RewardAmount, ListenDuration, QualityScore, ZkProofHash
};
use crate::shared::domain::events::DomainEvent;
use crate::bounded_contexts::music::domain::value_objects::{SongId, ArtistId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteListenSessionCommand {
    pub session_id: String,
    pub listen_duration_seconds: u32,
    pub quality_score: f64,
    pub zk_proof_hash: String,
    pub song_duration_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteListenSessionResponse {
    pub session_id: String,
    pub status: String,
    pub listen_duration_seconds: u32,
    pub quality_score: f64,
    pub is_eligible_for_reward: bool,
    pub completed_at: String,
}

pub struct CompleteListenSessionUseCase;

impl CompleteListenSessionUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(
        &self,
        mut session: ListenSession,
        command: CompleteListenSessionCommand,
    ) -> Result<(ListenSession, CompleteListenSessionResponse, Box<dyn DomainEvent>), String> {
        // Validate command
        self.validate_command(&command)?;

        // Parse value objects
        let listen_duration = ListenDuration::new(command.listen_duration_seconds)
            .map_err(|e| format!("Invalid listen duration: {}", e))?;

        let quality_score = QualityScore::new(command.quality_score)
            .map_err(|e| format!("Invalid quality score: {}", e))?;

        let zk_proof = ZkProofHash::new(command.zk_proof_hash)
            .map_err(|e| format!("Invalid ZK proof hash: {}", e))?;

        // Complete the session
        let event = session.complete_session(
            listen_duration.clone(),
            quality_score.clone(),
            zk_proof,
            command.song_duration_seconds,
        )?;

        // Check eligibility for reward
        let is_eligible = session.is_eligible_for_reward(command.song_duration_seconds);

        // Build response
        let response = CompleteListenSessionResponse {
            session_id: command.session_id,
            status: format!("{:?}", session.status()),
            listen_duration_seconds: listen_duration.seconds(),
            quality_score: quality_score.score(),
            is_eligible_for_reward: is_eligible,
            completed_at: chrono::Utc::now().to_rfc3339(),
        };

        Ok((session, response, event))
    }

    fn validate_command(&self, command: &CompleteListenSessionCommand) -> Result<(), String> {
        if command.session_id.is_empty() {
            return Err("Session ID cannot be empty".to_string());
        }

        if command.listen_duration_seconds == 0 {
            return Err("Listen duration must be greater than 0".to_string());
        }

        if command.song_duration_seconds == 0 {
            return Err("Song duration must be greater than 0".to_string());
        }

        if !(0.0..=1.0).contains(&command.quality_score) {
            return Err("Quality score must be between 0.0 and 1.0".to_string());
        }

        if command.zk_proof_hash.is_empty() {
            return Err("ZK proof hash cannot be empty".to_string());
        }

        if command.zk_proof_hash.len() != 64 {
            return Err("ZK proof hash must be 64 characters".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_contexts::listen_reward::domain::RewardTier;
    use crate::bounded_contexts::music::domain::value_objects::{SongId, ArtistId};

    fn create_test_session() -> ListenSession {
        let (session, _) = ListenSession::new(
            Uuid::new_v4(),
            SongId::new(),
            ArtistId::new(),
            RewardTier::Basic,
        );
        session
    }

    fn create_valid_command() -> CompleteListenSessionCommand {
        CompleteListenSessionCommand {
            session_id: Uuid::new_v4().to_string(),
            listen_duration_seconds: 120,
            quality_score: 0.8,
            zk_proof_hash: "a".repeat(64),
            song_duration_seconds: 180,
        }
    }

    #[test]
    fn test_complete_session_success() {
        let use_case = CompleteListenSessionUseCase::new();
        let session = create_test_session();
        let command = create_valid_command();

        let result = use_case.execute(session, command.clone());
        
        assert!(result.is_ok());
        let (updated_session, response, event) = result.unwrap();
        
        assert_eq!(response.session_id, command.session_id);
        assert_eq!(response.listen_duration_seconds, command.listen_duration_seconds);
        assert_eq!(response.quality_score, command.quality_score);
        assert!(response.is_eligible_for_reward);
        assert_eq!(event.event_type(), "ListenSessionCompleted");
    }

    #[test]
    fn test_complete_session_too_short_duration() {
        let use_case = CompleteListenSessionUseCase::new();
        let session = create_test_session();
        let mut command = create_valid_command();
        command.listen_duration_seconds = 10; // Too short for 180s song
        command.song_duration_seconds = 180;

        let result = use_case.execute(session, command);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("too short for reward eligibility"));
    }

    #[test]
    fn test_complete_session_invalid_quality_score() {
        let use_case = CompleteListenSessionUseCase::new();
        let session = create_test_session();
        let mut command = create_valid_command();
        command.quality_score = 1.5; // Invalid score > 1.0

        let result = use_case.execute(session, command);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Quality score must be between 0.0 and 1.0"));
    }

    #[test]
    fn test_complete_session_invalid_zk_proof() {
        let use_case = CompleteListenSessionUseCase::new();
        let session = create_test_session();
        let mut command = create_valid_command();
        command.zk_proof_hash = "invalid_short_hash".to_string();

        let result = use_case.execute(session, command);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("ZK proof hash must be 64 characters"));
    }

    #[test]
    fn test_complete_session_empty_session_id() {
        let use_case = CompleteListenSessionUseCase::new();
        let session = create_test_session();
        let mut command = create_valid_command();
        command.session_id = String::new();

        let result = use_case.execute(session, command);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Session ID cannot be empty"));
    }

    #[test]
    fn test_complete_session_minimum_valid_duration() {
        let use_case = CompleteListenSessionUseCase::new();
        let session = create_test_session();
        let mut command = create_valid_command();
        command.listen_duration_seconds = 30; // Minimum for reward
        command.song_duration_seconds = 180;

        let result = use_case.execute(session, command.clone());
        
        assert!(result.is_ok());
        let (_, response, _) = result.unwrap();
        assert!(response.is_eligible_for_reward);
    }
} 