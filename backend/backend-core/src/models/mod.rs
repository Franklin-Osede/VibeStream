pub mod user;
pub mod artist;
pub mod song;
pub mod playlist;
pub mod playlist_song;
pub mod contract;
pub mod song_nft;
pub mod royalty_payment;

pub use user::Model as User;
pub use artist::Model as Artist;
pub use song::Model as Song;
pub use playlist::Model as Playlist;
pub use playlist_song::Model as PlaylistSong;
pub use contract::Model as Contract;
pub use song_nft::Model as SongNft;
pub use royalty_payment::Model as RoyaltyPayment;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub wallet_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Song {
    pub id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub duration_seconds: i32,
    pub ipfs_hash: String,
    pub created_at: DateTime<Utc>,
} 