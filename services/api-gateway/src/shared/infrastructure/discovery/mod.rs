pub mod rss;
pub mod webhook;
pub mod service;
pub mod controllers;



use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    pub rss_feeds: Vec<String>,
    pub webhook_endpoints: Vec<String>,
    pub update_interval_seconds: u64,
    pub max_items_per_feed: usize,
    pub enable_caching: bool,
    pub cache_ttl_seconds: u64,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            rss_feeds: vec![
                "https://feeds.example.com/music".to_string(),
                "https://feeds.example.com/artists".to_string(),
            ],
            webhook_endpoints: vec![
                "https://api.spotify.com/webhooks".to_string(),
                "https://api.apple.com/webhooks".to_string(),
            ],
            update_interval_seconds: 300, // 5 minutes
            max_items_per_feed: 50,
            enable_caching: true,
            cache_ttl_seconds: 3600, // 1 hour
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryItem {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    pub source: String,
    pub content_type: DiscoveryContentType,
    pub published_at: DateTime<Utc>,
    pub discovered_at: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryContentType {
    Song,
    Album,
    Artist,
    Playlist,
    News,
    Event,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryResponse {
    pub success: bool,
    pub items: Vec<DiscoveryItem>,
    pub total_count: usize,
    pub next_page_token: Option<String>,
    pub error: Option<String>,
}

pub trait DiscoveryService: Send + Sync {
    async fn discover_content(&self, query: Option<String>, content_type: Option<DiscoveryContentType>) -> Result<DiscoveryResponse, Box<dyn std::error::Error>>;
    async fn subscribe_to_feed(&self, feed_url: String) -> Result<bool, Box<dyn std::error::Error>>;
    async fn register_webhook(&self, endpoint: String, events: Vec<String>) -> Result<bool, Box<dyn std::error::Error>>;
    async fn get_recent_discoveries(&self, limit: usize) -> Result<Vec<DiscoveryItem>, Box<dyn std::error::Error>>;
    async fn mark_as_processed(&self, item_id: Uuid) -> Result<bool, Box<dyn std::error::Error>>;
} 