use async_trait::async_trait;
use bytes::Bytes;
use std::io::Result as IoResult;
use uuid::Uuid;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::path::Path;
use chrono::{DateTime, Utc};

use super::{AudioFileStorage, AudioFileMetadata};
use crate::bounded_contexts::music::domain::value_objects::FileFormat;

/// Audio file upload service with validation and processing
pub struct AudioFileService {
    storage: Box<dyn AudioFileStorage>,
    max_file_size: u64,
    allowed_formats: Vec<FileFormat>,
}

impl AudioFileService {
    pub fn new(storage: Box<dyn AudioFileStorage>, max_file_size: u64) -> Self {
        Self {
            storage,
            max_file_size,
            allowed_formats: vec![
                FileFormat::Mp3,
                FileFormat::Flac,
                FileFormat::Wav,
                FileFormat::Aac,
                FileFormat::Ogg,
                FileFormat::M4a,
            ],
        }
    }

    /// Upload and process audio file
    pub async fn upload_audio_file(
        &self,
        file_data: Bytes,
        original_filename: &str,
        artist_id: Uuid,
        song_id: Uuid,
    ) -> IoResult<AudioUploadResult> {
        // Validate file size
        if file_data.len() as u64 > self.max_file_size {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("File size {} exceeds maximum {}", file_data.len(), self.max_file_size)
            ));
        }

        // Validate file format
        let file_format = self.detect_file_format(&file_data, original_filename)?;
        
        // Generate unique filename
        let unique_filename = format!("{}_{}_{}.{}", 
            artist_id, song_id, Utc::now().timestamp(), file_format.extension());

        // Extract metadata
        let metadata = self.extract_audio_metadata(&file_data, &file_format).await?;

        // Upload to storage
        let storage_url = self.storage.upload_audio(
            file_data,
            &unique_filename,
            &self.get_content_type(&file_format)
        ).await?;

        // Get streaming URL
        let streaming_url = self.storage.get_streaming_url(&storage_url).await?;

        Ok(AudioUploadResult {
            storage_url,
            streaming_url,
            file_format,
            metadata,
            processed_at: Utc::now(),
        })
    }

    /// Get streaming URL for existing audio file
    pub async fn get_streaming_url(&self, storage_url: &str) -> IoResult<String> {
        self.storage.get_streaming_url(storage_url).await
    }

    /// Delete audio file
    pub async fn delete_audio_file(&self, storage_url: &str) -> IoResult<()> {
        self.storage.delete_audio(storage_url).await
    }

    /// Detect file format from content and filename
    fn detect_file_format(&self, data: &Bytes, filename: &str) -> IoResult<FileFormat> {
        // Check file extension
        let extension = Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        let format = FileFormat::from_extension(extension)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

        // Validate format is allowed
        if !self.allowed_formats.contains(&format) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("File format {} is not allowed", format)
            ));
        }

        // TODO: Validate file signature (magic bytes)
        self.validate_file_signature(data, &format)?;

        Ok(format)
    }

    /// Validate file signature (magic bytes)
    fn validate_file_signature(&self, data: &Bytes, format: &FileFormat) -> IoResult<()> {
        if data.len() < 4 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "File too small to validate"
            ));
        }

        let header = &data[..4];
        let is_valid = match format {
            FileFormat::Mp3 => {
                // MP3 files start with ID3 tag or frame sync
                header.starts_with(b"ID3") || 
                (header[0] == 0xFF && (header[1] & 0xE0) == 0xE0)
            },
            FileFormat::Flac => header.starts_with(b"fLaC"),
            FileFormat::Wav => header.starts_with(b"RIFF"),
            FileFormat::Aac => {
                // AAC files can have various headers
                header.starts_with(b"ADIF") || 
                (header[0] == 0xFF && (header[1] & 0xF0) == 0xF0)
            },
            FileFormat::Ogg => header.starts_with(b"OggS"),
            FileFormat::M4a => {
                // M4A files are MP4 containers
                data.len() >= 8 && &data[4..8] == b"ftyp"
            },
        };

        if !is_valid {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid file signature for format {}", format)
            ));
        }

        Ok(())
    }

    /// Extract audio metadata from file
    async fn extract_audio_metadata(&self, data: &Bytes, format: &FileFormat) -> IoResult<AudioFileMetadata> {
        // TODO: Implement actual metadata extraction using libraries like symphonia
        // For now, return basic metadata
        Ok(AudioFileMetadata {
            file_size: data.len() as u64,
            content_type: self.get_content_type(format),
            duration_seconds: None, // TODO: Extract from file
            bitrate: None, // TODO: Extract from file
            sample_rate: None, // TODO: Extract from file
            channels: None, // TODO: Extract from file
            availability_score: Some(1.0), // Default availability score
            peer_count: Some(0), // Default peer count
            created_at: Utc::now(),
        })
    }

    /// Get MIME content type for file format
    fn get_content_type(&self, format: &FileFormat) -> String {
        match format {
            FileFormat::Mp3 => "audio/mpeg".to_string(),
            FileFormat::Flac => "audio/flac".to_string(),
            FileFormat::Wav => "audio/wav".to_string(),
            FileFormat::Aac => "audio/aac".to_string(),
            FileFormat::Ogg => "audio/ogg".to_string(),
            FileFormat::M4a => "audio/mp4".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AudioUploadResult {
    pub storage_url: String,
    pub streaming_url: String,
    pub file_format: FileFormat,
    pub metadata: AudioFileMetadata,
    pub processed_at: DateTime<Utc>,
}

/// Audio file validation errors
#[derive(Debug, thiserror::Error)]
pub enum AudioValidationError {
    #[error("File size {0} exceeds maximum {1}")]
    FileSizeExceeded(u64, u64),
    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),
    #[error("Invalid file signature")]
    InvalidSignature,
    #[error("Corrupted audio file")]
    CorruptedFile,
    #[error("Missing required metadata")]
    MissingMetadata,
}

/// Audio processing configuration
#[derive(Debug, Clone)]
pub struct AudioProcessingConfig {
    pub max_file_size: u64,
    pub allowed_formats: Vec<FileFormat>,
    pub auto_normalize: bool,
    pub generate_waveform: bool,
    pub extract_metadata: bool,
}

impl Default for AudioProcessingConfig {
    fn default() -> Self {
        Self {
            max_file_size: 100 * 1024 * 1024, // 100MB
            allowed_formats: vec![
                FileFormat::Mp3,
                FileFormat::Flac,
                FileFormat::Wav,
                FileFormat::Aac,
                FileFormat::Ogg,
                FileFormat::M4a,
            ],
            auto_normalize: false,
            generate_waveform: false,
            extract_metadata: true,
        }
    }
} 