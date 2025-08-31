use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::shared::domain::errors::AppError;

// =============================================================================
// PLAYLIST ENTITY
// =============================================================================

#[derive(Debug, Clone)]
pub struct Playlist {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub song_count: u32,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Playlist {
    pub fn new(
        id: Uuid,
        name: String,
        description: Option<String>,
        is_public: bool,
        created_by: Uuid,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            description,
            is_public,
            song_count: 0,
            created_by,
            created_at: now,
            updated_at: now,
        }
    }
}

// =============================================================================
// PLAYLIST REPOSITORY TRAIT
// =============================================================================

#[async_trait]
pub trait PlaylistRepository: Send + Sync {
    /// Save a playlist to the repository
    async fn save(&self, playlist: &Playlist) -> Result<(), AppError>;
    
    /// Find a playlist by ID
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Playlist>, AppError>;
    
    /// Find playlists by creator ID
    async fn find_by_creator(&self, creator_id: &Uuid) -> Result<Vec<Playlist>, AppError>;
    
    /// Find public playlists
    async fn find_public_playlists(&self, page: u32, page_size: u32) -> Result<Vec<Playlist>, AppError>;
    
    /// Find all playlists with pagination
    async fn find_all(&self, page: u32, page_size: u32) -> Result<Vec<Playlist>, AppError>;
    
    /// Update a playlist
    async fn update(&self, playlist: &Playlist) -> Result<(), AppError>;
    
    /// Delete a playlist by ID
    async fn delete(&self, id: &Uuid) -> Result<(), AppError>;
    
    /// Count total playlists
    async fn count(&self) -> Result<u64, AppError>;
    
    /// Search playlists by name
    async fn search_by_name(&self, name: &str) -> Result<Vec<Playlist>, AppError>;
    
    /// Add song to playlist
    async fn add_song(&self, playlist_id: &Uuid, song_id: &Uuid) -> Result<(), AppError>;
    
    /// Remove song from playlist
    async fn remove_song(&self, playlist_id: &Uuid, song_id: &Uuid) -> Result<(), AppError>;
    
    /// Get songs in playlist
    async fn get_songs(&self, playlist_id: &Uuid) -> Result<Vec<Uuid>, AppError>;
}
