pub mod postgres_song_repository;
pub mod postgres_album_repository;
pub mod postgres_playlist_repository;

pub use postgres_song_repository::*;
pub use postgres_album_repository::*;
pub use postgres_playlist_repository::*;

// Temporary implementation of MusicCatalogRepository for compilation
use async_trait::async_trait;
use uuid::Uuid;
use crate::bounded_contexts::music::domain::aggregates::MusicCatalogAggregate;
use crate::bounded_contexts::music::domain::repositories::{MusicCatalogRepository, RepositoryResult, RepositoryError};
use crate::shared::domain::events::DomainEvent;

pub struct TemporaryMusicCatalogRepository;

impl TemporaryMusicCatalogRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl MusicCatalogRepository for TemporaryMusicCatalogRepository {
    async fn save_aggregate(&self, _aggregate: &MusicCatalogAggregate) -> RepositoryResult<()> {
        // Temporary implementation for compilation
        Ok(())
    }

    async fn load_aggregate(&self, _aggregate_id: &Uuid) -> RepositoryResult<Option<MusicCatalogAggregate>> {
        // Temporary implementation for compilation
        Ok(None)
    }

    async fn get_events_since(&self, _aggregate_id: &Uuid, _version: u64) -> RepositoryResult<Vec<Box<dyn DomainEvent>>> {
        // Temporary implementation for compilation
        Ok(vec![])
    }
} 