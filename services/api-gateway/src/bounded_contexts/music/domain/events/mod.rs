use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::bounded_contexts::music::domain::value_objects::{
    SongId, AlbumId, ArtistId, SongTitle, Genre, ListenCount
};

pub trait DomainEvent: Send + Sync + std::fmt::Debug {
    fn event_type(&self) -> &'static str;
    fn aggregate_id(&self) -> String;
    fn occurred_on(&self) -> DateTime<Utc>;
    fn event_data(&self) -> Value;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongUploaded {
    pub song_id: SongId,
    pub artist_id: ArtistId,
    pub title: SongTitle,
    pub genre: Genre,
    pub duration_seconds: u32,
    pub uploaded_at: DateTime<Utc>,
}

impl DomainEvent for SongUploaded {
    fn event_type(&self) -> &'static str {
        "music.song.uploaded"
    }
    
    fn aggregate_id(&self) -> String {
        self.song_id.value().to_string()
    }
    
    fn occurred_on(&self) -> DateTime<Utc> {
        self.uploaded_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongListened {
    pub song_id: SongId,
    pub listener_id: Uuid,
    pub listen_count: u64,
    pub listen_duration_seconds: u32,
    pub listened_at: DateTime<Utc>,
}

impl DomainEvent for SongListened {
    fn event_type(&self) -> &'static str {
        "music.song.listened"
    }
    
    fn aggregate_id(&self) -> String {
        self.song_id.value().to_string()
    }
    
    fn occurred_on(&self) -> DateTime<Utc> {
        self.listened_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumCreated {
    pub album_id: AlbumId,
    pub artist_id: ArtistId,
    pub title: String,
    pub song_ids: Vec<SongId>,
    pub created_at: DateTime<Utc>,
}

impl DomainEvent for AlbumCreated {
    fn event_type(&self) -> &'static str {
        "music.album.created"
    }
    
    fn aggregate_id(&self) -> String {
        self.album_id.value().to_string()
    }
    
    fn occurred_on(&self) -> DateTime<Utc> {
        self.created_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongAvailableForCampaign {
    pub song_id: SongId,
    pub artist_id: ArtistId,
    pub listen_count: u64,
    pub marked_at: DateTime<Utc>,
}

impl DomainEvent for SongAvailableForCampaign {
    fn event_type(&self) -> &'static str {
        "music.song.available_for_campaign"
    }
    
    fn aggregate_id(&self) -> String {
        self.song_id.value().to_string()
    }
    
    fn occurred_on(&self) -> DateTime<Utc> {
        self.marked_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongAvailableForOwnership {
    pub song_id: SongId,
    pub artist_id: ArtistId,
    pub revenue_threshold_reached: f64,
    pub marked_at: DateTime<Utc>,
}

impl DomainEvent for SongAvailableForOwnership {
    fn event_type(&self) -> &'static str {
        "music.song.available_for_ownership"
    }
    
    fn aggregate_id(&self) -> String {
        self.song_id.value().to_string()
    }
    
    fn occurred_on(&self) -> DateTime<Utc> {
        self.marked_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistCreated {
    pub playlist_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub song_ids: Vec<SongId>,
    pub created_at: DateTime<Utc>,
}

impl DomainEvent for PlaylistCreated {
    fn event_type(&self) -> &'static str {
        "music.playlist.created"
    }
    
    fn aggregate_id(&self) -> String {
        self.playlist_id.to_string()
    }
    
    fn occurred_on(&self) -> DateTime<Utc> {
        self.created_at
    }
    
    fn event_data(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
} 