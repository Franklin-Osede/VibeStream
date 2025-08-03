use async_trait::async_trait;
use uuid::Uuid;
use crate::bounded_contexts::user::domain::repositories::UserRepository;

/// Mock repository for User context testing
#[derive(Debug, Clone)]
pub struct MockUserRepository;

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn find_by_id(&self, _user_id: &Uuid) -> Result<Option<()>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Some(()))
    }
    
    async fn find_all(&self) -> Result<Vec<()>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(vec![])
    }
    
    async fn create(&self, _user: &()) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
    
    async fn update(&self, _user: &()) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
    
    async fn delete(&self, _user_id: &Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
} 