use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Datelike};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::bounded_contexts::music::domain::value_objects::{
    ArtistId, SongId, AlbumId, Genre, IpfsHash
};
use crate::bounded_contexts::music::domain::events::{
    ArtistProfileUpdated, ArtistGenreAdded, ArtistGenreRemoved, ArtistVerified,
    ArtistFollowed, ArtistUnfollowed
};
use crate::shared::domain::events::{DomainEvent, EventMetadata};

/// Artist entity representing musicians and content creators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Artist {
    id: ArtistId,
    user_id: Uuid, // Link to User Context
    profile: ArtistProfile,
    stats: ArtistStats,
    primary_genres: Vec<Genre>,
    is_verified: bool,
    is_featured: bool,
    is_active: bool,
    follower_count: u64,
    following_count: u64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// Artist profile information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArtistProfile {
    stage_name: String,
    bio: Option<String>,
    location: Option<String>,
    website: Option<String>,
    social_links: HashMap<String, String>,
    avatar_ipfs: Option<IpfsHash>,
    banner_ipfs: Option<IpfsHash>,
    debut_year: Option<u16>,
    record_label: Option<String>,
    management_contact: Option<String>,
    booking_contact: Option<String>,
}

/// Artist statistics and metrics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArtistStats {
    total_songs: u32,
    total_albums: u32,
    total_listens: u64,
    total_revenue: f64,
    monthly_listeners: u64,
    streaming_hours: u64,
    fan_engagement_score: f64,
    trending_score: f64,
    last_release_date: Option<DateTime<Utc>>,
    peak_monthly_listeners: u64,
    peak_monthly_listeners_date: Option<DateTime<Utc>>,
    countries_with_listeners: HashSet<String>,
    top_songs: Vec<SongId>,
    recent_activities: Vec<ArtistActivity>,
}

/// Artist activity tracking
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArtistActivity {
    activity_type: ArtistActivityType,
    title: String,
    description: Option<String>,
    related_id: Option<Uuid>, // Song ID, Album ID, etc.
    occurred_at: DateTime<Utc>,
}

/// Types of artist activities
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtistActivityType {
    SongReleased,
    AlbumReleased,
    PlaylistCreated,
    CollaborationAnnounced,
    ConcertAnnounced,
    AchievementUnlocked,
    MilestoneReached,
    InterviewPublished,
    BehindTheScenes,
}

/// Artist tier based on popularity and engagement
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtistTier {
    Emerging,    // New artists
    Rising,      // Growing audience
    Established, // Solid fan base
    Popular,     // Mainstream success
    Superstar,   // Global recognition
}

/// Artist verification status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationStatus {
    is_verified: bool,
    verified_at: Option<DateTime<Utc>>,
    verified_by: Option<Uuid>,
    verification_type: Option<VerificationType>,
}

/// Types of verification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationType {
    Identity,        // Identity verified
    Professional,   // Professional musician
    Label,          // Signed to label
    Independent,    // Independent verified artist
}

impl Artist {
    /// Create new artist profile
    pub fn new(
        user_id: Uuid,
        stage_name: String,
        primary_genre: Genre,
    ) -> Result<Self, String> {
        if stage_name.trim().is_empty() {
            return Err("Stage name cannot be empty".to_string());
        }

        if stage_name.len() > 100 {
            return Err("Stage name cannot exceed 100 characters".to_string());
        }

        let now = Utc::now();
        let profile = ArtistProfile {
            stage_name,
            bio: None,
            location: None,
            website: None,
            social_links: HashMap::new(),
            avatar_ipfs: None,
            banner_ipfs: None,
            debut_year: None,
            record_label: None,
            management_contact: None,
            booking_contact: None,
        };

        let stats = ArtistStats {
            total_songs: 0,
            total_albums: 0,
            total_listens: 0,
            total_revenue: 0.0,
            monthly_listeners: 0,
            streaming_hours: 0,
            fan_engagement_score: 0.0,
            trending_score: 0.0,
            last_release_date: None,
            peak_monthly_listeners: 0,
            peak_monthly_listeners_date: None,
            countries_with_listeners: HashSet::new(),
            top_songs: Vec::new(),
            recent_activities: Vec::new(),
        };

        Ok(Self {
            id: ArtistId::new(),
            user_id,
            profile,
            stats,
            primary_genres: vec![primary_genre],
            is_verified: false,
            is_featured: false,
            is_active: true,
            follower_count: 0,
            following_count: 0,
            created_at: now,
            updated_at: now,
        })
    }

