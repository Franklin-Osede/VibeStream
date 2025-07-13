use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

use super::super::domain::value_objects::*;

/// Chunk Manager - Manages video chunks for P2P streaming
pub struct ChunkManager {
    stream_chunks: Arc<RwLock<HashMap<String, StreamChunkInfo>>>,
    chunk_cache: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    chunk_availability: Arc<RwLock<HashMap<String, Vec<u32>>>>,
}

impl ChunkManager {
    pub fn new() -> Self {
        println!("📦 Initializing Chunk Manager");
        
        Self {
            stream_chunks: Arc::new(RwLock::new(HashMap::new())),
            chunk_cache: Arc::new(RwLock::new(HashMap::new())),
            chunk_availability: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Initialize chunk management for a stream
    pub async fn initialize_stream(&self, stream_id: &str, total_chunks: u32) -> Result<(), ChunkError> {
        println!("🎬 Initializing chunks for stream: {} ({} chunks)", stream_id, total_chunks);
        
        let chunk_info = StreamChunkInfo {
            stream_id: stream_id.to_string(),
            total_chunks,
            chunk_size: 1024 * 1024, // 1MB default
            available_chunks: vec![],
            quality_variants: HashMap::new(),
            created_at: chrono::Utc::now(),
        };

        {
            let mut chunks = self.stream_chunks.write().await;
            chunks.insert(stream_id.to_string(), chunk_info);
        }

        // Initialize availability tracking
        {
            let mut availability = self.chunk_availability.write().await;
            availability.insert(stream_id.to_string(), vec![]);
        }

        Ok(())
    }

    /// Get a specific chunk
    pub async fn get_chunk(&self, stream_id: &str, chunk_index: u32, quality: VideoQuality) -> Result<Vec<u8>, ChunkError> {
        let chunk_key = format!("{}:{}:{}", stream_id, chunk_index, quality_to_string(quality));
        
        // Check cache first
        {
            let cache = self.chunk_cache.read().await;
            if let Some(chunk_data) = cache.get(&chunk_key) {
                return Ok(chunk_data.clone());
            }
        }

        // Load from storage
        let chunk_data = self.load_chunk_from_storage(stream_id, chunk_index, quality).await?;
        
        // Cache the chunk
        {
            let mut cache = self.chunk_cache.write().await;
            cache.insert(chunk_key, chunk_data.clone());
        }

        Ok(chunk_data)
    }

    /// Mark a chunk as available
    pub async fn mark_chunk_available(&self, stream_id: &str, chunk_index: u32, quality: VideoQuality) -> Result<(), ChunkError> {
        println!("✅ Marking chunk {} available for stream {} at quality {:?}", chunk_index, stream_id, quality);
        
        // Update stream chunks
        {
            let mut chunks = self.stream_chunks.write().await;
            if let Some(chunk_info) = chunks.get_mut(stream_id) {
                if !chunk_info.available_chunks.contains(&chunk_index) {
                    chunk_info.available_chunks.push(chunk_index);
                    chunk_info.available_chunks.sort();
                }
            }
        }

        // Update availability tracking
        {
            let mut availability = self.chunk_availability.write().await;
            if let Some(available_chunks) = availability.get_mut(stream_id) {
                if !available_chunks.contains(&chunk_index) {
                    available_chunks.push(chunk_index);
                    available_chunks.sort();
                }
            }
        }

        Ok(())
    }

    /// Get available chunks for a stream
    pub async fn get_available_chunks(&self, stream_id: &str) -> Result<Vec<u32>, ChunkError> {
        let availability = self.chunk_availability.read().await;
        Ok(availability.get(stream_id).cloned().unwrap_or_default())
    }

    /// Get chunk availability percentage
    pub async fn get_availability_percentage(&self, stream_id: &str) -> Result<f64, ChunkError> {
        let chunks = self.stream_chunks.read().await;
        let availability = self.chunk_availability.read().await;
        
        if let (Some(chunk_info), Some(available_chunks)) = (chunks.get(stream_id), availability.get(stream_id)) {
            let percentage = available_chunks.len() as f64 / chunk_info.total_chunks as f64;
            Ok(percentage)
        } else {
            Ok(0.0)
        }
    }

    /// Preload chunks for better streaming experience
    pub async fn preload_chunks(&self, stream_id: &str, start_chunk: u32, count: u32, quality: VideoQuality) -> Result<(), ChunkError> {
        println!("⏳ Preloading {} chunks starting from {} for stream {}", count, start_chunk, stream_id);
        
        for i in 0..count {
            let chunk_index = start_chunk + i;
            
            // Check if chunk is already cached
            let chunk_key = format!("{}:{}:{}", stream_id, chunk_index, quality_to_string(quality));
            {
                let cache = self.chunk_cache.read().await;
                if cache.contains_key(&chunk_key) {
                    continue; // Already cached
                }
            }

            // Load and cache chunk
            match self.load_chunk_from_storage(stream_id, chunk_index, quality).await {
                Ok(chunk_data) => {
                    let mut cache = self.chunk_cache.write().await;
                    cache.insert(chunk_key, chunk_data);
                }
                Err(_) => {
                    // Chunk not available yet, skip
                    break;
                }
            }
        }

        Ok(())
    }

    /// Get chunk information for a stream
    pub async fn get_chunk_info(&self, stream_id: &str) -> Result<StreamChunkInfo, ChunkError> {
        let chunks = self.stream_chunks.read().await;
        chunks.get(stream_id)
            .cloned()
            .ok_or(ChunkError::StreamNotFound)
    }

    /// Add quality variant for a chunk
    pub async fn add_quality_variant(&self, stream_id: &str, chunk_index: u32, quality: VideoQuality, file_path: String) -> Result<(), ChunkError> {
        let mut chunks = self.stream_chunks.write().await;
        if let Some(chunk_info) = chunks.get_mut(stream_id) {
            chunk_info.quality_variants.insert(
                format!("{}:{}", chunk_index, quality_to_string(quality)),
                ChunkVariant {
                    quality,
                    file_path,
                    file_size: 0, // Will be updated when file is processed
                }
            );
        }
        Ok(())
    }

    /// Cleanup chunks for a stream
    pub async fn cleanup_stream(&self, stream_id: &str) -> Result<(), ChunkError> {
        println!("🧹 Cleaning up chunks for stream: {}", stream_id);
        
        // Remove from chunk info
        {
            let mut chunks = self.stream_chunks.write().await;
            chunks.remove(stream_id);
        }

        // Remove from cache
        {
            let mut cache = self.chunk_cache.write().await;
            let keys_to_remove: Vec<String> = cache.keys()
                .filter(|key| key.starts_with(&format!("{}:", stream_id)))
                .cloned()
                .collect();
            
            for key in keys_to_remove {
                cache.remove(&key);
            }
        }

        // Remove from availability
        {
            let mut availability = self.chunk_availability.write().await;
            availability.remove(stream_id);
        }

        Ok(())
    }

    /// Load chunk from storage (mock implementation)
    async fn load_chunk_from_storage(&self, stream_id: &str, chunk_index: u32, quality: VideoQuality) -> Result<Vec<u8>, ChunkError> {
        // Mock implementation - would read from actual storage
        println!("📂 Loading chunk {} from storage for stream {} at quality {:?}", chunk_index, stream_id, quality);
        
        // Simulate chunk data
        let chunk_size = 1024 * 1024; // 1MB
        let mut chunk_data = Vec::with_capacity(chunk_size);
        
        // Fill with mock data
        for i in 0..chunk_size {
            chunk_data.push((i % 256) as u8);
        }
        
        Ok(chunk_data)
    }

    /// Get cache statistics
    pub async fn get_cache_stats(&self) -> CacheStats {
        let cache = self.chunk_cache.read().await;
        let chunks = self.stream_chunks.read().await;
        
        let total_cached_chunks = cache.len();
        let total_streams = chunks.len();
        let total_cached_size: usize = cache.values().map(|chunk| chunk.len()).sum();
        
        CacheStats {
            total_cached_chunks,
            total_streams,
            total_cached_size_mb: total_cached_size as f64 / (1024.0 * 1024.0),
        }
    }
}

/// Stream Chunk Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunkInfo {
    pub stream_id: String,
    pub total_chunks: u32,
    pub chunk_size: usize,
    pub available_chunks: Vec<u32>,
    pub quality_variants: HashMap<String, ChunkVariant>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Chunk Variant (different quality versions)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkVariant {
    pub quality: VideoQuality,
    pub file_path: String,
    pub file_size: u64,
}

/// Cache Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub total_cached_chunks: usize,
    pub total_streams: usize,
    pub total_cached_size_mb: f64,
}

/// Chunk Error
#[derive(Debug, thiserror::Error)]
pub enum ChunkError {
    #[error("Stream not found")]
    StreamNotFound,
    #[error("Chunk not found")]
    ChunkNotFound,
    #[error("Chunk not available")]
    ChunkNotAvailable,
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Invalid chunk index")]
    InvalidChunkIndex,
}

/// Helper function to convert quality to string
fn quality_to_string(quality: VideoQuality) -> String {
    match quality {
        VideoQuality::Low => "low".to_string(),
        VideoQuality::Medium => "medium".to_string(),
        VideoQuality::High => "high".to_string(),
        VideoQuality::UltraHD => "ultrahd".to_string(),
        VideoQuality::Auto => "auto".to_string(),
    }
} 