use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::connection::ConnectionState;
use super::engine::{ChunkMessage, ChunkRequest, ChunkResponse};

/// RTCDataChannel - Represents a WebRTC data channel
pub struct RTCDataChannel {
    channel_id: String,
    label: String,
    connection_state: Arc<RwLock<ConnectionState>>,
    data_channel_state: Arc<RwLock<DataChannelState>>,
    message_queue: Arc<Mutex<Vec<DataChannelMessage>>>,
    chunk_cache: Arc<RwLock<std::collections::HashMap<u32, Vec<u8>>>>,
    stats: Arc<Mutex<DataChannelStats>>,
}

impl RTCDataChannel {
    pub async fn new(
        channel_id: &str,
        label: &str,
        connection_state: ConnectionState,
    ) -> Result<Self, DataChannelError> {
        println!("📡 Creating RTCDataChannel: {} ({})", channel_id, label);
        
        let channel = Self {
            channel_id: channel_id.to_string(),
            label: label.to_string(),
            connection_state: Arc::new(RwLock::new(connection_state)),
            data_channel_state: Arc::new(RwLock::new(DataChannelState::Connecting)),
            message_queue: Arc::new(Mutex::new(Vec::new())),
            chunk_cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
            stats: Arc::new(Mutex::new(DataChannelStats::default())),
        };

        // Simulate connection establishment
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        
        {
            let mut state = channel.data_channel_state.write().await;
            *state = DataChannelState::Open;
        }

        println!("✅ RTCDataChannel created: {}", channel_id);
        Ok(channel)
    }

    /// Send a message through the data channel
    pub async fn send_message(&self, message: &ChunkMessage) -> Result<(), DataChannelError> {
        let state = self.data_channel_state.read().await;
        if *state != DataChannelState::Open {
            return Err(DataChannelError::ChannelNotOpen);
        }

        println!("📤 Sending chunk message: chunk {} ({} bytes) via channel {}", 
                 message.chunk_index, message.data.len(), self.channel_id);

        // Add message to queue
        {
            let mut queue = self.message_queue.lock().await;
            queue.push(DataChannelMessage::ChunkData {
                chunk_index: message.chunk_index,
                quality: message.quality.clone(),
                data: message.data.clone(),
                timestamp: message.timestamp,
            });
        }

        // Update stats
        {
            let mut stats = self.stats.lock().await;
            stats.messages_sent += 1;
            stats.bytes_sent += message.data.len() as u64;
            stats.last_activity = chrono::Utc::now();
        }

        // Cache chunk data
        {
            let mut cache = self.chunk_cache.write().await;
            cache.insert(message.chunk_index, message.data.clone());
        }

        Ok(())
    }

    /// Request a chunk from the data channel
    pub async fn request_chunk(&self, request: &ChunkRequest) -> Result<ChunkResponse, DataChannelError> {
        let state = self.data_channel_state.read().await;
        if *state != DataChannelState::Open {
            return Err(DataChannelError::ChannelNotOpen);
        }

        println!("📥 Requesting chunk: {} at quality {} via channel {}", 
                 request.chunk_index, request.quality, self.channel_id);

        // Check cache first
        {
            let cache = self.chunk_cache.read().await;
            if let Some(chunk_data) = cache.get(&request.chunk_index) {
                println!("✅ Chunk {} found in cache", request.chunk_index);
                
                // Update stats
                {
                    let mut stats = self.stats.lock().await;
                    stats.chunks_served += 1;
                    stats.bytes_served += chunk_data.len() as u64;
                }

                return Ok(ChunkResponse {
                    request_id: request.request_id.clone(),
                    chunk_data: Some(chunk_data.clone()),
                    error: None,
                });
            }
        }

        // Add request to queue
        {
            let mut queue = self.message_queue.lock().await;
            queue.push(DataChannelMessage::ChunkRequest {
                request_id: request.request_id.clone(),
                chunk_index: request.chunk_index,
                quality: request.quality.clone(),
            });
        }

        // Update stats
        {
            let mut stats = self.stats.lock().await;
            stats.requests_received += 1;
            stats.last_activity = chrono::Utc::now();
        }

        // Simulate chunk retrieval (in real implementation, this would fetch from storage)
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        // Mock chunk data
        let mock_chunk_data = vec![0x1, 0x2, 0x3, 0x4, 0x5]; // 5 bytes mock data
        
        // Cache the mock data
        {
            let mut cache = self.chunk_cache.write().await;
            cache.insert(request.chunk_index, mock_chunk_data.clone());
        }

        // Update stats
        {
            let mut stats = self.stats.lock().await;
            stats.chunks_served += 1;
            stats.bytes_served += mock_chunk_data.len() as u64;
        }

        Ok(ChunkResponse {
            request_id: request.request_id.clone(),
            chunk_data: Some(mock_chunk_data),
            error: None,
        })
    }

