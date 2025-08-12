//! Adapter para el contexto de listen_reward
//! 
//! Maneja la conversión entre vibestream_types y entidades locales
//! protegiendo el dominio de cambios en los contratos externos.

use super::{Adapter, AdapterError, AdapterConfig};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Importar tipos de vibestream_types (externos)
use vibestream_types::{
    StartListenSessionCommand as ExternalStartListenSessionCommand,
    ListenSession as ExternalListenSession,
    ListenReward as ExternalListenReward,
};

// Importar entidades locales
use crate::bounded_contexts::listen_reward::domain::entities::{
    StartListenSessionCommand,
    ListenSession,
    ListenReward,
};

/// Adapter para el contexto de listen_reward
pub struct ListenRewardAdapter {
    config: AdapterConfig,
}

impl ListenRewardAdapter {
    pub fn new(config: AdapterConfig) -> Self {
        Self { config }
    }
    
    /// Adaptar StartListenSessionCommand externo a interno
    pub fn adapt_start_listen_command(
        &self,
        external: ExternalStartListenSessionCommand,
    ) -> Result<StartListenSessionCommand, AdapterError> {
        // Validar campos requeridos
        if external.user_id.is_nil() {
            return Err(AdapterError::MissingRequiredField {
                field: "user_id".to_string(),
            });
        }
        
        // Mapear campos con validación
        let command = StartListenSessionCommand {
            user_id: external.user_id,
            song_id: external.song_id.unwrap_or_else(Uuid::nil), // Campo opcional en externo
            artist_id: external.artist_id.unwrap_or_else(Uuid::nil), // Campo opcional en externo
            session_duration: external.session_duration.unwrap_or(300), // Default 5 minutos
            timestamp: external.timestamp.unwrap_or_else(Utc::now),
        };
        
        // Validaciones adicionales según configuración
        if self.config.strict_validation {
            if command.session_duration == 0 {
                return Err(AdapterError::ValueOutOfRange {
                    field: "session_duration".to_string(),
                    min: "1".to_string(),
                    max: "3600".to_string(),
                });
            }
        }
        
        Ok(command)
    }
    
    /// Adaptar ListenSession interno a externo
    pub fn adapt_listen_session_to_external(
        &self,
        internal: &ListenSession,
    ) -> Result<ExternalListenSession, AdapterError> {
        Ok(ExternalListenSession {
            session_id: internal.session_id,
            user_id: internal.user_id,
            song_id: Some(internal.song_id), // Campo requerido en interno, opcional en externo
            artist_id: Some(internal.artist_id), // Campo requerido en interno, opcional en externo
            start_time: internal.start_time,
            end_time: internal.end_time,
            duration_seconds: internal.duration_seconds,
            reward_amount: Some(internal.reward_amount), // Campo requerido en interno, opcional en externo
            status: internal.status.clone(),
        })
    }
    
    /// Adaptar ListenSession externo a interno
    pub fn adapt_listen_session_from_external(
        &self,
        external: ExternalListenSession,
    ) -> Result<ListenSession, AdapterError> {
        // Validar campos requeridos
        if external.session_id.is_nil() {
            return Err(AdapterError::MissingRequiredField {
                field: "session_id".to_string(),
            });
        }
        
        if external.user_id.is_nil() {
            return Err(AdapterError::MissingRequiredField {
                field: "user_id".to_string(),
            });
        }
        
        // Mapear campos con manejo de opcionales
        let session = ListenSession {
            session_id: external.session_id,
            user_id: external.user_id,
            song_id: external.song_id.unwrap_or_else(Uuid::nil),
            artist_id: external.artist_id.unwrap_or_else(Uuid::nil),
            start_time: external.start_time,
            end_time: external.end_time,
            duration_seconds: external.duration_seconds,
            reward_amount: external.reward_amount.unwrap_or(0.0),
            status: external.status,
        };
        
        Ok(session)
    }
    
    /// Adaptar ListenReward interno a externo
    pub fn adapt_listen_reward_to_external(
        &self,
        internal: &ListenReward,
    ) -> Result<ExternalListenReward, AdapterError> {
        Ok(ExternalListenReward {
            reward_id: internal.reward_id,
            user_id: internal.user_id,
            session_id: internal.session_id,
            amount: internal.amount,
            currency: Some(internal.currency.clone()), // Campo requerido en interno, opcional en externo
            timestamp: internal.timestamp,
            status: internal.status.clone(),
            transaction_hash: internal.transaction_hash.clone(),
        })
    }
    
