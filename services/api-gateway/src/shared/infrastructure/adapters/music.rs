//! Adapter para el contexto de music
//! 
//! Maneja la conversión entre vibestream_types y entidades locales
//! protegiendo el dominio de cambios en los contratos externos.

use super::{Adapter, AdapterError, AdapterConfig};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Adapter para el contexto de music
pub struct MusicAdapter {
    config: AdapterConfig,
}

impl MusicAdapter {
    pub fn new(config: AdapterConfig) -> Self {
        Self { config }
    }
    
    // Implementaciones básicas - expandir según necesidades específicas
    pub fn adapt_song_request(
        &self,
        _external: serde_json::Value, // Placeholder para vibestream_types
    ) -> Result<serde_json::Value, AdapterError> {
        // TODO: Implementar mapeo específico cuando se definan los tipos
        Ok(serde_json::Value::Null)
    }
}

// DTOs específicos para la capa de presentación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongRequest {
    pub title: String,
    pub artist_id: Uuid,
    pub genre: String,
    pub duration: u32,
    pub file_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongResponse {
    pub song_id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub artist_name: String,
    pub genre: String,
    pub duration: u32,
    pub file_url: Option<String>,
    pub play_count: u64,
    pub like_count: u64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongListResponse {
    pub songs: Vec<SongResponse>,
    pub total_count: u64,
    pub page: u32,
    pub limit: u32,
}









