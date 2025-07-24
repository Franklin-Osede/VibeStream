use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;

use crate::bounded_contexts::music::domain::{
    entities::{Playlist, PlaylistTrack},
    value_objects::{PlaylistId, PlaylistName, SongId, SongDuration},
    repositories::{RepositoryResult, RepositoryError},
};

// Internal structs for database mapping
#[derive(FromRow)]
struct PlaylistRow {
    id: Uuid,
    name: String,
    creator_id: Uuid,
    description: Option<String>,
    is_public: bool,
    is_collaborative: bool,
    cover_image_url: Option<String>,
    follower_count: i32,
    like_count: i32,
    listen_count: i64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(FromRow)]
struct PlaylistTrackRow {
    song_id: Uuid,
    position: i32,
    added_at: DateTime<Utc>,
    added_by: Uuid,
}

#[async_trait]
pub trait PlaylistRepository: Send + Sync {
    async fn save(&self, playlist: &Playlist) -> RepositoryResult<()>;
    async fn find_by_id(&self, id: &PlaylistId) -> RepositoryResult<Option<Playlist>>;
    async fn find_by_user(&self, user_id: Uuid) -> RepositoryResult<Vec<Playlist>>;
    async fn find_public_playlists(&self, limit: Option<usize>) -> RepositoryResult<Vec<Playlist>>;
    async fn search_by_name(&self, query: &str, limit: Option<usize>) -> RepositoryResult<Vec<Playlist>>;
    async fn get_trending_playlists(&self, limit: Option<usize>) -> RepositoryResult<Vec<Playlist>>;
}

pub struct PostgresPlaylistRepository {
    pool: PgPool,
}

impl PostgresPlaylistRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn row_to_playlist(&self, row: PlaylistRow, tracks: Vec<PlaylistTrack>) -> RepositoryResult<Playlist> {
        let playlist = Playlist::from_persistence(
            PlaylistId::from_uuid(row.id),
            PlaylistName::new(row.name).map_err(|e| RepositoryError::SerializationError(e))?,
            row.creator_id,
            row.description,
            row.is_public,
            None, // cover_image_ipfs: Option<IpfsHash> - placeholder
            None, // total_duration: Option<SongDuration> - calculated later
            row.listen_count as u64,
            row.created_at,
            row.updated_at,
        );
        
        // Set additional properties not in from_persistence
        // These would be set via domain methods in a real implementation
        // playlist.set_collaborative(row.is_collaborative);
        // playlist.set_follower_count(row.follower_count as u64);
        // playlist.set_like_count(row.like_count as u64);
        
        Ok(playlist)
    }
}

#[async_trait]
impl PlaylistRepository for PostgresPlaylistRepository {
    async fn save(&self, playlist: &Playlist) -> RepositoryResult<()> {
        let mut tx = self.pool.begin()
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // Save playlist
        sqlx::query(
            r#"INSERT INTO playlists (id, name, creator_id, description, is_public, is_collaborative, 
                                     cover_image_url, follower_count, like_count, listen_count, created_at, updated_at)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
               ON CONFLICT (id) DO UPDATE SET
               name = EXCLUDED.name, description = EXCLUDED.description, 
               is_public = EXCLUDED.is_public, updated_at = EXCLUDED.updated_at"#
        )
        .bind(playlist.id().value())
        .bind(playlist.name().value())
        .bind(playlist.creator_id())
        .bind(playlist.description())
        .bind(playlist.is_public())
        .bind(playlist.is_collaborative())
        .bind(playlist.cover_image_url())
        .bind(playlist.follower_count() as i32)
        .bind(playlist.like_count() as i32)
        .bind(playlist.listen_count() as i64)
        .bind(playlist.created_at())
        .bind(playlist.updated_at())
        .execute(&mut *tx)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // Save playlist tracks
        sqlx::query("DELETE FROM playlist_tracks WHERE playlist_id = $1")
        .bind(playlist.id().value())
        .execute(&mut *tx)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        for track in playlist.tracks() {
            sqlx::query(
                r#"INSERT INTO playlist_tracks (playlist_id, song_id, position, added_at, added_by)
                   VALUES ($1, $2, $3, $4, $5)"#
            )
            .bind(playlist.id().value())
            .bind(track.song_id().value())
            .bind(track.position() as i32)
            .bind(track.added_at())
            .bind(track.added_by())
            .execute(&mut *tx)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        }

