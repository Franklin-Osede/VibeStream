// Genre Statistics Entity
// This module contains the genre statistics entity for tracking popularity and trends

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::bounded_contexts::music::domain::value_objects::Genre;
use std::collections::HashMap;

/// Statistics for music genres
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenreStats {
    pub genre: Genre,
    pub total_songs: u64,
    pub total_albums: u64,
    pub total_artists: u64,
    pub total_listens: u64,
    pub streaming_hours: u64,
    pub popularity_score: f64,
    pub trend_score: f64,
    pub monthly_growth: f64,
    pub peak_popularity_date: Option<DateTime<Utc>>,
    pub peak_popularity_score: f64,
    pub geographic_distribution: HashMap<String, f64>, // Country -> percentage
    pub age_group_distribution: HashMap<String, f64>, // Age group -> percentage
    pub top_artists: Vec<TopArtistInGenre>,
    pub top_songs: Vec<TopSongInGenre>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Top artist in a specific genre
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopArtistInGenre {
    pub artist_id: uuid::Uuid,
    pub artist_name: String,
    pub listens_in_genre: u64,
    pub songs_in_genre: u32,
    pub rank: u32,
}

/// Top song in a specific genre
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopSongInGenre {
    pub song_id: uuid::Uuid,
    pub song_title: String,
    pub artist_name: String,
    pub listens: u64,
    pub rank: u32,
}

/// Time-based genre analytics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenreTrendAnalytics {
    pub genre: Genre,
    pub daily_listens: Vec<DailyGenreStats>,
    pub weekly_listens: Vec<WeeklyGenreStats>,
    pub monthly_listens: Vec<MonthlyGenreStats>,
    pub seasonal_patterns: SeasonalPatterns,
}

/// Daily genre statistics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DailyGenreStats {
    pub date: chrono::NaiveDate,
    pub listens: u64,
    pub unique_listeners: u64,
    pub new_songs_added: u32,
    pub engagement_score: f64,
}

/// Weekly genre statistics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeeklyGenreStats {
    pub week_start: chrono::NaiveDate,
    pub listens: u64,
    pub unique_listeners: u64,
    pub new_songs_added: u32,
    pub growth_rate: f64,
}

/// Monthly genre statistics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonthlyGenreStats {
    pub month: u32,
    pub year: u32,
    pub listens: u64,
    pub unique_listeners: u64,
    pub new_artists: u32,
    pub new_songs: u32,
    pub revenue_generated: f64,
}

/// Seasonal listening patterns
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeasonalPatterns {
    pub spring_preference: f64,
    pub summer_preference: f64,
    pub autumn_preference: f64,
    pub winter_preference: f64,
    pub holiday_spikes: Vec<HolidaySpike>,
}

/// Holiday listening spikes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HolidaySpike {
    pub holiday_name: String,
    pub spike_percentage: f64,
    pub typical_dates: Vec<chrono::NaiveDate>,
}

impl GenreStats {
    /// Create new genre statistics
    pub fn new(genre: Genre) -> Self {
        let now = Utc::now();
        
        Self {
            genre,
            total_songs: 0,
            total_albums: 0,
            total_artists: 0,
            total_listens: 0,
            streaming_hours: 0,
            popularity_score: 0.0,
            trend_score: 0.0,
            monthly_growth: 0.0,
            peak_popularity_date: None,
            peak_popularity_score: 0.0,
            geographic_distribution: HashMap::new(),
            age_group_distribution: HashMap::new(),
            top_artists: Vec::new(),
            top_songs: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Update statistics with new listen data
    pub fn update_listen_stats(&mut self, listens: u64, streaming_minutes: u64) {
        self.total_listens += listens;
        self.streaming_hours += streaming_minutes / 60;
        self.updated_at = Utc::now();
    }

    /// Add new song to genre
    pub fn add_song(&mut self) {
        self.total_songs += 1;
        self.updated_at = Utc::now();
    }

    /// Add new album to genre
    pub fn add_album(&mut self) {
        self.total_albums += 1;
        self.updated_at = Utc::now();
    }

    /// Add new artist to genre
    pub fn add_artist(&mut self) {
        self.total_artists += 1;
        self.updated_at = Utc::now();
    }

    /// Update popularity score
    pub fn update_popularity_score(&mut self, score: f64) {
        if score > self.peak_popularity_score {
            self.peak_popularity_score = score;
            self.peak_popularity_date = Some(Utc::now());
        }
        self.popularity_score = score;
        self.updated_at = Utc::now();
    }

    /// Update trend score
    pub fn update_trend_score(&mut self, score: f64) {
        self.trend_score = score;
        self.updated_at = Utc::now();
    }

    /// Update monthly growth
    pub fn update_monthly_growth(&mut self, growth: f64) {
        self.monthly_growth = growth;
        self.updated_at = Utc::now();
    }

    /// Check if genre is trending
    pub fn is_trending(&self) -> bool {
        self.trend_score > 70.0 && self.monthly_growth > 10.0
    }

    /// Check if genre is popular
    pub fn is_popular(&self) -> bool {
        self.popularity_score > 80.0
    }

    /// Get genre category based on stats
    pub fn get_genre_category(&self) -> GenreCategory {
        match self.popularity_score {
            score if score >= 90.0 => GenreCategory::Mainstream,
            score if score >= 70.0 => GenreCategory::Popular,
            score if score >= 50.0 => GenreCategory::Growing,
            score if score >= 30.0 => GenreCategory::Niche,
            _ => GenreCategory::Emerging,
        }
    }
}

/// Genre popularity categories
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GenreCategory {
    Mainstream,  // 90%+ popularity
    Popular,     // 70-89% popularity
    Growing,     // 50-69% popularity
    Niche,       // 30-49% popularity
    Emerging,    // <30% popularity
}

impl Default for GenreStats {
    fn default() -> Self {
        Self::new(Genre::new("Pop".to_string()).expect("Pop should be a valid genre"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genre_stats_creation() {
        let stats = GenreStats::new(Genre::Rock);
        assert_eq!(stats.genre, Genre::Rock);
        assert_eq!(stats.total_songs, 0);
        assert_eq!(stats.total_listens, 0);
    }

    #[test]
    fn test_popularity_score_update() {
        let mut stats = GenreStats::new(Genre::Jazz);
        stats.update_popularity_score(85.5);
        assert_eq!(stats.popularity_score, 85.5);
        assert_eq!(stats.peak_popularity_score, 85.5);
        assert!(stats.peak_popularity_date.is_some());
    }

    #[test]
    fn test_genre_category() {
        let mut stats = GenreStats::new(Genre::Classical);
        
        stats.update_popularity_score(95.0);
        assert_eq!(stats.get_genre_category(), GenreCategory::Mainstream);
        
        stats.update_popularity_score(75.0);
        assert_eq!(stats.get_genre_category(), GenreCategory::Popular);
        
        stats.update_popularity_score(55.0);
        assert_eq!(stats.get_genre_category(), GenreCategory::Growing);
    }

    #[test]
    fn test_trending_check() {
        let mut stats = GenreStats::new(Genre::Electronic);
        
        stats.update_trend_score(75.0);
        stats.update_monthly_growth(15.0);
        assert!(stats.is_trending());
        
        stats.update_trend_score(65.0);
        assert!(!stats.is_trending());
    }
} 