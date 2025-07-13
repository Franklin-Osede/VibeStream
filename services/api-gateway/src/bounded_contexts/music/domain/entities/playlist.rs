use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashSet;
use uuid::Uuid;

use crate::bounded_contexts::music::domain::value_objects::{
    PlaylistId, PlaylistName, SongId, SongDuration, Genre
};
use crate::bounded_contexts::music::domain::events::{
    PlaylistCreated, PlaylistUpdated, PlaylistMadePublic, PlaylistMadePrivate,
    SongAddedToPlaylist, SongRemovedFromPlaylist, PlaylistShared
};
use crate::shared::domain::events::DomainEvent;
use crate::bounded_contexts::music::domain::value_objects::IpfsHash;
use crate::shared::domain::events::EventMetadata;
use crate::shared::domain::errors::AppError;

/// Playlist entity for organizing songs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Playlist {
    id: PlaylistId,
    name: PlaylistName,
    creator_id: Uuid,
    description: Option<String>,
    tracks: Vec<PlaylistTrack>,
    is_public: bool,
    is_collaborative: bool,
    is_featured: bool,
    collaborators: HashSet<Uuid>,
    tags: Vec<String>,
    total_duration: Option<SongDuration>,
    listen_count: u64,
    like_count: u64,
    follower_count: u64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    cover_image_ipfs: Option<IpfsHash>,
}

/// Track within a playlist
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaylistTrack {
    song_id: SongId,
    position: u32,
    title: String,
    artist_name: String,
    duration: SongDuration,
    added_by: Uuid,
    added_at: DateTime<Utc>,
}

/// Playlist metadata for external consumption
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaylistMetadata {
    pub id: PlaylistId,
    pub name: String,
    pub creator_id: Uuid,
    pub description: Option<String>,
    pub track_count: usize,
    pub total_duration_seconds: Option<u32>,
    pub is_public: bool,
    pub is_collaborative: bool,
    pub is_featured: bool,
    pub collaborator_count: usize,
    pub tags: Vec<String>,
    pub listen_count: u64,
    pub like_count: u64,
    pub follower_count: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Playlist visibility settings
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaylistVisibility {
    Public,
    Private,
    Unlisted, // Can be accessed with direct link but not discoverable
}

/// Playlist collaborative permissions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CollaborativePermission {
    AddOnly,    // Can only add songs
    FullAccess, // Can add, remove, and reorder songs
    Moderate,   // Full access + can manage other collaborators
}

