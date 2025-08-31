use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;

use crate::bounded_contexts::music::domain::{
    entities::Album,
    value_objects::{AlbumId, AlbumTitle, ArtistId, Genre, ReleaseType},
    repositories::{RepositoryResult, RepositoryError, album_repository::AlbumRepository as DomainAlbumRepository},
};

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

    fn row_to_album(&self, row: AlbumRow) -> RepositoryResult<Album> {
        let album = Album::from_persistence(
            AlbumId::from_uuid(row.id),
            AlbumTitle::new(row.title).map_err(|e| RepositoryError::SerializationError(e))?,
            ArtistId::from_uuid(row.artist_id),
            None, // description: Option<String>
            ReleaseType::Album, // release_type: ReleaseType - default value
            Genre::new(row.genre).map_err(|e| RepositoryError::SerializationError(e))?, // genre: Genre
            None, // release_date: Option<DateTime<Utc>>
            None, // cover_art_ipfs: Option<IpfsHash>
            row.is_published, // is_published: bool
            false, // is_featured: bool - default value
            0, // listen_count: u64 - default value
            0.0, // revenue_generated: f64 - default value
            row.created_at, // created_at: DateTime<Utc>
            row.updated_at, // updated_at: DateTime<Utc>
        );
        Ok(album)
    }
}

#[async_trait]
impl DomainAlbumRepository for PostgresAlbumRepository {
    async fn save(&self, album: &Album) -> RepositoryResult<()> {
        sqlx::query(
            r#"INSERT INTO albums (id, title, artist_id, genre, is_published, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7)
               ON CONFLICT (id) DO UPDATE SET
               title = EXCLUDED.title, updated_at = EXCLUDED.updated_at"#
        )
        .bind(album.id().value())
        .bind(album.title().value())
        .bind(album.artist_id().value())
        .bind(album.genre().value()) // Convert Genre to string
        .bind(album.is_published())
        .bind(album.created_at())
        .bind(album.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }

    async fn find_by_id(&self, id: &Uuid) -> RepositoryResult<Option<Album>> {
        let row: Option<AlbumRow> = sqlx::query_as(
            "SELECT id, title, artist_id, genre, is_published, created_at, updated_at FROM albums WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        if let Some(row) = row {
            Ok(Some(self.row_to_album(row)?))
        } else {
            Ok(None)
        }
    }

    async fn find_by_artist_id(&self, artist_id: &Uuid) -> RepositoryResult<Vec<Album>> {
        let rows: Vec<AlbumRow> = sqlx::query_as(
            "SELECT id, title, artist_id, genre, is_published, created_at, updated_at FROM albums WHERE artist_id = $1"
        )
        .bind(artist_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let albums: RepositoryResult<Vec<Album>> = rows.into_iter()
            .map(|row| self.row_to_album(row))
            .collect();
        
        albums
    }

    async fn find_all(&self, page: u32, page_size: u32) -> RepositoryResult<Vec<Album>> {
        let offset = (page - 1) * page_size;
        let rows: Vec<AlbumRow> = sqlx::query_as(
            "SELECT id, title, artist_id, genre, is_published, created_at, updated_at FROM albums ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let albums: RepositoryResult<Vec<Album>> = rows.into_iter()
            .map(|row| self.row_to_album(row))
            .collect();
        
        albums
    }

    async fn update(&self, album: &Album) -> RepositoryResult<()> {
        sqlx::query(
            r#"UPDATE albums SET title = $2, artist_id = $3, genre = $4, is_published = $5, updated_at = $6 WHERE id = $1"#
        )
        .bind(album.id().value())
        .bind(album.title().value())
        .bind(album.artist_id().value())
        .bind(album.genre().value())
        .bind(album.is_published())
        .bind(album.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }

    async fn delete(&self, id: &Uuid) -> RepositoryResult<()> {
        sqlx::query("DELETE FROM albums WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }

    async fn count(&self) -> RepositoryResult<u64> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM albums")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        Ok(count as u64)
    }

    async fn search_by_title(&self, title: &str) -> RepositoryResult<Vec<Album>> {
        let rows: Vec<AlbumRow> = sqlx::query_as(
            "SELECT id, title, artist_id, genre, is_published, created_at, updated_at FROM albums WHERE title ILIKE $1"
        )
        .bind(format!("%{}%", title))
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let albums: RepositoryResult<Vec<Album>> = rows.into_iter()
            .map(|row| self.row_to_album(row))
            .collect();
        
        albums
    }
} 