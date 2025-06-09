pub mod user;
pub mod artist;
pub mod song;
pub mod playlist;
pub mod nft;
pub mod royalty;
pub mod auth;

// Re-export common types
pub use user::UserService;
pub use artist::ArtistService;
pub use song::SongService;
pub use playlist::PlaylistService;
pub use nft::NftService;
pub use royalty::RoyaltyService;
pub use auth::AuthService; 

use crate::{
    models::{User, Song},
    error::AppError,
};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub struct UserService {
    db: DatabaseConnection,
}

impl UserService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_user(&self, id: Uuid) -> Result<User, AppError> {
        // TODO: Implementar
        Err(AppError::NotImplemented)
    }
}

pub struct SongService {
    db: DatabaseConnection,
}

impl SongService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_song(&self, id: Uuid) -> Result<Song, AppError> {
        // TODO: Implementar
        Err(AppError::NotImplemented)
    }
} 