impl Playlist {
    /// Create new playlist
    pub fn new(
        name: PlaylistName,
        creator_id: Uuid,
        description: Option<String>,
        is_public: bool,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: PlaylistId::new(),
            name,
            creator_id,
            description,
            tracks: Vec::new(),
            is_public,
            is_collaborative: false,
            is_featured: false,
            collaborators: HashSet::new(),
            tags: Vec::new(),
            total_duration: None,
            listen_count: 0,
            like_count: 0,
            follower_count: 0,
            created_at: now,
            updated_at: now,
            cover_image_ipfs: None,
        }
    }

    /// Create playlist with specific ID (for loading from repository)
    pub fn with_id(
        id: PlaylistId,
        name: PlaylistName,
        creator_id: Uuid,
        description: Option<String>,
        is_public: bool,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            creator_id,
            description,
            tracks: Vec::new(),
            is_public,
            is_collaborative: false,
            is_featured: false,
            collaborators: HashSet::new(),
            tags: Vec::new(),
            total_duration: None,
            listen_count: 0,
            like_count: 0,
            follower_count: 0,
            created_at: now,
            updated_at: now,
            cover_image_ipfs: None,
        }
    }

    /// Create Playlist from database persistence data
    pub fn from_persistence(
        id: PlaylistId,
        name: PlaylistName,
        creator_id: Uuid,
        description: Option<String>,
        is_public: bool,
        cover_image_ipfs: Option<IpfsHash>,
        total_duration: Option<SongDuration>,
        listen_count: u64,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            creator_id,
            description,
            is_public,
            tracks: Vec::new(), // Tracks loaded separately
            is_collaborative: false,
            is_featured: false,
            collaborators: HashSet::new(),
            tags: Vec::new(),
            total_duration,
            listen_count,
            like_count: 0,
            follower_count: 0,
            created_at,
            updated_at,
            cover_image_ipfs,
        }
    }

    /// Get cover image URL (method missing from original)
    pub fn cover_image_url(&self) -> Option<String> {
        self.cover_image_ipfs.as_ref().map(|hash| 
            format!("https://ipfs.io/ipfs/{}", hash.value())
        )
    }

    // Getters
    pub fn id(&self) -> &PlaylistId {
        &self.id
    }

    pub fn name(&self) -> &PlaylistName {
        &self.name
    }

    pub fn creator_id(&self) -> Uuid {
        self.creator_id
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn tracks(&self) -> &[PlaylistTrack] {
        &self.tracks
    }

    pub fn is_public(&self) -> bool {
        self.is_public
    }

    pub fn is_collaborative(&self) -> bool {
        self.is_collaborative
    }

    pub fn is_featured(&self) -> bool {
        self.is_featured
    }

    pub fn collaborators(&self) -> &HashSet<Uuid> {
        &self.collaborators
    }

    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    pub fn listen_count(&self) -> u64 {
        self.listen_count
    }

    pub fn like_count(&self) -> u64 {
        self.like_count
    }

    pub fn follower_count(&self) -> u64 {
        self.follower_count
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    /// Business logic methods

    /// Add song to playlist
    pub fn add_song(
        &mut self,
        song_id: SongId,
        title: String,
        artist_name: String,
        duration: SongDuration,
        added_by: Uuid,
        position: Option<u32>,
    ) -> Result<Box<dyn DomainEvent>, String> {
        // Check permissions
        if !self.can_user_modify(&added_by) {
            return Err("User does not have permission to add songs to this playlist".to_string());
        }

        // Check if song already exists
        if self.tracks.iter().any(|track| track.song_id == song_id) {
            return Err("Song already exists in playlist".to_string());
        }

        let final_position = position.unwrap_or((self.tracks.len() + 1) as u32);

        // Validate position
        if final_position > (self.tracks.len() + 1) as u32 {
            return Err("Invalid position for new track".to_string());
        }

        let track = PlaylistTrack {
            song_id: song_id.clone(),
            position: final_position,
            title: title.clone(),
            artist_name: artist_name.clone(),
            duration: duration.clone(),
            added_by,
            added_at: Utc::now(),
        };

        // Insert at specified position
        if final_position <= self.tracks.len() as u32 {
            // Reposition existing tracks
            for existing_track in &mut self.tracks {
                if existing_track.position >= final_position {
                    existing_track.position += 1;
                }
            }
            // Insert new track
            let insert_index = (final_position - 1) as usize;
            self.tracks.insert(insert_index, track);
        } else {
            // Add at end
            self.tracks.push(track);
        }

        self.recalculate_total_duration();
        self.updated_at = Utc::now();

        // Create domain event
        Ok(Box::new(SongAddedToPlaylist {
            metadata: EventMetadata::with_type_and_aggregate(
                "SongAddedToPlaylist",
                self.id.to_uuid(),
                "Playlist",
            ),
            playlist_id: self.id.clone(),
            song_id: song_id.clone(),
            position: self.tracks.len() as u32,
            added_by: self.creator_id,
            added_at: Utc::now(),
        }))
    }

    /// Remove song from playlist
    pub fn remove_song(
        &mut self,
        song_id: &SongId,
        removed_by: Uuid,
    ) -> Result<Box<dyn DomainEvent>, String> {
        // Check permissions
        if !self.can_user_modify(&removed_by) {
            return Err("User does not have permission to remove songs from this playlist".to_string());
        }

        // Find and remove track
        let track_index = self.tracks
            .iter()
            .position(|track| &track.song_id == song_id)
            .ok_or("Song not found in playlist")?;

        let removed_track = self.tracks.remove(track_index);

        // Reposition remaining tracks
        for track in &mut self.tracks {
            if track.position > removed_track.position {
                track.position -= 1;
            }
        }

        self.recalculate_total_duration();
        self.updated_at = Utc::now();

        // Create domain event
        Ok(Box::new(SongRemovedFromPlaylist {
            metadata: EventMetadata::with_type_and_aggregate(
                "SongRemovedFromPlaylist",
                self.id.to_uuid(),
                "Playlist",
            ),
            playlist_id: self.id.clone(),
            song_id: song_id.clone(),
            removed_by: self.creator_id,
            removed_at: Utc::now(),
        }))
    }

    /// Reorder songs in playlist
    pub fn reorder_songs(
        &mut self,
        song_order: Vec<SongId>,
        reordered_by: Uuid,
    ) -> Result<(), String> {
        // Check permissions
        if !self.can_user_modify(&reordered_by) {
            return Err("User does not have permission to reorder this playlist".to_string());
        }

        // Validate all songs exist and order is complete
        if song_order.len() != self.tracks.len() {
            return Err("Song order must include all tracks in playlist".to_string());
        }

        for song_id in &song_order {
            if !self.tracks.iter().any(|track| &track.song_id == song_id) {
                return Err(format!("Song {} not found in playlist", song_id));
            }
        }

        // Reorder tracks
        let mut reordered_tracks = Vec::new();
        for (new_position, song_id) in song_order.iter().enumerate() {
            if let Some(mut track) = self.tracks
                .iter()
                .find(|track| &track.song_id == song_id)
                .cloned()
            {
                track.position = (new_position + 1) as u32;
                reordered_tracks.push(track);
            }
        }

        self.tracks = reordered_tracks;
        self.updated_at = Utc::now();

        Ok(())
    }

    /// Update playlist metadata
    pub fn update_metadata(
        &mut self,
        name: Option<PlaylistName>,
        description: Option<String>,
        tags: Option<Vec<String>>,
        updated_by: Uuid,
    ) -> Result<Box<dyn DomainEvent>, String> {
        // Check permissions (only creator or collaborators with moderate permissions)
        if self.creator_id != updated_by && !self.collaborators.contains(&updated_by) {
            return Err("User does not have permission to update this playlist".to_string());
        }

        let mut changes = std::collections::HashMap::new();

        if let Some(new_name) = name {
            changes.insert("name".to_string(), self.name.value().to_string());
            self.name = new_name;
        }

        if let Some(new_description) = description {
            changes.insert("description".to_string(), 
                self.description.clone().unwrap_or_default());
            self.description = Some(new_description);
        }

        if let Some(new_tags) = tags {
            // Validate tags
            for tag in &new_tags {
                if tag.trim().is_empty() || tag.len() > 50 {
                    return Err("Invalid tag format".to_string());
                }
            }
            self.tags = new_tags;
        }

        self.updated_at = Utc::now();

        Ok(Box::new(PlaylistUpdated {
            metadata: EventMetadata::with_type_and_aggregate(
                "PlaylistUpdated",
                self.id.to_uuid(),
                "Playlist",
            ),
            playlist_id: self.id.clone(),
            user_id: self.creator_id,
            updated_fields: changes.into_keys().collect(),
            updated_at: Utc::now(),
        }))
    }

    /// Make playlist public
    pub fn make_public(&mut self, user_id: Uuid) -> Result<Box<dyn DomainEvent>, String> {
        if self.creator_id != user_id {
            return Err("Only creator can change playlist visibility".to_string());
        }

        if self.is_public {
            return Err("Playlist is already public".to_string());
        }

        self.is_public = true;
        self.updated_at = Utc::now();

        Ok(Box::new(PlaylistMadePublic {
            metadata: EventMetadata::with_type_and_aggregate(
                "PlaylistMadePublic",
                self.id.to_uuid(),
                "Playlist",
            ),
            playlist_id: self.id.clone(),
            user_id: self.creator_id,
            made_public_at: Utc::now(),
        }))
    }

    /// Make playlist private
    pub fn make_private(&mut self, user_id: Uuid) -> Result<Box<dyn DomainEvent>, String> {
        if self.creator_id != user_id {
            return Err("Only creator can change playlist visibility".to_string());
        }

        if !self.is_public {
            return Err("Playlist is already private".to_string());
        }

        self.is_public = false;
        self.is_featured = false; // Private playlists cannot be featured
        self.updated_at = Utc::now();

        Ok(Box::new(PlaylistMadePrivate {
            metadata: EventMetadata::with_type_and_aggregate(
                "PlaylistMadePrivate",
                self.id.to_uuid(),
                "Playlist",
            ),
            playlist_id: self.id.clone(),
            user_id: self.creator_id,
            made_private_at: Utc::now(),
        }))
    }

    /// Enable collaborative editing
    pub fn enable_collaboration(&mut self, user_id: Uuid) -> Result<(), String> {
        if self.creator_id != user_id {
            return Err("Only creator can enable collaboration".to_string());
        }

        self.is_collaborative = true;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Disable collaborative editing
    pub fn disable_collaboration(&mut self, user_id: Uuid) -> Result<(), String> {
        if self.creator_id != user_id {
            return Err("Only creator can disable collaboration".to_string());
        }

        self.is_collaborative = false;
        self.collaborators.clear();
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Add collaborator
    pub fn add_collaborator(
        &mut self,
        collaborator_id: Uuid,
        added_by: Uuid,
    ) -> Result<(), String> {
        if self.creator_id != added_by {
            return Err("Only creator can add collaborators".to_string());
        }

        if !self.is_collaborative {
            return Err("Playlist is not collaborative".to_string());
        }

        if collaborator_id == self.creator_id {
            return Err("Creator is already a collaborator".to_string());
        }

        self.collaborators.insert(collaborator_id);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Remove collaborator
    pub fn remove_collaborator(
        &mut self,
        collaborator_id: Uuid,
        removed_by: Uuid,
    ) -> Result<(), String> {
        if self.creator_id != removed_by && removed_by != collaborator_id {
            return Err("Only creator or the collaborator themselves can remove collaboration".to_string());
        }

        if !self.collaborators.contains(&collaborator_id) {
            return Err("User is not a collaborator".to_string());
        }

        self.collaborators.remove(&collaborator_id);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Mark as featured
    pub fn mark_as_featured(&mut self) {
        if !self.is_public {
            return; // Only public playlists can be featured
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

    /// Add like
    pub fn add_like(&mut self) {
        self.like_count += 1;
        self.updated_at = Utc::now();
    }

    /// Remove like
    pub fn remove_like(&mut self) {
        if self.like_count > 0 {
            self.like_count -= 1;
            self.updated_at = Utc::now();
        }
    }

    /// Add follower
    pub fn add_follower(&mut self) {
        self.follower_count += 1;
        self.updated_at = Utc::now();
    }

    /// Remove follower
    pub fn remove_follower(&mut self) {
        if self.follower_count > 0 {
            self.follower_count -= 1;
            self.updated_at = Utc::now();
        }
    }

    /// Get track by song ID
    pub fn get_track(&self, song_id: &SongId) -> Option<&PlaylistTrack> {
        self.tracks.iter().find(|track| &track.song_id == song_id)
    }

    /// Get track by position
    pub fn get_track_by_position(&self, position: u32) -> Option<&PlaylistTrack> {
        self.tracks.iter().find(|track| track.position == position)
    }

    /// Get total duration
    pub fn get_total_duration(&self) -> Option<&SongDuration> {
        self.total_duration.as_ref()
    }

    /// Get dominant genres in playlist
    pub fn get_dominant_genres(&self) -> Vec<String> {
        // This would typically analyze the genres of songs in the playlist
        // For now, return empty vector - would need song metadata
        Vec::new()
    }

    /// Check if playlist is empty
    pub fn is_empty(&self) -> bool {
        self.tracks.is_empty()
    }

    /// Get playlist metadata
    pub fn get_metadata(&self) -> PlaylistMetadata {
        PlaylistMetadata {
            id: self.id.clone(),
            name: self.name.value().to_string(),
            creator_id: self.creator_id,
            description: self.description.clone(),
            track_count: self.tracks.len(),
            total_duration_seconds: self.total_duration.as_ref().map(|d| d.seconds()),
            is_public: self.is_public,
            is_collaborative: self.is_collaborative,
            is_featured: self.is_featured,
            collaborator_count: self.collaborators.len(),
            tags: self.tags.clone(),
            listen_count: self.listen_count,
            like_count: self.like_count,
            follower_count: self.follower_count,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }

    // Private helper methods

    /// Check if user can modify playlist
    fn can_user_modify(&self, user_id: &Uuid) -> bool {
        *user_id == self.creator_id || 
        (self.is_collaborative && self.collaborators.contains(user_id))
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

impl PlaylistTrack {
    pub fn new(
        song_id: SongId,
        title: String,
        artist_name: String,
        duration: SongDuration,
        track_order: u32,
    ) -> Self {
        Self {
            song_id,
            position: track_order, // Assuming position is the track_order
            title,
            artist_name,
            duration,
            added_by: Uuid::new_v4(), // Placeholder, actual added_by will be set later
            added_at: Utc::now(),
        }
    }

    pub fn song_id(&self) -> &SongId {
        &self.song_id
    }

    pub fn position(&self) -> u32 {
        self.position
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn artist_name(&self) -> &str {
        &self.artist_name
    }

    pub fn duration(&self) -> &SongDuration {
        &self.duration
    }

    pub fn added_by(&self) -> Uuid {
        self.added_by
    }

    pub fn added_at(&self) -> DateTime<Utc> {
        self.added_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_playlist_creation() {
        let name = PlaylistName::new("My Playlist".to_string()).unwrap();
        let creator_id = Uuid::new_v4();

        let playlist = Playlist::new(
            name.clone(),
            creator_id,
            Some("Test description".to_string()),
            true,
        );

        assert_eq!(playlist.name(), &name);
        assert_eq!(playlist.creator_id(), creator_id);
        assert!(playlist.is_public());
        assert!(!playlist.is_collaborative());
        assert!(playlist.tracks().is_empty());
    }

    #[test]
    fn test_add_song_to_playlist() {
        let name = PlaylistName::new("My Playlist".to_string()).unwrap();
        let creator_id = Uuid::new_v4();

        let mut playlist = Playlist::new(name, creator_id, None, true);

        let song_id = SongId::new();
        let duration = SongDuration::new(180).unwrap();

        let result = playlist.add_song(
            song_id.clone(),
            "Test Song".to_string(),
            "Test Artist".to_string(),
            duration,
            creator_id,
            None,
        );

        assert!(result.is_ok());
        assert_eq!(playlist.tracks().len(), 1);
        assert_eq!(playlist.tracks()[0].song_id(), &song_id);
        assert_eq!(playlist.tracks()[0].position(), 1);
    }

    #[test]
    fn test_collaboration_permissions() {
        let name = PlaylistName::new("Collaborative Playlist".to_string()).unwrap();
        let creator_id = Uuid::new_v4();
        let collaborator_id = Uuid::new_v4();
        let other_user_id = Uuid::new_v4();

        let mut playlist = Playlist::new(name, creator_id, None, true);

        // Enable collaboration
        assert!(playlist.enable_collaboration(creator_id).is_ok());
        assert!(playlist.is_collaborative());

        // Add collaborator
        assert!(playlist.add_collaborator(collaborator_id, creator_id).is_ok());
        assert!(playlist.collaborators().contains(&collaborator_id));

        // Collaborator should be able to add songs
        let song_id = SongId::new();
        let duration = SongDuration::new(180).unwrap();

        let result = playlist.add_song(
            song_id,
            "Test Song".to_string(),
            "Test Artist".to_string(),
            duration,
            collaborator_id,
            None,
        );
        assert!(result.is_ok());

        // Non-collaborator should not be able to add songs
        let song_id2 = SongId::new();
        let duration2 = SongDuration::new(200).unwrap();

        let result2 = playlist.add_song(
            song_id2,
            "Test Song 2".to_string(),
            "Test Artist 2".to_string(),
            duration2,
            other_user_id,
            None,
        );
        assert!(result2.is_err());
    }

    #[test]
    fn test_playlist_visibility() {
        let name = PlaylistName::new("Test Playlist".to_string()).unwrap();
        let creator_id = Uuid::new_v4();

        let mut playlist = Playlist::new(name, creator_id, None, false);
        assert!(!playlist.is_public());

        // Make public
        assert!(playlist.make_public(creator_id).is_ok());
        assert!(playlist.is_public());

        // Make private again
        assert!(playlist.make_private(creator_id).is_ok());
        assert!(!playlist.is_public());
        assert!(!playlist.is_featured()); // Should not be featured when private
    }
} 