use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::bounded_contexts::music::domain::value_objects::{
    SongId, AlbumId, ArtistId, SongTitle, Genre, ListenCount, PlaylistId
};
use crate::shared::domain::events::{DomainEvent, EventMetadata};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongUploaded {
    pub metadata: EventMetadata,
    pub song_id: SongId,
    pub artist_id: ArtistId,
    pub title: SongTitle,
    pub genre: Genre,
    pub duration_seconds: u32,
    pub uploaded_at: DateTime<Utc>,
}

impl DomainEvent for SongUploaded {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.song.uploaded"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.song_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Song"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.uploaded_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongListened {
    pub metadata: EventMetadata,
    pub song_id: SongId,
    pub listener_id: Uuid,
    pub listen_count: u64,
    pub listen_duration_seconds: u32,
    pub listened_at: DateTime<Utc>,
}

impl DomainEvent for SongListened {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.song.listened"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.song_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Song"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.listened_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumCreated {
    pub metadata: EventMetadata,
    pub album_id: AlbumId,
    pub artist_id: ArtistId,
    pub title: String,
    pub song_ids: Vec<SongId>,
    pub created_at: DateTime<Utc>,
}

impl DomainEvent for AlbumCreated {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.album.created"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.album_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Album"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// New Album Events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumUpdated {
    pub metadata: EventMetadata,
    pub album_id: AlbumId,
    pub artist_id: ArtistId,
    pub updated_fields: Vec<String>,
    pub updated_at: DateTime<Utc>,
}

impl DomainEvent for AlbumUpdated {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.album.updated"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.album_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Album"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumPublished {
    pub metadata: EventMetadata,
    pub album_id: AlbumId,
    pub artist_id: ArtistId,
    pub published_at: DateTime<Utc>,
}

impl DomainEvent for AlbumPublished {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.album.published"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.album_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Album"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.published_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumUnpublished {
    pub metadata: EventMetadata,
    pub album_id: AlbumId,
    pub artist_id: ArtistId,
    pub unpublished_at: DateTime<Utc>,
}

impl DomainEvent for AlbumUnpublished {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.album.unpublished"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.album_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Album"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.unpublished_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongAddedToAlbum {
    pub metadata: EventMetadata,
    pub album_id: AlbumId,
    pub song_id: SongId,
    pub track_number: u32,
    pub added_at: DateTime<Utc>,
}

impl DomainEvent for SongAddedToAlbum {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.album.song_added"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.album_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Album"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.added_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongRemovedFromAlbum {
    pub metadata: EventMetadata,
    pub album_id: AlbumId,
    pub song_id: SongId,
    pub removed_at: DateTime<Utc>,
}

impl DomainEvent for SongRemovedFromAlbum {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.album.song_removed"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.album_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Album"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.removed_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongAvailableForCampaign {
    pub metadata: EventMetadata,
    pub song_id: SongId,
    pub artist_id: ArtistId,
    pub listen_count: u64,
    pub marked_at: DateTime<Utc>,
}

impl DomainEvent for SongAvailableForCampaign {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.song.available_for_campaign"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.song_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Song"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.marked_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongAvailableForOwnership {
    pub metadata: EventMetadata,
    pub song_id: SongId,
    pub artist_id: ArtistId,
    pub revenue_threshold_reached: f64,
    pub marked_at: DateTime<Utc>,
}

impl DomainEvent for SongAvailableForOwnership {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.song.available_for_ownership"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.song_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Song"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.marked_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistCreated {
    pub metadata: EventMetadata,
    pub playlist_id: PlaylistId,
    pub user_id: Uuid,
    pub name: String,
    pub song_ids: Vec<SongId>,
    pub created_at: DateTime<Utc>,
}

impl DomainEvent for PlaylistCreated {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.playlist.created"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.playlist_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Playlist"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// New Playlist Events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistUpdated {
    pub metadata: EventMetadata,
    pub playlist_id: PlaylistId,
    pub user_id: Uuid,
    pub updated_fields: Vec<String>,
    pub updated_at: DateTime<Utc>,
}

impl DomainEvent for PlaylistUpdated {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.playlist.updated"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.playlist_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Playlist"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistMadePublic {
    pub metadata: EventMetadata,
    pub playlist_id: PlaylistId,
    pub user_id: Uuid,
    pub made_public_at: DateTime<Utc>,
}

impl DomainEvent for PlaylistMadePublic {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.playlist.made_public"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.playlist_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Playlist"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.made_public_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistMadePrivate {
    pub metadata: EventMetadata,
    pub playlist_id: PlaylistId,
    pub user_id: Uuid,
    pub made_private_at: DateTime<Utc>,
}

impl DomainEvent for PlaylistMadePrivate {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.playlist.made_private"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.playlist_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Playlist"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.made_private_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongAddedToPlaylist {
    pub metadata: EventMetadata,
    pub playlist_id: PlaylistId,
    pub song_id: SongId,
    pub position: u32,
    pub added_by: Uuid,
    pub added_at: DateTime<Utc>,
}

impl DomainEvent for SongAddedToPlaylist {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.playlist.song_added"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.playlist_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Playlist"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.added_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongRemovedFromPlaylist {
    pub metadata: EventMetadata,
    pub playlist_id: PlaylistId,
    pub song_id: SongId,
    pub removed_by: Uuid,
    pub removed_at: DateTime<Utc>,
}

impl DomainEvent for SongRemovedFromPlaylist {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.playlist.song_removed"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.playlist_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Playlist"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.removed_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistShared {
    pub metadata: EventMetadata,
    pub playlist_id: PlaylistId,
    pub shared_by: Uuid,
    pub shared_with: Vec<Uuid>,
    pub shared_at: DateTime<Utc>,
}

impl DomainEvent for PlaylistShared {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.playlist.shared"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.playlist_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Playlist"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.shared_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
} 

// Artist Events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistProfileCreated {
    pub metadata: EventMetadata,
    pub artist_id: ArtistId,
    pub user_id: Uuid,
    pub stage_name: String,
    pub primary_genre: Genre,
    pub created_at: DateTime<Utc>,
}

impl DomainEvent for ArtistProfileCreated {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.artist.profile_created"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.artist_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Artist"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistProfileUpdated {
    pub metadata: EventMetadata,
    pub artist_id: ArtistId,
    pub updated_fields: Vec<String>,
    pub updated_at: DateTime<Utc>,
}

impl DomainEvent for ArtistProfileUpdated {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.artist.profile_updated"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.artist_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Artist"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistVerified {
    pub metadata: EventMetadata,
    pub artist_id: ArtistId,
    pub verified_by: Uuid,
    pub verification_type: String,
    pub verified_at: DateTime<Utc>,
}

impl DomainEvent for ArtistVerified {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.artist.verified"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.artist_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Artist"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.verified_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistGenreAdded {
    pub metadata: EventMetadata,
    pub artist_id: ArtistId,
    pub genre: Genre,
    pub added_at: DateTime<Utc>,
}

impl DomainEvent for ArtistGenreAdded {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.artist.genre_added"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.artist_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Artist"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.added_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistGenreRemoved {
    pub metadata: EventMetadata,
    pub artist_id: ArtistId,
    pub genre: Genre,
    pub removed_at: DateTime<Utc>,
}

impl DomainEvent for ArtistGenreRemoved {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.artist.genre_removed"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.artist_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Artist"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.removed_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistFollowed {
    pub metadata: EventMetadata,
    pub artist_id: ArtistId,
    pub follower_id: Uuid,
    pub followed_at: DateTime<Utc>,
}

impl DomainEvent for ArtistFollowed {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.artist.followed"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.artist_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Artist"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.followed_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistUnfollowed {
    pub metadata: EventMetadata,
    pub artist_id: ArtistId,
    pub follower_id: Uuid,
    pub unfollowed_at: DateTime<Utc>,
}

impl DomainEvent for ArtistUnfollowed {
    fn metadata(&self) -> &EventMetadata {
        &self.metadata
    }
    
    fn event_type(&self) -> &str {
        "music.artist.unfollowed"
    }
    
    fn aggregate_id(&self) -> Uuid {
        *self.artist_id.value()
    }
    
    fn aggregate_type(&self) -> &str {
        "Artist"
    }
    
    fn occurred_at(&self) -> DateTime<Utc> {
        self.unfollowed_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
} 