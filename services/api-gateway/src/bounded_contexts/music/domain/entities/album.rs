use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::bounded_contexts::music::domain::value_objects::{
    AlbumId, AlbumTitle, ArtistId, SongId, Genre, ReleaseType,
    IpfsHash, SongDuration
};
use crate::bounded_contexts::music::domain::events::{
    AlbumCreated, AlbumUpdated, AlbumPublished, AlbumUnpublished,
    SongAddedToAlbum, SongRemovedFromAlbum
};
use crate::shared::domain::events::DomainEvent;

/// Album entity representing a collection of songs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Album {
    id: AlbumId,
    title: AlbumTitle,
    artist_id: ArtistId,
    description: Option<String>,
    release_type: ReleaseType,
    genre: Genre,
    release_date: Option<DateTime<Utc>>,
    tracks: Vec<AlbumTrack>,
    cover_art_ipfs: Option<IpfsHash>,
    total_duration: Option<SongDuration>,
    is_published: bool,
    is_featured: bool,
    listen_count: u64,
    revenue_generated: f64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// Track within an album
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlbumTrack {
    song_id: SongId,
    track_number: u32,
    title: String,
    duration: SongDuration,
    is_bonus_track: bool,
    added_at: DateTime<Utc>,
}

/// Album metadata for external consumption
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlbumMetadata {
    pub id: AlbumId,
    pub title: String,
    pub artist_id: ArtistId,
    pub description: Option<String>,
    pub release_type: String,
    pub genre: String,
    pub release_date: Option<DateTime<Utc>>,
    pub track_count: usize,
    pub total_duration_seconds: Option<u32>,
    pub cover_art_url: Option<String>,
    pub is_published: bool,
    pub is_featured: bool,
    pub listen_count: u64,
    pub revenue_generated: f64,
    pub created_at: DateTime<Utc>,
}