        tx.commit()
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &PlaylistId) -> RepositoryResult<Option<Playlist>> {
        let row: Option<PlaylistRow> = sqlx::query_as(
            r#"SELECT id, name, creator_id, description, is_public, is_collaborative, 
                      cover_image_url, follower_count, like_count, listen_count, created_at, updated_at
               FROM playlists WHERE id = $1"#
        )
        .bind(id.value())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        if let Some(row) = row {
            // Get playlist tracks
            let track_rows: Vec<PlaylistTrackRow> = sqlx::query_as(
                r#"SELECT song_id, position, added_at, added_by 
                   FROM playlist_tracks WHERE playlist_id = $1 ORDER BY position"#
            )
            .bind(id.value())
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

            let tracks: Result<Vec<_>, _> = track_rows.into_iter().map(|track_row| {
                // In a real implementation, you'd join with songs table to get title, artist_name, and duration
                // For now, using placeholder values
                Ok(PlaylistTrack::new(
                    SongId::from_uuid(track_row.song_id),
                    "Unknown Title".to_string(), // title: String - placeholder
                    "Unknown Artist".to_string(), // artist_name: String - placeholder  
                    SongDuration::new(180).unwrap(), // duration: SongDuration - default 3 minutes
                    track_row.position as u32, // track_order: u32
                ))
            }).collect();

            Ok(Some(self.row_to_playlist(row, tracks?)?))
        } else {
            Ok(None)
        }
    }

    async fn find_by_user(&self, user_id: Uuid) -> RepositoryResult<Vec<Playlist>> {
        let rows: Vec<PlaylistRow> = sqlx::query_as(
            r#"SELECT id, name, creator_id, description, is_public, is_collaborative, 
                      cover_image_url, follower_count, like_count, listen_count, created_at, updated_at
               FROM playlists WHERE creator_id = $1 ORDER BY created_at DESC"#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut playlists = Vec::new();
        for row in rows {
            let playlist = self.row_to_playlist(row, Vec::new())?; // Load tracks separately if needed
            playlists.push(playlist);
        }
        Ok(playlists)
    }

    async fn find_public_playlists(&self, limit: Option<usize>) -> RepositoryResult<Vec<Playlist>> {
        let limit = limit.unwrap_or(50) as i64;
        
        let rows: Vec<PlaylistRow> = sqlx::query_as(
            r#"SELECT id, name, creator_id, description, is_public, is_collaborative, 
                      cover_image_url, follower_count, like_count, listen_count, created_at, updated_at
               FROM playlists WHERE is_public = true ORDER BY follower_count DESC LIMIT $1"#
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut playlists = Vec::new();
        for row in rows {
            let playlist = self.row_to_playlist(row, Vec::new())?;
            playlists.push(playlist);
        }
        Ok(playlists)
    }

    async fn search_by_name(&self, query: &str, limit: Option<usize>) -> RepositoryResult<Vec<Playlist>> {
        let limit = limit.unwrap_or(50) as i64;
        let search_pattern = format!("%{}%", query);
        
        let rows: Vec<PlaylistRow> = sqlx::query_as(
            r#"SELECT id, name, creator_id, description, is_public, is_collaborative, 
                      cover_image_url, follower_count, like_count, listen_count, created_at, updated_at
               FROM playlists WHERE name ILIKE $1 AND is_public = true LIMIT $2"#
        )
        .bind(search_pattern)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut playlists = Vec::new();
        for row in rows {
            let playlist = self.row_to_playlist(row, Vec::new())?;
            playlists.push(playlist);
        }
        Ok(playlists)
    }

    async fn get_trending_playlists(&self, limit: Option<usize>) -> RepositoryResult<Vec<Playlist>> {
        let limit = limit.unwrap_or(20) as i64;
        
        let rows: Vec<PlaylistRow> = sqlx::query_as(
            r#"SELECT id, name, creator_id, description, is_public, is_collaborative, 
                      cover_image_url, follower_count, like_count, listen_count, created_at, updated_at
               FROM playlists WHERE is_public = true ORDER BY like_count DESC, follower_count DESC LIMIT $1"#
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut playlists = Vec::new();
        for row in rows {
            let playlist = self.row_to_playlist(row, Vec::new())?;
            playlists.push(playlist);
        }
        Ok(playlists)
    }
} 