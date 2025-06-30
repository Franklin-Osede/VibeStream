use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::music::domain::{
    Song, SongId, ArtistId, Genre, SongTitle, SongDuration, RoyaltyPercentage, ListenCount
};
use crate::bounded_contexts::music::domain::repositories::{
    SongRepository, RepositoryResult, RepositoryError
};

pub struct PostgresSongRepository {
    pool: PgPool,
}

impl PostgresSongRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SongRepository for PostgresSongRepository {
    async fn save(&self, song: &Song) -> RepositoryResult<()> {
        let query = r#"
            INSERT INTO songs (
                id, title, artist_id, duration_seconds, genre, 
                royalty_percentage, listen_count, revenue_generated,
                is_available_for_campaign, is_available_for_ownership,
                created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ON CONFLICT (id) DO UPDATE SET
                title = EXCLUDED.title,
                duration_seconds = EXCLUDED.duration_seconds,
                genre = EXCLUDED.genre,
                royalty_percentage = EXCLUDED.royalty_percentage,
                listen_count = EXCLUDED.listen_count,
                revenue_generated = EXCLUDED.revenue_generated,
                is_available_for_campaign = EXCLUDED.is_available_for_campaign,
                is_available_for_ownership = EXCLUDED.is_available_for_ownership,
                updated_at = EXCLUDED.updated_at
        "#;

        sqlx::query(query)
            .bind(song.id().value())
            .bind(song.title().value())
            .bind(song.artist_id().value())
            .bind(song.duration().seconds() as i32)
            .bind(song.genre().value())
            .bind(song.royalty_percentage().value())
            .bind(song.listen_count().value() as i64)
            .bind(song.revenue_generated())
            .bind(song.is_available_for_campaign())
            .bind(song.is_available_for_ownership())
            .bind(song.created_at())
            .bind(Utc::now()) // updated_at
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &SongId) -> RepositoryResult<Option<Song>> {
        let query = r#"
            SELECT id, title, artist_id, duration_seconds, genre,
                   royalty_percentage, listen_count, revenue_generated,
                   is_available_for_campaign, is_available_for_ownership,
                   created_at, updated_at
            FROM songs WHERE id = $1
        "#;

        let row = sqlx::query(query)
            .bind(id.value())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        if let Some(row) = row {
            let song = self.row_to_song(row)?;
            Ok(Some(song))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, id: &SongId) -> RepositoryResult<()> {
        let query = "DELETE FROM songs WHERE id = $1";
        
        sqlx::query(query)
            .bind(id.value())
            .execute(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        Ok(())
    }

    async fn find_by_artist(&self, artist_id: &ArtistId) -> RepositoryResult<Vec<Song>> {
        let query = r#"
            SELECT id, title, artist_id, duration_seconds, genre,
                   royalty_percentage, listen_count, revenue_generated,
                   is_available_for_campaign, is_available_for_ownership,
                   created_at, updated_at
            FROM songs WHERE artist_id = $1
            ORDER BY created_at DESC
        "#;

        let rows = sqlx::query(query)
            .bind(artist_id.value())
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let songs = rows.into_iter()
            .map(|row| self.row_to_song(row))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(songs)
    }

    async fn find_by_genre(&self, genre: &Genre) -> RepositoryResult<Vec<Song>> {
        let query = r#"
            SELECT id, title, artist_id, duration_seconds, genre,
                   royalty_percentage, listen_count, revenue_generated,
                   is_available_for_campaign, is_available_for_ownership,
                   created_at, updated_at
            FROM songs WHERE genre = $1
            ORDER BY listen_count DESC
        "#;

        let rows = sqlx::query(query)
            .bind(genre.value())
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let songs = rows.into_iter()
            .map(|row| self.row_to_song(row))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(songs)
    }

    async fn find_trending(&self, limit: Option<usize>) -> RepositoryResult<Vec<Song>> {
        let limit = limit.unwrap_or(50) as i64;
        
        let query = r#"
            SELECT id, title, artist_id, duration_seconds, genre,
                   royalty_percentage, listen_count, revenue_generated,
                   is_available_for_campaign, is_available_for_ownership,
                   created_at, updated_at
            FROM songs 
            WHERE created_at > NOW() - INTERVAL '30 days'
              AND listen_count > 0
            ORDER BY (listen_count::float / EXTRACT(EPOCH FROM (NOW() - created_at)) * 86400) DESC
            LIMIT $1
        "#;

        let rows = sqlx::query(query)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let songs = rows.into_iter()
            .map(|row| self.row_to_song(row))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(songs)
    }

    async fn find_popular(&self, limit: Option<usize>) -> RepositoryResult<Vec<Song>> {
        let limit = limit.unwrap_or(100) as i64;
        
        let query = r#"
            SELECT id, title, artist_id, duration_seconds, genre,
                   royalty_percentage, listen_count, revenue_generated,
                   is_available_for_campaign, is_available_for_ownership,
                   created_at, updated_at
            FROM songs 
            WHERE listen_count >= 10000
            ORDER BY listen_count DESC
            LIMIT $1
        "#;

        let rows = sqlx::query(query)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let songs = rows.into_iter()
            .map(|row| self.row_to_song(row))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(songs)
    }

    async fn search_by_title(&self, query: &str, limit: Option<usize>) -> RepositoryResult<Vec<Song>> {
        let limit = limit.unwrap_or(50) as i64;
        let search_term = format!("%{}%", query.to_lowercase());
        
        let sql_query = r#"
            SELECT id, title, artist_id, duration_seconds, genre,
                   royalty_percentage, listen_count, revenue_generated,
                   is_available_for_campaign, is_available_for_ownership,
                   created_at, updated_at
            FROM songs 
            WHERE LOWER(title) LIKE $1
            ORDER BY listen_count DESC
            LIMIT $2
        "#;

        let rows = sqlx::query(sql_query)
            .bind(search_term)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let songs = rows.into_iter()
            .map(|row| self.row_to_song(row))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(songs)
    }

    async fn count_by_artist(&self, artist_id: &ArtistId) -> RepositoryResult<usize> {
        let query = "SELECT COUNT(*) as count FROM songs WHERE artist_id = $1";
        
        let row = sqlx::query(query)
            .bind(artist_id.value())
            .fetch_one(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let count: i64 = row.get("count");
        Ok(count as usize)
    }

    async fn get_total_listens(&self) -> RepositoryResult<u64> {
        let query = "SELECT SUM(listen_count) as total FROM songs";
        
        let row = sqlx::query(query)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| RepositoryError::Database(e.to_string()))?;

        let total: Option<i64> = row.get("total");
        Ok(total.unwrap_or(0) as u64)
    }
}

impl PostgresSongRepository {
    fn row_to_song(&self, row: sqlx::postgres::PgRow) -> RepositoryResult<Song> {
        let id: Uuid = row.get("id");
        let title: String = row.get("title");
        let artist_id: Uuid = row.get("artist_id");
        let duration_seconds: i32 = row.get("duration_seconds");
        let genre: String = row.get("genre");
        let royalty_percentage: f64 = row.get("royalty_percentage");
        let listen_count: i64 = row.get("listen_count");
        let revenue_generated: f64 = row.get("revenue_generated");
        let is_available_for_campaign: bool = row.get("is_available_for_campaign");
        let is_available_for_ownership: bool = row.get("is_available_for_ownership");
        let created_at: DateTime<Utc> = row.get("created_at");

        // Reconstruct Song (this is a simplified version - in real implementation 
        // you might need to use a builder pattern or factory)
        let song_id = SongId::from_uuid(id);
        let song_title = SongTitle::new(title)
            .map_err(|e| RepositoryError::Serialization(e))?;
        let song_artist_id = ArtistId::from_uuid(artist_id);
        let song_duration = SongDuration::new(duration_seconds as u32)
            .map_err(|e| RepositoryError::Serialization(e))?;
        let song_genre = Genre::new(genre)
            .map_err(|e| RepositoryError::Serialization(e))?;
        let song_royalty = RoyaltyPercentage::new(royalty_percentage)
            .map_err(|e| RepositoryError::Serialization(e))?;

        // Create song with basic data
        let mut song = Song::new(
            song_title,
            song_artist_id,
            song_duration,
            song_genre,
            song_royalty,
        );

        // Note: In a real implementation, you'd need to add methods to Song 
        // to set these fields or use a more sophisticated reconstruction approach
        // For now, this is a conceptual implementation

        Ok(song)
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