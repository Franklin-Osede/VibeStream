//! Adapter para el contexto de fan_ventures
//! 
//! Maneja la conversión entre vibestream_types y entidades locales
//! protegiendo el dominio de cambios en los contratos externos.

use super::{Adapter, AdapterError, AdapterConfig};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Importar tipos de vibestream_types (externos)
use vibestream_types::{
    ArtistVenture as ExternalArtistVenture,
    CreateVentureRequest as ExternalCreateVentureRequest,
    VentureBenefit as ExternalVentureBenefit,
    BenefitType as ExternalBenefitType,
};

// Importar entidades locales
use crate::bounded_contexts::fan_ventures::domain::entities::{
    ArtistVenture,
    CreateVentureRequest,
    VentureBenefit,
    BenefitType,
};

/// Adapter para el contexto de fan_ventures
pub struct FanVenturesAdapter {
    config: AdapterConfig,
}

impl FanVenturesAdapter {
    pub fn new(config: AdapterConfig) -> Self {
        Self { config }
    }
    
    /// Adaptar CreateVentureRequest externo a interno
    pub fn adapt_create_venture_request(
        &self,
        external: ExternalCreateVentureRequest,
    ) -> Result<CreateVentureRequest, AdapterError> {
        // Validar campos requeridos
        if external.artist_id.is_nil() {
            return Err(AdapterError::MissingRequiredField {
                field: "artist_id".to_string(),
            });
        }
        
        if external.title.is_empty() {
            return Err(AdapterError::MissingRequiredField {
                field: "title".to_string(),
            });
        }
        
        // Mapear campos con manejo de opcionales
        let request = CreateVentureRequest {
            artist_id: external.artist_id,
            title: external.title,
            description: external.description, // Campo opcional en ambos
            funding_goal: external.funding_goal,
            benefits: external.benefits
                .into_iter()
                .map(|b| self.adapt_venture_benefit(b))
                .collect::<Result<Vec<_>, _>>()?,
        };
        
        // Validaciones adicionales según configuración
        if self.config.strict_validation {
            if request.funding_goal <= 0.0 {
                return Err(AdapterError::ValueOutOfRange {
                    field: "funding_goal".to_string(),
                    min: "0.01".to_string(),
                    max: "1000000.0".to_string(),
                });
            }
            
            if request.benefits.is_empty() {
                return Err(AdapterError::ValidationError {
                    message: "At least one benefit is required".to_string(),
                });
            }
        }
        
        Ok(request)
    }
    
    /// Adaptar VentureBenefit externo a interno
    pub fn adapt_venture_benefit(
        &self,
        external: ExternalVentureBenefit,
    ) -> Result<VentureBenefit, AdapterError> {
        if external.title.is_empty() {
            return Err(AdapterError::MissingRequiredField {
                field: "benefit.title".to_string(),
            });
        }
        
        let benefit = VentureBenefit {
            title: external.title,
            description: external.description, // Campo opcional
            benefit_type: self.adapt_benefit_type(external.benefit_type)?,
            min_investment: external.min_investment,
        };
        
        Ok(benefit)
    }
    
    /// Adaptar BenefitType externo a interno
    pub fn adapt_benefit_type(
        &self,
        external: ExternalBenefitType,
    ) -> Result<BenefitType, AdapterError> {
        match external {
            ExternalBenefitType::DigitalContent => Ok(BenefitType::DigitalContent),
            ExternalBenefitType::ExclusiveAccess => Ok(BenefitType::ExclusiveAccess),
            ExternalBenefitType::Merchandise => Ok(BenefitType::Merchandise),
            ExternalBenefitType::ConcertTickets => Ok(BenefitType::ConcertTickets),
            ExternalBenefitType::RevenueShare => Ok(BenefitType::RevenueShare),
            _ => Err(AdapterError::TypeMismatch {
                field: "benefit_type".to_string(),
                expected: "Known benefit type".to_string(),
                actual: format!("{:?}", external),
            }),
        }
    }
    
