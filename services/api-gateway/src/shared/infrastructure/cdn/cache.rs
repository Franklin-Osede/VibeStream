use super::{ContentMetadata, ContentType};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct CDNCache {
    content: Arc<RwLock<HashMap<Uuid, CachedContent>>>,
    config: CacheConfig,
}

#[derive(Debug, Clone)]
pub struct CachedContent {
    pub metadata: ContentMetadata,
    pub data: Vec<u8>,
    pub accessed_at: DateTime<Utc>,
    pub access_count: u64,
}

#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub max_size_mb: u64,
    pub ttl_seconds: u64,
    pub max_items: usize,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_size_mb: 1024, // 1GB
            ttl_seconds: 3600, // 1 hour
            max_items: 10000,
        }
    }
}

impl CDNCache {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            content: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    pub async fn get(&self, content_id: Uuid) -> Option<CachedContent> {
        let mut cache = self.content.write().await;
        
        if let Some(content) = cache.get_mut(&content_id) {
            content.accessed_at = Utc::now();
            content.access_count += 1;
            Some(content.clone())
        } else {
            None
        }
    }

    pub async fn set(&self, content_id: Uuid, metadata: ContentMetadata, data: Vec<u8>) -> bool {
        let mut cache = self.content.write().await;
        
        // Check if we need to evict items
        if cache.len() >= self.config.max_items {
            self.evict_oldest(&mut cache).await;
        }
        
        let cached_content = CachedContent {
            metadata,
            data,
            accessed_at: Utc::now(),
            access_count: 0,
        };
        
        cache.insert(content_id, cached_content);
        true
    }

    pub async fn remove(&self, content_id: Uuid) -> bool {
        let mut cache = self.content.write().await;
        cache.remove(&content_id).is_some()
    }

    pub async fn exists(&self, content_id: Uuid) -> bool {
        let cache = self.content.read().await;
        cache.contains_key(&content_id)
    }

    pub async fn get_metadata(&self, content_id: Uuid) -> Option<ContentMetadata> {
        let cache = self.content.read().await;
        cache.get(&content_id).map(|content| content.metadata.clone())
    }

    pub async fn cleanup_expired(&self) -> u64 {
        let now = Utc::now();
        let mut cache = self.content.write().await;
        let initial_size = cache.len();
        
        cache.retain(|_, content| {
            let age = now.signed_duration_since(content.accessed_at);
            age.num_seconds() < self.config.ttl_seconds as i64
        });
        
        (initial_size - cache.len()) as u64
    }

    pub async fn get_stats(&self) -> CacheStats {
        let cache = self.content.read().await;
        let mut stats = CacheStats::default();
        
        for content in cache.values() {
            stats.total_items += 1;
            stats.total_size += content.data.len() as u64;
            stats.total_accesses += content.access_count;
            
            match content.metadata.content_type {
                ContentType::Audio => stats.audio_items += 1,
                ContentType::Video => stats.video_items += 1,
                ContentType::Image => stats.image_items += 1,
                ContentType::Document => stats.document_items += 1,
                ContentType::Other => stats.other_items += 1,
            }
        }
        
        stats
    }

    async fn evict_oldest(&self, cache: &mut HashMap<Uuid, CachedContent>) {
        if cache.is_empty() {
            return;
        }
        
        // Find the oldest accessed item
        let oldest_key = cache
            .iter()
            .min_by_key(|(_, content)| content.accessed_at)
            .map(|(key, _)| *key);
        
        if let Some(key) = oldest_key {
            cache.remove(&key);
        }
    }
}

#[derive(Debug, Default)]
pub struct CacheStats {
    pub total_items: u64,
    pub total_size: u64,
    pub total_accesses: u64,
    pub audio_items: u64,
    pub video_items: u64,
    pub image_items: u64,
    pub document_items: u64,
    pub other_items: u64,
} 