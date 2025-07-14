use axum::{
    extract::{Multipart, State, Path},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use std::sync::Arc;

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::music::infrastructure::storage::{
    AudioFileService, AudioUploadResult, StorageConfig, create_storage
};
use crate::bounded_contexts::music::domain::value_objects::{FileFormat, AudioQuality};
use super::video_upload_controller::VideoUploadController;

/// Audio upload controller
pub struct AudioUploadController {
    audio_service: Arc<AudioFileService>,
}

impl AudioUploadController {
    pub fn new(storage_config: StorageConfig) -> Self {
        let storage = create_storage(storage_config);
        let max_file_size = 100 * 1024 * 1024; // 100MB
        let audio_service = Arc::new(AudioFileService::new(storage, max_file_size));
        
        Self { audio_service }
    }
}

// ============================================================================
// REQUEST/RESPONSE DTOs
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadAudioRequest {
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub title: String,
    pub expected_format: Option<String>,
    pub expected_duration: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadAudioResponse {
    pub upload_id: String,
    pub song_id: Uuid,
    pub storage_url: String,
    pub streaming_url: String,
    pub file_format: String,
    pub file_size: u64,
    pub duration_seconds: Option<u32>,
    pub audio_quality: Option<String>,
    pub bitrate: Option<u32>,
    pub uploaded_at: DateTime<Utc>,
    pub processing_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadProgressResponse {
    pub upload_id: String,
    pub status: UploadStatus,
    pub progress_percentage: f32,
    pub message: String,
    pub estimated_completion: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UploadStatus {
    Uploading,
    Processing,
    Validating,
    Completed,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub errors: Option<Vec<String>>,
}

// ============================================================================
// UPLOAD ENDPOINTS
// ============================================================================

/// Upload audio file
pub async fn upload_audio(
    State((controller, _)): State<(Arc<AudioUploadController>, Arc<VideoUploadController>)>,
    mut multipart: Multipart,
) -> Result<Json<ApiResponse<UploadAudioResponse>>, StatusCode> {
    
    let mut file_data: Option<Bytes> = None;
    let mut filename: Option<String> = None;
    let mut metadata: Option<UploadAudioRequest> = None;

    // Parse multipart form data
    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let field_name = field.name().unwrap_or("unknown");
        
        match field_name {
            "audio_file" => {
                filename = field.file_name().map(|s| s.to_string());
                let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                file_data = Some(data);
            },
            "metadata" => {
                let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                let metadata_str = String::from_utf8(data.to_vec()).map_err(|_| StatusCode::BAD_REQUEST)?;
                metadata = serde_json::from_str(&metadata_str).ok();
            },
            _ => {
                // Ignore unknown fields
                let _ = field.bytes().await;
            }
        }
    }

    // Validate required fields
    let file_data = file_data.ok_or(StatusCode::BAD_REQUEST)?;
    let filename = filename.ok_or(StatusCode::BAD_REQUEST)?;
    let metadata = metadata.ok_or(StatusCode::BAD_REQUEST)?;

    // Process upload
    match controller.audio_service.upload_audio_file(
        file_data,
        &filename,
        metadata.artist_id,
        metadata.song_id,
    ).await {
        Ok(result) => {
            let response = UploadAudioResponse {
                upload_id: Uuid::new_v4().to_string(),
                song_id: metadata.song_id,
                storage_url: result.storage_url,
                streaming_url: result.streaming_url,
                file_format: result.file_format.to_string(),
                file_size: result.metadata.file_size,
                duration_seconds: result.metadata.duration_seconds,
                audio_quality: None, // TODO: Detect from bitrate
                bitrate: result.metadata.bitrate,
                uploaded_at: result.processed_at,
                processing_status: "completed".to_string(),
            };

            Ok(Json(ApiResponse {
                success: true,
                data: Some(response),
                message: Some("Audio file uploaded successfully".to_string()),
                errors: None,
            }))
        },
        Err(e) => {
            eprintln!("Upload error: {}", e);
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Upload failed".to_string()),
                errors: Some(vec![e.to_string()]),
            }))
        }
    }
}

/// Get upload progress
pub async fn get_upload_progress(
    State((_controller, _)): State<(Arc<AudioUploadController>, Arc<VideoUploadController>)>,
    Path(upload_id): Path<String>,
) -> Result<Json<ApiResponse<UploadProgressResponse>>, StatusCode> {
    // TODO: Implement actual progress tracking
    let progress = UploadProgressResponse {
        upload_id: upload_id.clone(),
        status: UploadStatus::Completed,
        progress_percentage: 100.0,
        message: "Upload completed successfully".to_string(),
        estimated_completion: None,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(progress),
        message: None,
        errors: None,
    }))
}

/// Get streaming URL for uploaded audio
pub async fn get_streaming_url(
    State((controller, _)): State<(Arc<AudioUploadController>, Arc<VideoUploadController>)>,
    Path(song_id): Path<Uuid>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    // TODO: Get storage URL from database using song_id
    let storage_url = format!("local://song_{}.mp3", song_id);
    
    match controller.audio_service.get_streaming_url(&storage_url).await {
        Ok(streaming_url) => {
            Ok(Json(ApiResponse {
                success: true,
                data: Some(streaming_url),
                message: None,
                errors: None,
            }))
        },
        Err(e) => {
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Failed to get streaming URL".to_string()),
                errors: Some(vec![e.to_string()]),
            }))
        }
    }
}

/// Delete uploaded audio
pub async fn delete_audio(
    State((controller, _)): State<(Arc<AudioUploadController>, Arc<VideoUploadController>)>,
    Path(song_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // TODO: Get storage URL from database using song_id
    let storage_url = format!("local://song_{}.mp3", song_id);
    
    match controller.audio_service.delete_audio_file(&storage_url).await {
        Ok(_) => {
            Ok(Json(ApiResponse {
                success: true,
                data: Some(()),
                message: Some("Audio file deleted successfully".to_string()),
                errors: None,
            }))
        },
        Err(e) => {
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Failed to delete audio file".to_string()),
                errors: Some(vec![e.to_string()]),
            }))
        }
    }
}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

/// Validate audio file before upload
pub fn validate_audio_upload(
    file_size: u64,
    filename: &str,
    max_size: u64,
) -> Result<(), String> {
    // Check file size
    if file_size > max_size {
        return Err(format!("File size {} exceeds maximum {}", file_size, max_size));
    }

    // Check file extension
    let extension = std::path::Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    FileFormat::from_extension(extension)?;

    Ok(())
}

/// Extract audio quality from bitrate
pub fn detect_audio_quality(bitrate: Option<u32>) -> Option<AudioQuality> {
    match bitrate {
        Some(br) => Some(AudioQuality::from_bitrate(br)),
        None => None,
    }
} 