pub mod repositories;
pub mod streaming;
pub mod webrtc;
pub mod transcoding;

pub use repositories::*;
pub use streaming::*;
pub use webrtc::*;
pub use transcoding::*;

/// Configuración de infraestructura P2P
#[derive(Debug, Clone)]
pub struct P2PInfrastructureConfig {
    /// Configuración de WebRTC
    pub webrtc_enabled: bool,
    pub ice_servers: Vec<String>,
    pub signaling_server_url: String,
    
    /// Configuración de streaming
    pub chunk_size_bytes: usize,
    pub buffer_target_seconds: u32,
    pub quality_levels: Vec<String>,
    
    /// Configuración de transcoding
    pub transcoding_enabled: bool,
    pub ffmpeg_path: String,
    pub output_formats: Vec<String>,
    
    /// Configuración de analíticas
    pub analytics_enabled: bool,
    pub metrics_collection_interval_seconds: u64,
    pub retention_days: u32,
}

impl Default for P2PInfrastructureConfig {
    fn default() -> Self {
        Self {
            webrtc_enabled: true,
            ice_servers: vec![
                "stun:stun.l.google.com:19302".to_string(),
                "stun:stun1.l.google.com:19302".to_string(),
            ],
            signaling_server_url: "ws://localhost:8080/signaling".to_string(),
            chunk_size_bytes: 64 * 1024, // 64KB
            buffer_target_seconds: 10,
            quality_levels: vec![
                "UltraHD".to_string(),
                "FullHD".to_string(),
                "HD".to_string(),
                "SD".to_string(),
                "Low".to_string(),
            ],
            transcoding_enabled: true,
            ffmpeg_path: "ffmpeg".to_string(),
            output_formats: vec![
                "mp4".to_string(),
                "webm".to_string(),
                "hls".to_string(),
            ],
            analytics_enabled: true,
            metrics_collection_interval_seconds: 30,
            retention_days: 30,
        }
    }
} 