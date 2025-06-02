use sea_orm::{DatabaseConnection, DbErr, ActiveValue::Set};
use uuid::Uuid;
use bytes::Bytes;
use chrono;

use crate::{
    error::AppError,
    db::models::song::{self, Model as Song, ActiveModel},
    repositories::song::SongRepository,
};

pub struct SongService {
    song_repository: Box<dyn SongRepository>,
}

impl SongService {
    pub fn new(song_repository: Box<dyn SongRepository>) -> Self {
        Self { song_repository }
    }

    pub async fn create_song(
        &self,
        db: &DatabaseConnection,
        title: String,
        artist_id: Uuid,
        duration: i32,
        genre: Option<String>,
        file_data: Bytes,
    ) -> Result<Song, AppError> {
        // TODO: Upload file to storage service
        let file_url = "temporary_url".to_string();

        let song = ActiveModel {
            id: Set(Uuid::new_v4()),
            title: Set(title),
            artist_id: Set(artist_id),
            duration: Set(duration),
            genre: Set(genre),
            file_url: Set(file_url),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
        };

        self.song_repository
            .create(db, song)
            .await
            .map_err(AppError::DatabaseError)
    }

    pub async fn get_song(
        &self,
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<Song>, AppError> {
        self.song_repository
            .find_by_id(db, id)
            .await
            .map_err(AppError::DatabaseError)
    }

    pub async fn get_songs_by_artist(
        &self,
        db: &DatabaseConnection,
        artist_id: Uuid,
    ) -> Result<Vec<Song>, AppError> {
        self.song_repository
            .find_by_artist(db, artist_id)
            .await
            .map_err(AppError::DatabaseError)
    }

    pub async fn delete_song(
        &self,
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<(), AppError> {
        // TODO: Delete file from storage service
        self.song_repository
            .delete(db, id)
            .await
            .map_err(AppError::DatabaseError)?;
        Ok(())
    }
} 