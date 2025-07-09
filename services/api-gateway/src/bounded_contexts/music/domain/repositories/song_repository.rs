use async_trait::async_trait;
use uuid::Uuid;

use crate::bounded_contexts::music::domain::{
    Song, SongId, ArtistId, Genre, MusicCatalogAggregate
};

pub type RepositoryResult<T> = Result<T, RepositoryError>;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Song not found: {0}")]
    NotFound(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Connection error: {0}")]
    Connection(String),
}

#[async_trait]
pub trait SongRepository: Send + Sync {
    // Basic CRUD operations
    async fn save(&self, song: &Song) -> RepositoryResult<()>;
    async fn update(&self, song: &Song) -> RepositoryResult<()>;
    async fn find_by_id(&self, id: &SongId) -> RepositoryResult<Option<Song>>;
    async fn delete(&self, id: &SongId) -> RepositoryResult<()>;
    
    // Query operations
    async fn find_by_artist(&self, artist_id: &ArtistId) -> RepositoryResult<Vec<Song>>;
    async fn find_by_genre(&self, genre: &Genre) -> RepositoryResult<Vec<Song>>;
    async fn find_trending(&self, limit: Option<usize>) -> RepositoryResult<Vec<Song>>;
    async fn find_popular(&self, limit: Option<usize>) -> RepositoryResult<Vec<Song>>;
    async fn search_by_title(&self, query: &str, limit: Option<usize>) -> RepositoryResult<Vec<Song>>;
    
    // Analytics
    async fn count_by_artist(&self, artist_id: &ArtistId) -> RepositoryResult<usize>;
    async fn get_total_listens(&self) -> RepositoryResult<u64>;
}

#[async_trait]
pub trait MusicCatalogRepository: Send + Sync {
    // Aggregate operations
    async fn save_aggregate(&self, aggregate: &MusicCatalogAggregate) -> RepositoryResult<()>;
    async fn load_aggregate(&self, aggregate_id: &Uuid) -> RepositoryResult<Option<MusicCatalogAggregate>>;
    
    // Event sourcing support
    async fn get_events_since(&self, aggregate_id: &Uuid, version: u64) -> RepositoryResult<Vec<Box<dyn crate::bounded_contexts::music::domain::events::DomainEvent>>>;
} 