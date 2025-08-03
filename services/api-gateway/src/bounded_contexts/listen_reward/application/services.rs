use std::sync::Arc;
use serde_json::Value;

/// Mock application service for Listen Reward context
#[derive(Clone)]
pub struct MockListenRewardApplicationService;

impl MockListenRewardApplicationService {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn create_session(&self, _request: Value) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
    
    pub async fn get_all_sessions(&self) -> Result<Vec<()>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(vec![])
    }
} 