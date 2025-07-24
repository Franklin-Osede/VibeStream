use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SongId(Uuid);

impl SongId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }

    pub fn value(&self) -> &Uuid {
        &self.0
    }

    pub fn to_uuid(&self) -> Uuid {
        self.0
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl fmt::Display for SongId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for SongId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArtistId(Uuid);

impl ArtistId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }

    pub fn value(&self) -> &Uuid {
        &self.0
    }

    pub fn to_uuid(&self) -> Uuid {
        self.0
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl fmt::Display for ArtistId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for ArtistId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AlbumId(Uuid);

impl AlbumId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn value(&self) -> &Uuid {
        &self.0
    }

    pub fn to_uuid(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for AlbumId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlaylistId(Uuid);

impl PlaylistId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn value(&self) -> &Uuid {
        &self.0
    }

    pub fn to_uuid(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for PlaylistId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SongTitle(String);

impl SongTitle {
    pub fn new(title: String) -> Result<Self, String> {
        let title = title.trim().to_string();
        
        if title.is_empty() {
            return Err("Song title cannot be empty".to_string());
        }
        
        if title.len() > 200 {
            return Err("Song title cannot exceed 200 characters".to_string());
        }
        
        // Check for inappropriate content (basic filter)
        let forbidden_words = ["explicit", "nsfw", "inappropriate"]; // Simplified list
        let lower_title = title.to_lowercase();
        for word in forbidden_words {
            if lower_title.contains(word) {
                return Err("Song title contains inappropriate content".to_string());
            }
        }
        
        Ok(Self(title))
    }
    
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SongTitle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlbumTitle(String);

impl AlbumTitle {
    pub fn new(title: String) -> Result<Self, String> {
        let title = title.trim().to_string();
        
        if title.is_empty() {
            return Err("Album title cannot be empty".to_string());
        }
        
        if title.len() > 200 {
            return Err("Album title cannot exceed 200 characters".to_string());
        }
        
        Ok(Self(title))
    }
    
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for AlbumTitle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaylistName(String);

impl PlaylistName {
    pub fn new(name: String) -> Result<Self, String> {
        let name = name.trim().to_string();
        
        if name.is_empty() {
            return Err("Playlist name cannot be empty".to_string());
        }
        
        if name.len() > 100 {
            return Err("Playlist name cannot exceed 100 characters".to_string());
        }
        
        Ok(Self(name))
    }
    
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SongDuration {
    seconds: u32,
}

impl SongDuration {
    pub fn new(seconds: u32) -> Result<Self, String> {
        if seconds == 0 {
            return Err("Song duration cannot be zero".to_string());
        }
        
        if seconds > 3600 { // Max 1 hour
            return Err("Song duration cannot exceed 1 hour".to_string());
        }
        
        Ok(Self { seconds })
    }
    
    pub fn from_minutes_seconds(minutes: u32, seconds: u32) -> Result<Self, String> {
        if seconds >= 60 {
            return Err("Seconds must be less than 60".to_string());
        }
        
        let total_seconds = minutes * 60 + seconds;
        Self::new(total_seconds)
    }
    
    pub fn seconds(&self) -> u32 {
        self.seconds
    }
    
    pub fn minutes(&self) -> u32 {
        self.seconds / 60
    }
    
    pub fn remaining_seconds(&self) -> u32 {
        self.seconds % 60
    }
    
    pub fn as_formatted_string(&self) -> String {
        let minutes = self.minutes();
        let seconds = self.remaining_seconds();
        format!("{}:{:02}", minutes, seconds)
    }
}

impl fmt::Display for SongDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_formatted_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListenCount {
    count: u64,
}

impl ListenCount {
    pub fn new() -> Self {
        Self { count: 0 }
    }
    
    pub fn from_value(count: u64) -> Self {
        Self { count }
    }
    
    pub fn increment(&mut self) {
        self.count = self.count.saturating_add(1);
    }
    
    pub fn add(&mut self, amount: u64) {
        self.count = self.count.saturating_add(amount);
    }
    
    pub fn value(&self) -> u64 {
        self.count
    }

    pub fn count(&self) -> u64 {
        self.count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Genre(String);

impl Genre {
    pub fn new(genre: String) -> Result<Self, String> {
        let genre = genre.trim().to_lowercase();
        if genre.is_empty() {
            return Err("Genre cannot be empty".to_string());
        }
        
        // Expanded genre validation
        let valid_genres = [
            "rock", "pop", "jazz", "classical", "electronic", "hip-hop", 
            "reggae", "country", "blues", "folk", "alternative", "indie",
            "metal", "punk", "funk", "soul", "r&b", "latin", "world",
            "edm", "house", "techno", "ambient", "experimental", "gospel",
            "ska", "reggaeton", "trap", "drill", "afrobeat", "kpop",
            "jpop", "bossa nova", "tango", "flamenco", "celtic", "bluegrass"
        ];
        
        if !valid_genres.contains(&genre.as_str()) {
            return Err(format!("Invalid genre: {}. Supported genres: {:?}", genre, valid_genres));
        }
        
        Ok(Self(genre))
    }
    
    pub fn value(&self) -> &str {
        &self.0
    }
    
    pub fn all_valid_genres() -> Vec<&'static str> {
        vec![
            "rock", "pop", "jazz", "classical", "electronic", "hip-hop", 
            "reggae", "country", "blues", "folk", "alternative", "indie",
            "metal", "punk", "funk", "soul", "r&b", "latin", "world",
            "edm", "house", "techno", "ambient", "experimental", "gospel",
            "ska", "reggaeton", "trap", "drill", "afrobeat", "kpop",
            "jpop", "bossa nova", "tango", "flamenco", "celtic", "bluegrass"
        ]
    }
}

impl fmt::Display for Genre {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpfsHash(String);

impl IpfsHash {
    pub fn new(hash: String) -> Result<Self, String> {
        if hash.trim().is_empty() {
            return Err("IPFS hash cannot be empty".to_string());
        }
        
        // IPFS hash validation (CIDv0 and CIDv1 support)
        if hash.starts_with("Qm") && hash.len() == 46 {
            // CIDv0 format
            if !hash.chars().all(|c| c.is_ascii_alphanumeric()) {
                return Err("Invalid IPFS hash format".to_string());
            }
        } else if hash.starts_with("bafy") || hash.starts_with("bafk") {
            // CIDv1 format
            if hash.len() < 50 {
                return Err("Invalid IPFS CIDv1 hash format".to_string());
            }
        } else {
            return Err("Invalid IPFS hash format. Must be CIDv0 (Qm...) or CIDv1 (bafy... or bafk...)".to_string());
        }
        
        Ok(Self(hash))
    }
    
    pub fn value(&self) -> &str {
        &self.0
    }
    
    pub fn is_cidv0(&self) -> bool {
        self.0.starts_with("Qm")
    }
    
    pub fn is_cidv1(&self) -> bool {
        self.0.starts_with("bafy") || self.0.starts_with("bafk")
    }
}

impl fmt::Display for IpfsHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoyaltyPercentage {
    percentage: f64,
}

impl RoyaltyPercentage {
    pub fn new(percentage: f64) -> Result<Self, String> {
        if percentage < 0.0 || percentage > 100.0 {
            return Err("Royalty percentage must be between 0 and 100".to_string());
        }
        
        Ok(Self { percentage })
    }
    
    pub fn value(&self) -> f64 {
        self.percentage
    }

    pub fn percentage(&self) -> f64 {
        self.percentage
    }
    
    pub fn as_decimal(&self) -> f64 {
        self.percentage / 100.0
    }
}

impl fmt::Display for RoyaltyPercentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.percentage)
    }
}

/// Audio quality levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioQuality {
    Low,      // 128kbps
    Medium,   // 256kbps  
    High,     // 320kbps
    Lossless, // FLAC/WAV
}

impl AudioQuality {
    pub fn bitrate(&self) -> u32 {
        match self {
            Self::Low => 128,
            Self::Medium => 256,
            Self::High => 320,
            Self::Lossless => 1411, // CD quality
        }
    }
    
    pub fn from_bitrate(bitrate: u32) -> Self {
        match bitrate {
            0..=128 => Self::Low,
            129..=256 => Self::Medium,
            257..=320 => Self::High,
            _ => Self::Lossless,
        }
    }
}

impl fmt::Display for AudioQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => write!(f, "Low (128kbps)"),
            Self::Medium => write!(f, "Medium (256kbps)"),
            Self::High => write!(f, "High (320kbps)"),
            Self::Lossless => write!(f, "Lossless (1411kbps)"),
        }
    }
}

/// Song file format
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileFormat {
    Mp3,
    Flac,
    Wav,
    Aac,
    Ogg,
    M4a,
}

impl FileFormat {
    pub fn from_extension(ext: &str) -> Result<Self, String> {
        match ext.to_lowercase().as_str() {
            "mp3" => Ok(Self::Mp3),
            "flac" => Ok(Self::Flac),
            "wav" => Ok(Self::Wav),
            "aac" => Ok(Self::Aac),
            "ogg" => Ok(Self::Ogg),
            "m4a" => Ok(Self::M4a),
            _ => Err(format!("Unsupported file format: {}", ext)),
        }
    }
    
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Mp3 => "mp3",
            Self::Flac => "flac",
            Self::Wav => "wav",
            Self::Aac => "aac",
            Self::Ogg => "ogg",
            Self::M4a => "m4a",
        }
    }
    
    pub fn is_lossless(&self) -> bool {
        matches!(self, Self::Flac | Self::Wav)
    }
    
    pub fn supported_formats() -> Vec<&'static str> {
        vec!["mp3", "flac", "wav", "aac", "ogg", "m4a"]
    }
}

