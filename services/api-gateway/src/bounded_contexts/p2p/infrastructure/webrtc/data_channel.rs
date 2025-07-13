use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use webrtc::data_channel::RTCDataChannel as WebRTCDataChannel;
use webrtc::data_channel::data_channel_message::DataChannelMessage as WebRTCDataChannelMessage;
use webrtc::Error as WebRTCError;

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
    // Real WebRTC data channel (optional)
    real_channel: Option<Arc<WebRTCDataChannel>>,
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
            real_channel: None,
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

    /// Create new RTCDataChannel with real WebRTC data channel
    pub async fn new_real(
        channel_id: &str,
        label: &str,
        connection_state: ConnectionState,
        real_channel: WebRTCDataChannel,
    ) -> Result<Self, DataChannelError> {
        println!("📡 Creating Real RTCDataChannel: {} ({})", channel_id, label);
        
        let channel = Self {
            channel_id: channel_id.to_string(),
            label: label.to_string(),
            connection_state: Arc::new(RwLock::new(connection_state)),
            data_channel_state: Arc::new(RwLock::new(DataChannelState::Connecting)),
            message_queue: Arc::new(Mutex::new(Vec::new())),
            chunk_cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
            stats: Arc::new(Mutex::new(DataChannelStats::default())),
            real_channel: Some(Arc::new(real_channel)),
        };

        // Wait for real data channel to open
        if let Some(ref real_ch) = channel.real_channel {
            // Set up message handler
            let channel_id = channel_id.to_string();
            let real_ch_clone = Arc::clone(real_ch);
            
            tokio::spawn(async move {
                while let Ok(msg) = real_ch_clone.receive().await {
                    println!("📨 Received real data channel message on channel {}: {} bytes", 
                             channel_id, msg.data.len());
                }
            });
        }

        // Update state to open
        {
            let mut state = channel.data_channel_state.write().await;
            *state = DataChannelState::Open;
        }

        println!("✅ Real RTCDataChannel created: {}", channel_id);
        Ok(channel)
    }

    /// Send a message through the data channel (Real implementation)
    pub async fn send_message_real(&self, message: &ChunkMessage) -> Result<(), DataChannelError> {
        if let Some(ref real_ch) = self.real_channel {
            // Serialize message
            let message_data = serde_json::to_vec(message)
                .map_err(|e| DataChannelError::SerializationError(e.to_string()))?;

            // Create WebRTC data channel message
            let webrtc_message = WebRTCDataChannelMessage::Binary(message_data);

            // Send through real data channel
            real_ch.send(&webrtc_message).await
                .map_err(|e| DataChannelError::SendError(e.to_string()))?;

            // Update stats
            {
                let mut stats = self.stats.lock().await;
                stats.messages_sent += 1;
                stats.bytes_sent += message_data.len() as u64;
            }

            println!("✅ Real message sent through data channel: {}", self.channel_id);
            Ok(())
        } else {
            Err(DataChannelError::ChannelNotAvailable)
        }
    }

    /// Request chunk through data channel (Real implementation)
    pub async fn request_chunk_real(&self, request: &ChunkRequest) -> Result<ChunkResponse, DataChannelError> {
        if let Some(ref real_ch) = self.real_channel {
            // Serialize request
            let request_data = serde_json::to_vec(request)
                .map_err(|e| DataChannelError::SerializationError(e.to_string()))?;

            // Create WebRTC data channel message
            let webrtc_message = WebRTCDataChannelMessage::Binary(request_data);

            // Send request through real data channel
            real_ch.send(&webrtc_message).await
                .map_err(|e| DataChannelError::SendError(e.to_string()))?;

            // For now, return a mock response
            // In a real implementation, you would wait for the response
            let response = ChunkResponse {
                request_id: request.request_id.clone(),
                chunk_data: Some(vec![0x1, 0x2, 0x3, 0x4]), // Mock chunk data
                error: None,
            };

            // Update stats
            {
                let mut stats = self.stats.lock().await;
                stats.messages_sent += 1;
                stats.bytes_sent += request_data.len() as u64;
            }

            println!("✅ Real chunk request sent through data channel: {}", self.channel_id);
            Ok(response)
        } else {
            Err(DataChannelError::ChannelNotAvailable)
        }
    }

    /// Send a message through the data channel (Mock implementation)
    pub async fn send_message(&self, message: &ChunkMessage) -> Result<(), DataChannelError> {
        println!("📤 Sending message through data channel: {} ({})", self.channel_id, self.label);

        // Check if channel is open
        let state = self.data_channel_state.read().await;
        if *state != DataChannelState::Open {
            return Err(DataChannelError::ChannelNotOpen);
        }

        // Add message to queue
        {
            let mut queue = self.message_queue.lock().await;
            let data_channel_message = DataChannelMessage {
                message_type: "chunk".to_string(),
                data: serde_json::to_vec(message)
                    .map_err(|e| DataChannelError::SerializationError(e.to_string()))?,
                timestamp: chrono::Utc::now(),
            };
            queue.push(data_channel_message);
        }

        // Update stats
        {
            let mut stats = self.stats.lock().await;
            stats.messages_sent += 1;
            stats.bytes_sent += message.data.len() as u64;
        }

        println!("✅ Message queued for data channel: {}", self.channel_id);
        Ok(())
    }

    /// Request chunk through data channel (Mock implementation)
    pub async fn request_chunk(&self, request: &ChunkRequest) -> Result<ChunkResponse, DataChannelError> {
        println!("📦 Requesting chunk through data channel: {} ({})", self.channel_id, self.label);

        // Check if channel is open
        let state = self.data_channel_state.read().await;
        if *state != DataChannelState::Open {
            return Err(DataChannelError::ChannelNotOpen);
        }

        // Add request to queue
        {
            let mut queue = self.message_queue.lock().await;
            let data_channel_message = DataChannelMessage {
                message_type: "chunk_request".to_string(),
                data: serde_json::to_vec(request)
                    .map_err(|e| DataChannelError::SerializationError(e.to_string()))?,
                timestamp: chrono::Utc::now(),
            };
            queue.push(data_channel_message);
        }

        // Update stats
        {
            let mut stats = self.stats.lock().await;
            stats.messages_sent += 1;
            stats.bytes_sent += request.request_id.len() as u64;
        }

        // Simulate response (in real implementation, this would wait for actual response)
        let response = ChunkResponse {
            request_id: request.request_id.clone(),
            chunk_data: Some(vec![0x1, 0x2, 0x3, 0x4]), // Mock chunk data
            error: None,
        };

        println!("✅ Chunk request queued for data channel: {}", self.channel_id);
        Ok(response)
    }

    /// Get data channel state
    pub async fn get_state(&self) -> DataChannelState {
        self.data_channel_state.read().await.clone()
    }

    /// Get data channel statistics
    pub async fn get_stats(&self) -> DataChannelStats {
        self.stats.lock().await.clone()
    }

    /// Close the data channel (Real implementation)
    pub async fn close_real(&self) -> Result<(), DataChannelError> {
        println!("🔌 Closing real data channel: {}", self.channel_id);

        if let Some(ref real_ch) = self.real_channel {
            real_ch.close().await
                .map_err(|e| DataChannelError::CloseError(e.to_string()))?;
        }

        // Update state
        {
            let mut state = self.data_channel_state.write().await;
            *state = DataChannelState::Closed;
        }

        println!("✅ Real data channel closed: {}", self.channel_id);
        Ok(())
    }

    /// Close the data channel (Mock implementation)
    pub async fn close(&self) -> Result<(), DataChannelError> {
        println!("🔌 Closing data channel: {}", self.channel_id);

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

        println!("✅ Data channel closed: {}", self.channel_id);
        Ok(())
    }

    /// Get queued messages
    pub async fn get_queued_messages(&self) -> Vec<DataChannelMessage> {
        let mut queue = self.message_queue.lock().await;
        queue.drain(..).collect()
    }

    /// Cache a chunk
    pub async fn cache_chunk(&self, chunk_index: u32, data: Vec<u8>) -> Result<(), DataChannelError> {
        let mut cache = self.chunk_cache.write().await;
        cache.insert(chunk_index, data);
        Ok(())
    }

    /// Get cached chunk
    pub async fn get_cached_chunk(&self, chunk_index: u32) -> Option<Vec<u8>> {
        let cache = self.chunk_cache.read().await;
        cache.get(&chunk_index).cloned()
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

/// Data Channel Message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataChannelMessage {
    pub message_type: String,
    pub data: Vec<u8>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Data Channel Statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataChannelStats {
    pub messages_sent: u64,
    pub messages_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub errors: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

/// Data Channel Error
#[derive(Debug, thiserror::Error)]
pub enum DataChannelError {
    #[error("Channel not open")]
    ChannelNotOpen,
    #[error("Channel not available")]
    ChannelNotAvailable,
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Send error: {0}")]
    SendError(String),
    #[error("Receive error: {0}")]
    ReceiveError(String),
    #[error("Close error: {0}")]
    CloseError(String),
    #[error("Channel not found")]
    ChannelNotFound,
    #[error("Invalid message format")]
    InvalidMessageFormat,
}

impl From<webrtc::Error> for DataChannelError {
    fn from(err: webrtc::Error) -> Self {
        DataChannelError::SendError(err.to_string())
    }
} 