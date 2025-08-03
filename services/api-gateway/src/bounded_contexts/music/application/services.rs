use std::sync::Arc;
use serde_json::Value;
use uuid::Uuid;
use crate::bounded_contexts::music::domain::entities::Song;

/// Mock application service for Music context
#[derive(Clone)]
pub struct MockMusicApplicationService;

impl MockMusicApplicationService {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn create_song(&self, _request: Value) -> Result<Song, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Song {
            song_id: Uuid::new_v4(),
            title: "Mock Song".to_string(),
            artist_id: Uuid::new_v4(),
            album_id: Some(Uuid::new_v4()),
            duration: 180,
            genre: "Rock".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }
    
    pub async fn discover_songs(&self, _query: Option<String>, _genre: Option<String>) -> Result<Vec<Song>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(vec![
            Song {
                song_id: Uuid::new_v4(),
                title: "Discovered Song 1".to_string(),
                artist_id: Uuid::new_v4(),
                album_id: Some(Uuid::new_v4()),
                duration: 180,
                genre: "Rock".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
            Song {
                song_id: Uuid::new_v4(),
                title: "Discovered Song 2".to_string(),
                artist_id: Uuid::new_v4(),
                album_id: Some(Uuid::new_v4()),
                duration: 200,
                genre: "Pop".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }
        ])
    }
} 