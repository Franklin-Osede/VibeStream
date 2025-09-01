use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;

use crate::bounded_contexts::music::domain::{
    repositories::{playlist_repository::{Playlist, PlaylistRepository as DomainPlaylistRepository}},
    value_objects::{PlaylistId, PlaylistName},
};
use crate::bounded_contexts::user::domain::UserId;
use crate::shared::domain::errors::AppError;

// Internal struct for database mapping
#[derive(FromRow)]
struct PlaylistRow {
    id: Uuid,
    name: String,
    description: Option<String>,
    is_public: bool,
    song_count: i32,
    created_by: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct PostgresPlaylistRepository {
    pool: PgPool,
}

impl PostgresPlaylistRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn row_to_playlist(&self, row: PlaylistRow) -> Result<Playlist, AppError> {
        Ok(Playlist::new(
            row.id,
            row.name,
            row.description,
            row.is_public,
            row.created_by,
        ))
    }
}

#[async_trait]
impl DomainPlaylistRepository for PostgresPlaylistRepository {
    async fn save(&self, playlist: &Playlist) -> Result<(), AppError> {
        sqlx::query(
            r#"INSERT INTO playlists (id, name, description, is_public, song_count, created_by, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
               ON CONFLICT (id) DO UPDATE SET
               name = EXCLUDED.name, description = EXCLUDED.description, is_public = EXCLUDED.is_public, updated_at = EXCLUDED.updated_at"#
        )
        .bind(playlist.id)
        .bind(&playlist.name)
        .bind(&playlist.description)
        .bind(playlist.is_public)
        .bind(playlist.song_count as i32)
        .bind(playlist.created_by)
        .bind(playlist.created_at)
        .bind(playlist.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Playlist>, AppError> {
        let row: Option<PlaylistRow> = sqlx::query_as(
            "SELECT id, name, description, is_public, song_count, created_by, created_at, updated_at FROM playlists WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if let Some(row) = row {
            Ok(Some(self.row_to_playlist(row)?))
        } else {
            Ok(None)
        }
    }

    async fn find_by_creator(&self, creator_id: &Uuid) -> Result<Vec<Playlist>, AppError> {
        let rows: Vec<PlaylistRow> = sqlx::query_as(
            "SELECT id, name, description, is_public, song_count, created_by, created_at, updated_at FROM playlists WHERE created_by = $1"
        )
        .bind(creator_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let playlists: Result<Vec<Playlist>, AppError> = rows.into_iter()
            .map(|row| self.row_to_playlist(row))
            .collect();
        
        playlists
    }

    async fn find_public_playlists(&self, page: u32, page_size: u32) -> Result<Vec<Playlist>, AppError> {
        let offset = (page - 1) * page_size;
        let rows: Vec<PlaylistRow> = sqlx::query_as(
            "SELECT id, name, description, is_public, song_count, created_by, created_at, updated_at FROM playlists WHERE is_public = true ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let playlists: Result<Vec<Playlist>, AppError> = rows.into_iter()
            .map(|row| self.row_to_playlist(row))
            .collect();
        
        playlists
    }

    async fn find_all(&self, page: u32, page_size: u32) -> Result<Vec<Playlist>, AppError> {
        let offset = (page - 1) * page_size;
        let rows: Vec<PlaylistRow> = sqlx::query_as(
            "SELECT id, name, description, is_public, song_count, created_by, created_at, updated_at FROM playlists ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let playlists: Result<Vec<Playlist>, AppError> = rows.into_iter()
            .map(|row| self.row_to_playlist(row))
            .collect();
        
        playlists
    }

    async fn update(&self, playlist: &Playlist) -> Result<(), AppError> {
        sqlx::query(
            r#"UPDATE playlists SET name = $2, description = $3, is_public = $4, song_count = $5, updated_at = $6 WHERE id = $1"#
        )
        .bind(playlist.id)
        .bind(&playlist.name)
        .bind(&playlist.description)
        .bind(playlist.is_public)
        .bind(playlist.song_count as i32)
        .bind(playlist.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM playlists WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }

    async fn count(&self) -> Result<u64, AppError> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM playlists")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(count as u64)
    }

    async fn search_by_name(&self, name: &str) -> Result<Vec<Playlist>, AppError> {
        let rows: Vec<PlaylistRow> = sqlx::query_as(
            "SELECT id, name, description, is_public, song_count, created_by, created_at, updated_at FROM playlists WHERE name ILIKE $1"
        )
        .bind(format!("%{}%", name))
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let playlists: Result<Vec<Playlist>, AppError> = rows.into_iter()
            .map(|row| self.row_to_playlist(row))
            .collect();
        
        playlists
    }

    async fn add_song(&self, playlist_id: &Uuid, song_id: &Uuid) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Add song to playlist
        sqlx::query(
            "INSERT INTO playlist_songs (playlist_id, song_id, added_at) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING"
        )
        .bind(playlist_id)
        .bind(song_id)
        .bind(Utc::now())
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Update song count
        sqlx::query(
            "UPDATE playlists SET song_count = song_count + 1, updated_at = $2 WHERE id = $1"
        )
        .bind(playlist_id)
        .bind(Utc::now())
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tx.commit().await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }

    async fn remove_song(&self, playlist_id: &Uuid, song_id: &Uuid) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Remove song from playlist
        sqlx::query(
            "DELETE FROM playlist_songs WHERE playlist_id = $1 AND song_id = $2"
        )
        .bind(playlist_id)
        .bind(song_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Update song count
        sqlx::query(
            "UPDATE playlists SET song_count = GREATEST(song_count - 1, 0), updated_at = $2 WHERE id = $1"
        )
        .bind(playlist_id)
        .bind(Utc::now())
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        tx.commit().await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }

    async fn get_songs(&self, playlist_id: &Uuid) -> Result<Vec<Uuid>, AppError> {
        let rows: Vec<(Uuid,)> = sqlx::query_as(
            "SELECT song_id FROM playlist_songs WHERE playlist_id = $1 ORDER BY added_at ASC"
        )
        .bind(playlist_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let song_ids: Vec<Uuid> = rows.into_iter().map(|(song_id,)| song_id).collect();
        Ok(song_ids)
    }
} 