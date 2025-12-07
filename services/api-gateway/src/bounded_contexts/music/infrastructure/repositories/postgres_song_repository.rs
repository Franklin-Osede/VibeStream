use async_trait::async_trait;
use sqlx::{PgPool, Row};

use crate::bounded_contexts::music::domain::{
    Song, SongId, ArtistId, Genre, 
    value_objects::{SongTitle, SongDuration, RoyaltyPercentage, ListenCount}
};
use crate::bounded_contexts::music::domain::repositories::{SongRepository, RepositoryResult, RepositoryError};

pub struct PostgresSongRepository {
    pool: PgPool,
}

impl PostgresSongRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Convert database row to Song entity
    fn row_to_song(&self, row: sqlx::postgres::PgRow) -> Result<Song, RepositoryError> {
        
        let id = SongId::from_uuid(
            row.try_get("id").map_err(|e| RepositoryError::SerializationError(e.to_string()))?
        );
        
        let title = SongTitle::new(
            row.try_get("title").map_err(|e| RepositoryError::SerializationError(e.to_string()))?
        ).map_err(|e| RepositoryError::SerializationError(e.to_string()))?;
        
        let artist_id = ArtistId::from_uuid(
            row.try_get("artist_id").map_err(|e| RepositoryError::SerializationError(e.to_string()))?
        );
        
        let duration_seconds: i32 = row.try_get("duration_seconds").map_err(|e| RepositoryError::SerializationError(e.to_string()))?;
        let duration = SongDuration::new(duration_seconds as u32).map_err(|e| RepositoryError::ValidationError(e.to_string()))?;
        
        let genre = Genre::new(
            row.try_get("genre").map_err(|e| RepositoryError::SerializationError(e.to_string()))?
        ).map_err(|e| RepositoryError::SerializationError(e.to_string()))?;
        
        let royalty_percentage = RoyaltyPercentage::new(
            row.try_get("royalty_percentage").map_err(|e| RepositoryError::SerializationError(e.to_string()))?
        ).map_err(|e| RepositoryError::SerializationError(e.to_string()))?;

        // Create song with basic fields first
        let mut song = Song::new(title, artist_id, duration, genre, royalty_percentage);

        // Set additional fields from database
        let listen_count: i64 = row.try_get("listen_count").unwrap_or(0);
        song.set_listen_count(ListenCount::from_value(listen_count as u64));

        let revenue: f64 = row.try_get("revenue_generated").unwrap_or(0.0);
        song.set_revenue_generated(revenue);

        Ok(song)
    }
}

