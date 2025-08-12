use std::sync::Arc;
use async_trait::async_trait;
use serde_json::Value;
use uuid::Uuid;
use sqlx::PgPool;
use crate::bounded_contexts::fan_ventures::{
    domain::entities::{ArtistVenture, FanInvestment, VentureBenefit},
    infrastructure::postgres_repository::PostgresFanVenturesRepository,
};

/// Application service for Fan Ventures following DDD principles
/// Orchestrates use cases and coordinates between domain and infrastructure
#[derive(Clone)]
pub struct FanVenturesApplicationService {
    venture_repository: Arc<PostgresFanVenturesRepository>,
}

impl FanVenturesApplicationService {
    /// Create a new FanVenturesApplicationService
    pub fn new(
        venture_repository: Arc<PostgresFanVenturesRepository>,
    ) -> Self {
        Self { venture_repository }
    }
    
    /// Create a new venture (Use Case: CreateVenture)
    pub async fn create_venture(&self, request: crate::bounded_contexts::fan_ventures::presentation::controllers::CreateVentureRequest) -> Result<ArtistVenture, Box<dyn std::error::Error + Send + Sync>> {
        // Validate request
        let artist_id = request.artist_id;
        let title = request.title;
        let description = request.description;
        let funding_goal = request.funding_goal;
            
        // Create venture entity
        let venture = ArtistVenture {
            id: Uuid::new_v4(),
            artist_id,
            title,
            description: Some(description),
            category: crate::bounded_contexts::fan_ventures::domain::entities::VentureCategory::Other,
            tags: vec![],
            risk_level: crate::bounded_contexts::fan_ventures::domain::entities::RiskLevel::Medium,
            expected_return: 0.0,
            artist_rating: 0.0,
            artist_previous_ventures: 0,
            artist_success_rate: 0.0,
            funding_goal,
            current_funding: 0.0,
            min_investment: 0.0,
            max_investment: None,
            status: crate::bounded_contexts::fan_ventures::domain::entities::VentureStatus::Draft,
            start_date: None,
            end_date: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            benefits: Vec::new(),
        };
        
        // Save to repository
        self.venture_repository.create_venture(&venture).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        
        Ok(venture)
    }
    
    /// Get all ventures (Use Case: ListVentures)
    pub async fn get_all_ventures(&self) -> Result<Vec<ArtistVenture>, Box<dyn std::error::Error + Send + Sync>> {
        self.venture_repository.list_open_ventures(None).await.map_err(|e| e.into())
    }
    
    /// Get venture by ID (Use Case: GetVentureDetails)
    pub async fn get_venture_by_id(&self, venture_id: &Uuid) -> Result<Option<ArtistVenture>, Box<dyn std::error::Error + Send + Sync>> {
        self.venture_repository.get_venture(*venture_id).await.map_err(|e| e.into())
    }
    
    /// Get ventures by artist (Use Case: GetArtistVentures)
    pub async fn get_ventures_by_artist(&self, artist_id: &Uuid) -> Result<Vec<ArtistVenture>, Box<dyn std::error::Error + Send + Sync>> {
        self.venture_repository.list_ventures_by_artist(*artist_id).await.map_err(|e| e.into())
    }
}

/// Mock implementation for testing
#[derive(Clone)]
pub struct MockFanVenturesApplicationService;

impl MockFanVenturesApplicationService {
    /// Create a mock service for testing
    pub fn new() -> Self { Self }
    
    pub async fn create_venture(&self, _request: crate::bounded_contexts::fan_ventures::presentation::controllers::CreateVentureRequest) -> Result<ArtistVenture, Box<dyn std::error::Error + Send + Sync>> {
        Ok(ArtistVenture {
            id: Uuid::new_v4(),
            artist_id: Uuid::new_v4(),
            title: "Mock Venture".to_string(),
            description: Some("Mock Description".to_string()),
            category: crate::bounded_contexts::fan_ventures::domain::entities::VentureCategory::Other,
            tags: vec![],
            risk_level: crate::bounded_contexts::fan_ventures::domain::entities::RiskLevel::Medium,
            expected_return: 0.0,
            artist_rating: 0.0,
            artist_previous_ventures: 0,
            artist_success_rate: 0.0,
            funding_goal: 1000.0,
            current_funding: 0.0,
            min_investment: 0.0,
            max_investment: None,
            status: crate::bounded_contexts::fan_ventures::domain::entities::VentureStatus::Open,
            start_date: None,
            end_date: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            benefits: Vec::new(),
        })
    }
    
    pub async fn get_all_ventures(&self) -> Result<Vec<ArtistVenture>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(vec![])
    }
    
    pub async fn get_venture_by_id(&self, _venture_id: &Uuid) -> Result<Option<ArtistVenture>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(None)
    }
    
    pub async fn get_ventures_by_artist(&self, _artist_id: &Uuid) -> Result<Vec<ArtistVenture>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(vec![])
    }
} 