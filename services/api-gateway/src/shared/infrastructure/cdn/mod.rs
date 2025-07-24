pub mod service;
pub mod cache;
pub mod edge;
pub mod errors;
pub mod controllers;

pub use errors::CDNError;
pub use service::CloudCDNService;

use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CDNConfig {
    pub base_url: String,
    pub cache_ttl_seconds: u64,
    pub max_file_size_mb: u64,
    pub enable_compression: bool,
    pub edge_locations: Vec<String>,
}

impl Default for CDNConfig {
    fn default() -> Self {
        Self {
            base_url: "https://cdn.vibestream.com".to_string(),
            cache_ttl_seconds: 3600, // 1 hour
            max_file_size_mb: 100,
            enable_compression: true,
            edge_locations: vec![
                "us-east-1".to_string(),
                "us-west-2".to_string(),
                "eu-west-1".to_string(),
                "ap-southeast-1".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentMetadata {
    pub content_id: Uuid,
    pub content_type: ContentType,
    pub file_size: u64,
    pub mime_type: String,
    pub checksum: String,
    pub uploaded_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub edge_locations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContentType {
    Audio,
    Video,
    Image,
    Document,
    Other,
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentType::Audio => write!(f, "audio"),
            ContentType::Video => write!(f, "video"),
            ContentType::Image => write!(f, "image"),
            ContentType::Document => write!(f, "document"),
            ContentType::Other => write!(f, "other"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CDNResponse {
    pub success: bool,
    pub url: Option<String>,
    pub error: Option<String>,
    pub metadata: Option<ContentMetadata>,
}

#[async_trait::async_trait]
pub trait CDNService: Send + Sync {
    async fn upload_content(&self, content: Vec<u8>, content_type: ContentType, filename: String) -> Result<CDNResponse, CDNError>;
    async fn get_content_url(&self, content_id: Uuid) -> Result<String, CDNError>;
    async fn delete_content(&self, content_id: Uuid) -> Result<bool, CDNError>;
    async fn get_content_metadata(&self, content_id: Uuid) -> Result<Option<ContentMetadata>, CDNError>;
    async fn purge_cache(&self, content_id: Uuid) -> Result<bool, CDNError>;
} 