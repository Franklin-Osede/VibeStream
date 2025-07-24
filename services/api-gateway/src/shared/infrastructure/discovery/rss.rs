use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RSSFeed {
    pub id: Uuid,
    pub url: String,
    pub title: String,
    pub description: Option<String>,
    pub last_updated: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RSSItem {
    pub title: String,
    pub description: Option<String>,
    pub link: String,
    pub pub_date: Option<DateTime<Utc>>,
    pub author: Option<String>,
    pub categories: Vec<String>,
}

pub struct RSSService {
    feeds: Vec<RSSFeed>,
}

impl RSSService {
    pub fn new() -> Self {
        Self {
            feeds: Vec::new(),
        }
    }

    pub async fn add_feed(&mut self, url: String, title: String, description: Option<String>) -> Uuid {
        let feed = RSSFeed {
            id: Uuid::new_v4(),
            url,
            title,
            description,
            last_updated: Utc::now(),
            is_active: true,
        };
        
        let id = feed.id;
        self.feeds.push(feed);
        id
    }

    pub async fn remove_feed(&mut self, feed_id: Uuid) -> bool {
        if let Some(index) = self.feeds.iter().position(|f| f.id == feed_id) {
            self.feeds.remove(index);
            true
        } else {
            false
        }
    }

    pub async fn get_feeds(&self) -> Vec<RSSFeed> {
        self.feeds.clone()
    }

    pub async fn fetch_latest_items(&self, _feed_id: Uuid) -> Result<Vec<RSSItem>, Box<dyn std::error::Error + Send + Sync>> {
        // In a real implementation, this would fetch from the RSS feed URL
        // For now, return mock data
        Ok(vec![
            RSSItem {
                title: "New Music Release".to_string(),
                description: Some("Latest music from trending artists".to_string()),
                link: "https://example.com/music/1".to_string(),
                pub_date: Some(Utc::now()),
                author: Some("Music Blog".to_string()),
                categories: vec!["music".to_string(), "release".to_string()],
            }
        ])
    }

    pub async fn search_items(&self, query: &str) -> Result<Vec<RSSItem>, Box<dyn std::error::Error + Send + Sync>> {
        // In a real implementation, this would search across all RSS feeds
        // For now, return mock data
        Ok(vec![
            RSSItem {
                title: format!("Search result for: {}", query),
                description: Some("Found in RSS feeds".to_string()),
                link: "https://example.com/search/1".to_string(),
                pub_date: Some(Utc::now()),
                author: Some("Search Engine".to_string()),
                categories: vec!["search".to_string()],
            }
        ])
    }
} 