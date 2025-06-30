use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SongId(Uuid);

impl SongId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }

    pub fn from_string(id_str: &str) -> Result<Self, String> {
        Uuid::parse_str(id_str)
            .map(Self::from_uuid)
            .map_err(|_| "Invalid UUID format".to_string())
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for SongId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SongTitle(String);

impl SongTitle {
    pub fn new(title: String) -> Result<Self, String> {
        if title.trim().is_empty() {
            return Err("Song title cannot be empty".to_string());
        }
        if title.len() > 200 {
            return Err("Song title cannot exceed 200 characters".to_string());
        }
        Ok(Self(title.trim().to_string()))
    }
    
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AlbumId(Uuid);

impl AlbumId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_uuid(id: Uuid) -> Self {
        Self(id)
    }
    
    pub fn value(&self) -> Uuid {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlbumTitle(String);

impl AlbumTitle {
    pub fn new(title: String) -> Result<Self, String> {
        if title.trim().is_empty() {
            return Err("Album title cannot be empty".to_string());
        }
        if title.len() > 200 {
            return Err("Album title cannot exceed 200 characters".to_string());
        }
        Ok(Self(title.trim().to_string()))
    }
    
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArtistId {
    value: Uuid,
}

impl ArtistId {
    pub fn new() -> Self {
        Self {
            value: Uuid::new_v4(),
        }
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self { value: uuid }
    }

    pub fn from_string(s: &str) -> Result<Self, String> {
        let uuid = Uuid::parse_str(s)
            .map_err(|e| format!("Invalid UUID format: {}", e))?;
        Ok(Self::from_uuid(uuid))
    }

    pub fn value(&self) -> Uuid {
        self.value
    }

    pub fn to_string(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SongDuration {
    seconds: u32,
}

impl SongDuration {
    pub fn new(seconds: u32) -> Result<Self, String> {
        if seconds == 0 {
            return Err("Song duration must be greater than 0".to_string());
        }
        if seconds > 3600 { // Max 1 hour
            return Err("Song duration cannot exceed 1 hour".to_string());
        }
        Ok(Self { seconds })
    }
    
    pub fn seconds(&self) -> u32 {
        self.seconds
    }
    
    pub fn minutes(&self) -> f64 {
        self.seconds as f64 / 60.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
        self.count += 1;
    }
    
    pub fn value(&self) -> u64 {
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
        
        // Validate against known genres
        let valid_genres = [
            "rock", "pop", "jazz", "classical", "electronic", "hip-hop", 
            "reggae", "country", "blues", "folk", "alternative", "indie",
            "metal", "punk", "funk", "soul", "r&b", "latin", "world"
        ];
        
        if !valid_genres.contains(&genre.as_str()) {
            return Err(format!("Invalid genre: {}", genre));
        }
        
        Ok(Self(genre))
    }
    
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpfsHash(String);

impl IpfsHash {
    pub fn new(hash: String) -> Result<Self, String> {
        if hash.trim().is_empty() {
            return Err("IPFS hash cannot be empty".to_string());
        }
        
        // Basic IPFS hash validation (starts with Qm and has proper length)
        if !hash.starts_with("Qm") || hash.len() != 46 {
            return Err("Invalid IPFS hash format".to_string());
        }
        
        Ok(Self(hash))
    }
    
    pub fn value(&self) -> &str {
        &self.0
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
    
    pub fn percentage(&self) -> f64 {
        self.percentage
    }
    
    pub fn value(&self) -> f64 {
        self.percentage
    }
    
    pub fn as_decimal(&self) -> f64 {
        self.percentage / 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_song_title_validation() {
        assert!(SongTitle::new("Valid Title".to_string()).is_ok());
        assert!(SongTitle::new("".to_string()).is_err());
        assert!(SongTitle::new("a".repeat(201)).is_err());
    }

    #[test]
    fn test_song_duration_validation() {
        assert!(SongDuration::new(180).is_ok());
        assert!(SongDuration::new(0).is_err());
        assert!(SongDuration::new(3700).is_err());
    }

    #[test]
    fn test_genre_validation() {
        assert!(Genre::new("rock".to_string()).is_ok());
        assert!(Genre::new("ROCK".to_string()).is_ok()); // Case insensitive
        assert!(Genre::new("invalid_genre".to_string()).is_err());
    }

    #[test]
    fn test_royalty_percentage_validation() {
        assert!(RoyaltyPercentage::new(50.0).is_ok());
        assert!(RoyaltyPercentage::new(-1.0).is_err());
        assert!(RoyaltyPercentage::new(101.0).is_err());
    }

    #[test]
    fn test_listen_count() {
        let mut count = ListenCount::new();
        assert_eq!(count.value(), 0);
        count.increment();
        assert_eq!(count.value(), 1);
    }
} 