impl Album {
    /// Create new album
    pub fn new(
        title: AlbumTitle,
        artist_id: ArtistId,
        release_type: ReleaseType,
        genre: Genre,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: AlbumId::new(),
            title,
            artist_id,
            description: None,
            release_type,
            genre,
            release_date: None,
            tracks: Vec::new(),
            cover_art_ipfs: None,
            total_duration: None,
            is_published: false,
            is_featured: false,
            listen_count: 0,
            revenue_generated: 0.0,
            created_at: now,
            updated_at: now,
        }
    }

    /// Create album with specific ID (for loading from repository)
    pub fn with_id(
        id: AlbumId,
        title: AlbumTitle,
        artist_id: ArtistId,
        release_type: ReleaseType,
        genre: Genre,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            title,
            artist_id,
            description: None,
            release_type,
            genre,
            release_date: None,
            tracks: Vec::new(),
            cover_art_ipfs: None,
            total_duration: None,
            is_published: false,
            is_featured: false,
            listen_count: 0,
            revenue_generated: 0.0,
            created_at: now,
            updated_at: now,
        }
    }

    // Getters
    pub fn id(&self) -> &AlbumId {
        &self.id
    }

    pub fn title(&self) -> &AlbumTitle {
        &self.title
    }

    pub fn artist_id(&self) -> &ArtistId {
        &self.artist_id
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn release_type(&self) -> &ReleaseType {
        &self.release_type
    }

    pub fn genre(&self) -> &Genre {
        &self.genre
    }

    pub fn release_date(&self) -> Option<DateTime<Utc>> {
        self.release_date
    }

    pub fn tracks(&self) -> &[AlbumTrack] {
        &self.tracks
    }

    pub fn cover_art_ipfs(&self) -> Option<&IpfsHash> {
        self.cover_art_ipfs.as_ref()
    }

    pub fn is_published(&self) -> bool {
        self.is_published
    }

    pub fn is_featured(&self) -> bool {
        self.is_featured
    }

    pub fn listen_count(&self) -> u64 {
        self.listen_count
    }

    pub fn revenue_generated(&self) -> f64 {
        self.revenue_generated
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    /// Business logic methods

    /// Add song to album
    pub fn add_song(
        &mut self,
        song_id: SongId,
        title: String,
        duration: SongDuration,
        is_bonus_track: bool,
    ) -> Result<Box<dyn DomainEvent>, String> {
        // Validate album is not published
        if self.is_published {
            return Err("Cannot add songs to published album".to_string());
        }

        // Check if song already exists
        if self.tracks.iter().any(|track| track.song_id == song_id) {
            return Err("Song already exists in album".to_string());
        }

        // Calculate track number
        let track_number = if is_bonus_track {
            // Bonus tracks get numbers after regular tracks
            let regular_tracks = self.tracks.iter().filter(|t| !t.is_bonus_track).count();
            (regular_tracks + 1) as u32
        } else {
            // Regular tracks get sequential numbers
            let last_regular_track = self.tracks
                .iter()
                .filter(|t| !t.is_bonus_track)
                .map(|t| t.track_number)
                .max()
                .unwrap_or(0);
            last_regular_track + 1
        };

        let track = AlbumTrack {
            song_id: song_id.clone(),
            track_number,
            title: title.clone(),
            duration: duration.clone(),
            is_bonus_track,
            added_at: Utc::now(),
        };

        self.tracks.push(track);
        self.recalculate_total_duration();
        self.updated_at = Utc::now();

        // Create domain event
        Ok(Box::new(SongAddedToAlbum {
            album_id: self.id.clone(),
            song_id,
            track_number,
            title,
            is_bonus_track,
            added_at: Utc::now(),
        }))
    }

    /// Remove song from album
    pub fn remove_song(&mut self, song_id: &SongId) -> Result<Box<dyn DomainEvent>, String> {
        // Validate album is not published
        if self.is_published {
            return Err("Cannot remove songs from published album".to_string());
        }

        // Find and remove track
        let track_index = self.tracks
            .iter()
            .position(|track| &track.song_id == song_id)
            .ok_or("Song not found in album")?;

        let removed_track = self.tracks.remove(track_index);

        // Renumber tracks
        self.renumber_tracks();
        self.recalculate_total_duration();
        self.updated_at = Utc::now();

        // Create domain event
        Ok(Box::new(SongRemovedFromAlbum {
            album_id: self.id.clone(),
            song_id: song_id.clone(),
            track_number: removed_track.track_number,
            removed_at: Utc::now(),
        }))
    }

    /// Update album metadata
    pub fn update_metadata(
        &mut self,
        title: Option<AlbumTitle>,
        description: Option<String>,
        genre: Option<Genre>,
        release_date: Option<DateTime<Utc>>,
    ) -> Result<Box<dyn DomainEvent>, String> {
        let mut changes = HashMap::new();

        if let Some(new_title) = title {
            changes.insert("title".to_string(), self.title.value().to_string());
            self.title = new_title;
        }

        if let Some(new_description) = description {
            changes.insert("description".to_string(), 
                self.description.clone().unwrap_or_default());
            self.description = Some(new_description);
        }

        if let Some(new_genre) = genre {
            changes.insert("genre".to_string(), self.genre.value().to_string());
            self.genre = new_genre;
        }

        if let Some(new_release_date) = release_date {
            self.release_date = Some(new_release_date);
        }

        self.updated_at = Utc::now();

        Ok(Box::new(AlbumUpdated {
            album_id: self.id.clone(),
            changes,
            updated_at: self.updated_at,
        }))
    }

    /// Set cover art
    pub fn set_cover_art(&mut self, ipfs_hash: IpfsHash) {
        self.cover_art_ipfs = Some(ipfs_hash);
        self.updated_at = Utc::now();
    }

    /// Publish album
    pub fn publish(&mut self) -> Result<Box<dyn DomainEvent>, String> {
        if self.is_published {
            return Err("Album is already published".to_string());
        }

        if self.tracks.is_empty() {
            return Err("Cannot publish album without tracks".to_string());
        }

        // Validate minimum requirements for different release types
        match self.release_type {
            ReleaseType::Single => {
                if self.tracks.len() > 3 {
                    return Err("Single cannot have more than 3 tracks".to_string());
                }
            }
            ReleaseType::Ep => {
                if self.tracks.len() < 3 || self.tracks.len() > 6 {
                    return Err("EP must have between 3-6 tracks".to_string());
                }
            }
            ReleaseType::Album => {
                if self.tracks.len() < 7 {
                    return Err("Album must have at least 7 tracks".to_string());
                }
            }
            _ => {} // Other types have no specific requirements
        }

        self.is_published = true;
        if self.release_date.is_none() {
            self.release_date = Some(Utc::now());
        }
        self.updated_at = Utc::now();

        Ok(Box::new(AlbumPublished {
            album_id: self.id.clone(),
            artist_id: self.artist_id.clone(),
            title: self.title.value().to_string(),
            release_type: self.release_type.clone(),
            track_count: self.tracks.len(),
            published_at: self.updated_at,
        }))
    }

    /// Unpublish album
    pub fn unpublish(&mut self) -> Result<Box<dyn DomainEvent>, String> {
        if !self.is_published {
            return Err("Album is not published".to_string());
        }

        self.is_published = false;
        self.updated_at = Utc::now();

        Ok(Box::new(AlbumUnpublished {
            album_id: self.id.clone(),
            reason: "Manual unpublish".to_string(),
            unpublished_at: self.updated_at,
        }))
    }

    /// Mark as featured
    pub fn mark_as_featured(&mut self) {
        if !self.is_published {
            return; // Only published albums can be featured
        }
        
        self.is_featured = true;
        self.updated_at = Utc::now();
    }

    /// Unmark as featured
    pub fn unmark_as_featured(&mut self) {
        self.is_featured = false;
        self.updated_at = Utc::now();
    }

    /// Record listen
    pub fn record_listen(&mut self) {
        self.listen_count += 1;
        self.updated_at = Utc::now();
    }

    /// Add revenue
    pub fn add_revenue(&mut self, amount: f64) {
        if amount > 0.0 {
            self.revenue_generated += amount;
            self.updated_at = Utc::now();
        }
    }

    /// Get track by song ID
    pub fn get_track(&self, song_id: &SongId) -> Option<&AlbumTrack> {
        self.tracks.iter().find(|track| &track.song_id == song_id)
    }

    /// Get track by track number
    pub fn get_track_by_number(&self, track_number: u32) -> Option<&AlbumTrack> {
        self.tracks.iter().find(|track| track.track_number == track_number)
    }

    /// Get total duration
    pub fn get_total_duration(&self) -> Option<&SongDuration> {
        self.total_duration.as_ref()
    }

    /// Check if album is complete (has all required metadata)
    pub fn is_complete(&self) -> bool {
        !self.tracks.is_empty() &&
        self.cover_art_ipfs.is_some() &&
        self.description.is_some() &&
        self.release_date.is_some()
    }

    /// Get album metadata
    pub fn get_metadata(&self) -> AlbumMetadata {
        AlbumMetadata {
            id: self.id.clone(),
            title: self.title.value().to_string(),
            artist_id: self.artist_id.clone(),
            description: self.description.clone(),
            release_type: format!("{}", self.release_type),
            genre: self.genre.value().to_string(),
            release_date: self.release_date,
            track_count: self.tracks.len(),
            total_duration_seconds: self.total_duration.as_ref().map(|d| d.seconds()),
            cover_art_url: self.cover_art_ipfs.as_ref().map(|hash| 
                format!("https://ipfs.io/ipfs/{}", hash.value())
            ),
            is_published: self.is_published,
            is_featured: self.is_featured,
            listen_count: self.listen_count,
            revenue_generated: self.revenue_generated,
            created_at: self.created_at,
        }
    }

    // Private helper methods

    /// Renumber tracks after removal
    fn renumber_tracks(&mut self) {
        // Separate regular and bonus tracks
        let mut regular_tracks: Vec<_> = self.tracks
            .iter_mut()
            .filter(|t| !t.is_bonus_track)
            .collect();
        
        let mut bonus_tracks: Vec<_> = self.tracks
            .iter_mut()
            .filter(|t| t.is_bonus_track)
            .collect();

        // Renumber regular tracks
        for (i, track) in regular_tracks.iter_mut().enumerate() {
            track.track_number = (i + 1) as u32;
        }

        // Renumber bonus tracks
        let regular_count = regular_tracks.len();
        for (i, track) in bonus_tracks.iter_mut().enumerate() {
            track.track_number = (regular_count + i + 1) as u32;
        }
    }

    /// Recalculate total duration
    fn recalculate_total_duration(&mut self) {
        if self.tracks.is_empty() {
            self.total_duration = None;
        } else {
            let total_seconds: u32 = self.tracks
                .iter()
                .map(|track| track.duration.seconds())
                .sum();
            
            self.total_duration = SongDuration::new(total_seconds).ok();
        }
    }
}

impl AlbumTrack {
    pub fn song_id(&self) -> &SongId {
        &self.song_id
    }

    pub fn track_number(&self) -> u32 {
        self.track_number
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn duration(&self) -> &SongDuration {
        &self.duration
    }

    pub fn is_bonus_track(&self) -> bool {
        self.is_bonus_track
    }

    pub fn added_at(&self) -> DateTime<Utc> {
        self.added_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_contexts::music::domain::value_objects::SongDuration;

    #[test]
    fn test_album_creation() {
        let title = AlbumTitle::new("Test Album".to_string()).unwrap();
        let artist_id = ArtistId::new();
        let genre = Genre::new("rock".to_string()).unwrap();

        let album = Album::new(title.clone(), artist_id.clone(), ReleaseType::Album, genre.clone());

        assert_eq!(album.title(), &title);
        assert_eq!(album.artist_id(), &artist_id);
        assert_eq!(album.genre(), &genre);
        assert!(!album.is_published());
        assert!(album.tracks().is_empty());
    }

    #[test]
    fn test_add_song_to_album() {
        let title = AlbumTitle::new("Test Album".to_string()).unwrap();
        let artist_id = ArtistId::new();
        let genre = Genre::new("rock".to_string()).unwrap();

        let mut album = Album::new(title, artist_id, ReleaseType::Album, genre);

        let song_id = SongId::new();
        let duration = SongDuration::new(180).unwrap();

        let result = album.add_song(
            song_id.clone(),
            "Test Song".to_string(),
            duration,
            false,
        );

        assert!(result.is_ok());
        assert_eq!(album.tracks().len(), 1);
        assert_eq!(album.tracks()[0].song_id(), &song_id);
        assert_eq!(album.tracks()[0].track_number(), 1);
    }

    #[test]
    fn test_publish_album_validation() {
        let title = AlbumTitle::new("Test Album".to_string()).unwrap();
        let artist_id = ArtistId::new();
        let genre = Genre::new("rock".to_string()).unwrap();

        let mut album = Album::new(title, artist_id, ReleaseType::Album, genre);

        // Cannot publish empty album
        assert!(album.publish().is_err());

        // Add enough tracks for album
        for i in 0..7 {
            let song_id = SongId::new();
            let duration = SongDuration::new(180).unwrap();
            album.add_song(
                song_id,
                format!("Track {}", i + 1),
                duration,
                false,
            ).unwrap();
        }

        // Now should be able to publish
        assert!(album.publish().is_ok());
        assert!(album.is_published());
    }

    #[test]
    fn test_single_track_limit() {
        let title = AlbumTitle::new("Test Single".to_string()).unwrap();
        let artist_id = ArtistId::new();
        let genre = Genre::new("pop".to_string()).unwrap();

        let mut album = Album::new(title, artist_id, ReleaseType::Single, genre);

        // Add 3 tracks (maximum for single)
        for i in 0..3 {
            let song_id = SongId::new();
            let duration = SongDuration::new(180).unwrap();
            album.add_song(
                song_id,
                format!("Track {}", i + 1),
                duration,
                false,
            ).unwrap();
        }

        // Should be able to publish
        assert!(album.publish().is_ok());

        // Unpublish to add more tracks
        album.unpublish().unwrap();

        // Add one more track (should exceed limit)
        let song_id = SongId::new();
        let duration = SongDuration::new(180).unwrap();
        album.add_song(song_id, "Extra Track".to_string(), duration, false).unwrap();

        // Should not be able to publish with 4 tracks
        assert!(album.publish().is_err());
    }
} 