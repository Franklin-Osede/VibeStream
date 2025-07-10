use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::bounded_contexts::listen_reward::domain::entities::ListenSession;
use crate::bounded_contexts::listen_reward::domain::value_objects::RewardTier;
use crate::shared::domain::events::DomainEvent;
use crate::bounded_contexts::music::domain::value_objects::{SongId, ArtistId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartListenSessionCommand {
    pub user_id: Uuid,
    pub song_id: String,
    pub artist_id: String,
    pub user_tier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartListenSessionResponse {
    pub session_id: String,
    pub user_id: Uuid,
    pub song_id: String,
    pub artist_id: String,
    pub user_tier: String,
    pub started_at: String,
}

pub struct StartListenSessionUseCase;

impl StartListenSessionUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(
        &self,
        command: StartListenSessionCommand,
    ) -> Result<(StartListenSessionResponse, Box<dyn DomainEvent>), String> {
        // Validate command
        self.validate_command(&command)?;

        // Parse value objects
        let song_id = SongId::from_string(&command.song_id)
            .map_err(|e| format!("Invalid song ID: {}", e))?;
        
        let artist_id = ArtistId::from_string(&command.artist_id)
            .map_err(|e| format!("Invalid artist ID: {}", e))?;
        
        let user_tier = RewardTier::from_string(&command.user_tier)
            .map_err(|e| format!("Invalid user tier: {}", e))?;

        // Create listen session
        let (session, event) = ListenSession::new(
            command.user_id,
            song_id,
            artist_id,
            user_tier.clone(),
        );

        // Build response
        let response = StartListenSessionResponse {
            session_id: session.id().to_string(),
            user_id: session.user_id(),
            song_id: command.song_id,
            artist_id: command.artist_id,
            user_tier: user_tier.to_string(),
            started_at: session.started_at().to_rfc3339(),
        };

        Ok((response, event))
    }

    fn validate_command(&self, command: &StartListenSessionCommand) -> Result<(), String> {
        if command.song_id.is_empty() {
            return Err("Song ID cannot be empty".to_string());
        }

        if command.artist_id.is_empty() {
            return Err("Artist ID cannot be empty".to_string());
        }

        if command.user_tier.is_empty() {
            return Err("User tier cannot be empty".to_string());
        }

        // Validate tier is valid
        RewardTier::from_string(&command.user_tier)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_command() -> StartListenSessionCommand {
        StartListenSessionCommand {
            user_id: Uuid::new_v4(),
            song_id: Uuid::new_v4().to_string(),
            artist_id: Uuid::new_v4().to_string(),
            user_tier: "basic".to_string(),
        }
    }

    #[test]
    fn test_start_listen_session_success() {
        let use_case = StartListenSessionUseCase::new();
        let command = create_valid_command();

        let result = use_case.execute(command.clone());
        
        assert!(result.is_ok());
        let (response, event) = result.unwrap();
        
        assert_eq!(response.user_id, command.user_id);
        assert_eq!(response.song_id, command.song_id);
        assert_eq!(response.artist_id, command.artist_id);
        assert_eq!(response.user_tier, command.user_tier);
        assert_eq!(event.event_type(), "ListenSessionStarted");
    }

    #[test]
    fn test_start_listen_session_empty_song_id() {
        let use_case = StartListenSessionUseCase::new();
        let mut command = create_valid_command();
        command.song_id = String::new();

        let result = use_case.execute(command);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Song ID cannot be empty"));
    }

    #[test]
    fn test_start_listen_session_empty_artist_id() {
        let use_case = StartListenSessionUseCase::new();
        let mut command = create_valid_command();
        command.artist_id = String::new();

        let result = use_case.execute(command);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Artist ID cannot be empty"));
    }

    #[test]
    fn test_start_listen_session_invalid_tier() {
        let use_case = StartListenSessionUseCase::new();
        let mut command = create_valid_command();
        command.user_tier = "invalid_tier".to_string();

        let result = use_case.execute(command);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid user tier"));
    }

    #[test]
    fn test_start_listen_session_premium_tier() {
        let use_case = StartListenSessionUseCase::new();
        let mut command = create_valid_command();
        command.user_tier = "premium".to_string();

        let result = use_case.execute(command);
        
        assert!(result.is_ok());
        let (response, _) = result.unwrap();
        assert_eq!(response.user_tier, "premium");
    }
} 