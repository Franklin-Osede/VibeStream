use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

/// Buffer Manager - Manages streaming buffers for sessions
pub struct BufferManager {
    session_buffers: Arc<RwLock<HashMap<String, SessionBuffer>>>,
    buffer_config: BufferConfig,
    buffer_stats: Arc<RwLock<HashMap<String, BufferStats>>>,
}

impl BufferManager {
    pub fn new() -> Self {
        println!("📊 Initializing Buffer Manager");
        
        Self {
            session_buffers: Arc::new(RwLock::new(HashMap::new())),
            buffer_config: BufferConfig::default(),
            buffer_stats: Arc<RwLock::new(HashMap::new())),
        }
    }

    /// Initialize buffer for a session
    pub async fn initialize_session(&self, session_id: &str) -> Result<(), BufferError> {
        println!("📊 Initializing buffer for session: {}", session_id);
        
        let buffer = SessionBuffer {
            session_id: session_id.to_string(),
            buffer_level: 0.0,
            target_buffer_seconds: self.buffer_config.target_buffer_seconds,
            max_buffer_seconds: self.buffer_config.max_buffer_seconds,
            chunks: Vec::new(),
            last_update: chrono::Utc::now(),
        };

        let stats = BufferStats::new();

        {
            let mut buffers = self.session_buffers.write().await;
            buffers.insert(session_id.to_string(), buffer);
        }

        {
            let mut stats_map = self.buffer_stats.write().await;
            stats_map.insert(session_id.to_string(), stats);
        }

        println!("✅ Buffer initialized for session: {}", session_id);
        Ok(())
    }

    /// Update buffer level for a session
    pub async fn update_buffer_level(&self, session_id: &str, buffer_level: f64) {
        let mut buffers = self.session_buffers.write().await;
        if let Some(buffer) = buffers.get_mut(session_id) {
            buffer.buffer_level = buffer_level.clamp(0.0, 1.0);
            buffer.last_update = chrono::Utc::now();
        }

        // Update stats
        let mut stats_map = self.buffer_stats.write().await;
        if let Some(stats) = stats_map.get_mut(session_id) {
            stats.update_buffer_level(buffer_level);
        }
    }

    /// Add chunk to buffer
    pub async fn add_chunk(&self, session_id: &str, chunk_index: u32, chunk_data: Vec<u8>) -> Result<(), BufferError> {
        let mut buffers = self.session_buffers.write().await;
        if let Some(buffer) = buffers.get_mut(session_id) {
            let chunk = BufferChunk {
                index: chunk_index,
                data: chunk_data,
                added_at: chrono::Utc::now(),
            };

            buffer.chunks.push(chunk);
            buffer.chunks.sort_by_key(|c| c.index);

            // Maintain buffer size
            while buffer.chunks.len() > self.buffer_config.max_chunks {
                buffer.chunks.remove(0);
            }

            // Update buffer level
            let buffer_level = buffer.chunks.len() as f64 / self.buffer_config.max_chunks as f64;
            buffer.buffer_level = buffer_level.clamp(0.0, 1.0);
        }

        Ok(())
    }

    /// Get chunk from buffer
    pub async fn get_chunk(&self, session_id: &str, chunk_index: u32) -> Option<Vec<u8>> {
        let buffers = self.session_buffers.read().await;
        if let Some(buffer) = buffers.get(session_id) {
            buffer.chunks.iter()
                .find(|chunk| chunk.index == chunk_index)
                .map(|chunk| chunk.data.clone())
        } else {
            None
        }
    }

    /// Get buffer status
    pub async fn get_buffer_status(&self, session_id: &str) -> Option<BufferStatus> {
        let buffers = self.session_buffers.read().await;
        if let Some(buffer) = buffers.get(session_id) {
            Some(BufferStatus {
                session_id: session_id.to_string(),
                buffer_level: buffer.buffer_level,
                chunk_count: buffer.chunks.len(),
                target_seconds: buffer.target_buffer_seconds,
                max_seconds: buffer.max_buffer_seconds,
                last_update: buffer.last_update,
            })
        } else {
            None
        }
    }

    /// Check if buffer is healthy
    pub async fn is_buffer_healthy(&self, session_id: &str) -> bool {
        let buffers = self.session_buffers.read().await;
        if let Some(buffer) = buffers.get(session_id) {
            buffer.buffer_level >= self.buffer_config.min_healthy_level
        } else {
            false
        }
    }

    /// Get buffer statistics
    pub async fn get_buffer_stats(&self, session_id: &str) -> Option<BufferStats> {
        let stats_map = self.buffer_stats.read().await;
        stats_map.get(session_id).cloned()
    }

    /// Get all buffer statuses
    pub async fn get_all_buffer_statuses(&self) -> Vec<BufferStatus> {
        let buffers = self.session_buffers.read().await;
        buffers.values()
            .map(|buffer| BufferStatus {
                session_id: buffer.session_id.clone(),
                buffer_level: buffer.buffer_level,
                chunk_count: buffer.chunks.len(),
                target_seconds: buffer.target_buffer_seconds,
                max_seconds: buffer.max_buffer_seconds,
                last_update: buffer.last_update,
            })
            .collect()
    }

    /// Cleanup session buffer
    pub async fn cleanup_session(&self, session_id: &str) {
        println!("🧹 Cleaning up buffer for session: {}", session_id);
        
        {
            let mut buffers = self.session_buffers.write().await;
            buffers.remove(session_id);
        }

        {
            let mut stats_map = self.buffer_stats.write().await;
            stats_map.remove(session_id);
        }
    }

    /// Get buffer configuration
    pub fn get_config(&self) -> &BufferConfig {
        &self.buffer_config
    }

    /// Update buffer configuration
    pub fn update_config(&mut self, config: BufferConfig) {
        self.buffer_config = config;
    }

    /// Get buffer health metrics
    pub async fn get_health_metrics(&self) -> BufferHealthMetrics {
        let buffers = self.session_buffers.read().await;
        let stats_map = self.buffer_stats.read().await;
        
        let total_sessions = buffers.len();
        let healthy_sessions = buffers.values()
            .filter(|buffer| buffer.buffer_level >= self.buffer_config.min_healthy_level)
            .count();
        
        let average_buffer_level: f64 = if total_sessions > 0 {
            buffers.values()
                .map(|buffer| buffer.buffer_level)
                .sum::<f64>() / total_sessions as f64
        } else {
            0.0
        };

        BufferHealthMetrics {
            total_sessions,
            healthy_sessions,
            average_buffer_level,
            unhealthy_sessions: total_sessions - healthy_sessions,
        }
    }
}