    /// Adaptar ArtistVenture interno a externo
    pub fn adapt_artist_venture_to_external(
        &self,
        internal: &ArtistVenture,
    ) -> Result<ExternalArtistVenture, AdapterError> {
        Ok(ExternalArtistVenture {
            venture_id: internal.id, // Mapear id interno a venture_id externo
            artist_id: internal.artist_id,
            title: internal.title.clone(),
            description: internal.description.clone(), // Campo opcional en ambos
            funding_goal: internal.funding_goal,
            current_funding: internal.current_funding,
            status: internal.status.clone(),
            created_at: internal.created_at,
            updated_at: internal.updated_at,
            benefits: internal.benefits
                .iter()
                .map(|b| self.adapt_venture_benefit_to_external(b))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
    
    /// Adaptar ArtistVenture externo a interno
    pub fn adapt_artist_venture_from_external(
        &self,
        external: ExternalArtistVenture,
    ) -> Result<ArtistVenture, AdapterError> {
        // Validar campos requeridos
        if external.venture_id.is_nil() {
            return Err(AdapterError::MissingRequiredField {
                field: "venture_id".to_string(),
            });
        }
        
        if external.artist_id.is_nil() {
            return Err(AdapterError::MissingRequiredField {
                field: "artist_id".to_string(),
            });
        }
        
        if external.title.is_empty() {
            return Err(AdapterError::MissingRequiredField {
                field: "title".to_string(),
            });
        }
        
        // Mapear campos con manejo de opcionales
        let venture = ArtistVenture {
            id: external.venture_id, // Mapear venture_id externo a id interno
            artist_id: external.artist_id,
            title: external.title,
            description: external.description, // Campo opcional
            funding_goal: external.funding_goal,
            current_funding: external.current_funding,
            status: external.status,
            created_at: external.created_at,
            updated_at: external.updated_at,
            benefits: external.benefits
                .into_iter()
                .map(|b| self.adapt_venture_benefit_from_external(b))
                .collect::<Result<Vec<_>, _>>()?,
        };
        
        Ok(venture)
    }
    
    /// Adaptar VentureBenefit interno a externo
    pub fn adapt_venture_benefit_to_external(
        &self,
        internal: &VentureBenefit,
    ) -> Result<ExternalVentureBenefit, AdapterError> {
        Ok(ExternalVentureBenefit {
            title: internal.title.clone(),
            description: internal.description.clone(), // Campo opcional
            benefit_type: self.adapt_benefit_type_to_external(&internal.benefit_type)?,
            min_investment: internal.min_investment,
        })
    }
    
    /// Adaptar VentureBenefit externo a interno
    pub fn adapt_venture_benefit_from_external(
        &self,
        external: ExternalVentureBenefit,
    ) -> Result<VentureBenefit, AdapterError> {
        self.adapt_venture_benefit(external)
    }
    
    /// Adaptar BenefitType interno a externo
    pub fn adapt_benefit_type_to_external(
        &self,
        internal: &BenefitType,
    ) -> Result<ExternalBenefitType, AdapterError> {
        match internal {
            BenefitType::DigitalContent => Ok(ExternalBenefitType::DigitalContent),
            BenefitType::ExclusiveAccess => Ok(ExternalBenefitType::ExclusiveAccess),
            BenefitType::Merchandise => Ok(ExternalBenefitType::Merchandise),
            BenefitType::ConcertTickets => Ok(ExternalBenefitType::ConcertTickets),
            BenefitType::RevenueShare => Ok(ExternalBenefitType::RevenueShare),
        }
    }
}

// Implementar trait Adapter para compatibilidad
impl Adapter<ExternalCreateVentureRequest, CreateVentureRequest> for FanVenturesAdapter {
    fn adapt(&self, input: ExternalCreateVentureRequest) -> Result<CreateVentureRequest, AdapterError> {
        self.adapt_create_venture_request(input)
    }
}

impl Adapter<ExternalArtistVenture, ArtistVenture> for FanVenturesAdapter {
    fn adapt(&self, input: ExternalArtistVenture) -> Result<ArtistVenture, AdapterError> {
        self.adapt_artist_venture_from_external(input)
    }
}

// DTOs específicos para la capa de presentación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVentureRequestDto {
    pub artist_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub funding_goal: f64,
    pub benefits: Vec<VentureBenefitDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentureBenefitDto {
    pub title: String,
    pub description: Option<String>,
    pub benefit_type: String,
    pub min_investment: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistVentureResponse {
    pub venture_id: Uuid,
    pub artist_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub funding_goal: f64,
    pub current_funding: f64,
    pub funding_percentage: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub benefits: Vec<VentureBenefitResponse>,
    pub investor_count: u32,
    pub days_remaining: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentureBenefitResponse {
    pub title: String,
    pub description: Option<String>,
    pub benefit_type: String,
    pub min_investment: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentureListResponse {
    pub ventures: Vec<ArtistVentureResponse>,
    pub total_count: u64,
    pub page: u32,
    pub limit: u32,
}

// Tests unitarios para el adapter
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_adapt_create_venture_request_success() {
        let adapter = FanVenturesAdapter::new(AdapterConfig::default());
        let artist_id = Uuid::new_v4();
        
        let external = ExternalCreateVentureRequest {
            artist_id,
            title: "Test Venture".to_string(),
            description: Some("Test Description".to_string()),
            funding_goal: 1000.0,
            benefits: vec![
                ExternalVentureBenefit {
                    title: "Digital Content".to_string(),
                    description: Some("Exclusive songs".to_string()),
                    benefit_type: ExternalBenefitType::DigitalContent,
                    min_investment: 25.0,
                }
            ],
        };
        
        let result = adapter.adapt_create_venture_request(external);
        assert!(result.is_ok());
        
        let request = result.unwrap();
        assert_eq!(request.artist_id, artist_id);
        assert_eq!(request.title, "Test Venture");
        assert_eq!(request.description, Some("Test Description".to_string()));
        assert_eq!(request.funding_goal, 1000.0);
        assert_eq!(request.benefits.len(), 1);
    }
    
    #[test]
    fn test_adapt_create_venture_request_missing_title() {
        let adapter = FanVenturesAdapter::new(AdapterConfig::default());
        let artist_id = Uuid::new_v4();
        
        let external = ExternalCreateVentureRequest {
            artist_id,
            title: "".to_string(),
            description: None,
            funding_goal: 1000.0,
            benefits: vec![],
        };
        
        let result = adapter.adapt_create_venture_request(external);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            AdapterError::MissingRequiredField { field } => {
                assert_eq!(field, "title");
            }
            _ => panic!("Expected MissingRequiredField error"),
        }
    }
    
    #[test]
    fn test_adapt_artist_venture_id_mapping() {
        let adapter = FanVenturesAdapter::new(AdapterConfig::default());
        let venture_id = Uuid::new_v4();
        let artist_id = Uuid::new_v4();
        
        let external = ExternalArtistVenture {
            venture_id,
            artist_id,
            title: "Test Venture".to_string(),
            description: Some("Test Description".to_string()),
            funding_goal: 1000.0,
            current_funding: 500.0,
            status: "active".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            benefits: vec![],
        };
        
        let result = adapter.adapt_artist_venture_from_external(external);
        assert!(result.is_ok());
        
        let venture = result.unwrap();
        assert_eq!(venture.id, venture_id); // venture_id externo mapeado a id interno
        assert_eq!(venture.artist_id, artist_id);
    }
}
