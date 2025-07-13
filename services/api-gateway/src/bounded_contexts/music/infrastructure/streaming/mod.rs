pub mod audio_streaming;
pub mod stream_processor;
pub mod quality_adapter;

pub use audio_streaming::*;
pub use stream_processor::*;
pub use quality_adapter::*;

use async_trait::async_trait;
use std::io::Result as IoResult;
use bytes::Bytes;
use uuid::Uuid;

use crate::bounded_contexts::music::domain::value_objects::{AudioQuality, FileFormat};

/// Audio streaming service interface
#[async_trait]
pub trait AudioStreamingService: Send + Sync {
    /// Start streaming session
    async fn start_stream(&self, song_id: Uuid, user_id: Uuid, quality: AudioQuality) -> IoResult<StreamSession>;
    
    /// Get audio chunk for streaming
    async fn get_audio_chunk(&self, session_id: &str, chunk_index: u32) -> IoResult<AudioChunk>;
    
    /// End streaming session
    async fn end_stream(&self, session_id: &str) -> IoResult<StreamMetrics>;
    
    /// Get available qualities for a song
    async fn get_available_qualities(&self, song_id: Uuid) -> IoResult<Vec<AudioQuality>>;
}

/// Streaming session information
#[derive(Debug, Clone)]
pub struct StreamSession {
    pub session_id: String,
    pub song_id: Uuid,
    pub user_id: Uuid,
    pub quality: AudioQuality,
    pub format: FileFormat,
    pub total_chunks: u32,
    pub chunk_size: u32,
    pub duration_seconds: u32,
    pub bitrate: u32,
    pub started_at: chrono::DateTime<chrono::Utc>,
}

/// Audio chunk for streaming
#[derive(Debug)]
pub struct AudioChunk {
    pub chunk_index: u32,
    pub data: Bytes,
    pub is_final: bool,
    pub timestamp_ms: u32,
}

/// Streaming metrics
#[derive(Debug, Clone)]
pub struct StreamMetrics {
    pub session_id: String,
    pub total_bytes_streamed: u64,
    pub chunks_requested: u32,
    pub duration_streamed_seconds: u32,
    pub average_bitrate: u32,
    pub buffer_underruns: u32,
    pub ended_at: chrono::DateTime<chrono::Utc>,
}

/// Streaming configuration
#[derive(Debug, Clone)]
pub struct StreamingConfig {
    pub chunk_size: u32,
    pub buffer_size: u32,
    pub max_concurrent_streams: u32,
    pub session_timeout_seconds: u32,
    pub enable_adaptive_bitrate: bool,
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            chunk_size: 64 * 1024, // 64KB chunks
            buffer_size: 5, // 5 chunks buffer
            max_concurrent_streams: 1000,
            session_timeout_seconds: 3600, // 1 hour
            enable_adaptive_bitrate: true,
        }
    }
} 