    /// Adaptar ListenReward externo a interno
    pub fn adapt_listen_reward_from_external(
        &self,
        external: ExternalListenReward,
    ) -> Result<ListenReward, AdapterError> {
        // Validar campos requeridos
        if external.reward_id.is_nil() {
            return Err(AdapterError::MissingRequiredField {
                field: "reward_id".to_string(),
            });
        }
        
        if external.user_id.is_nil() {
            return Err(AdapterError::MissingRequiredField {
                field: "user_id".to_string(),
            });
        }
        
        if external.session_id.is_nil() {
            return Err(AdapterError::MissingRequiredField {
                field: "session_id".to_string(),
            });
        }
        
        // Mapear campos con manejo de opcionales
        let reward = ListenReward {
            reward_id: external.reward_id,
            user_id: external.user_id,
            session_id: external.session_id,
            amount: external.amount,
            currency: external.currency.unwrap_or_else(|| self.config.default_values.default_currency.clone()),
            timestamp: external.timestamp,
            status: external.status,
            transaction_hash: external.transaction_hash,
        };
        
        Ok(reward)
    }
}

// Implementar trait Adapter para compatibilidad
impl Adapter<ExternalStartListenSessionCommand, StartListenSessionCommand> for ListenRewardAdapter {
    fn adapt(&self, input: ExternalStartListenSessionCommand) -> Result<StartListenSessionCommand, AdapterError> {
        self.adapt_start_listen_command(input)
    }
}

impl Adapter<ExternalListenSession, ListenSession> for ListenRewardAdapter {
    fn adapt(&self, input: ExternalListenSession) -> Result<ListenSession, AdapterError> {
        self.adapt_listen_session_from_external(input)
    }
}

impl Adapter<ExternalListenReward, ListenReward> for ListenRewardAdapter {
    fn adapt(&self, input: ExternalListenReward) -> Result<ListenReward, AdapterError> {
        self.adapt_listen_reward_from_external(input)
    }
}

// DTOs específicos para la capa de presentación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartListenSessionRequest {
    pub user_id: Uuid,
    pub song_id: Option<Uuid>,
    pub artist_id: Option<Uuid>,
    pub session_duration: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartListenSessionResponse {
    pub session_id: Uuid,
    pub status: String,
    pub estimated_reward: f64,
    pub session_duration: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenSessionResponse {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_seconds: u32,
    pub reward_amount: f64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenRewardResponse {
    pub reward_id: Uuid,
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub amount: f64,
    pub currency: String,
    pub timestamp: DateTime<Utc>,
    pub status: String,
    pub transaction_hash: Option<String>,
}

// Tests unitarios para el adapter
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_adapt_start_listen_command_success() {
        let adapter = ListenRewardAdapter::new(AdapterConfig::default());
        let user_id = Uuid::new_v4();
        let song_id = Uuid::new_v4();
        
        let external = ExternalStartListenSessionCommand {
            user_id,
            song_id: Some(song_id),
            artist_id: None,
            session_duration: Some(300),
            timestamp: Some(Utc::now()),
        };
        
        let result = adapter.adapt_start_listen_command(external);
        assert!(result.is_ok());
        
        let command = result.unwrap();
        assert_eq!(command.user_id, user_id);
        assert_eq!(command.song_id, song_id);
        assert_eq!(command.session_duration, 300);
    }
    
    #[test]
    fn test_adapt_start_listen_command_missing_user_id() {
        let adapter = ListenRewardAdapter::new(AdapterConfig::default());
        
        let external = ExternalStartListenSessionCommand {
            user_id: Uuid::nil(),
            song_id: None,
            artist_id: None,
            session_duration: None,
            timestamp: None,
        };
        
        let result = adapter.adapt_start_listen_command(external);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            AdapterError::MissingRequiredField { field } => {
                assert_eq!(field, "user_id");
            }
            _ => panic!("Expected MissingRequiredField error"),
        }
    }
}
