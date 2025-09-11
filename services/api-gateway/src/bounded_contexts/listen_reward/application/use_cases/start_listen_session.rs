use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::bounded_contexts::listen_reward::domain::entities::ListenSession;
use crate::bounded_contexts::listen_reward::domain::value_objects::RewardTier;
use crate::shared::domain::events::DomainEvent;
use vibestream_types::{SongContract, ArtistContract};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartListenSessionCommand {
    pub user_id: Uuid,
    pub song_contract: SongContract,
    pub artist_contract: ArtistContract,
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
        let user_tier = RewardTier::from_string(&command.user_tier)
            .map_err(|e| format!("Invalid user tier: {}", e))?;

        // Create listen session
        let song_id = command.song_contract.id;
        let artist_id = command.artist_contract.id;
        
        let (session, event) = ListenSession::new(
            command.user_id,
            command.song_contract,
            command.artist_contract,
            user_tier.clone(),
        );

        // Build response
        let response = StartListenSessionResponse {
            session_id: session.id().to_string(),
            user_id: session.user_id(),
            song_id: song_id.to_string(),
            artist_id: artist_id.to_string(),
            user_tier: user_tier.to_string(),
            started_at: session.started_at().to_rfc3339(),
        };

        Ok((response, event))
    }

    fn validate_command(&self, command: &StartListenSessionCommand) -> Result<(), String> {
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
        let song_contract = SongContract {
            id: Uuid::new_v4(),
            title: "Test Song".to_string(),
            artist_id: Uuid::new_v4(),
            artist_name: "Test Artist".to_string(),
            duration_seconds: Some(180),
            genre: Some("Pop".to_string()),
            ipfs_hash: None,
            metadata_url: None,
            nft_contract_address: None,
            nft_token_id: None,
            royalty_percentage: None,
            is_minted: false,
            created_at: chrono::Utc::now(),
        };
        
        let artist_contract = ArtistContract {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            stage_name: "Test Artist".to_string(),
            bio: Some("Test bio".to_string()),
            profile_image_url: None,
            verified: true,
            created_at: chrono::Utc::now(),
        };
        
        StartListenSessionCommand {
            user_id: Uuid::new_v4(),
            song_contract,
            artist_contract,
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
        assert_eq!(response.song_id, command.song_contract.id);
        assert_eq!(response.artist_id, command.song_contract.artist_id);
        assert_eq!(response.user_tier, command.user_tier);
        assert_eq!(event.event_type(), "ListenSessionStarted");
    }

    #[test]
    fn test_start_listen_session_empty_song_id() {
        let use_case = StartListenSessionUseCase::new();
        let mut command = create_valid_command();
        command.song_contract.id = Uuid::new_v4();

        let result = use_case.execute(command);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Song ID cannot be empty"));
    }

    #[test]
    fn test_start_listen_session_empty_artist_id() {
        let use_case = StartListenSessionUseCase::new();
        let mut command = create_valid_command();
        command.song_contract.artist_id = Uuid::new_v4();

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