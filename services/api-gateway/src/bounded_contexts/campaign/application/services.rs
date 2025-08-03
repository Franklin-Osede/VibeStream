use std::sync::Arc;
use serde_json::Value;

/// Mock application service for Campaign context
#[derive(Clone)]
pub struct MockCampaignApplicationService;

impl MockCampaignApplicationService {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn create_campaign(&self, _request: Value) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
    
    pub async fn get_all_campaigns(&self) -> Result<Vec<()>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(vec![])
    }
} 