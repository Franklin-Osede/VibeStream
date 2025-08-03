use async_trait::async_trait;
use uuid::Uuid;
use crate::bounded_contexts::campaign::domain::repositories::CampaignRepository;

/// Mock repository for Campaign context testing
#[derive(Debug, Clone)]
pub struct MockCampaignRepository;

#[async_trait]
impl CampaignRepository for MockCampaignRepository {
    async fn find_by_id(&self, _campaign_id: &Uuid) -> Result<Option<()>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Some(()))
    }
    
    async fn find_all(&self) -> Result<Vec<()>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(vec![])
    }
    
    async fn create(&self, _campaign: &()) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
    
    async fn update(&self, _campaign: &()) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
    
    async fn delete(&self, _campaign_id: &Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
} 