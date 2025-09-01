use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;

use crate::bounded_contexts::music::domain::repositories::album_repository::{Album as RepoAlbum, AlbumRepository as DomainAlbumRepository};
use crate::shared::domain::errors::AppError;

// Internal struct for database mapping
#[derive(FromRow)]
struct AlbumRow {
    id: Uuid,
    title: String,
    artist_id: Uuid,
    genre: String,
    is_published: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct PostgresAlbumRepository {
    pool: PgPool,
}

impl PostgresAlbumRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn row_to_album(&self, row: AlbumRow) -> Result<RepoAlbum, AppError> {
        let album = RepoAlbum {
            id: row.id,
            title: row.title,
            artist_id: row.artist_id,
            description: None,
            release_date: None,
            song_count: 0,
            created_at: row.created_at,
            updated_at: row.updated_at,
        };
        Ok(album)
    }
}

#[async_trait]
impl DomainAlbumRepository for PostgresAlbumRepository {
    async fn save(&self, album: &RepoAlbum) -> Result<(), AppError> {
        sqlx::query(
            r#"INSERT INTO albums (id, title, artist_id, genre, is_published, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7)
               ON CONFLICT (id) DO UPDATE SET
               title = EXCLUDED.title, updated_at = EXCLUDED.updated_at"#
        )
        .bind(album.id)
        .bind(&album.title)
        .bind(album.artist_id)
        .bind("unknown")
        .bind(true)
        .bind(album.created_at)
        .bind(album.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<RepoAlbum>, AppError> {
        let row: Option<AlbumRow> = sqlx::query_as(
            "SELECT id, title, artist_id, genre, is_published, created_at, updated_at FROM albums WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if let Some(row) = row {
            Ok(Some(self.row_to_album(row)?))
        } else {
            Ok(None)
        }
    }

    async fn find_by_artist_id(&self, artist_id: &Uuid) -> Result<Vec<RepoAlbum>, AppError> {
        let rows: Vec<AlbumRow> = sqlx::query_as(
            "SELECT id, title, artist_id, genre, is_published, created_at, updated_at FROM albums WHERE artist_id = $1"
        )
        .bind(artist_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut albums: Vec<RepoAlbum> = Vec::new();
        for row in rows {
            albums.push(self.row_to_album(row)?);
        }
        Ok(albums)
    }

    async fn find_all(&self, page: u32, page_size: u32) -> Result<Vec<RepoAlbum>, AppError> {
        let offset = (page - 1) * page_size;
        let rows: Vec<AlbumRow> = sqlx::query_as(
            "SELECT id, title, artist_id, genre, is_published, created_at, updated_at FROM albums ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut albums: Vec<RepoAlbum> = Vec::new();
        for row in rows {
            albums.push(self.row_to_album(row)?);
        }
        Ok(albums)
    }

    async fn update(&self, album: &RepoAlbum) -> Result<(), AppError> {
        sqlx::query(
            r#"UPDATE albums SET title = $2, artist_id = $3, genre = $4, is_published = $5, updated_at = $6 WHERE id = $1"#
        )
        .bind(album.id)
        .bind(&album.title)
        .bind(album.artist_id)
        .bind("unknown")
        .bind(true)
        .bind(album.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM albums WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }

    async fn count(&self) -> Result<u64, AppError> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM albums")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(count as u64)
    }

    async fn search_by_title(&self, title: &str) -> Result<Vec<RepoAlbum>, AppError> {
        let rows: Vec<AlbumRow> = sqlx::query_as(
            "SELECT id, title, artist_id, genre, is_published, created_at, updated_at FROM albums WHERE title ILIKE $1"
        )
        .bind(format!("%{}%", title))
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut albums: Vec<RepoAlbum> = Vec::new();
        for row in rows {
            albums.push(self.row_to_album(row)?);
        }
        Ok(albums)
    }
} 