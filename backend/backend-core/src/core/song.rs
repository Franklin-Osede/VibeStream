use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub ipfs_hash: String,
    pub duration_seconds: u32,
    pub royalty_rate: f32,  // Percentage of earnings that goes to rights holders
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_song() {
        let song = Song {
            id: Uuid::new_v4(),
            title: "Test Song".to_string(),
            artist_id: Uuid::new_v4(),
            ipfs_hash: "QmTest123".to_string(),
            duration_seconds: 180,
            royalty_rate: 0.7, // 70% goes to rights holders
        };

        assert_eq!(song.title, "Test Song");
        assert!(song.duration_seconds > 0);
        assert!(song.royalty_rate > 0.0 && song.royalty_rate <= 1.0);
    }
}

// Domain logic for songs
impl Song {
    pub fn new(
        title: String,
        artist_id: Uuid,
        ipfs_hash: String,
        duration_seconds: u32,
        royalty_rate: f32,
    ) -> Result<Self, &'static str> {
        // Validate inputs
        if title.is_empty() {
            return Err("Title cannot be empty");
        }
        if duration_seconds == 0 {
            return Err("Duration must be greater than 0");
        }
        if !(0.0..=1.0).contains(&royalty_rate) {
            return Err("Royalty rate must be between 0 and 1");
        }

        Ok(Self {
            id: Uuid::new_v4(),
            title,
            artist_id,
            ipfs_hash,
            duration_seconds,
            royalty_rate,
        })
    }

    pub fn calculate_earnings(&self, stream_count: u32, price_per_stream: f32) -> f32 {
        let total_earnings = stream_count as f32 * price_per_stream;
        total_earnings * self.royalty_rate
    }
} 