use super::{CDNConfig, CDNResponse, ContentMetadata, ContentType, CDNService, CDNError};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use reqwest::Client;
use sha2::{Sha256, Digest};

pub struct CloudCDNService {
    config: CDNConfig,
    client: Client,
    cache: Arc<RwLock<HashMap<Uuid, ContentMetadata>>>,
}

impl CloudCDNService {
    pub fn new(config: CDNConfig) -> Self {
        Self {
            config,
            client: Client::new(),
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn new_with_default_config() -> Self {
        Self::new(CDNConfig::default())
    }

    fn calculate_checksum(&self, content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        format!("{:x}", hasher.finalize())
    }

    fn generate_content_url(&self, content_id: Uuid) -> String {
        format!("{}/content/{}", self.config.base_url, content_id)
    }

    fn get_best_edge_location(&self, user_location: Option<&str>) -> String {
        // Simple edge location selection - in production, use geolocation
        if let Some(location) = user_location {
            match location {
                "US" | "CA" => "us-east-1".to_string(),
                "EU" => "eu-west-1".to_string(),
                "AP" => "ap-southeast-1".to_string(),
                _ => "us-east-1".to_string(),
            }
        } else {
            self.config.edge_locations.first()
                .cloned()
                .unwrap_or_else(|| "us-east-1".to_string())
        }
    }
}

#[async_trait::async_trait]
impl CDNService for CloudCDNService {
    async fn upload_content(&self, content: Vec<u8>, content_type: ContentType, filename: String) -> Result<CDNResponse, CDNError> {
        // Validate file size
        let file_size_mb = content.len() as u64 / (1024 * 1024);
        if file_size_mb > self.config.max_file_size_mb {
            return Err(CDNError::FileTooLarge {
                size: file_size_mb,
                max_size: self.config.max_file_size_mb,
            });
        }

        let content_id = Uuid::new_v4();
        let checksum = self.calculate_checksum(&content);
        let edge_location = self.get_best_edge_location(None);

        // In a real implementation, this would upload to cloud storage (S3, CloudFront, etc.)
        let metadata = ContentMetadata {
            content_id,
            content_type,
            file_size: content.len() as u64,
            mime_type: self.get_mime_type(&filename),
            checksum,
            uploaded_at: chrono::Utc::now(),
            expires_at: None,
            edge_locations: vec![edge_location],
        };

        // Store in cache
        {
            let mut cache = self.cache.write().await;
            cache.insert(content_id, metadata.clone());
        }

        let url = self.generate_content_url(content_id);

        Ok(CDNResponse {
            success: true,
            url: Some(url),
            error: None,
            metadata: Some(metadata),
        })
    }

    async fn get_content_url(&self, content_id: Uuid) -> Result<String, CDNError> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if cache.contains_key(&content_id) {
                return Ok(self.generate_content_url(content_id));
            }
        }

        // In a real implementation, this would check cloud storage
        // For now, return error if not in cache
        Err(CDNError::ContentNotFound { content_id })
    }

    async fn delete_content(&self, content_id: Uuid) -> Result<bool, CDNError> {
        // Remove from cache
        {
            let mut cache = self.cache.write().await;
            cache.remove(&content_id);
        }

        // In a real implementation, this would delete from cloud storage
        Ok(true)
    }

    async fn get_content_metadata(&self, content_id: Uuid) -> Result<Option<ContentMetadata>, CDNError> {
        let cache = self.cache.read().await;
        Ok(cache.get(&content_id).cloned())
    }

    async fn purge_cache(&self, content_id: Uuid) -> Result<bool, CDNError> {
        // In a real implementation, this would purge from CDN cache
        // For now, just remove from local cache
        {
            let mut cache = self.cache.write().await;
            cache.remove(&content_id);
        }
        Ok(true)
    }
}

impl CloudCDNService {
    fn get_mime_type(&self, filename: &str) -> String {
        match filename.split('.').last().unwrap_or("").to_lowercase().as_str() {
            "mp3" => "audio/mpeg".to_string(),
            "wav" => "audio/wav".to_string(),
            "flac" => "audio/flac".to_string(),
            "mp4" => "video/mp4".to_string(),
            "avi" => "video/x-msvideo".to_string(),
            "jpg" | "jpeg" => "image/jpeg".to_string(),
            "png" => "image/png".to_string(),
            "gif" => "image/gif".to_string(),
            "pdf" => "application/pdf".to_string(),
            _ => "application/octet-stream".to_string(),
        }
    }

    pub async fn get_content_stats(&self) -> HashMap<String, u64> {
        let cache = self.cache.read().await;
        let mut stats = HashMap::new();
        
        for metadata in cache.values() {
            let content_type = match &metadata.content_type {
                ContentType::Audio => "audio",
                ContentType::Video => "video",
                ContentType::Image => "image",
                ContentType::Document => "document",
                ContentType::Other => "other",
            };
            
            *stats.entry(content_type.to_string()).or_insert(0) += 1;
        }
        
        stats
    }

    pub async fn cleanup_expired_content(&self) -> u64 {
        let now = chrono::Utc::now();
        let mut cache = self.cache.write().await;
        let initial_size = cache.len();
        
        cache.retain(|_, metadata| {
            if let Some(expires_at) = metadata.expires_at {
                now < expires_at
            } else {
                true // Keep content without expiration
            }
        });
        
        (initial_size - cache.len()) as u64
    }
} 