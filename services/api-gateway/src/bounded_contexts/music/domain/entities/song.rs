use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::bounded_contexts::music::domain::value_objects::{
    SongId, SongTitle, ArtistId, SongDuration, Genre, IpfsHash, 
    RoyaltyPercentage, ListenCount, Tempo, ReleaseType, SongMood, FileFormat, AudioQuality
};
use crate::shared::domain::events::DomainEvent;
use crate::bounded_contexts::music::domain::events::{
    SongListened, SongAvailableForCampaign, SongAvailableForOwnership
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    id: SongId,
    title: SongTitle,
    artist_id: ArtistId,
    duration: SongDuration,
    genre: Genre,
    mood: Option<SongMood>,
    file_format: Option<FileFormat>,
    audio_quality: Option<AudioQuality>,
    tempo: Option<Tempo>,
    release_type: Option<ReleaseType>,
    ipfs_hash: Option<IpfsHash>,
    royalty_percentage: RoyaltyPercentage,
    listen_count: ListenCount,
    revenue_generated: f64,
    is_available_for_campaign: bool,
    is_available_for_ownership: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Song {
    pub fn new(
        title: SongTitle,
        artist_id: ArtistId,
        duration: SongDuration,
        genre: Genre,
        royalty_percentage: RoyaltyPercentage,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: SongId::new(),
            title,
            artist_id,
            duration,
            genre,
            mood: None,
            file_format: None,
            audio_quality: None,
            tempo: None,
            release_type: None,
            ipfs_hash: None,
            royalty_percentage,
            listen_count: ListenCount::new(),
            revenue_generated: 0.0,
            is_available_for_campaign: false,
            is_available_for_ownership: false,
            created_at: now,
            updated_at: now,
        }
    }

    // Getters
    pub fn id(&self) -> &SongId {
        &self.id
    }

    pub fn title(&self) -> &SongTitle {
        &self.title
    }

    pub fn artist_id(&self) -> &ArtistId {
        &self.artist_id
    }

    pub fn duration(&self) -> &SongDuration {
        &self.duration
    }

    pub fn genre(&self) -> &Genre {
        &self.genre
    }

    pub fn mood(&self) -> Option<&SongMood> {
        self.mood.as_ref()
    }

    pub fn file_format(&self) -> Option<&FileFormat> {
        self.file_format.as_ref()
    }

    pub fn audio_quality(&self) -> Option<&AudioQuality> {
        self.audio_quality.as_ref()
    }

    pub fn tempo(&self) -> Option<&Tempo> {
        self.tempo.as_ref()
    }

    pub fn release_type(&self) -> Option<&ReleaseType> {
        self.release_type.as_ref()
    }

    pub fn ipfs_hash(&self) -> Option<&IpfsHash> {
        self.ipfs_hash.as_ref()
    }

    pub fn royalty_percentage(&self) -> &RoyaltyPercentage {
        &self.royalty_percentage
    }

    pub fn listen_count(&self) -> &ListenCount {
        &self.listen_count
    }

    pub fn set_listen_count(&mut self, listen_count: ListenCount) {
        self.listen_count = listen_count;
        self.updated_at = Utc::now();
    }

    pub fn revenue_generated(&self) -> f64 {
        self.revenue_generated
    }

    pub fn set_revenue_generated(&mut self, revenue: f64) {
        self.revenue_generated = revenue;
        self.updated_at = Utc::now();
    }

    pub fn is_available_for_campaign(&self) -> bool {
        self.is_available_for_campaign
    }

    pub fn is_available_for_ownership(&self) -> bool {
        self.is_available_for_ownership
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // Rich domain behaviors
    pub fn can_create_campaign(&self) -> bool {
        self.is_available_for_campaign && self.listen_count.value() >= 100
    }

    pub fn can_create_ownership_contract(&self) -> bool {
        self.is_available_for_ownership && self.revenue_generated >= 1000.0
    }

    pub fn record_listen(&mut self, listener_id: Uuid, listen_duration_seconds: u32) -> Result<Box<dyn DomainEvent>, String> {
        // Domain validation: listen must be at least 30 seconds or 50% of song
        let min_duration = std::cmp::min(30, (self.duration.seconds() as f64 * 0.5) as u32);
        
        if listen_duration_seconds < min_duration {
            return Err("Listen duration too short to count as valid listen".to_string());
        }

        self.listen_count.increment();
        self.updated_at = Utc::now();

        // Check if song becomes eligible for campaigns
        if !self.is_available_for_campaign && self.listen_count.value() >= 100 {
            self.is_available_for_campaign = true;
            return Ok(Box::new(SongAvailableForCampaign {
                song_id: self.id.clone(),
                artist_id: self.artist_id.clone(),
                listen_count: self.listen_count.value(),
                marked_at: Utc::now(),
                metadata: crate::shared::domain::events::EventMetadata::with_type_and_aggregate(
                    "SongAvailableForCampaign",
                    self.id.to_uuid(),
                    "Song",
                ),
            }));
        }

        Ok(Box::new(SongListened {
            song_id: self.id.clone(),
            listener_id,
            listen_count: self.listen_count.value(),
            listen_duration_seconds,
            listened_at: Utc::now(),
            metadata: crate::shared::domain::events::EventMetadata::new(),
        }))
    }

    pub fn add_revenue(&mut self, amount: f64) -> Result<Option<Box<dyn DomainEvent>>, String> {
        if amount < 0.0 {
            return Err("Revenue amount cannot be negative".to_string());
        }

        let old_revenue = self.revenue_generated;
        self.revenue_generated += amount;
        self.updated_at = Utc::now();

        // Check if song becomes eligible for fractional ownership
        if !self.is_available_for_ownership && old_revenue < 1000.0 && self.revenue_generated >= 1000.0 {
            self.is_available_for_ownership = true;
            return Ok(Some(Box::new(SongAvailableForOwnership {
                song_id: self.id.clone(),
                artist_id: self.artist_id.clone(),
                revenue_threshold_reached: self.revenue_generated,
                marked_at: Utc::now(),
                metadata: crate::shared::domain::events::EventMetadata::new(),
            })));
        }

        Ok(None)
    }

    pub fn calculate_artist_revenue(&self, total_revenue: f64) -> f64 {
        total_revenue * self.royalty_percentage.as_decimal()
    }

    pub fn set_ipfs_hash(&mut self, ipfs_hash: IpfsHash) {
        self.ipfs_hash = Some(ipfs_hash);
        self.updated_at = Utc::now();
    }

    pub fn update_title(&mut self, new_title: SongTitle) -> Result<(), String> {
        // Domain rule: Can't change title if song has significant listens
        if self.listen_count.value() > 1000 {
            return Err("Cannot change title of song with more than 1000 listens".to_string());
        }

        self.title = new_title;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn update_royalty_percentage(&mut self, new_percentage: RoyaltyPercentage) -> Result<(), String> {
        // Domain rule: Can't change royalty if ownership contracts exist
        if self.is_available_for_ownership {
            return Err("Cannot change royalty percentage once fractional ownership is available".to_string());
        }

        self.royalty_percentage = new_percentage;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn is_popular(&self) -> bool {
        self.listen_count.value() >= 10000
    }

    pub fn is_trending(&self) -> bool {
        // Simple trending logic: created in last 30 days and has good listen ratio
        let thirty_days_ago = Utc::now() - chrono::Duration::days(30);
        let is_recent = self.created_at > thirty_days_ago;
        let listen_rate = self.listen_count.value() as f64 / 
            (Utc::now() - self.created_at).num_days().max(1) as f64;
        
        is_recent && listen_rate >= 50.0
    }

    pub fn get_metadata(&self) -> SongMetadata {
        SongMetadata {
            id: self.id.clone(),
            title: self.title.value().to_string(),
            artist_id: self.artist_id.clone(),
            duration_seconds: self.duration.seconds(),
            duration_minutes: self.duration.minutes() as f64,
            genre: self.genre.value().to_string(),
            listen_count: self.listen_count.value(),
            revenue_generated: self.revenue_generated,
            is_popular: self.is_popular(),
            is_trending: self.is_trending(),
            created_at: self.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongMetadata {
    pub id: SongId,
    pub title: String,
    pub artist_id: ArtistId,
    pub duration_seconds: u32,
    pub duration_minutes: f64,
    pub genre: String,
    pub listen_count: u64,
    pub revenue_generated: f64,
    pub is_popular: bool,
    pub is_trending: bool,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_song() -> Song {
        Song::new(
            SongTitle::new("Test Song".to_string()).unwrap(),
            ArtistId::new(),
            SongDuration::new(180).unwrap(),
            Genre::new("rock".to_string()).unwrap(),
            RoyaltyPercentage::new(70.0).unwrap(),
        )
    }

    #[test]
    fn test_song_creation() {
        let song = create_test_song();
        assert_eq!(song.title().value(), "Test Song");
        assert_eq!(song.listen_count().value(), 0);
        assert!(!song.is_available_for_campaign());
        assert!(!song.is_available_for_ownership());
    }

    #[test]
    fn test_record_listen_valid_duration() {
        let mut song = create_test_song();
        let listener_id = Uuid::new_v4();
        
        let result = song.record_listen(listener_id, 90); // 90 seconds = 50% of 180s song
        assert!(result.is_ok());
        assert_eq!(song.listen_count().value(), 1);
    }

    #[test]
    fn test_record_listen_invalid_duration() {
        let mut song = create_test_song();
        let listener_id = Uuid::new_v4();
        
        let result = song.record_listen(listener_id, 10); // Too short
        assert!(result.is_err());
        assert_eq!(song.listen_count().value(), 0);
    }

    #[test]
    fn test_campaign_eligibility() {
        let mut song = create_test_song();
        assert!(!song.can_create_campaign());
        
        // Simulate 100 listens
        for _ in 0..100 {
            let _ = song.record_listen(Uuid::new_v4(), 90);
        }
        
        assert!(song.can_create_campaign());
    }

    #[test]
    fn test_ownership_eligibility() {
        let mut song = create_test_song();
        assert!(!song.can_create_ownership_contract());
        
        let _ = song.add_revenue(1500.0);
        assert!(song.can_create_ownership_contract());
    }

    #[test]
    fn test_revenue_calculation() {
        let song = create_test_song();
        let artist_revenue = song.calculate_artist_revenue(1000.0);
        assert_eq!(artist_revenue, 700.0); // 70% royalty
    }

    #[test]
    fn test_title_update_restrictions() {
        let mut song = create_test_song();
        
        // Should work with few listens
        let new_title = SongTitle::new("New Title".to_string()).unwrap();
        assert!(song.update_title(new_title).is_ok());
        
        // Simulate many listens
        for _ in 0..1001 {
            let _ = song.record_listen(Uuid::new_v4(), 90);
        }
        
        // Should fail with many listens
        let another_title = SongTitle::new("Another Title".to_string()).unwrap();
        assert!(song.update_title(another_title).is_err());
    }
} 