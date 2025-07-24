// Music Context - CQRS Queries
// 
// This module contains all queries for reading Music bounded context data.
// Following CQRS pattern with dedicated read models and optimized queries.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::shared::application::query::{Query, QueryHandler};
use crate::shared::domain::errors::AppError;

use crate::bounded_contexts::music::domain::{
    value_objects::{SongId, Genre},
    repositories::{SongRepository, MusicCatalogRepository},
};

// ============================================================================
// SONG QUERIES
// ============================================================================

/// Get song by ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSongQuery {
    pub song_id: Uuid,
}

impl Query for GetSongQuery {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongDetailDTO {
    pub id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub duration_seconds: u32,
    pub genre: String,
    pub royalty_percentage: f64,
    pub listen_count: u64,
    pub revenue_generated: f64,
    pub is_popular: bool,
    pub is_trending: bool,
    pub ipfs_hash: Option<String>,
    pub audio_quality: Option<String>,
    pub file_format: Option<String>,
    pub mood: Option<String>,
    pub tempo: Option<u32>,
    pub release_type: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Search songs with filters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSongsQuery {
    pub search_text: Option<String>,
    pub genre: Option<String>,
    pub artist_id: Option<Uuid>,
    pub min_duration: Option<u32>,
    pub max_duration: Option<u32>,
    pub mood: Option<String>,
    pub release_type: Option<String>,
    pub is_trending: Option<bool>,
    pub is_popular: Option<bool>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub sort_by: Option<String>, // "title", "created_at", "listen_count", "trending_score"
    pub sort_order: Option<String>, // "asc", "desc"
}

impl Query for SearchSongsQuery {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSongsResult {
    pub songs: Vec<SongDetailDTO>,
    pub total_count: usize,
    pub has_more: bool,
}

/// Get trending songs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTrendingSongsQuery {
    pub limit: Option<usize>,
    pub genre: Option<String>,
    pub time_range: Option<String>, // "24h", "7d", "30d"
}

impl Query for GetTrendingSongsQuery {}

/// Get popular songs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPopularSongsQuery {
    pub limit: Option<usize>,
    pub genre: Option<String>,
    pub time_range: Option<String>, // "24h", "7d", "30d", "all_time"
}

impl Query for GetPopularSongsQuery {}

/// Get songs by artist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetArtistSongsQuery {
    pub artist_id: Uuid,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub include_albums: Option<bool>,
    pub include_singles: Option<bool>,
}

impl Query for GetArtistSongsQuery {}

// ============================================================================
// ALBUM QUERIES
// ============================================================================

/// Get album by ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAlbumQuery {
    pub album_id: Uuid,
    pub include_songs: Option<bool>,
}

