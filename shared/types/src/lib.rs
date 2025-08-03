use serde::{Deserialize, Serialize};
use async_trait::async_trait;

pub mod blockchain;
pub mod messages;
pub mod errors;
pub mod models;
pub mod contracts;
pub mod integration_events;

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

// Re-exports de contratos compartidos
pub use contracts::*;
pub use integration_events::*;

// Re-export commonly used external types
pub use uuid::Uuid;
pub use chrono::{DateTime, Utc};
pub use rust_decimal::Decimal;

// CQRS Command Pattern
/// Representa un comando que muta el estado del sistema.
pub trait Command: Send + Sync {}

/// Un manejador de comando.
#[async_trait]
pub trait CommandHandler<C: Command>: Send + Sync {
    type Output: Send + 'static;
    type Error: Send + 'static;

    async fn handle(&self, command: C) -> std::result::Result<Self::Output, Self::Error>;
}

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