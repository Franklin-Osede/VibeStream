use super::{rss::RSSService, webhook::WebhookService};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryConfig {
    pub enable_rss: bool,
    pub enable_webhooks: bool,
    pub max_rss_feeds: usize,
    pub max_webhook_endpoints: usize,
    pub refresh_interval_seconds: u64,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            enable_rss: true,
            enable_webhooks: true,
            max_rss_feeds: 100,
            max_webhook_endpoints: 50,
            refresh_interval_seconds: 300, // 5 minutes
        }
    }
}

pub struct DiscoveryService {
    config: DiscoveryConfig,
    rss_service: RSSService,
    webhook_service: WebhookService,
}

impl DiscoveryService {
    pub fn new(config: DiscoveryConfig) -> Self {
        Self {
            config,
            rss_service: RSSService::new(),
            webhook_service: WebhookService::new(),
        }
    }

    pub async fn add_rss_feed(&mut self, url: String, title: String, description: Option<String>) -> Result<Uuid, Box<dyn std::error::Error + Send + Sync>> {
        if !self.config.enable_rss {
            return Err("RSS feeds are disabled".into());
        }

        let feeds = self.rss_service.get_feeds().await;
        if feeds.len() >= self.config.max_rss_feeds {
            return Err("Maximum number of RSS feeds reached".into());
        }

        Ok(self.rss_service.add_feed(url, title, description).await)
    }

    pub async fn remove_rss_feed(&mut self, feed_id: Uuid) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        if !self.config.enable_rss {
            return Err("RSS feeds are disabled".into());
        }

        Ok(self.rss_service.remove_feed(feed_id).await)
    }

    pub async fn register_webhook(&mut self, url: String, name: String, description: Option<String>, events: Vec<String>) -> Result<Uuid, Box<dyn std::error::Error + Send + Sync>> {
        if !self.config.enable_webhooks {
            return Err("Webhooks are disabled".into());
        }

        let endpoints = self.webhook_service.get_endpoints().await;
        if endpoints.len() >= self.config.max_webhook_endpoints {
            return Err("Maximum number of webhook endpoints reached".into());
        }

        Ok(self.webhook_service.register_endpoint(url, name, description, events).await)
    }

    pub async fn unregister_webhook(&mut self, endpoint_id: Uuid) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        if !self.config.enable_webhooks {
            return Err("Webhooks are disabled".into());
        }

        Ok(self.webhook_service.unregister_endpoint(endpoint_id).await)
    }

    pub async fn search_content(&self, query: &str) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
        let mut results = Vec::new();

        // Search in RSS feeds
        if self.config.enable_rss {
            if let Ok(rss_items) = self.rss_service.search_items(query).await {
                for item in rss_items {
                    results.push(serde_json::to_value(item)?);
                }
            }
        }

        // In a real implementation, you would also search in:
        // - Internal content database
        // - External APIs
        // - Social media feeds
        // - etc.

        Ok(results)
    }

    pub async fn trigger_event(&mut self, event_type: &str, data: serde_json::Value) -> Result<u32, Box<dyn std::error::Error + Send + Sync>> {
        if !self.config.enable_webhooks {
            return Ok(0);
        }

        self.webhook_service.trigger_event(event_type, data).await
    }

    pub async fn get_stats(&self) -> serde_json::Value {
        let rss_feeds = self.rss_service.get_feeds().await;
        let webhook_endpoints = self.webhook_service.get_endpoints().await;

        serde_json::json!({
            "rss": {
                "enabled": self.config.enable_rss,
                "total_feeds": rss_feeds.len(),
                "active_feeds": rss_feeds.iter().filter(|f| f.is_active).count(),
            },
            "webhooks": {
                "enabled": self.config.enable_webhooks,
                "total_endpoints": webhook_endpoints.len(),
                "active_endpoints": webhook_endpoints.iter().filter(|e| e.is_active).count(),
            },
            "config": {
                "max_rss_feeds": self.config.max_rss_feeds,
                "max_webhook_endpoints": self.config.max_webhook_endpoints,
                "refresh_interval_seconds": self.config.refresh_interval_seconds,
            }
        })
    }
} 