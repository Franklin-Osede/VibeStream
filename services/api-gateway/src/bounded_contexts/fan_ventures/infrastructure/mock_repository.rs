use async_trait::async_trait;
use uuid::Uuid;
use crate::bounded_contexts::fan_ventures::domain::{
    entities::{ArtistVenture, FanInvestment, VentureBenefit},
};
use crate::bounded_contexts::fan_ventures::domain::repositories::ArtistVentureRepository;

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
            id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            title: "Mock Venture".to_string(),
            description: Some("Mock Description".to_string()),
            category: crate::bounded_contexts::fan_ventures::domain::entities::VentureCategory::Music,
            tags: vec!["mock".to_string()],
            risk_level: crate::bounded_contexts::fan_ventures::domain::entities::RiskLevel::Low,
            expected_return: 0.15,
            artist_rating: 4.5,
            artist_previous_ventures: 2,
            artist_success_rate: 0.8,
            funding_goal: 1000.0,
            current_funding: 500.0,
            min_investment: 10.0,
            max_investment: Some(1000.0),
            status: crate::bounded_contexts::fan_ventures::domain::entities::VentureStatus::Open,
            start_date: Some(chrono::Utc::now()),
            end_date: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            benefits: vec![],
        }))
    }
    
    async fn find_by_artist(&self, _artist_id: &Uuid) -> Result<Vec<ArtistVenture>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(vec![
            ArtistVenture {
                id: Uuid::new_v4(),
                artist_id: *_artist_id,
                title: "Mock Venture 1".to_string(),
                description: Some("Mock Description 1".to_string()),
                category: crate::bounded_contexts::fan_ventures::domain::entities::VentureCategory::Music,
                tags: vec!["mock".to_string()],
                risk_level: crate::bounded_contexts::fan_ventures::domain::entities::RiskLevel::Low,
                expected_return: 0.15,
                artist_rating: 4.5,
                artist_previous_ventures: 2,
                artist_success_rate: 0.8,
                funding_goal: 1000.0,
                current_funding: 500.0,
                min_investment: 10.0,
                max_investment: Some(1000.0),
                status: crate::bounded_contexts::fan_ventures::domain::entities::VentureStatus::Open,
                start_date: Some(chrono::Utc::now()),
                end_date: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                benefits: vec![],
            },
            ArtistVenture {
                id: Uuid::new_v4(),
                artist_id: *_artist_id,
                title: "Mock Venture 2".to_string(),
                description: Some("Mock Description 2".to_string()),
                category: crate::bounded_contexts::fan_ventures::domain::entities::VentureCategory::Music,
                tags: vec!["mock".to_string()],
                risk_level: crate::bounded_contexts::fan_ventures::domain::entities::RiskLevel::Low,
                expected_return: 0.15,
                artist_rating: 4.5,
                artist_previous_ventures: 2,
                artist_success_rate: 0.8,
                funding_goal: 2000.0,
                current_funding: 1500.0,
                min_investment: 10.0,
                max_investment: Some(1000.0),
                status: crate::bounded_contexts::fan_ventures::domain::entities::VentureStatus::Open,
                start_date: Some(chrono::Utc::now()),
                end_date: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                benefits: vec![],
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
                id: Uuid::new_v4(),
                artist_id: Uuid::new_v4(),
                title: "Active Venture 1".to_string(),
                description: Some("Active Description 1".to_string()),
                category: crate::bounded_contexts::fan_ventures::domain::entities::VentureCategory::Music,
                tags: vec!["active".to_string()],
                risk_level: crate::bounded_contexts::fan_ventures::domain::entities::RiskLevel::Low,
                expected_return: 0.15,
                artist_rating: 4.5,
                artist_previous_ventures: 2,
                artist_success_rate: 0.8,
                funding_goal: 1000.0,
                current_funding: 500.0,
                min_investment: 10.0,
                max_investment: Some(1000.0),
                status: crate::bounded_contexts::fan_ventures::domain::entities::VentureStatus::Open,
                start_date: Some(chrono::Utc::now()),
                end_date: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                benefits: vec![],
            },
            ArtistVenture {
                id: Uuid::new_v4(),
                artist_id: Uuid::new_v4(),
                title: "Active Venture 2".to_string(),
                description: Some("Active Description 2".to_string()),
                category: crate::bounded_contexts::fan_ventures::domain::entities::VentureCategory::Music,
                tags: vec!["active".to_string()],
                risk_level: crate::bounded_contexts::fan_ventures::domain::entities::RiskLevel::Low,
                expected_return: 0.15,
                artist_rating: 4.5,
                artist_previous_ventures: 2,
                artist_success_rate: 0.8,
                funding_goal: 2000.0,
                current_funding: 1500.0,
                min_investment: 10.0,
                max_investment: Some(1000.0),
                status: crate::bounded_contexts::fan_ventures::domain::entities::VentureStatus::Open,
                start_date: Some(chrono::Utc::now()),
                end_date: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                benefits: vec![],
            }
        ])
    }
} 