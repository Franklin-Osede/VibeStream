use sea_orm::{DatabaseConnection, DbErr, ActiveValue::Set};
use uuid::Uuid;
use crate::{
    db::models::playlist::{self, Model as Playlist, ActiveModel},
    error::AppError,
    repositories::playlist::PlaylistRepository,
};

pub struct PlaylistService {
    playlist_repository: Box<dyn PlaylistRepository>,
}

impl PlaylistService {
    pub fn new(playlist_repository: Box<dyn PlaylistRepository>) -> Self {
        Self { playlist_repository }
    }

    pub async fn create_playlist(
        &self,
        db: &DatabaseConnection,
        name: String,
        user_id: Uuid,
        description: Option<String>,
        is_public: bool,
    ) -> Result<Playlist, AppError> {
        let playlist = ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(name),
            user_id: Set(user_id),
            description: Set(description),
            is_public: Set(is_public),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
        };

        self.playlist_repository
            .create(db, playlist)
            .await
            .map_err(AppError::DatabaseError)
    }

    pub async fn get_playlist(
        &self,
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<Playlist>, AppError> {
        self.playlist_repository
            .find_by_id(db, id)
            .await
            .map_err(AppError::DatabaseError)
    }

    pub async fn get_user_playlists(
        &self,
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<Vec<Playlist>, AppError> {
        self.playlist_repository
            .find_by_user(db, user_id)
            .await
            .map_err(AppError::DatabaseError)
    }

    pub async fn add_song(
        &self,
        db: &DatabaseConnection,
        playlist_id: Uuid,
        song_id: Uuid,
    ) -> Result<(), AppError> {
        self.playlist_repository
            .add_song(db, playlist_id, song_id)
            .await
            .map_err(AppError::DatabaseError)
    }

    pub async fn remove_song(
        &self,
        db: &DatabaseConnection,
        playlist_id: Uuid,
        song_id: Uuid,
    ) -> Result<(), AppError> {
        self.playlist_repository
            .remove_song(db, playlist_id, song_id)
            .await
            .map_err(AppError::DatabaseError)
    }

    pub async fn delete_playlist(
        &self,
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<(), AppError> {
        self.playlist_repository
            .delete(db, id)
            .await
            .map_err(AppError::DatabaseError)?;
        Ok(())
    }
} 