use sea_orm::{DatabaseConnection, DbErr, ActiveValue::Set};
use uuid::Uuid;
use crate::{
    models::song::{self, Model as Song, ActiveModel},
    error::AppError,
    repositories::song::SongRepository,
};

#[derive(Clone)]
pub struct SongService {
    db: DatabaseConnection,
}

impl SongService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create_song(&self, song: Song) -> Result<Song, Box<dyn std::error::Error>> {
        // Implementación pendiente
        Ok(song)
    }

    pub async fn get_song(&self, id: Uuid) -> Result<Option<Song>, Box<dyn std::error::Error>> {
        // Implementación pendiente
        Ok(None)
    }

    pub async fn update_song(&self, song: Song) -> Result<Song, Box<dyn std::error::Error>> {
        // Implementación pendiente
        Ok(song)
    }

    pub async fn delete_song(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        // Implementación pendiente
        Ok(())
    }
} 