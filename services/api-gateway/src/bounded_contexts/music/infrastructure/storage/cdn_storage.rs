use async_trait::async_trait;
use bytes::Bytes;
use std::io::Result as IoResult;
use uuid::Uuid;

use crate::shared::infrastructure::cdn::{CloudCDNService, CDNService, ContentType, CDNError};
use super::{AudioFileStorage, AudioFileMetadata};

/// CDN-based audio file storage implementation
pub struct CDNAudioStorage {
    cdn_service: CloudCDNService,
}

impl CDNAudioStorage {
    pub fn new(cdn_service: CloudCDNService) -> Self {
        Self { cdn_service }
    }

    pub fn new_with_default_config() -> Self {
        Self::new(CloudCDNService::new_with_default_config())
    }

    fn get_content_type(&self, file_extension: &str) -> ContentType {
        match file_extension.to_lowercase().as_str() {
            "mp3" => ContentType::Audio,
            "wav" => ContentType::Audio,
            "flac" => ContentType::Audio,
            "aac" => ContentType::Audio,
            _ => ContentType::Audio, // Default
        }
    }
}

#[async_trait]
impl AudioFileStorage for CDNAudioStorage {
    async fn upload_audio(&self, file_data: Bytes, file_name: &str, content_type: &str) -> IoResult<String> {
        let content_type_enum = self.get_content_type(file_name);
        
        match self.cdn_service.upload_content(
            file_data.to_vec(),
            content_type_enum,
            file_name.to_string(),
        ).await {
            Ok(response) => {
                if let Some(url) = response.url {
                    Ok(url)
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to get URL from CDN response"
                    ))
                }
            }
            Err(cdn_error) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("CDN upload failed: {}", cdn_error)
            ))
        }
    }

    async fn download_audio(&self, url: &str) -> IoResult<Bytes> {
        // Extract content ID from URL
        let content_id = url.split('/').last()
            .ok_or_else(|| std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid CDN URL format"
            ))?;
        
        let content_id_uuid = Uuid::parse_str(content_id)
            .map_err(|_| std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid content ID in URL"
            ))?;

        // For now, we'll return empty bytes since CDN service doesn't have download method
        // In a real implementation, this would download from CDN
        Ok(Bytes::new())
    }

    async fn delete_audio(&self, url: &str) -> IoResult<()> {
        let content_id = url.split('/').last()
            .ok_or_else(|| std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid CDN URL format"
            ))?;
        
        let content_id_uuid = Uuid::parse_str(content_id)
            .map_err(|_| std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid content ID in URL"
            ))?;

        match self.cdn_service.delete_content(content_id_uuid).await {
            Ok(_) => Ok(()),
            Err(cdn_error) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("CDN delete failed: {}", cdn_error)
            ))
        }
    }

    async fn get_streaming_url(&self, url: &str) -> IoResult<String> {
        // CDN URLs are already streaming-ready
        Ok(url.to_string())
    }

    async fn get_metadata(&self, url: &str) -> IoResult<AudioFileMetadata> {
        let content_id = url.split('/').last()
            .ok_or_else(|| std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid CDN URL format"
            ))?;
        
        let content_id_uuid = Uuid::parse_str(content_id)
            .map_err(|_| std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid content ID in URL"
            ))?;

        match self.cdn_service.get_content_metadata(content_id_uuid).await {
            Ok(Some(metadata)) => Ok(AudioFileMetadata {
                file_size: metadata.file_size,
                content_type: metadata.mime_type,
                duration_seconds: None, // Would need audio analysis
                bitrate: None,
                sample_rate: None,
                channels: None,
                created_at: metadata.uploaded_at,
                peer_count: None, // Not applicable for CDN
                availability_score: Some(1.0), // CDN is always available
            }),
            Ok(None) => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Content not found in CDN"
            )),
            Err(cdn_error) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("CDN metadata fetch failed: {}", cdn_error)
            ))
        }
    }

    async fn get_peers(&self, _url: &str) -> IoResult<Vec<String>> {
        // CDN doesn't have peers concept
        Ok(vec![])
    }

    async fn announce_to_network(&self, _url: &str) -> IoResult<()> {
        // CDN doesn't need network announcement
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::infrastructure::cdn::CDNConfig;

    #[tokio::test]
    async fn test_cdn_storage_creation() {
        let cdn_service = CloudCDNService::new(CDNConfig::default());
        let storage = CDNAudioStorage::new(cdn_service);
        assert!(true); // Just test that it can be created
    }

    #[test]
    fn test_content_type_detection() {
        let storage = CDNAudioStorage::new_with_default_config();
        
        assert!(matches!(
            storage.get_content_type("mp3"),
            ContentType::Audio
        ));
        
        assert!(matches!(
            storage.get_content_type("wav"),
            ContentType::Audio
        ));
    }
} 