//! Anti-Corruption Layer (ACL) para mapear entre vibestream_types y entidades locales
//! 
//! Esta capa protege el dominio interno de cambios en los contratos externos
//! y proporciona una interfaz estable para la aplicación.

pub mod listen_reward;
pub mod fan_ventures;
pub mod notifications;
pub mod music;

use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Trait base para todos los adapters
pub trait Adapter<TInput, TOutput> {
    fn adapt(&self, input: TInput) -> Result<TOutput, AdapterError>;
}

/// Errores específicos de adaptación
#[derive(Debug, thiserror::Error)]
pub enum AdapterError {
    #[error("Campo requerido faltante: {field}")]
    MissingRequiredField { field: String },
    
    #[error("Tipo de dato incompatible: {field} esperaba {expected}, recibió {actual}")]
    TypeMismatch { field: String, expected: String, actual: String },
    
    #[error("Valor fuera de rango: {field} debe estar entre {min} y {max}")]
    ValueOutOfRange { field: String, min: String, max: String },
    
    #[error("Error de validación: {message}")]
    ValidationError { message: String },
    
    #[error("Error interno del adapter: {message}")]
    InternalError { message: String },
}

/// Configuración global para adapters
#[derive(Clone)]
pub struct AdapterConfig {
    pub strict_validation: bool,
    pub allow_missing_optional_fields: bool,
    pub default_values: DefaultValues,
}

#[derive(Clone)]
pub struct DefaultValues {
    pub default_currency: String,
    pub default_locale: String,
    pub default_timezone: String,
}

impl Default for AdapterConfig {
    fn default() -> Self {
        Self {
            strict_validation: true,
            allow_missing_optional_fields: true,
            default_values: DefaultValues {
                default_currency: "USD".to_string(),
                default_locale: "en-US".to_string(),
                default_timezone: "UTC".to_string(),
            },
        }
    }
}

/// Registry central de adapters
#[derive(Clone)]
pub struct AdapterRegistry {
    pub config: AdapterConfig,
    pub listen_reward_adapter: Arc<listen_reward::ListenRewardAdapter>,
    pub fan_ventures_adapter: Arc<fan_ventures::FanVenturesAdapter>,
    pub notifications_adapter: Arc<notifications::NotificationsAdapter>,
    pub music_adapter: Arc<music::MusicAdapter>,
}

impl AdapterRegistry {
    pub fn new(config: AdapterConfig) -> Self {
        Self {
            listen_reward_adapter: Arc::new(listen_reward::ListenRewardAdapter::new(config.clone())),
            fan_ventures_adapter: Arc::new(fan_ventures::FanVenturesAdapter::new(config.clone())),
            notifications_adapter: Arc::new(notifications::NotificationsAdapter::new(config.clone())),
            music_adapter: Arc::new(music::MusicAdapter::new(config.clone())),
            config,
        }
    }
    
    pub fn default() -> Self {
        Self::new(AdapterConfig::default())
    }
}

/// DTOs comunes para la capa de presentación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub request_id: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: Utc::now(),
            request_id: None,
        }
    }
    
    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            timestamp: Utc::now(),
            request_id: None,
        }
    }
}

/// Paginación estándar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<SortOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInfo {
    pub page: u32,
    pub limit: u32,
    pub total: u64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_prev: bool,
}