/// Session Buffer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionBuffer {
    pub session_id: String,
    pub buffer_level: f64,
    pub target_buffer_seconds: u32,
    pub max_buffer_seconds: u32,
    pub chunks: Vec<BufferChunk>,
    pub last_update: chrono::DateTime<chrono::Utc>,
}

/// Buffer Chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferChunk {
    pub index: u32,
    pub data: Vec<u8>,
    pub added_at: chrono::DateTime<chrono::Utc>,
}

/// Buffer Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferStatus {
    pub session_id: String,
    pub buffer_level: f64,
    pub chunk_count: usize,
    pub target_seconds: u32,
    pub max_seconds: u32,
    pub last_update: chrono::DateTime<chrono::Utc>,
}

/// Buffer Statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BufferStats {
    pub min_buffer_level: f64,
    pub max_buffer_level: f64,
    pub average_buffer_level: f64,
    pub buffer_underruns: u32,
    pub buffer_overruns: u32,
    pub total_updates: u32,
    pub last_update: chrono::DateTime<chrono::Utc>,
}

impl BufferStats {
    pub fn new() -> Self {
        Self {
            last_update: chrono::Utc::now(),
            ..Default::default()
        }
    }

    pub fn update_buffer_level(&mut self, buffer_level: f64) {
        if self.total_updates == 0 {
            self.min_buffer_level = buffer_level;
            self.max_buffer_level = buffer_level;
            self.average_buffer_level = buffer_level;
        } else {
            self.min_buffer_level = self.min_buffer_level.min(buffer_level);
            self.max_buffer_level = self.max_buffer_level.max(buffer_level);
            
            // Update running average
            let total = self.average_buffer_level * self.total_updates as f64 + buffer_level;
            self.average_buffer_level = total / (self.total_updates + 1) as f64;
        }

        // Track underruns and overruns
        if buffer_level < 0.1 {
            self.buffer_underruns += 1;
        }
        if buffer_level > 0.95 {
            self.buffer_overruns += 1;
        }

        self.total_updates += 1;
        self.last_update = chrono::Utc::now();
    }
}

/// Buffer Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferConfig {
    pub target_buffer_seconds: u32,
    pub max_buffer_seconds: u32,
    pub min_healthy_level: f64,
    pub max_chunks: usize,
    pub cleanup_interval_ms: u64,
}

impl Default for BufferConfig {
    fn default() -> Self {
        Self {
            target_buffer_seconds: 10,  // 10 seconds target
            max_buffer_seconds: 30,     // 30 seconds maximum
            min_healthy_level: 0.2,     // 20% minimum healthy level
            max_chunks: 100,            // Maximum 100 chunks in buffer
            cleanup_interval_ms: 30000, // 30 seconds cleanup interval
        }
    }
}

/// Buffer Health Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferHealthMetrics {
    pub total_sessions: usize,
    pub healthy_sessions: usize,
    pub unhealthy_sessions: usize,
    pub average_buffer_level: f64,
}

/// Buffer Error
#[derive(Debug, thiserror::Error)]
pub enum BufferError {
    #[error("Session not found")]
    SessionNotFound,
    #[error("Buffer full")]
    BufferFull,
    #[error("Invalid buffer level")]
    InvalidBufferLevel,
    #[error("Buffer error: {0}")]
    GeneralError(String),
} 