impl Query for GetAlbumQuery {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumDetailDTO {
    pub id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub description: Option<String>,
    pub genre: String,
    pub release_date: Option<DateTime<Utc>>,
    pub cover_art_ipfs: Option<String>,
    pub album_type: String,
    pub track_count: usize,
    pub total_duration_seconds: u32,
    pub total_listens: u64,
    pub is_published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub songs: Option<Vec<SongDetailDTO>>,
}

/// Get albums by artist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetArtistAlbumsQuery {
    pub artist_id: Uuid,
    pub album_type: Option<String>, // "album", "ep", "compilation"
    pub is_published: Option<bool>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl Query for GetArtistAlbumsQuery {}

/// Search albums
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAlbumsQuery {
    pub search_text: Option<String>,
    pub genre: Option<String>,
    pub artist_id: Option<Uuid>,
    pub album_type: Option<String>,
    pub release_year: Option<i32>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl Query for SearchAlbumsQuery {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchAlbumsResult {
    pub albums: Vec<AlbumDetailDTO>,
    pub total_count: usize,
    pub has_more: bool,
}

// ============================================================================
// PLAYLIST QUERIES
// ============================================================================

/// Get playlist by ID
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPlaylistQuery {
    pub playlist_id: Uuid,
    pub user_id: Option<Uuid>, // For permission checking
    pub include_songs: Option<bool>,
}

impl Query for GetPlaylistQuery {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistDetailDTO {
    pub id: Uuid,
    pub name: String,
    pub creator_id: Uuid,
    pub description: Option<String>,
    pub is_public: bool,
    pub is_collaborative: bool,
    pub cover_image_url: Option<String>,
    pub tags: Vec<String>,
    pub track_count: usize,
    pub total_duration_seconds: u32,
    pub follower_count: u32,
    pub like_count: u32,
    pub listen_count: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub songs: Option<Vec<PlaylistTrackDTO>>,
    pub is_followed_by_user: Option<bool>,
    pub is_liked_by_user: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistTrackDTO {
    pub song_id: Uuid,
    pub position: u32,
    pub added_at: DateTime<Utc>,
    pub added_by: Uuid,
    pub song_details: SongDetailDTO,
}

/// Get user's playlists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserPlaylistsQuery {
    pub user_id: Uuid,
    pub include_public: Option<bool>,
    pub include_private: Option<bool>,
    pub include_collaborative: Option<bool>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl Query for GetUserPlaylistsQuery {}

/// Search public playlists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchPlaylistsQuery {
    pub search_text: Option<String>,
    pub tags: Option<Vec<String>>,
    pub creator_id: Option<Uuid>,
    pub min_tracks: Option<usize>,
    pub max_tracks: Option<usize>,
    pub sort_by: Option<String>, // "name", "created_at", "follower_count", "like_count"
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl Query for SearchPlaylistsQuery {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchPlaylistsResult {
    pub playlists: Vec<PlaylistDetailDTO>,
    pub total_count: usize,
    pub has_more: bool,
}

// ============================================================================
// ARTIST QUERIES
// ============================================================================

/// Get artist profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetArtistProfileQuery {
    pub artist_id: Uuid,
    pub include_stats: Option<bool>,
}

impl Query for GetArtistProfileQuery {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistProfileDTO {
    pub id: Uuid,
    pub user_id: Uuid,
    pub display_name: String,
    pub bio: Option<String>,
    pub profile_image_url: Option<String>,
    pub banner_image_url: Option<String>,
    pub social_links: std::collections::HashMap<String, String>,
    pub genres: Vec<String>,
    pub location: Option<String>,
    pub is_verified: bool,
    pub verification_type: Option<String>,
    pub follower_count: u32,
    pub total_songs: u32,
    pub total_albums: u32,
    pub total_listens: u64,
    pub total_revenue: f64,
    pub monthly_listeners: u32,
    pub created_at: DateTime<Utc>,
    pub stats: Option<ArtistStatsDTO>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistStatsDTO {
    pub top_songs: Vec<SongDetailDTO>,
    pub recent_releases: Vec<SongDetailDTO>,
    pub genre_distribution: std::collections::HashMap<String, u32>,
    pub monthly_growth: f64,
    pub engagement_rate: f64,
}

/// Search artists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchArtistsQuery {
    pub search_text: Option<String>,
    pub genres: Option<Vec<String>>,
    pub location: Option<String>,
    pub is_verified: Option<bool>,
    pub min_followers: Option<u32>,
    pub sort_by: Option<String>, // "name", "follower_count", "total_listens"
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl Query for SearchArtistsQuery {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchArtistsResult {
    pub artists: Vec<ArtistProfileDTO>,
    pub total_count: usize,
    pub has_more: bool,
}

// ============================================================================
// ANALYTICS QUERIES
// ============================================================================

/// Get music analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMusicAnalyticsQuery {
    pub entity_type: String, // "song", "album", "artist", "platform"
    pub entity_id: Option<Uuid>,
    pub time_range: String, // "24h", "7d", "30d", "90d", "1y"
    pub metrics: Vec<String>, // "listens", "revenue", "engagement", "demographics"
}

impl Query for GetMusicAnalyticsQuery {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicAnalyticsDTO {
    pub entity_type: String,
    pub entity_id: Option<Uuid>,
    pub time_range: String,
    pub total_listens: u64,
    pub unique_listeners: u32,
    pub total_revenue: f64,
    pub engagement_score: f64,
    pub trending_score: f64,
    pub geographic_distribution: std::collections::HashMap<String, u32>,
    pub device_distribution: std::collections::HashMap<String, u32>,
    pub hourly_patterns: Vec<HourlyMetricDTO>,
    pub daily_patterns: Vec<DailyMetricDTO>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourlyMetricDTO {
    pub hour: u8, // 0-23
    pub listens: u64,
    pub unique_listeners: u32,
    pub revenue: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyMetricDTO {
    pub date: DateTime<Utc>,
    pub listens: u64,
    pub unique_listeners: u32,
    pub revenue: f64,
    pub engagement_score: f64,
}

// ============================================================================
// QUERY HANDLERS
// ============================================================================

pub struct GetSongQueryHandler {
    song_repository: Box<dyn SongRepository>,
}

impl GetSongQueryHandler {
    pub fn new(song_repository: Box<dyn SongRepository>) -> Self {
        Self { song_repository }
    }
}

#[async_trait]
impl QueryHandler<GetSongQuery> for GetSongQueryHandler {
    type Output = Option<SongDetailDTO>;

    async fn handle(&self, query: GetSongQuery) -> Result<Self::Output, AppError> {
        let song_id = SongId::from_uuid(query.song_id);
        
        if let Some(song) = self.song_repository.find_by_id(&song_id).await? {
            Ok(Some(SongDetailDTO {
                id: *song.id().value(),
                title: song.title().value().to_string(),
                artist_id: *song.artist_id().value(),
                duration_seconds: song.duration().seconds(),
                genre: song.genre().value().to_string(),
                royalty_percentage: song.royalty_percentage().value(),
                listen_count: song.listen_count().value(),
                revenue_generated: song.revenue_generated(),
                is_popular: song.is_popular(),
                is_trending: song.is_trending(),
                ipfs_hash: song.ipfs_hash().map(|h| h.value().to_string()),
                audio_quality: song.audio_quality().map(|q| q.to_string()),
                file_format: song.file_format().map(|f| f.to_string()),
                mood: song.mood().map(|m| m.to_string()),
                tempo: song.tempo().map(|t| t.bpm() as u32),
                release_type: song.release_type().map(|r| r.to_string()),
                created_at: song.created_at(),
                updated_at: song.updated_at(),
            }))
        } else {
            Ok(None)
        }
    }
}

pub struct SearchSongsQueryHandler {
    song_repository: Box<dyn SongRepository>,
}

impl SearchSongsQueryHandler {
    pub fn new(song_repository: Box<dyn SongRepository>) -> Self {
        Self { song_repository }
    }
}

#[async_trait]
impl QueryHandler<SearchSongsQuery> for SearchSongsQueryHandler {
    type Output = SearchSongsResult;

    async fn handle(&self, query: SearchSongsQuery) -> Result<Self::Output, AppError> {
        // For now, implementing basic search by title
        // In production, this would use advanced search with filters
        let songs = if let Some(search_text) = query.search_text {
            self.song_repository.search_by_title(&search_text, query.limit).await?
        } else if let Some(genre) = query.genre {
            let genre_obj = Genre::new(genre)
                .map_err(|e| AppError::ValidationError(e))?;
            self.song_repository.find_by_genre(&genre_obj).await?
        } else if query.is_trending == Some(true) {
            self.song_repository.find_trending(query.limit).await?
        } else if query.is_popular == Some(true) {
            self.song_repository.find_popular(query.limit).await?
        } else {
            self.song_repository.find_popular(query.limit).await?
        };

        let song_dtos: Vec<SongDetailDTO> = songs.into_iter().map(|song| {
            SongDetailDTO {
                id: *song.id().value(),
                title: song.title().value().to_string(),
                artist_id: *song.artist_id().value(),
                duration_seconds: song.duration().seconds(),
                genre: song.genre().value().to_string(),
                royalty_percentage: song.royalty_percentage().value(),
                listen_count: song.listen_count().value(),
                revenue_generated: song.revenue_generated(),
                is_popular: song.is_popular(),
                is_trending: song.is_trending(),
                ipfs_hash: song.ipfs_hash().map(|h| h.value().to_string()),
                audio_quality: song.audio_quality().map(|q| q.to_string()),
                file_format: song.file_format().map(|f| f.to_string()),
                mood: song.mood().map(|m| m.to_string()),
                tempo: song.tempo().map(|t| t.bpm() as u32),
                release_type: song.release_type().map(|r| r.to_string()),
                created_at: song.created_at(),
                updated_at: song.updated_at(),
            }
        }).collect();

        let total_count = song_dtos.len();
        let has_more = query.limit.map_or(false, |limit| total_count >= limit);

        Ok(SearchSongsResult {
            songs: song_dtos,
            total_count,
            has_more,
        })
    }
}

// Additional query handlers follow the same pattern...
// For brevity, showing the structure with GetSongQueryHandler and SearchSongsQueryHandler

pub struct GetAlbumQueryHandler {
    catalog_repository: Box<dyn MusicCatalogRepository>,
}

pub struct GetPlaylistQueryHandler {
    catalog_repository: Box<dyn MusicCatalogRepository>,
}

pub struct GetArtistProfileQueryHandler {
    catalog_repository: Box<dyn MusicCatalogRepository>,
}

pub struct GetMusicAnalyticsQueryHandler {
    catalog_repository: Box<dyn MusicCatalogRepository>,
}

// Removed redundant pub use block that was causing name duplications
// All structs are already public and defined in this module 