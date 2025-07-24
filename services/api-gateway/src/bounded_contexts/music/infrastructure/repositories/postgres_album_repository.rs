use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;

use crate::bounded_contexts::music::domain::{
    entities::Album,
    value_objects::{AlbumId, AlbumTitle, ArtistId, Genre, ReleaseType},
    repositories::{RepositoryResult, RepositoryError},
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

#[async_trait]
pub trait AlbumRepository: Send + Sync {
    async fn save(&self, album: &Album) -> RepositoryResult<()>;
    async fn find_by_id(&self, id: &AlbumId) -> RepositoryResult<Option<Album>>;
    async fn find_by_artist(&self, artist_id: &ArtistId) -> RepositoryResult<Vec<Album>>;
    async fn find_by_genre(&self, genre: &str) -> RepositoryResult<Vec<Album>>;
    async fn search_by_title(&self, query: &str, limit: Option<usize>) -> RepositoryResult<Vec<Album>>;
    async fn get_top_albums(&self, limit: Option<usize>) -> RepositoryResult<Vec<Album>>;
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
impl AlbumRepository for PostgresAlbumRepository {
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

    async fn find_by_id(&self, id: &AlbumId) -> RepositoryResult<Option<Album>> {
        let row: Option<AlbumRow> = sqlx::query_as(
            "SELECT id, title, artist_id, genre, is_published, created_at, updated_at FROM albums WHERE id = $1"
        )
        .bind(id.value())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        if let Some(row) = row {
            Ok(Some(self.row_to_album(row)?))
        } else {
            Ok(None)
        }
    }

    async fn find_by_artist(&self, artist_id: &ArtistId) -> RepositoryResult<Vec<Album>> {
        let rows: Vec<AlbumRow> = sqlx::query_as(
            "SELECT id, title, artist_id, genre, is_published, created_at, updated_at FROM albums WHERE artist_id = $1 ORDER BY created_at DESC"
        )
        .bind(artist_id.value())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut albums = Vec::new();
        for row in rows {
            albums.push(self.row_to_album(row)?);
        }
        Ok(albums)
    }

    async fn find_by_genre(&self, genre: &str) -> RepositoryResult<Vec<Album>> {
        let rows: Vec<AlbumRow> = sqlx::query_as(
            "SELECT id, title, artist_id, genre, is_published, created_at, updated_at FROM albums WHERE genre = $1 AND is_published = true"
        )
        .bind(genre)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut albums = Vec::new();
        for row in rows {
            albums.push(self.row_to_album(row)?);
        }
        Ok(albums)
    }

    async fn search_by_title(&self, query: &str, limit: Option<usize>) -> RepositoryResult<Vec<Album>> {
        let limit = limit.unwrap_or(50) as i64;
        let search_pattern = format!("%{}%", query);
        
        let rows: Vec<AlbumRow> = sqlx::query_as(
            "SELECT id, title, artist_id, genre, is_published, created_at, updated_at FROM albums WHERE title ILIKE $1 LIMIT $2"
        )
        .bind(search_pattern)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut albums = Vec::new();
        for row in rows {
            albums.push(self.row_to_album(row)?);
        }
        Ok(albums)
    }

    async fn get_top_albums(&self, limit: Option<usize>) -> RepositoryResult<Vec<Album>> {
        let limit = limit.unwrap_or(20) as i64;
        
        let rows: Vec<AlbumRow> = sqlx::query_as(
            "SELECT id, title, artist_id, genre, is_published, created_at, updated_at FROM albums WHERE is_published = true ORDER BY created_at DESC LIMIT $1"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut albums = Vec::new();
        for row in rows {
            albums.push(self.row_to_album(row)?);
        }
        Ok(albums)
    }
} 