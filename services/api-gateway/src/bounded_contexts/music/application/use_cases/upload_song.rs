use uuid::Uuid;

use crate::bounded_contexts::music::domain::{
    MusicCatalogAggregate, SongTitle, ArtistId, SongDuration, Genre, RoyaltyPercentage, SongId
};

#[derive(Debug)]
pub struct UploadSongCommand {
    pub title: String,
    pub artist_id: Uuid,
    pub duration_seconds: u32,
    pub genre: String,
    pub royalty_percentage: f64,
}

#[derive(Debug)]
pub struct UploadSongResult {
    pub song_id: SongId,
    pub success: bool,
    pub message: String,
}

pub struct UploadSongUseCase {
    // For now, we'll keep it simple without dependencies
}

impl UploadSongUseCase {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn execute(&self, command: UploadSongCommand) -> Result<UploadSongResult, String> {
        // Validate input
        let title = SongTitle::new(command.title)
            .map_err(|e| format!("Invalid title: {}", e))?;
        
        let artist_id = ArtistId::from_uuid(command.artist_id);
        
        let duration = SongDuration::new(command.duration_seconds)
            .map_err(|e| format!("Invalid duration: {}", e))?;
        
        let genre = Genre::new(command.genre)
            .map_err(|e| format!("Invalid genre: {}", e))?;
        
        let royalty_percentage = RoyaltyPercentage::new(command.royalty_percentage)
            .map_err(|e| format!("Invalid royalty percentage: {}", e))?;

        // Create aggregate
        let mut catalog = MusicCatalogAggregate::new();
        
        // Execute business logic
        let song_id = catalog.upload_song(title, artist_id, duration, genre, royalty_percentage)?;
        
        // TODO: In production, save to repository and publish events
        // self.repository.save(song).await?;
        // self.event_bus.publish_events(catalog.get_uncommitted_events()).await?;
        
        Ok(UploadSongResult {
            song_id,
            success: true,
            message: "Song uploaded successfully".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_upload_song_success() {
        let use_case = UploadSongUseCase::new();
        
        let command = UploadSongCommand {
            title: "Test Song".to_string(),
            artist_id: Uuid::new_v4(),
            duration_seconds: 180,
            genre: "rock".to_string(),
            royalty_percentage: 70.0,
        };

        let result = use_case.execute(command).await;
        assert!(result.is_ok());
        
        let upload_result = result.unwrap();
        assert!(upload_result.success);
        assert_eq!(upload_result.message, "Song uploaded successfully");
    }

    #[tokio::test]
    async fn test_upload_song_invalid_title() {
        let use_case = UploadSongUseCase::new();
        
        let command = UploadSongCommand {
            title: "".to_string(), // Invalid empty title
            artist_id: Uuid::new_v4(),
            duration_seconds: 180,
            genre: "rock".to_string(),
            royalty_percentage: 70.0,
        };

        let result = use_case.execute(command).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid title"));
    }

    #[tokio::test]
    async fn test_upload_song_invalid_genre() {
        let use_case = UploadSongUseCase::new();
        
        let command = UploadSongCommand {
            title: "Test Song".to_string(),
            artist_id: Uuid::new_v4(),
            duration_seconds: 180,
            genre: "invalid_genre".to_string(), // Invalid genre
            royalty_percentage: 70.0,
        };

        let result = use_case.execute(command).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid genre"));
    }

    #[tokio::test]
    async fn test_upload_song_invalid_duration() {
        let use_case = UploadSongUseCase::new();
        
        let command = UploadSongCommand {
            title: "Test Song".to_string(),
            artist_id: Uuid::new_v4(),
            duration_seconds: 0, // Invalid duration
            genre: "rock".to_string(),
            royalty_percentage: 70.0,
        };

        let result = use_case.execute(command).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid duration"));
    }
} 