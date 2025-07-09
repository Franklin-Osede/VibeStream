// Music Application Commands
// This module contains command structures and handlers for music operations

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::sync::Arc;
use crate::shared::application::command::{Command, CommandHandler};
use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::music::domain::{
    entities::{Song, Album, Playlist},
    value_objects::*,
    repositories::{
        SongRepository,
        MusicCatalogRepository
    },
    aggregates::music_catalog_aggregate::MusicCatalogAggregate,
};
use serde::{Deserialize, Serialize};

// Command definitions
#[derive(Debug, Deserialize)]
pub struct CreateSongCommand {
    pub title: String,
    pub artist_id: Uuid,
    pub duration_seconds: u32,
    pub genre: String,
    pub mood: Option<String>,
    pub file_format: Option<String>,
    pub tempo: Option<u16>,
    pub release_type: Option<String>,
    pub royalty_percentage: f64,
    pub ipfs_hash: Option<String>,
}

impl Command for CreateSongCommand {}

#[derive(Debug, Deserialize)]
pub struct UpdateSongCommand {
    pub song_id: Uuid,
    pub title: Option<String>,
    pub genre: Option<String>,
    pub mood: Option<String>,
    pub file_format: Option<String>,
    pub tempo: Option<u16>,
    pub release_type: Option<String>,
    pub royalty_percentage: Option<f64>,
    pub ipfs_hash: Option<String>,
}

impl Command for UpdateSongCommand {}

#[derive(Debug, Deserialize)]
pub struct DeleteSongCommand {
    pub song_id: Uuid,
    pub artist_id: Uuid,
}

impl Command for DeleteSongCommand {}

#[derive(Debug, Deserialize)]
pub struct RecordListenCommand {
    pub song_id: Uuid,
    pub listener_id: Uuid,
    pub listen_duration_seconds: u32,
}

impl Command for RecordListenCommand {}

// Simplified Command Handlers (only for Song operations that work)

pub struct CreateSongHandler {
    song_repository: Arc<dyn SongRepository>,
}

impl CreateSongHandler {
    pub fn new(song_repository: Arc<dyn SongRepository>) -> Self {
        Self { song_repository }
    }
}

#[async_trait]
impl CommandHandler<CreateSongCommand> for CreateSongHandler {
    type Output = SongId;

    async fn handle(&self, command: CreateSongCommand) -> Result<Self::Output, AppError> {
        // Create value objects
        let title = SongTitle::new(command.title)
            .map_err(|e| AppError::ValidationError(e))?;
        let artist_id = ArtistId::from_uuid(command.artist_id);
        let duration = SongDuration::new(command.duration_seconds)
            .map_err(|e| AppError::ValidationError(e))?;
        let genre = Genre::new(command.genre)
            .map_err(|e| AppError::ValidationError(e))?;
        let royalty_percentage = RoyaltyPercentage::new(command.royalty_percentage)
            .map_err(|e| AppError::ValidationError(e))?;

        // Create song
        let mut song = Song::new(
            title,
            artist_id,
            duration,
            genre,
            royalty_percentage,
        );

        // Set IPFS hash if provided
        if let Some(ipfs_hash_str) = command.ipfs_hash {
            let ipfs_hash = IpfsHash::new(ipfs_hash_str)
                .map_err(|e| AppError::ValidationError(e))?;
            song.set_ipfs_hash(ipfs_hash);
        }

        // Save song
        let song_id = song.id().clone();
        self.song_repository.save(&song).await?;

        Ok(song_id)
    }
}

pub struct UpdateSongHandler {
    song_repository: Arc<dyn SongRepository>,
}

impl UpdateSongHandler {
    pub fn new(song_repository: Arc<dyn SongRepository>) -> Self {
        Self { song_repository }
    }
}

#[async_trait]
impl CommandHandler<UpdateSongCommand> for UpdateSongHandler {
    type Output = ();

    async fn handle(&self, command: UpdateSongCommand) -> Result<Self::Output, AppError> {
        let song_id = SongId::from_uuid(command.song_id);
        let mut song = self.song_repository.find_by_id(&song_id).await?
            .ok_or_else(|| AppError::NotFound("Song not found".to_string()))?;

        // Update title if provided
        if let Some(title_str) = command.title {
            let new_title = SongTitle::new(title_str)
                .map_err(|e| AppError::ValidationError(e))?;
            song.update_title(new_title)?;
        }

        // Update royalty percentage if provided
        if let Some(royalty_pct) = command.royalty_percentage {
            let new_royalty = RoyaltyPercentage::new(royalty_pct)
                .map_err(|e| AppError::ValidationError(e))?;
            song.update_royalty_percentage(new_royalty)?;
        }

        // Update IPFS hash if provided
        if let Some(ipfs_hash_str) = command.ipfs_hash {
            let ipfs_hash = IpfsHash::new(ipfs_hash_str)
                .map_err(|e| AppError::ValidationError(e))?;
            song.set_ipfs_hash(ipfs_hash);
        }

        // Save updated song
        self.song_repository.update(&song).await?;

        Ok(())
    }
}

pub struct DeleteSongHandler {
    song_repository: Arc<dyn SongRepository>,
}

impl DeleteSongHandler {
    pub fn new(song_repository: Arc<dyn SongRepository>) -> Self {
        Self { song_repository }
    }
}

#[async_trait]
impl CommandHandler<DeleteSongCommand> for DeleteSongHandler {
    type Output = ();

    async fn handle(&self, command: DeleteSongCommand) -> Result<Self::Output, AppError> {
        let song_id = SongId::from_uuid(command.song_id);
        let artist_id = ArtistId::from_uuid(command.artist_id);

        // Check if song exists
        let song = self.song_repository.find_by_id(&song_id).await?
            .ok_or_else(|| AppError::NotFound("Song not found".to_string()))?;

        // Verify ownership
        if song.artist_id() != &artist_id {
            return Err(AppError::PermissionDenied("Artist does not own this song".to_string()));
        }

        // Domain rule: Can't delete songs with significant revenue or ownership contracts
        if song.revenue_generated() > 1000.0 || song.is_available_for_ownership() {
            return Err(AppError::ValidationError(
                "Cannot delete song with significant revenue or ownership contracts".to_string()
            ));
        }

        // Delete song
        self.song_repository.delete(&song_id).await?;

        Ok(())
    }
}

pub struct RecordListenHandler {
    song_repository: Arc<dyn SongRepository>,
}

impl RecordListenHandler {
    pub fn new(song_repository: Arc<dyn SongRepository>) -> Self {
        Self { song_repository }
    }
}

#[async_trait]
impl CommandHandler<RecordListenCommand> for RecordListenHandler {
    type Output = ();

    async fn handle(&self, command: RecordListenCommand) -> Result<Self::Output, AppError> {
        let song_id = SongId::from_uuid(command.song_id);
        let mut song = self.song_repository.find_by_id(&song_id).await?
            .ok_or_else(|| AppError::NotFound("Song not found".to_string()))?;

        // Record the listen
        let _event = song.record_listen(command.listener_id, command.listen_duration_seconds)?;

        // Save updated song
        self.song_repository.update(&song).await?;

        Ok(())
    }
} 