impl fmt::Display for FileFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.extension().to_uppercase())
    }
}

/// Song mood/emotion classification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SongMood {
    Happy,
    Sad,
    Energetic,
    Calm,
    Romantic,
    Aggressive,
    Melancholic,
    Uplifting,
    Dark,
    Nostalgic,
    Triumphant,
    Mysterious,
}

impl SongMood {
    pub fn all_moods() -> Vec<Self> {
        vec![
            Self::Happy, Self::Sad, Self::Energetic, Self::Calm,
            Self::Romantic, Self::Aggressive, Self::Melancholic, Self::Uplifting,
            Self::Dark, Self::Nostalgic, Self::Triumphant, Self::Mysterious,
        ]
    }
    
    pub fn from_string(mood: &str) -> Result<Self, String> {
        match mood.to_lowercase().as_str() {
            "happy" => Ok(Self::Happy),
            "sad" => Ok(Self::Sad),
            "energetic" => Ok(Self::Energetic),
            "calm" => Ok(Self::Calm),
            "romantic" => Ok(Self::Romantic),
            "aggressive" => Ok(Self::Aggressive),
            "melancholic" => Ok(Self::Melancholic),
            "uplifting" => Ok(Self::Uplifting),
            "dark" => Ok(Self::Dark),
            "nostalgic" => Ok(Self::Nostalgic),
            "triumphant" => Ok(Self::Triumphant),
            "mysterious" => Ok(Self::Mysterious),
            _ => Err(format!("Invalid mood: {}", mood)),
        }
    }
}

