use async_trait::async_trait;
use bytes::Bytes;
use std::io::Result as IoResult;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use super::{AudioFileStorage, AudioFileMetadata};

/// Local file system storage for development
pub struct LocalAudioStorage {
    base_path: PathBuf,
    max_file_size: u64,
}

impl LocalAudioStorage {
    pub fn new(base_path: String, max_file_size: u64) -> Self {
        Self {
            base_path: PathBuf::from(base_path),
            max_file_size,
        }
    }

    /// Ensure storage directory exists
    async fn ensure_directory(&self) -> IoResult<()> {
        if !self.base_path.exists() {
            fs::create_dir_all(&self.base_path).await?;
        }
        Ok(())
    }

    /// Get full file path
    fn get_file_path(&self, file_name: &str) -> PathBuf {
        self.base_path.join(file_name)
    }

    /// Generate streaming URL for local files
    fn generate_streaming_url(&self, file_name: &str) -> String {
        format!("/api/v1/audio/stream/{}", file_name)
    }
}

#[async_trait]
impl AudioFileStorage for LocalAudioStorage {
    async fn upload_audio(&self, file_data: Bytes, file_name: &str, content_type: &str) -> IoResult<String> {
        // Validate file size
        if file_data.len() as u64 > self.max_file_size {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("File size {} exceeds maximum {}", file_data.len(), self.max_file_size)
            ));
        }

        // Ensure directory exists
        self.ensure_directory().await?;

        // Write file to disk
        let file_path = self.get_file_path(file_name);
        let mut file = fs::File::create(&file_path).await?;
        file.write_all(&file_data).await?;
        file.flush().await?;

        // Return local file URL
        Ok(format!("local://{}", file_name))
    }

    async fn download_audio(&self, url: &str) -> IoResult<Bytes> {
        // Extract filename from URL
        let file_name = url.strip_prefix("local://")
            .ok_or_else(|| std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid local URL format"
            ))?;

        let file_path = self.get_file_path(file_name);
        let data = fs::read(&file_path).await?;
        Ok(Bytes::from(data))
    }

    async fn delete_audio(&self, url: &str) -> IoResult<()> {
        let file_name = url.strip_prefix("local://")
            .ok_or_else(|| std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid local URL format"
            ))?;

        let file_path = self.get_file_path(file_name);
        if file_path.exists() {
            fs::remove_file(&file_path).await?;
        }
        Ok(())
    }

    async fn get_streaming_url(&self, url: &str) -> IoResult<String> {
        let file_name = url.strip_prefix("local://")
            .ok_or_else(|| std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid local URL format"
            ))?;

        Ok(self.generate_streaming_url(file_name))
    }

    async fn get_metadata(&self, url: &str) -> IoResult<AudioFileMetadata> {
        let file_name = url.strip_prefix("local://")
            .ok_or_else(|| std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid local URL format"
            ))?;

        let file_path = self.get_file_path(file_name);
        let metadata = fs::metadata(&file_path).await?;
        
        // Detect content type from extension
        let content_type = match Path::new(file_name).extension().and_then(|ext| ext.to_str()) {
            Some("mp3") => "audio/mpeg",
            Some("flac") => "audio/flac",
            Some("wav") => "audio/wav",
            Some("aac") => "audio/aac",
            Some("ogg") => "audio/ogg",
            Some("m4a") => "audio/mp4",
            _ => "application/octet-stream",
        };

        Ok(AudioFileMetadata {
            file_size: metadata.len() as u64,
            content_type: content_type.to_string(),
            duration_seconds: None, // TODO: Extract from file
            bitrate: None,
            sample_rate: None,
            channels: None,
            availability_score: Some(1.0), // Default availability score
            peer_count: Some(0), // Default peer count
            created_at: metadata.created()
                .map(|time| DateTime::from(time))
                .unwrap_or_else(|_| Utc::now()),
        })
    }

    async fn get_peers(&self, url: &str) -> IoResult<Vec<String>> {
        // Mock implementation - return empty list for local storage
        Ok(Vec::new())
    }

    async fn announce_to_network(&self, url: &str) -> IoResult<()> {
        // Mock implementation - no network announcement for local storage
        Ok(())
    }
} 