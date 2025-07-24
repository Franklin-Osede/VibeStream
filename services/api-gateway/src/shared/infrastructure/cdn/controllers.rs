use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use super::{service::CloudCDNService, CDNService};

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadContentRequest {
    pub content_type: String,
    pub file_name: String,
    pub content_size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadContentResponse {
    pub content_id: Uuid,
    pub url: String,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CDNStats {
    pub total_content: usize,
    pub total_size_mb: u64,
    pub cache_hit_rate: f64,
    pub edge_locations: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentMetadata {
    pub content_id: Uuid,
    pub file_name: String,
    pub content_type: String,
    pub size_bytes: u64,
    pub uploaded_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentMetadataResponse {
    pub content_id: Uuid,
    pub url: String,
    pub content_type: String,
    pub size: u64,
    pub checksum: String,
    pub uploaded_at: chrono::DateTime<chrono::Utc>,
}

pub async fn upload_content(
    State(service): State<Arc<CloudCDNService>>,
    Json(request): Json<UploadContentRequest>,
) -> Result<Json<UploadContentResponse>, StatusCode> {
    let content_id = Uuid::new_v4();
    
    // En una implementación real, aquí procesarías el archivo subido
    let url = service.get_content_url(content_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(UploadContentResponse {
        content_id,
        url,
        expires_at: None,
    }))
}

pub async fn get_content_metadata(
    State(service): State<Arc<CloudCDNService>>,
    Path(content_id): Path<Uuid>,
) -> Result<Json<ContentMetadataResponse>, StatusCode> {
    let metadata = service.get_content_metadata(content_id).await;
    
    match metadata {
        Ok(Some(metadata)) => {
            Ok(Json(ContentMetadataResponse {
                content_id: content_id.clone(),
                url: format!("https://cdn.vibestream.com/{}", content_id),
                content_type: metadata.content_type.to_string(),
                size: metadata.file_size,
                checksum: metadata.checksum,
                uploaded_at: metadata.uploaded_at,
            }))
        },
        Ok(None) => {
            Err(StatusCode::NOT_FOUND)
        },
        Err(_) => {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_content(
    State(service): State<Arc<CloudCDNService>>,
    Path(content_id): Path<Uuid>,
) -> StatusCode {
    match service.delete_content(content_id).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn get_cdn_stats(
    State(_service): State<Arc<CloudCDNService>>,
) -> Json<CDNStats> {
    // En una implementación real, obtendrías estadísticas reales
    Json(CDNStats {
        total_content: 0,
        total_size_mb: 0,
        cache_hit_rate: 0.0,
        edge_locations: 0,
    })
}

pub async fn purge_cache(
    State(service): State<Arc<CloudCDNService>>,
    Path(content_id): Path<Uuid>,
) -> StatusCode {
    match service.purge_cache(content_id).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
} 