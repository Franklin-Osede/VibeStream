use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::shared::domain::errors::AppError;

// =============================================================================
// ALBUM ENTITY
// =============================================================================

#[derive(Debug, Clone)]
pub struct Album {
    pub id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub description: Option<String>,
    pub release_date: Option<DateTime<Utc>>,
    pub song_count: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Album {
    pub fn new(
        id: Uuid,
        title: String,
        artist_id: Uuid,
        description: Option<String>,
        release_date: Option<DateTime<Utc>>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            title,
            artist_id,
            description,
            release_date,
            song_count: 0,
            created_at: now,
            updated_at: now,
        }
    }
}

// =============================================================================
// ALBUM REPOSITORY TRAIT
// =============================================================================

#[async_trait]
pub trait AlbumRepository: Send + Sync {
    /// Save an album to the repository
    async fn save(&self, album: &Album) -> Result<(), AppError>;
    
    /// Find an album by ID
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Album>, AppError>;
    
    /// Find albums by artist ID
    async fn find_by_artist_id(&self, artist_id: &Uuid) -> Result<Vec<Album>, AppError>;
    
    /// Find all albums with pagination
    async fn find_all(&self, page: u32, page_size: u32) -> Result<Vec<Album>, AppError>;
    
    /// Update an album
    async fn update(&self, album: &Album) -> Result<(), AppError>;
    
    /// Delete an album by ID
    async fn delete(&self, id: &Uuid) -> Result<(), AppError>;
    
    /// Count total albums
    async fn count(&self) -> Result<u64, AppError>;
    
    /// Search albums by title
    async fn search_by_title(&self, title: &str) -> Result<Vec<Album>, AppError>;
}