impl fmt::Display for SongMood {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mood_str = match self {
            Self::Happy => "Happy",
            Self::Sad => "Sad",
            Self::Energetic => "Energetic",
            Self::Calm => "Calm",
            Self::Romantic => "Romantic",
            Self::Aggressive => "Aggressive",
            Self::Melancholic => "Melancholic",
            Self::Uplifting => "Uplifting",
            Self::Dark => "Dark",
            Self::Nostalgic => "Nostalgic",
            Self::Triumphant => "Triumphant",
            Self::Mysterious => "Mysterious",
        };
        write!(f, "{}", mood_str)
    }
}

/// Tempo classification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tempo {
    bpm: u16,
}

impl Tempo {
    pub fn new(bpm: u16) -> Result<Self, String> {
        if bpm < 60 {
            return Err("Tempo cannot be less than 60 BPM".to_string());
        }
        
        if bpm > 200 {
            return Err("Tempo cannot exceed 200 BPM".to_string());
        }
        
        Ok(Self { bpm })
    }
    
    pub fn bpm(&self) -> u16 {
        self.bpm
    }
    
    pub fn classification(&self) -> &'static str {
        match self.bpm {
            60..=90 => "Slow",
            91..=120 => "Moderate",
            121..=140 => "Fast",
            141..=160 => "Very Fast",
            _ => "Extremely Fast",
        }
    }
}

