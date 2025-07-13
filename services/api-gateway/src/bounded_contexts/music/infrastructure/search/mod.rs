pub mod elasticsearch_search;
pub mod search_engine;
pub mod search_filters;

pub use elasticsearch_search::*;
pub use search_engine::*;
pub use search_filters::*;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::bounded_contexts::music::domain::value_objects::{Genre, SongMood, AudioQuality};

/// Advanced search service interface
#[async_trait]
pub trait MusicSearchService: Send + Sync {
    /// Search songs with advanced filters
    async fn search_songs(&self, query: SearchQuery) -> Result<SearchResults<SongSearchResult>, SearchError>;
    
    /// Search artists
    async fn search_artists(&self, query: SearchQuery) -> Result<SearchResults<ArtistSearchResult>, SearchError>;
    
    /// Search albums
    async fn search_albums(&self, query: SearchQuery) -> Result<SearchResults<AlbumSearchResult>, SearchError>;
    
    /// Search playlists
    async fn search_playlists(&self, query: SearchQuery) -> Result<SearchResults<PlaylistSearchResult>, SearchError>;
    
    /// Get search suggestions
    async fn get_suggestions(&self, partial_query: &str) -> Result<Vec<SearchSuggestion>, SearchError>;
    
    /// Get trending searches
    async fn get_trending_searches(&self) -> Result<Vec<TrendingSearch>, SearchError>;
}

/// Search query with filters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub text: String,
    pub filters: SearchFilters,
    pub sort: SearchSort,
    pub pagination: SearchPagination,
}

/// Search filters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFilters {
    pub genres: Option<Vec<Genre>>,
    pub moods: Option<Vec<SongMood>>,
    pub audio_qualities: Option<Vec<AudioQuality>>,
    pub duration_range: Option<DurationRange>,
    pub release_date_range: Option<DateRange>,
    pub artist_ids: Option<Vec<Uuid>>,
    pub is_trending: Option<bool>,
    pub is_popular: Option<bool>,
    pub min_listen_count: Option<u64>,
    pub language: Option<String>,
    pub explicit_content: Option<bool>,
}

/// Search sorting options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchSort {
    Relevance,
    PopularityDesc,
    PopularityAsc,
    ReleaseDateDesc,
    ReleaseDateAsc,
    DurationDesc,
    DurationAsc,
    TitleAsc,
    TitleDesc,
    ListenCountDesc,
    ListenCountAsc,
}

/// Search pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchPagination {
    pub page: u32,
    pub page_size: u32,
    pub max_results: Option<u32>,
}

/// Search results wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults<T> {
    pub results: Vec<T>,
    pub total_count: u64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
    pub search_time_ms: u64,
    pub facets: HashMap<String, Vec<SearchFacet>>,
}

/// Song search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongSearchResult {
    pub id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub artist_name: String,
    pub album_id: Option<Uuid>,
    pub album_title: Option<String>,
    pub duration_seconds: u32,
    pub genre: String,
    pub mood: Option<String>,
    pub audio_quality: Option<String>,
    pub listen_count: u64,
    pub is_trending: bool,
    pub is_popular: bool,
    pub relevance_score: f64,
    pub highlight: Option<SearchHighlight>,
}

/// Artist search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistSearchResult {
    pub id: Uuid,
    pub name: String,
    pub bio: Option<String>,
    pub genres: Vec<String>,
    pub follower_count: u64,
    pub song_count: u32,
    pub album_count: u32,
    pub is_verified: bool,
    pub relevance_score: f64,
    pub highlight: Option<SearchHighlight>,
}

/// Album search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumSearchResult {
    pub id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub artist_name: String,
    pub genre: String,
    pub track_count: u32,
    pub release_date: Option<chrono::DateTime<chrono::Utc>>,
    pub listen_count: u64,
    pub is_published: bool,
    pub relevance_score: f64,
    pub highlight: Option<SearchHighlight>,
}

/// Playlist search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistSearchResult {
    pub id: Uuid,
    pub name: String,
    pub creator_id: Uuid,
    pub creator_name: String,
    pub description: Option<String>,
    pub track_count: u32,
    pub follower_count: u64,
    pub is_public: bool,
    pub relevance_score: f64,
    pub highlight: Option<SearchHighlight>,
}

/// Search suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSuggestion {
    pub text: String,
    pub category: SearchCategory,
    pub count: u32,
}

/// Trending search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendingSearch {
    pub text: String,
    pub search_count: u64,
    pub trend_score: f64,
}

/// Search category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchCategory {
    Song,
    Artist,
    Album,
    Playlist,
    Genre,
    Mood,
}

/// Search highlight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHighlight {
    pub field: String,
    pub highlighted_text: String,
}

/// Search facet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchFacet {
    pub value: String,
    pub count: u64,
}

/// Duration range filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurationRange {
    pub min_seconds: Option<u32>,
    pub max_seconds: Option<u32>,
}

/// Date range filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub from: Option<chrono::DateTime<chrono::Utc>>,
    pub to: Option<chrono::DateTime<chrono::Utc>>,
}

/// Search errors
#[derive(Debug, thiserror::Error)]
pub enum SearchError {
    #[error("Search service unavailable")]
    ServiceUnavailable,
    #[error("Invalid search query: {0}")]
    InvalidQuery(String),
    #[error("Search timeout")]
    Timeout,
    #[error("Too many results: {0}")]
    TooManyResults(u32),
    #[error("Internal search error: {0}")]
    InternalError(String),
} 