pub mod repositories;
pub mod streaming;
pub mod webrtc;
pub mod transcoding;
pub mod storage;

pub use repositories::*;
pub use streaming::*;
pub use webrtc::*;
pub use transcoding::*;
pub use storage::*;

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
    
    /// Configuración de almacenamiento IPFS
    pub ipfs_enabled: bool,
    pub ipfs_node_url: String,
    pub ipfs_peer_nodes: Vec<String>,
    pub max_video_file_size: u64,
    pub enable_federation: bool,
    pub enable_content_discovery: bool,
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
            ipfs_enabled: true,
            ipfs_node_url: "http://localhost:5001".to_string(),
            ipfs_peer_nodes: vec![
                "http://peer1:5001".to_string(),
                "http://peer2:5001".to_string(),
            ],
            max_video_file_size: 2 * 1024 * 1024 * 1024, // 2GB
            enable_federation: true,
            enable_content_discovery: true,
        }
    }
}

/// Factory para crear componentes de infraestructura P2P
pub struct P2PInfrastructureFactory;

impl P2PInfrastructureFactory {
    /// Crear configuración de almacenamiento IPFS
    pub fn create_ipfs_storage_config(config: &P2PInfrastructureConfig) -> P2PStorageConfig {
        if config.ipfs_enabled {
            P2PStorageConfig::DistributedIPFS {
                local_node_url: config.ipfs_node_url.clone(),
                peer_nodes: config.ipfs_peer_nodes.clone(),
                max_file_size: config.max_video_file_size,
                enable_federation: config.enable_federation,
                enable_content_discovery: config.enable_content_discovery,
            }
        } else {
            P2PStorageConfig::Local {
                base_path: "./storage/video".to_string(),
                max_file_size: 500 * 1024 * 1024, // 500MB for local
            }
        }
    }
    
    /// Crear instancia de almacenamiento IPFS
    pub fn create_ipfs_storage(config: &P2PInfrastructureConfig) -> Box<dyn VideoFileStorage> {
        let storage_config = Self::create_ipfs_storage_config(config);
        create_p2p_storage(storage_config)
    }
    
    /// Crear instancia de almacenamiento IPFS asíncrona
    pub async fn create_ipfs_storage_async(config: &P2PInfrastructureConfig) -> std::io::Result<Box<dyn VideoFileStorage>> {
        let storage_config = Self::create_ipfs_storage_config(config);
        create_p2p_storage_async(storage_config).await
    }
} 