#[async_trait]
impl SongRepository for PostgresSongRepository {
    async fn save(&self, song: &Song) -> RepositoryResult<()> {
        sqlx::query(
            r#"INSERT INTO songs (id, title, artist_id, duration_seconds, genre, royalty_percentage, 
                                  listen_count, revenue_generated, is_available_for_campaign, 
                                  is_available_for_ownership, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
               ON CONFLICT (id) DO UPDATE SET
                   title = EXCLUDED.title,
                   genre = EXCLUDED.genre,
                   royalty_percentage = EXCLUDED.royalty_percentage,
                   listen_count = EXCLUDED.listen_count,
                   revenue_generated = EXCLUDED.revenue_generated,
                   is_available_for_campaign = EXCLUDED.is_available_for_campaign,
                   is_available_for_ownership = EXCLUDED.is_available_for_ownership,
                   updated_at = EXCLUDED.updated_at"#
        )
        .bind(song.id().to_uuid())
        .bind(song.title().to_string())
        .bind(song.artist_id().to_uuid())
        .bind(song.duration().seconds() as i32)
        .bind(song.genre().to_string())
        .bind(song.royalty_percentage().value())
        .bind(song.listen_count().value() as i64)
        .bind(song.revenue_generated())
        .bind(song.is_available_for_campaign())
        .bind(song.is_available_for_ownership())
        .bind(song.created_at())
        .bind(song.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn update(&self, song: &Song) -> RepositoryResult<()> {
        let affected_rows = sqlx::query(
            r#"UPDATE songs SET 
                   title = $2,
                   genre = $3,
                   royalty_percentage = $4,
                   listen_count = $5,
                   revenue_generated = $6,
                   is_available_for_campaign = $7,
                   is_available_for_ownership = $8,
                   updated_at = $9
               WHERE id = $1"#
        )
        .bind(song.id().to_uuid())
        .bind(song.title().to_string())
        .bind(song.genre().to_string())
        .bind(song.royalty_percentage().value())
        .bind(song.listen_count().value() as i64)
        .bind(song.revenue_generated())
        .bind(song.is_available_for_campaign())
        .bind(song.is_available_for_ownership())
        .bind(song.updated_at())
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        if affected_rows.rows_affected() == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }

    async fn find_by_id(&self, id: &SongId) -> RepositoryResult<Option<Song>> {
        let row = sqlx::query(
            r#"SELECT id, title, artist_id, duration_seconds, genre, royalty_percentage,
                      listen_count, revenue_generated, is_available_for_campaign,
                      is_available_for_ownership, created_at, updated_at
               FROM songs WHERE id = $1"#
        )
        .bind(id.to_uuid())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        match row {
            Some(row) => Ok(Some(self.row_to_song(row)?)),
            None => Ok(None),
        }
    }

    async fn delete(&self, id: &SongId) -> RepositoryResult<()> {
        let affected_rows = sqlx::query("DELETE FROM songs WHERE id = $1")
            .bind(id.to_uuid())
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        if affected_rows.rows_affected() == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }

    async fn find_all(&self, limit: usize, offset: usize) -> RepositoryResult<Vec<Song>> {
        let limit_val = limit as i64;
        let offset_val = offset as i64;
        
        let rows = sqlx::query(
            r#"SELECT id, title, artist_id, duration_seconds, genre, royalty_percentage,
                      listen_count, revenue_generated, is_available_for_campaign,
                      is_available_for_ownership, created_at, updated_at
               FROM songs 
               ORDER BY created_at DESC
               LIMIT $1 OFFSET $2"#
        )
        .bind(limit_val)
        .bind(offset_val)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut songs = Vec::new();
        for row in rows {
            songs.push(self.row_to_song(row)?);
        }

        Ok(songs)
    }

    async fn find_by_artist(&self, artist_id: &ArtistId) -> RepositoryResult<Vec<Song>> {
        let rows = sqlx::query(
            r#"SELECT id, title, artist_id, duration_seconds, genre, royalty_percentage,
                      listen_count, revenue_generated, is_available_for_campaign,
                      is_available_for_ownership, created_at, updated_at
               FROM songs WHERE artist_id = $1
               ORDER BY created_at DESC"#
        )
        .bind(artist_id.to_uuid())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut songs = Vec::new();
        for row in rows {
            songs.push(self.row_to_song(row)?);
        }

        Ok(songs)
    }

    async fn find_by_genre(&self, genre: &Genre) -> RepositoryResult<Vec<Song>> {
        let rows = sqlx::query(
            r#"SELECT id, title, artist_id, duration_seconds, genre, royalty_percentage,
                      listen_count, revenue_generated, is_available_for_campaign,
                      is_available_for_ownership, created_at, updated_at
               FROM songs WHERE genre = $1
               ORDER BY created_at DESC"#
        )
        .bind(genre.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut songs = Vec::new();
        for row in rows {
            songs.push(self.row_to_song(row)?);
        }

        Ok(songs)
    }

    async fn find_trending(&self, limit: Option<usize>) -> RepositoryResult<Vec<Song>> {
        let limit_val = limit.unwrap_or(50) as i64;
        
        let rows = sqlx::query(
            r#"SELECT id, title, artist_id, duration_seconds, genre, royalty_percentage,
                      listen_count, revenue_generated, is_available_for_campaign,
                      is_available_for_ownership, created_at, updated_at
               FROM songs 
               WHERE created_at > NOW() - INTERVAL '7 days'
               ORDER BY listen_count DESC, created_at DESC
               LIMIT $1"#
        )
        .bind(limit_val)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut songs = Vec::new();
        for row in rows {
            songs.push(self.row_to_song(row)?);
        }

        Ok(songs)
    }

    async fn find_popular(&self, limit: Option<usize>) -> RepositoryResult<Vec<Song>> {
        let limit_val = limit.unwrap_or(50) as i64;
        
        let rows = sqlx::query(
            r#"SELECT id, title, artist_id, duration_seconds, genre, royalty_percentage,
                      listen_count, revenue_generated, is_available_for_campaign,
                      is_available_for_ownership, created_at, updated_at
               FROM songs 
               ORDER BY listen_count DESC, revenue_generated DESC
               LIMIT $1"#
        )
        .bind(limit_val)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut songs = Vec::new();
        for row in rows {
            songs.push(self.row_to_song(row)?);
        }

        Ok(songs)
    }

    async fn search_by_title(&self, query: &str, limit: Option<usize>) -> RepositoryResult<Vec<Song>> {
        let limit_val = limit.unwrap_or(50) as i64;
        let search_pattern = format!("%{}%", query);
        
        let rows = sqlx::query(
            r#"SELECT id, title, artist_id, duration_seconds, genre, royalty_percentage,
                      listen_count, revenue_generated, is_available_for_campaign,
                      is_available_for_ownership, created_at, updated_at
               FROM songs 
               WHERE title ILIKE $1
               ORDER BY listen_count DESC, created_at DESC
               LIMIT $2"#
        )
        .bind(search_pattern)
        .bind(limit_val)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut songs = Vec::new();
        for row in rows {
            songs.push(self.row_to_song(row)?);
        }

        Ok(songs)
    }

    async fn count(&self) -> RepositoryResult<usize> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM songs")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let count: i64 = row.try_get("count").map_err(|e| RepositoryError::SerializationError(e.to_string()))?;
        Ok(count as usize)
    }

    async fn count_by_artist(&self, artist_id: &ArtistId) -> RepositoryResult<usize> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM songs WHERE artist_id = $1")
            .bind(artist_id.to_uuid())
            .fetch_one(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let count: i64 = row.try_get("count").map_err(|e| RepositoryError::SerializationError(e.to_string()))?;
        Ok(count as usize)
    }

    async fn get_total_listens(&self) -> RepositoryResult<u64> {
        let row = sqlx::query("SELECT SUM(listen_count) as total FROM songs")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let total: Option<i64> = row.try_get("total").ok();
        Ok(total.unwrap_or(0) as u64)
    }
}

// SQL Migration for songs table
pub const SONGS_TABLE_MIGRATION: &str = r#"
CREATE TABLE IF NOT EXISTS songs (
    id UUID PRIMARY KEY,
    title VARCHAR(200) NOT NULL,
    artist_id UUID NOT NULL,
    duration_seconds INTEGER NOT NULL CHECK (duration_seconds > 0),
    genre VARCHAR(50) NOT NULL,
    royalty_percentage DECIMAL(5,2) NOT NULL CHECK (royalty_percentage >= 0 AND royalty_percentage <= 100),
    listen_count BIGINT NOT NULL DEFAULT 0,
    revenue_generated DECIMAL(15,2) NOT NULL DEFAULT 0,
    is_available_for_campaign BOOLEAN NOT NULL DEFAULT FALSE,
    is_available_for_ownership BOOLEAN NOT NULL DEFAULT FALSE,
    ipfs_hash VARCHAR(100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_songs_artist_id ON songs(artist_id);
CREATE INDEX IF NOT EXISTS idx_songs_genre ON songs(genre);
CREATE INDEX IF NOT EXISTS idx_songs_listen_count ON songs(listen_count DESC);
CREATE INDEX IF NOT EXISTS idx_songs_created_at ON songs(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_songs_trending ON songs(created_at DESC, listen_count DESC) 
    WHERE created_at > NOW() - INTERVAL '30 days';
CREATE INDEX IF NOT EXISTS idx_songs_title_search ON songs USING gin(to_tsvector('english', title));
"#; 