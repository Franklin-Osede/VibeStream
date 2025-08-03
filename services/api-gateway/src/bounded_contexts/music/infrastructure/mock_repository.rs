use async_trait::async_trait;
use uuid::Uuid;
use crate::bounded_contexts::music::domain::{
    entities::Song,
    repositories::MusicRepository,
};

/// Mock repository for Music context testing
#[derive(Debug, Clone)]
pub struct MockMusicRepository;

#[async_trait]
impl MusicRepository for MockMusicRepository {
    async fn find_by_id(&self, _song_id: &Uuid) -> Result<Option<Song>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Some(Song {
            song_id: Uuid::new_v4(),
            title: "Mock Song".to_string(),
            artist_id: Uuid::new_v4(),
            album_id: Some(Uuid::new_v4()),
            duration: 180,
            genre: "Rock".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }))
    }
    
    async fn find_all(&self) -> Result<Vec<Song>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(vec![
            Song {
                song_id: Uuid::new_v4(),
                title: "Mock Song 1".to_string(),
                artist_id: Uuid::new_v4(),
                album_id: Some(Uuid::new_v4()),
                duration: 180,
                genre: "Rock".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
            Song {
                song_id: Uuid::new_v4(),
                title: "Mock Song 2".to_string(),
                artist_id: Uuid::new_v4(),
                album_id: Some(Uuid::new_v4()),
                duration: 200,
                genre: "Pop".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }
        ])
    }
    
    async fn create(&self, _song: &Song) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
    
    async fn update(&self, _song: &Song) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
    
    async fn delete(&self, _song_id: &Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
} 