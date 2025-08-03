use std::sync::Arc;
use async_trait::async_trait;
use serde_json::Value;
use uuid::Uuid;
use sqlx::PgPool;
use crate::bounded_contexts::fan_ventures::{
    domain::{
        entities::{ArtistVenture, FanInvestment, VentureBenefit},
        repositories::ArtistVentureRepository,
    },
    infrastructure::repositories::{
        PostgresVentureBenefitRepository,
        PostgresFanInvestmentRepository,
    },
};

/// Application service for Fan Ventures following DDD principles
/// Orchestrates use cases and coordinates between domain and infrastructure
#[derive(Clone)]
pub struct FanVenturesApplicationService {
    venture_repository: Arc<dyn ArtistVentureRepository + Send + Sync>,
    benefit_repository: Arc<PostgresVentureBenefitRepository>,
    investment_repository: Arc<PostgresFanInvestmentRepository>,
}

impl FanVenturesApplicationService {
    /// Create a new FanVenturesApplicationService
    pub fn new(
        venture_repository: Arc<dyn ArtistVentureRepository + Send + Sync>,
        benefit_repository: Arc<PostgresVentureBenefitRepository>,
        investment_repository: Arc<PostgresFanInvestmentRepository>,
    ) -> Self {
        Self {
            venture_repository,
            benefit_repository,
            investment_repository,
        }
    }
    
    /// Create a new venture (Use Case: CreateVenture)
    pub async fn create_venture(&self, request: Value) -> Result<ArtistVenture, Box<dyn std::error::Error + Send + Sync>> {
        // Validate request
        let artist_id = request["artist_id"]
            .as_str()
            .ok_or("artist_id is required")?
            .parse::<Uuid>()
            .map_err(|_| "Invalid artist_id")?;
            
        let title = request["title"]
            .as_str()
            .ok_or("title is required")?
            .to_string();
            
        let description = request["description"]
            .as_str()
            .ok_or("description is required")?
            .to_string();
            
        let funding_goal = request["funding_goal"]
            .as_f64()
            .ok_or("funding_goal is required")?;
            
        // Create venture entity
        let venture = ArtistVenture {
            venture_id: Uuid::new_v4(),
            artist_id,
            title,
            description,
            funding_goal,
            current_funding: 0.0,
            status: crate::bounded_contexts::fan_ventures::domain::entities::VentureStatus::Active,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        // Save to repository
        self.venture_repository.create(&venture).await?;
        
        // Create benefits if provided
        if let Some(benefits) = request["benefits"].as_array() {
            for benefit_data in benefits {
                let benefit = VentureBenefit {
                    benefit_id: Uuid::new_v4(),
                    venture_id: venture.venture_id,
                    title: benefit_data["title"].as_str().unwrap_or("").to_string(),
                    description: benefit_data["description"].as_str().unwrap_or("").to_string(),
                    benefit_type: crate::bounded_contexts::fan_ventures::domain::entities::BenefitType::DigitalContent, // Default
                    min_investment: benefit_data["min_investment"].as_f64().unwrap_or(0.0),
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                };
                
                self.benefit_repository.create(&benefit).await?;
            }
        }
        
        Ok(venture)
    }
    
    /// Get all ventures (Use Case: ListVentures)
    pub async fn get_all_ventures(&self) -> Result<Vec<ArtistVenture>, Box<dyn std::error::Error + Send + Sync>> {
        self.venture_repository.find_all_active().await
    }
    
    /// Get venture by ID (Use Case: GetVentureDetails)
    pub async fn get_venture_by_id(&self, venture_id: &Uuid) -> Result<Option<ArtistVenture>, Box<dyn std::error::Error + Send + Sync>> {
        self.venture_repository.find_by_id(venture_id).await
    }
    
    /// Get ventures by artist (Use Case: GetArtistVentures)
    pub async fn get_ventures_by_artist(&self, artist_id: &Uuid) -> Result<Vec<ArtistVenture>, Box<dyn std::error::Error + Send + Sync>> {
        self.venture_repository.find_by_artist(artist_id).await
    }
}

/// Mock implementation for testing
#[derive(Clone)]
pub struct MockFanVenturesApplicationService;

impl MockFanVenturesApplicationService {
    /// Create a mock service for testing
    pub fn new() -> Self {
        Self
    }
    
    /// Mock create venture
    pub async fn create_venture(&self, _request: Value) -> Result<ArtistVenture, Box<dyn std::error::Error + Send + Sync>> {
        Ok(ArtistVenture {
            venture_id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            title: "Mock Venture".to_string(),
            description: "Mock Description".to_string(),
            funding_goal: 1000.0,
            current_funding: 0.0,
            status: crate::bounded_contexts::fan_ventures::domain::entities::VentureStatus::Active,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }
    
    /// Mock get all ventures
    pub async fn get_all_ventures(&self) -> Result<Vec<ArtistVenture>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(vec![
            ArtistVenture {
                venture_id: Uuid::new_v4(),
                artist_id: Uuid::new_v4(),
                title: "Mock Venture 1".to_string(),
                description: "Mock Description 1".to_string(),
                funding_goal: 1000.0,
                current_funding: 500.0,
                status: crate::bounded_contexts::fan_ventures::domain::entities::VentureStatus::Active,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }
        ])
    }
    
    /// Mock get venture by id
    pub async fn get_venture_by_id(&self, _venture_id: &Uuid) -> Result<Option<ArtistVenture>, Box<dyn std::error::Error + Send + Sync>> {
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
    
    /// Mock get ventures by artist
    pub async fn get_ventures_by_artist(&self, _artist_id: &Uuid) -> Result<Vec<ArtistVenture>, Box<dyn std::error::Error + Send + Sync>> {
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
            }
        ])
    }
} 