    /// Create artist with specific ID (for loading from repository)
    pub fn with_id(
        id: ArtistId,
        user_id: Uuid,
        stage_name: String,
        primary_genre: Genre,
    ) -> Result<Self, String> {
        let mut artist = Self::new(user_id, stage_name, primary_genre)?;
        artist.id = id;
        Ok(artist)
    }

    // Getters
    pub fn id(&self) -> &ArtistId {
        &self.id
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn profile(&self) -> &ArtistProfile {
        &self.profile
    }

    pub fn stats(&self) -> &ArtistStats {
        &self.stats
    }

    pub fn stage_name(&self) -> &str {
        &self.profile.stage_name
    }

    pub fn primary_genres(&self) -> &[Genre] {
        &self.primary_genres
    }

    pub fn is_verified(&self) -> bool {
        self.is_verified
    }

    pub fn is_featured(&self) -> bool {
        self.is_featured
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn follower_count(&self) -> u64 {
        self.follower_count
    }

    pub fn following_count(&self) -> u64 {
        self.following_count
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    /// Business logic methods

    /// Update artist profile
    pub fn update_profile(
        &mut self,
        stage_name: Option<String>,
        bio: Option<String>,
        location: Option<String>,
        website: Option<String>,
        debut_year: Option<u16>,
        record_label: Option<String>,
    ) -> Result<Box<dyn DomainEvent>, String> {
        let mut changes = Vec::new();

        if let Some(new_stage_name) = stage_name {
            if new_stage_name.trim().is_empty() {
                return Err("Stage name cannot be empty".to_string());
            }
            if new_stage_name.len() > 50 {
                return Err("Stage name must be 50 characters or less".to_string());
            }
            self.profile.stage_name = new_stage_name;
            changes.push("stage_name".to_string());
        }

        if let Some(new_bio) = bio {
            if new_bio.len() > 1000 {
                return Err("Bio must be 1000 characters or less".to_string());
            }
            self.profile.bio = Some(new_bio);
            changes.push("bio".to_string());
        }

        if let Some(new_location) = location {
            if new_location.len() > 100 {
                return Err("Location must be 100 characters or less".to_string());
            }
            self.profile.location = Some(new_location);
            changes.push("location".to_string());
        }

        if let Some(new_website) = website {
            if !new_website.starts_with("http://") && !new_website.starts_with("https://") {
                return Err("Website must be a valid URL".to_string());
            }
            self.profile.website = Some(new_website);
            changes.push("website".to_string());
        }

        if let Some(new_debut_year) = debut_year {
            let current_year = Utc::now().year() as u16;
            if new_debut_year > current_year {
                return Err("Debut year cannot be in the future".to_string());
            }
            self.profile.debut_year = Some(new_debut_year);
            changes.push("debut_year".to_string());
        }

        if let Some(new_record_label) = record_label {
            if new_record_label.len() > 100 {
                return Err("Record label must be 100 characters or less".to_string());
            }
            self.profile.record_label = Some(new_record_label);
            changes.push("record_label".to_string());
        }

        self.updated_at = Utc::now();

        let metadata = EventMetadata {
            event_id: Uuid::new_v4(),
            event_type: "music.artist.profile_updated".to_string(),
            aggregate_id: *self.id.value(),
            aggregate_type: "Artist".to_string(),
            occurred_at: self.updated_at,
            correlation_id: None,
            user_id: Some(self.user_id),
            version: 1,
        };

        Ok(Box::new(ArtistProfileUpdated {
            metadata,
            artist_id: self.id.clone(),
            updated_fields: changes,
            updated_at: self.updated_at,
        }))
    }

    /// Set avatar image
    pub fn set_avatar(&mut self, ipfs_hash: IpfsHash) {
        self.profile.avatar_ipfs = Some(ipfs_hash);
        self.updated_at = Utc::now();
    }

    /// Set banner image
    pub fn set_banner(&mut self, ipfs_hash: IpfsHash) {
        self.profile.banner_ipfs = Some(ipfs_hash);
        self.updated_at = Utc::now();
    }

    /// Add social media link
    pub fn add_social_link(&mut self, platform: String, url: String) -> Result<(), String> {
        if platform.trim().is_empty() {
            return Err("Platform name cannot be empty".to_string());
        }

        if url.trim().is_empty() {
            return Err("URL cannot be empty".to_string());
        }

        // Basic URL validation
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err("URL must be a valid URL starting with http:// or https://".to_string());
        }

        self.profile.social_links.insert(platform.to_lowercase(), url);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Remove social media link
    pub fn remove_social_link(&mut self, platform: &str) {
        self.profile.social_links.remove(&platform.to_lowercase());
        self.updated_at = Utc::now();
    }

    /// Add primary genre
    pub fn add_primary_genre(&mut self, genre: Genre) -> Result<Box<dyn DomainEvent>, String> {
        if self.primary_genres.contains(&genre) {
            return Err("Genre already in primary genres".to_string());
        }

        if self.primary_genres.len() >= 5 {
            return Err("Cannot have more than 5 primary genres".to_string());
        }

        self.primary_genres.push(genre.clone());
        self.updated_at = Utc::now();

        let metadata = EventMetadata {
            event_id: Uuid::new_v4(),
            event_type: "music.artist.genre_added".to_string(),
            aggregate_id: *self.id.value(),
            aggregate_type: "Artist".to_string(),
            occurred_at: self.updated_at,
            correlation_id: None,
            user_id: Some(self.user_id),
            version: 1,
        };

        Ok(Box::new(ArtistGenreAdded {
            metadata,
            artist_id: self.id.clone(),
            genre: genre.clone(),
            added_at: self.updated_at,
        }))
    }

    /// Remove primary genre
    pub fn remove_primary_genre(&mut self, genre: &Genre) -> Result<Box<dyn DomainEvent>, String> {
        if self.primary_genres.len() <= 1 {
            return Err("Artist must have at least one primary genre".to_string());
        }

        let index = self.primary_genres
            .iter()
            .position(|g| g == genre)
            .ok_or("Genre not found in primary genres")?;

        self.primary_genres.remove(index);
        self.updated_at = Utc::now();

        let metadata = EventMetadata {
            event_id: Uuid::new_v4(),
            event_type: "music.artist.genre_removed".to_string(),
            aggregate_id: *self.id.value(),
            aggregate_type: "Artist".to_string(),
            occurred_at: self.updated_at,
            correlation_id: None,
            user_id: Some(self.user_id),
            version: 1,
        };

        Ok(Box::new(ArtistGenreRemoved {
            metadata,
            artist_id: self.id.clone(),
            genre: genre.clone(),
            removed_at: self.updated_at,
        }))
    }

    /// Verify artist
    pub fn verify(
        &mut self,
        verified_by: Uuid,
        verification_type: VerificationType,
    ) -> Result<Box<dyn DomainEvent>, String> {
        if self.is_verified {
            return Err("Artist is already verified".to_string());
        }

        self.is_verified = true;
        self.updated_at = Utc::now();

        let metadata = EventMetadata {
            event_id: Uuid::new_v4(),
            event_type: "music.artist.verified".to_string(),
            aggregate_id: *self.id.value(),
            aggregate_type: "Artist".to_string(),
            occurred_at: self.updated_at,
            correlation_id: None,
            user_id: Some(self.user_id),
            version: 1,
        };

        Ok(Box::new(ArtistVerified {
            metadata,
            artist_id: self.id.clone(),
            verified_by,
            verification_type: format!("{:?}", verification_type),
            verified_at: self.updated_at,
        }))
    }

    /// Mark as featured
    pub fn mark_as_featured(&mut self) {
        self.is_featured = true;
        self.updated_at = Utc::now();
    }

    /// Unmark as featured
    pub fn unmark_as_featured(&mut self) {
        self.is_featured = false;
        self.updated_at = Utc::now();
    }

    /// Deactivate artist
    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.is_featured = false;
        self.updated_at = Utc::now();
    }

    /// Reactivate artist
    pub fn reactivate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    /// Record new song release
    pub fn record_song_release(&mut self, song_id: SongId, song_title: String) {
        self.stats.total_songs += 1;
        self.stats.last_release_date = Some(Utc::now());
        
        // Add to top songs if not already there
        if !self.stats.top_songs.contains(&song_id) && self.stats.top_songs.len() < 10 {
            self.stats.top_songs.push(song_id.clone());
        }

        // Add activity
        let activity = ArtistActivity {
            activity_type: ArtistActivityType::SongReleased,
            title: format!("Released new song: {}", song_title),
            description: None,
            related_id: Some(song_id.value().clone()),
            occurred_at: Utc::now(),
        };
        
        self.add_activity(activity);
        self.updated_at = Utc::now();
    }

    /// Record new album release
    pub fn record_album_release(&mut self, album_id: AlbumId, album_title: String, track_count: u32) {
        self.stats.total_albums += 1;
        self.stats.total_songs += track_count;
        self.stats.last_release_date = Some(Utc::now());

        // Add activity
        let activity = ArtistActivity {
            activity_type: ArtistActivityType::AlbumReleased,
            title: format!("Released new album: {}", album_title),
            description: Some(format!("{} tracks", track_count)),
            related_id: Some(album_id.value().clone()),
            occurred_at: Utc::now(),
        };
        
        self.add_activity(activity);
        self.updated_at = Utc::now();
    }

    /// Record listen
    pub fn record_listen(&mut self, listener_country: Option<String>) {
        self.stats.total_listens += 1;
        
        if let Some(country) = listener_country {
            self.stats.countries_with_listeners.insert(country);
        }
        
        // Update monthly listeners (simplified - would need proper time-based tracking)
        self.stats.monthly_listeners += 1;
        
        if self.stats.monthly_listeners > self.stats.peak_monthly_listeners {
            self.stats.peak_monthly_listeners = self.stats.monthly_listeners;
            self.stats.peak_monthly_listeners_date = Some(Utc::now());
        }
        
        self.updated_at = Utc::now();
    }

    /// Add revenue
    pub fn add_revenue(&mut self, amount: f64) {
        if amount > 0.0 {
            self.stats.total_revenue += amount;
            self.updated_at = Utc::now();
        }
    }

    /// Add follower
    pub fn add_follower(&mut self, follower_id: Uuid) -> Box<dyn DomainEvent> {
        self.follower_count += 1;
        self.updated_at = Utc::now();

        let metadata = EventMetadata {
            event_id: Uuid::new_v4(),
            event_type: "music.artist.followed".to_string(),
            aggregate_id: *self.id.value(),
            aggregate_type: "Artist".to_string(),
            occurred_at: self.updated_at,
            correlation_id: None,
            user_id: Some(follower_id),
            version: 1,
        };

        Box::new(ArtistFollowed {
            metadata,
            artist_id: self.id.clone(),
            follower_id,
            followed_at: self.updated_at,
        })
    }

    /// Remove follower
    pub fn remove_follower(&mut self, follower_id: Uuid) -> Option<Box<dyn DomainEvent>> {
        if self.follower_count > 0 {
            self.follower_count -= 1;
            self.updated_at = Utc::now();

            let metadata = EventMetadata {
                event_id: Uuid::new_v4(),
                event_type: "music.artist.unfollowed".to_string(),
                aggregate_id: *self.id.value(),
                aggregate_type: "Artist".to_string(),
                occurred_at: self.updated_at,
                correlation_id: None,
                user_id: Some(follower_id),
                version: 1,
            };

            Some(Box::new(ArtistUnfollowed {
                metadata,
                artist_id: self.id.clone(),
                follower_id,
                unfollowed_at: self.updated_at,
            }))
        } else {
            None
        }
    }

    /// Calculate artist tier based on metrics
    pub fn calculate_tier(&self) -> ArtistTier {
        let followers = self.follower_count;
        let monthly_listeners = self.stats.monthly_listeners;
        let total_listens = self.stats.total_listens;

        match (followers, monthly_listeners, total_listens) {
            (f, ml, tl) if f >= 1_000_000 || ml >= 500_000 || tl >= 10_000_000 => ArtistTier::Superstar,
            (f, ml, tl) if f >= 100_000 || ml >= 50_000 || tl >= 1_000_000 => ArtistTier::Popular,
            (f, ml, tl) if f >= 10_000 || ml >= 5_000 || tl >= 100_000 => ArtistTier::Established,
            (f, ml, tl) if f >= 1_000 || ml >= 500 || tl >= 10_000 => ArtistTier::Rising,
            _ => ArtistTier::Emerging,
        }
    }

    /// Update fan engagement score
    pub fn update_engagement_score(&mut self, score: f64) {
        if score >= 0.0 && score <= 100.0 {
            self.stats.fan_engagement_score = score;
            self.updated_at = Utc::now();
        }
    }

    /// Update trending score
    pub fn update_trending_score(&mut self, score: f64) {
        if score >= 0.0 && score <= 100.0 {
            self.stats.trending_score = score;
            self.updated_at = Utc::now();
        }
    }

    /// Check if artist is trending
    pub fn is_trending(&self) -> bool {
        self.stats.trending_score >= 70.0
    }

    // Private helper methods

    /// Add activity to recent activities
    fn add_activity(&mut self, activity: ArtistActivity) {
        self.stats.recent_activities.push(activity);
        
        // Keep only last 50 activities
        if self.stats.recent_activities.len() > 50 {
            self.stats.recent_activities.drain(0..self.stats.recent_activities.len() - 50);
        }
    }
}

impl ArtistProfile {
    pub fn stage_name(&self) -> &str {
        &self.stage_name
    }

    pub fn bio(&self) -> Option<&str> {
        self.bio.as_deref()
    }

    pub fn location(&self) -> Option<&str> {
        self.location.as_deref()
    }

    pub fn website(&self) -> Option<&str> {
        self.website.as_deref()
    }

    pub fn social_links(&self) -> &HashMap<String, String> {
        &self.social_links
    }

    pub fn avatar_ipfs(&self) -> Option<&IpfsHash> {
        self.avatar_ipfs.as_ref()
    }

    pub fn banner_ipfs(&self) -> Option<&IpfsHash> {
        self.banner_ipfs.as_ref()
    }

    pub fn debut_year(&self) -> Option<u16> {
        self.debut_year
    }

    pub fn record_label(&self) -> Option<&str> {
        self.record_label.as_deref()
    }
}

impl ArtistStats {
    pub fn total_songs(&self) -> u32 {
        self.total_songs
    }

    pub fn total_albums(&self) -> u32 {
        self.total_albums
    }

    pub fn total_listens(&self) -> u64 {
        self.total_listens
    }

    pub fn total_revenue(&self) -> f64 {
        self.total_revenue
    }

    pub fn monthly_listeners(&self) -> u64 {
        self.monthly_listeners
    }

    pub fn fan_engagement_score(&self) -> f64 {
        self.fan_engagement_score
    }

    pub fn trending_score(&self) -> f64 {
        self.trending_score
    }

    pub fn recent_activities(&self) -> &[ArtistActivity] {
        &self.recent_activities
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artist_creation() {
        let user_id = Uuid::new_v4();
        let genre = Genre::new("rock".to_string()).unwrap();

        let artist = Artist::new(
            user_id,
            "Test Artist".to_string(),
            genre.clone(),
        ).unwrap();

        assert_eq!(artist.user_id(), user_id);
        assert_eq!(artist.stage_name(), "Test Artist");
        assert!(artist.primary_genres().contains(&genre));
        assert!(!artist.is_verified());
        assert!(artist.is_active());
    }

    #[test]
    fn test_profile_update() {
        let user_id = Uuid::new_v4();
        let genre = Genre::new("pop".to_string()).unwrap();

        let mut artist = Artist::new(
            user_id,
            "Original Name".to_string(),
            genre,
        ).unwrap();

        let result = artist.update_profile(
            Some("New Stage Name".to_string()),
            Some("New bio".to_string()),
            Some("New York".to_string()),
            Some("https://example.com".to_string()),
            Some(2020),
            Some("Test Records".to_string()),
        );

        assert!(result.is_ok());
        assert_eq!(artist.stage_name(), "New Stage Name");
        assert_eq!(artist.profile().bio(), Some("New bio"));
        assert_eq!(artist.profile().location(), Some("New York"));
    }

    #[test]
    fn test_genre_management() {
        let user_id = Uuid::new_v4();
        let rock_genre = Genre::new("rock".to_string()).unwrap();

        let mut artist = Artist::new(
            user_id,
            "Test Artist".to_string(),
            rock_genre.clone(),
        ).unwrap();

        // Add second genre
        let pop_genre = Genre::new("pop".to_string()).unwrap();
        let result = artist.add_primary_genre(pop_genre.clone());
        assert!(result.is_ok());
        assert_eq!(artist.primary_genres().len(), 2);

        // Remove original genre should fail (must have at least one)
        if artist.primary_genres().len() == 1 {
            let remove_result = artist.remove_primary_genre(&rock_genre);
            assert!(remove_result.is_err());
        }
    }

    #[test]
    fn test_artist_tier_calculation() {
        let user_id = Uuid::new_v4();
        let genre = Genre::new("electronic".to_string()).unwrap();

        let mut artist = Artist::new(
            user_id,
            "Electronic Artist".to_string(),
            genre,
        ).unwrap();

        // New artist should be Emerging
        assert_eq!(artist.calculate_tier(), ArtistTier::Emerging);

        // Simulate growth
        for _ in 0..15_000 {
            artist.add_follower(Uuid::new_v4());
        }

        // Should now be Established
        assert_eq!(artist.calculate_tier(), ArtistTier::Established);
    }

    #[test]
    fn test_social_links() {
        let user_id = Uuid::new_v4();
        let genre = Genre::new("jazz".to_string()).unwrap();

        let mut artist = Artist::new(
            user_id,
            "Jazz Artist".to_string(),
            genre,
        ).unwrap();

        // Add social link
        let result = artist.add_social_link(
            "Twitter".to_string(),
            "https://twitter.com/jazzartist".to_string(),
        );
        assert!(result.is_ok());
        assert!(artist.profile().social_links().contains_key("twitter"));

        // Remove social link
        artist.remove_social_link("twitter");
        assert!(!artist.profile().social_links().contains_key("twitter"));
    }
} 