impl fmt::Display for Tempo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} BPM ({})", self.bpm, self.classification())
    }
}

/// Release type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReleaseType {
    Single,
    Album,
    Ep,
    Compilation,
    Soundtrack,
    Live,
    Remix,
}

impl ReleaseType {
    pub fn from_string(release_type: &str) -> Result<Self, String> {
        match release_type.to_lowercase().as_str() {
            "single" => Ok(Self::Single),
            "album" => Ok(Self::Album),
            "ep" => Ok(Self::Ep),
            "compilation" => Ok(Self::Compilation),
            "soundtrack" => Ok(Self::Soundtrack),
            "live" => Ok(Self::Live),
            "remix" => Ok(Self::Remix),
            _ => Err(format!("Invalid release type: {}", release_type)),
        }
    }
}

impl fmt::Display for ReleaseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match self {
            Self::Single => "Single",
            Self::Album => "Album",
            Self::Ep => "EP",
            Self::Compilation => "Compilation",
            Self::Soundtrack => "Soundtrack",
            Self::Live => "Live",
            Self::Remix => "Remix",
        };
        write!(f, "{}", type_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_song_title_validation() {
        assert!(SongTitle::new("Valid Song Title".to_string()).is_ok());
        assert!(SongTitle::new("".to_string()).is_err());
        assert!(SongTitle::new("a".repeat(201)).is_err());
    }

    #[test]
    fn test_song_duration() {
        let duration = SongDuration::from_minutes_seconds(3, 45).unwrap();
        assert_eq!(duration.seconds(), 225);
        assert_eq!(duration.minutes(), 3);
        assert_eq!(duration.remaining_seconds(), 45);
        assert_eq!(duration.as_formatted_string(), "3:45");
    }

    #[test]
    fn test_genre_validation() {
        assert!(Genre::new("rock".to_string()).is_ok());
        assert!(Genre::new("invalid_genre".to_string()).is_err());
    }

    #[test]
    fn test_ipfs_hash_validation() {
        // Valid CIDv0
        assert!(IpfsHash::new("QmYjtig7VJQ6XsnUjqqJvj7QaMcCAwtrgNdahSiFofrE7o".to_string()).is_ok());
        
        // Valid CIDv1
        assert!(IpfsHash::new("bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi".to_string()).is_ok());
        
        // Invalid
        assert!(IpfsHash::new("invalid".to_string()).is_err());
    }

    #[test]
    fn test_royalty_percentage() {
        assert!(RoyaltyPercentage::new(50.0).is_ok());
        assert!(RoyaltyPercentage::new(-10.0).is_err());
        assert!(RoyaltyPercentage::new(150.0).is_err());
        
        let royalty = RoyaltyPercentage::new(25.0).unwrap();
        assert_eq!(royalty.as_decimal(), 0.25);
    }

    #[test]
    fn test_tempo() {
        let tempo = Tempo::new(120).unwrap();
        assert_eq!(tempo.bpm(), 120);
        assert_eq!(tempo.classification(), "Moderate");
        
        assert!(Tempo::new(50).is_err());
        assert!(Tempo::new(250).is_err());
    }

    #[test]
    fn test_file_format() {
        assert_eq!(FileFormat::from_extension("mp3").unwrap(), FileFormat::Mp3);
        assert_eq!(FileFormat::Mp3.extension(), "mp3");
        assert!(!FileFormat::Mp3.is_lossless());
        assert!(FileFormat::Flac.is_lossless());
    }
} 

// All value objects are already available directly in this module 