    /// Get data channel state
    pub async fn get_state(&self) -> DataChannelState {
        self.data_channel_state.read().await.clone()
    }

    /// Get channel statistics
    pub async fn get_stats(&self) -> DataChannelStats {
        self.stats.lock().await.clone()
    }

    /// Get cached chunks
    pub async fn get_cached_chunks(&self) -> Vec<u32> {
        let cache = self.chunk_cache.read().await;
        cache.keys().cloned().collect()
    }

    /// Check if chunk is cached
    pub async fn has_chunk(&self, chunk_index: u32) -> bool {
        let cache = self.chunk_cache.read().await;
        cache.contains_key(&chunk_index)
    }

    /// Get chunk from cache
    pub async fn get_chunk_from_cache(&self, chunk_index: u32) -> Option<Vec<u8>> {
        let cache = self.chunk_cache.read().await;
        cache.get(&chunk_index).cloned()
    }

    /// Clear cache
    pub async fn clear_cache(&self) -> Result<(), DataChannelError> {
        println!("🧹 Clearing cache for data channel: {}", self.channel_id);
        
        let mut cache = self.chunk_cache.write().await;
        cache.clear();
        
        Ok(())
    }

    /// Close the data channel
    pub async fn close(&self) -> Result<(), DataChannelError> {
        println!("🔌 Closing RTCDataChannel: {}", self.channel_id);
        
        // Update state
        {
            let mut state = self.data_channel_state.write().await;
            *state = DataChannelState::Closed;
        }

        // Clear message queue
        {
            let mut queue = self.message_queue.lock().await;
            queue.clear();
        }

        // Clear cache
        self.clear_cache().await?;

        println!("✅ RTCDataChannel closed: {}", self.channel_id);
        Ok(())
    }

    /// Get message queue length
    pub async fn get_queue_length(&self) -> usize {
        let queue = self.message_queue.lock().await;
        queue.len()
    }

    /// Process next message in queue
    pub async fn process_next_message(&self) -> Result<Option<DataChannelMessage>, DataChannelError> {
        let mut queue = self.message_queue.lock().await;
        Ok(queue.pop())
    }

    /// Get channel info
    pub fn get_info(&self) -> DataChannelInfo {
        DataChannelInfo {
            channel_id: self.channel_id.clone(),
            label: self.label.clone(),
        }
    }
}

/// Data Channel State
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataChannelState {
    Connecting,
    Open,
    Closing,
    Closed,
}

/// Data Channel Message Types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DataChannelMessage {
    ChunkData {
        chunk_index: u32,
        quality: String,
        data: Vec<u8>,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    ChunkRequest {
        request_id: String,
        chunk_index: u32,
        quality: String,
    },
    KeepAlive,
    Close,
}

/// Data Channel Statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataChannelStats {
    pub messages_sent: u64,
    pub messages_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub chunks_served: u64,
    pub bytes_served: u64,
    pub requests_received: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub last_activity: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl DataChannelStats {
    pub fn new() -> Self {
        Self {
            created_at: chrono::Utc::now(),
            last_activity: chrono::Utc::now(),
            ..Default::default()
        }
    }

    pub fn get_throughput_mbps(&self) -> f64 {
        let total_bytes = self.bytes_sent + self.bytes_received;
        let duration = chrono::Utc::now() - self.created_at;
        let duration_seconds = duration.num_seconds() as f64;
        
        if duration_seconds > 0.0 {
            (total_bytes as f64 * 8.0) / (duration_seconds * 1_000_000.0) // Convert to Mbps
        } else {
            0.0
        }
    }

    pub fn get_cache_hit_ratio(&self) -> f64 {
        let total_requests = self.cache_hits + self.cache_misses;
        if total_requests > 0 {
            self.cache_hits as f64 / total_requests as f64
        } else {
            0.0
        }
    }
}

/// Data Channel Info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataChannelInfo {
    pub channel_id: String,
    pub label: String,
}

/// Data Channel Error
#[derive(Debug, thiserror::Error)]
pub enum DataChannelError {
    #[error("Channel not open")]
    ChannelNotOpen,
    #[error("Channel not found")]
    ChannelNotFound,
    #[error("Message too large")]
    MessageTooLarge,
    #[error("Invalid message format")]
    InvalidMessageFormat,
    #[error("Chunk not found")]
    ChunkNotFound,
    #[error("Data channel error: {0}")]
    GeneralError(String),
} 