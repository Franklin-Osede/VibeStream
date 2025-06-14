use serde::{Deserialize, Serialize};

pub mod blockchain;
pub mod messages;
pub mod errors;
pub mod models;

// Re-exports principales
pub use blockchain::*;
pub use messages::*;
pub use errors::*;
pub use models::{
    User, CreateUser, LoginRequest, LoginResponse, UserRole,
    Artist, CreateArtist,
    Song, CreateSong,
    Playlist, CreatePlaylist, PlaylistSong,
    TransactionRecord, TransactionType,
    ListenEvent, CreateListenEvent
};

// Re-export commonly used external types
pub use uuid::Uuid;
pub use chrono::{DateTime, Utc};
pub use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestId(pub Uuid);

impl RequestId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for RequestId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timestamp(pub DateTime<Utc>);

impl Timestamp {
    pub fn now() -> Self {
        Self(Utc::now())
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
} 