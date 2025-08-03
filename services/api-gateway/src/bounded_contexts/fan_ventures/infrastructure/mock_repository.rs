use async_trait::async_trait;
use uuid::Uuid;
use crate::bounded_contexts::fan_ventures::domain::{
    entities::{ArtistVenture, FanInvestment, VentureBenefit},
    repositories::ArtistVentureRepository,
};

/// Repositorio mock para testing de Fan Ventures
#[derive(Debug, Clone)]
pub struct MockArtistVentureRepository;

#[async_trait]
impl ArtistVentureRepository for MockArtistVentureRepository {
    async fn create(&self, _venture: &ArtistVenture) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
    
    async fn find_by_id(&self, _venture_id: &Uuid) -> Result<Option<ArtistVenture>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(Some(ArtistVenture {
            venture_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            title: "Mock Venture".to_string(),
            description: "Mock Description".to_string(),
            funding_goal: 1000.0,
            current_funding: 500.0,
            status: crate::bounded_contexts::fan_ventures::domain::entities::VentureStatus::Active,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }))
    }
    
    async fn find_by_artist(&self, _artist_id: &Uuid) -> Result<Vec<ArtistVenture>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(vec![
            ArtistVenture {
                venture_id: Uuid::new_v4(),
                artist_id: *_artist_id,
                title: "Mock Venture 1".to_string(),
                description: "Mock Description 1".to_string(),
                funding_goal: 1000.0,
                current_funding: 500.0,
                status: crate::bounded_contexts::fan_ventures::domain::entities::VentureStatus::Active,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
            ArtistVenture {
                venture_id: Uuid::new_v4(),
                artist_id: *_artist_id,
                title: "Mock Venture 2".to_string(),
                description: "Mock Description 2".to_string(),
                funding_goal: 2000.0,
                current_funding: 1500.0,
                status: crate::bounded_contexts::fan_ventures::domain::entities::VentureStatus::Active,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }
        ])
    }
    
    async fn update(&self, _venture: &ArtistVenture) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
    
    async fn delete(&self, _venture_id: &Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
    
    async fn find_all_active(&self) -> Result<Vec<ArtistVenture>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(vec![
            ArtistVenture {
                venture_id: Uuid::new_v4(),
                artist_id: Uuid::new_v4(),
                title: "Active Venture 1".to_string(),
                description: "Active Description 1".to_string(),
                funding_goal: 1000.0,
                current_funding: 500.0,
                status: crate::bounded_contexts::fan_ventures::domain::entities::VentureStatus::Active,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
            ArtistVenture {
                venture_id: Uuid::new_v4(),
                artist_id: Uuid::new_v4(),
                title: "Active Venture 2".to_string(),
                description: "Active Description 2".to_string(),
                funding_goal: 2000.0,
                current_funding: 1500.0,
                status: crate::bounded_contexts::fan_ventures::domain::entities::VentureStatus::Active,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }
        ])
    }
} 