use async_trait::async_trait;
use uuid::Uuid;
use crate::bounded_contexts::music::domain::{
    entities::Song,
    value_objects::*,
};
use crate::bounded_contexts::music::domain::repositories::{SongRepository, RepositoryResult, RepositoryError};

/// Mock repository for Music context testing
#[derive(Debug, Clone)]
pub struct MockMusicRepository;

#[async_trait]
impl SongRepository for MockMusicRepository {
    async fn save(&self, _song: &Song) -> RepositoryResult<()> { Ok(()) }
    async fn update(&self, _song: &Song) -> RepositoryResult<()> { Ok(()) }
    async fn find_by_id(&self, _song_id: &crate::bounded_contexts::music::domain::value_objects::SongId) -> RepositoryResult<Option<Song>> {
        Ok(Some(Song::new(
            SongTitle::new("Mock Song".to_string()).expect("Valid title"),
            ArtistId::from_uuid(Uuid::new_v4()),
            SongDuration::new(180).expect("Valid duration"),
            Genre::new("Rock".to_string()).expect("Valid genre"),
            RoyaltyPercentage::new(0.15).expect("Valid royalty percentage"),
        )))
    }
    async fn delete(&self, _song_id: &crate::bounded_contexts::music::domain::value_objects::SongId) -> RepositoryResult<()> { Ok(()) }
    async fn find_by_artist(&self, _artist_id: &crate::bounded_contexts::music::domain::value_objects::ArtistId) -> RepositoryResult<Vec<Song>> { Ok(vec![]) }
    async fn find_by_genre(&self, _genre: &crate::bounded_contexts::music::domain::value_objects::Genre) -> RepositoryResult<Vec<Song>> { Ok(vec![]) }
    async fn find_trending(&self, _limit: Option<usize>) -> RepositoryResult<Vec<Song>> { Ok(vec![]) }
    async fn find_popular(&self, _limit: Option<usize>) -> RepositoryResult<Vec<Song>> { Ok(vec![]) }
    async fn search_by_title(&self, _query: &str, _limit: Option<usize>) -> RepositoryResult<Vec<Song>> {
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
    async fn count_by_artist(&self, _artist_id: &crate::bounded_contexts::music::domain::value_objects::ArtistId) -> RepositoryResult<usize> { Ok(0) }
    async fn get_total_listens(&self) -> RepositoryResult<u64> { Ok(0) }
} 