use async_trait::async_trait;
use uuid::Uuid;
use crate::bounded_contexts::music::domain::{
    entities::Song,
    value_objects::*,
};
use crate::bounded_contexts::music::domain::repositories::SongRepository;

/// Mock repository for Music context testing
#[derive(Debug, Clone)]
pub struct MockMusicRepository;

#[async_trait]
impl SongRepository for MockMusicRepository {
    async fn find_by_id(&self, _song_id: &Uuid) -> Result<Option<Song>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Some(Song::new(
            SongTitle::new("Mock Song".to_string()).expect("Valid title"),
            ArtistId::from_uuid(Uuid::new_v4()),
            SongDuration::new(180).expect("Valid duration"),
            Genre::new("Rock".to_string()).expect("Valid genre"),
            RoyaltyPercentage::new(0.15).expect("Valid royalty percentage"),
        )))
    }
    
    async fn find_all(&self) -> Result<Vec<Song>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(vec![
            Song::new(
                SongTitle::new("Mock Song 1".to_string()).expect("Valid title"),
                ArtistId::from_uuid(Uuid::new_v4()),
                SongDuration::new(180).expect("Valid duration"),
                Genre::new("Rock".to_string()).expect("Valid genre"),
                RoyaltyPercentage::new(0.15).expect("Valid royalty percentage"),
            ),
            Song::new(
                SongTitle::new("Mock Song 2".to_string()).expect("Valid title"),
                ArtistId::from_uuid(Uuid::new_v4()),
                SongDuration::new(200).expect("Valid duration"),
                Genre::new("Pop".to_string()).expect("Valid genre"),
                RoyaltyPercentage::new(0.20).expect("Valid royalty percentage"),
            )
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