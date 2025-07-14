use axum::{
    extract::{Multipart, State, Path, Query},
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
    ipfs_video_storage::{IPFSVideoStorage, VideoQuality, VideoFileStorage},
    StorageConfig
};
use super::upload_controller::AudioUploadController;

/// Video upload controller
pub struct VideoUploadController {
    video_storage: Arc<IPFSVideoStorage>,
}

impl VideoUploadController {
    pub fn new() -> Self {
        let video_storage = Arc::new(IPFSVideoStorage::new_distributed(
            "http://localhost:5001".to_string(),
            vec![
                "http://peer1:5001".to_string(),
                "http://peer2:5001".to_string(),
            ],
            500 * 1024 * 1024, // 500MB max file size
            true,  // enable federation
            true,  // enable content discovery
        ));
        
        Self { video_storage }
    }
}

// ============================================================================
// REQUEST/RESPONSE DTOs
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadVideoRequest {
    pub video_id: Uuid,
    pub artist_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub expected_quality: Option<String>,
    pub expected_duration: Option<u32>,
    pub is_music_video: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadVideoResponse {
    pub upload_id: String,
    pub video_id: Uuid,
    pub ipfs_hash: String,
    pub storage_url: String,
    pub streaming_url: String,
    pub file_size: u64,
    pub duration_seconds: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub frame_rate: Option<f32>,
    pub bitrate: Option<u32>,
    pub available_qualities: Vec<String>,
    pub chunk_count: u32,
    pub peer_count: u32,
    pub uploaded_at: DateTime<Utc>,
    pub processing_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoStreamingResponse {
    pub video_id: Uuid,
    pub streaming_url: String,
    pub available_qualities: Vec<String>,
    pub current_quality: String,
    pub chunk_urls: Vec<String>,
    pub peer_count: u32,
    pub availability_score: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoChunkResponse {
    pub chunk_index: u32,
    pub chunk_url: String,
    pub quality: String,
    pub timestamp: DateTime<Utc>,
    pub data_size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadProgressResponse {
    pub upload_id: String,
    pub status: UploadStatus,
    pub progress_percentage: f32,
    pub message: String,
    pub estimated_completion: Option<DateTime<Utc>>,
    pub current_operation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UploadStatus {
    Uploading,
    Processing,
    Transcoding,
    Validating,
    Announcing,
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
// VIDEO UPLOAD ENDPOINTS
// ============================================================================

/// Upload video file to IPFS
pub async fn upload_video(
    State((_, controller)): State<(Arc<AudioUploadController>, Arc<VideoUploadController>)>,
    mut multipart: Multipart,
) -> Result<Json<ApiResponse<UploadVideoResponse>>, StatusCode> {
    
    let mut file_data: Option<Bytes> = None;
    let mut filename: Option<String> = None;
    let mut content_type: Option<String> = None;
    let mut metadata: Option<UploadVideoRequest> = None;

    // Parse multipart form data
    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let field_name = field.name().unwrap_or("unknown");
        
        match field_name {
            "video_file" => {
                filename = field.file_name().map(|s| s.to_string());
                content_type = field.content_type().map(|ct| ct.to_string());
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
    let content_type = content_type.ok_or(StatusCode::BAD_REQUEST)?;
    let metadata = metadata.ok_or(StatusCode::BAD_REQUEST)?;

    // Validate video file
    validate_video_upload(file_data.len() as u64, &filename, 500 * 1024 * 1024)?;

    // Upload to IPFS
    match controller.video_storage.upload_video(file_data, &filename, &content_type).await {
        Ok(ipfs_hash) => {
            // Get metadata
            let video_metadata = controller.video_storage.get_metadata(&ipfs_hash).await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            // Get available qualities
            let qualities = controller.video_storage.get_available_qualities(&ipfs_hash).await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            // Get peers
            let peers = controller.video_storage.get_peers(&ipfs_hash).await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let response = UploadVideoResponse {
                upload_id: Uuid::new_v4().to_string(),
                video_id: metadata.video_id,
                ipfs_hash: ipfs_hash.clone(),
                storage_url: format!("ipfs://{}", ipfs_hash),
                streaming_url: format!("/api/videos/{}/stream", metadata.video_id),
                file_size: video_metadata.file_size,
                duration_seconds: video_metadata.duration_seconds,
                width: video_metadata.width,
                height: video_metadata.height,
                frame_rate: video_metadata.frame_rate,
                bitrate: video_metadata.bitrate,
                available_qualities: qualities.iter().map(|q| format!("{:?}", q)).collect(),
                chunk_count: video_metadata.chunk_count,
                peer_count: peers.len() as u32,
                uploaded_at: Utc::now(),
                processing_status: "completed".to_string(),
            };

            Ok(Json(ApiResponse {
                success: true,
                data: Some(response),
                message: Some("Video uploaded to IPFS successfully".to_string()),
                errors: None,
            }))
        },
        Err(e) => {
            eprintln!("Video upload error: {}", e);
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Video upload failed".to_string()),
                errors: Some(vec![e.to_string()]),
            }))
        }
    }
}

/// Get video streaming URL and metadata
pub async fn get_video_streaming(
    State((_, controller)): State<(Arc<AudioUploadController>, Arc<VideoUploadController>)>,
    Path(video_id): Path<Uuid>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<VideoStreamingResponse>>, StatusCode> {
    let quality = params.get("quality")
        .and_then(|q| parse_video_quality(q))
        .unwrap_or(VideoQuality::Medium);
    
    // TODO: Get IPFS hash from database using video_id
    let ipfs_hash = format!("QmVideoHash{}", video_id);
    
    match controller.video_storage.get_streaming_url(&ipfs_hash, &quality).await {
        Ok(streaming_url) => {
            let qualities = controller.video_storage.get_available_qualities(&ipfs_hash).await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            let peers = controller.video_storage.get_peers(&ipfs_hash).await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            let metadata = controller.video_storage.get_metadata(&ipfs_hash).await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            // Generate chunk URLs
            let chunk_urls = (0..metadata.chunk_count)
                .map(|i| format!("/api/videos/{}/chunks/{}", video_id, i))
                .collect();

            let response = VideoStreamingResponse {
                video_id,
                streaming_url,
                available_qualities: qualities.iter().map(|q| format!("{:?}", q)).collect(),
                current_quality: format!("{:?}", quality),
                chunk_urls,
                peer_count: peers.len() as u32,
                availability_score: metadata.availability_score.unwrap_or(1.0),
            };

            Ok(Json(ApiResponse {
                success: true,
                data: Some(response),
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

/// Get specific video chunk
pub async fn get_video_chunk(
    State((_, controller)): State<(Arc<AudioUploadController>, Arc<VideoUploadController>)>,
    Path((video_id, chunk_index)): Path<(Uuid, u32)>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ApiResponse<VideoChunkResponse>>, StatusCode> {
    let quality = params.get("quality")
        .and_then(|q| parse_video_quality(q))
        .unwrap_or(VideoQuality::Medium);
    
    // TODO: Get IPFS hash from database using video_id
    let ipfs_hash = format!("QmVideoHash{}", video_id);
    
    match controller.video_storage.get_video_chunk(&ipfs_hash, chunk_index, &quality).await {
        Ok(chunk) => {
            let response = VideoChunkResponse {
                chunk_index: chunk.chunk_index,
                chunk_url: format!("/api/videos/{}/chunks/{}", video_id, chunk_index),
                quality: format!("{:?}", chunk.quality),
                timestamp: chunk.timestamp,
                data_size: chunk.data.len() as u64,
            };

            Ok(Json(ApiResponse {
                success: true,
                data: Some(response),
                message: None,
                errors: None,
            }))
        },
        Err(e) => {
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Failed to get video chunk".to_string()),
                errors: Some(vec![e.to_string()]),
            }))
        }
    }
}

/// Get video metadata
pub async fn get_video_metadata(
    State((_, controller)): State<(Arc<AudioUploadController>, Arc<VideoUploadController>)>,
    Path(video_id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    // TODO: Get IPFS hash from database using video_id
    let ipfs_hash = format!("QmVideoHash{}", video_id);
    
    match controller.video_storage.get_metadata(&ipfs_hash).await {
        Ok(metadata) => {
            let metadata_json = serde_json::json!({
                "video_id": video_id,
                "file_size": metadata.file_size,
                "content_type": metadata.content_type,
                "duration_seconds": metadata.duration_seconds,
                "width": metadata.width,
                "height": metadata.height,
                "frame_rate": metadata.frame_rate,
                "bitrate": metadata.bitrate,
                "chunk_count": metadata.chunk_count,
                "peer_count": metadata.peer_count,
                "availability_score": metadata.availability_score,
                "created_at": metadata.created_at,
            });

            Ok(Json(ApiResponse {
                success: true,
                data: Some(metadata_json),
                message: None,
                errors: None,
            }))
        },
        Err(e) => {
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Failed to get video metadata".to_string()),
                errors: Some(vec![e.to_string()]),
            }))
        }
    }
}

/// Get upload progress
pub async fn get_video_upload_progress(
    State((_, _controller)): State<(Arc<AudioUploadController>, Arc<VideoUploadController>)>,
    Path(upload_id): Path<String>,
) -> Result<Json<ApiResponse<UploadProgressResponse>>, StatusCode> {
    // TODO: Implement actual progress tracking
    let progress = UploadProgressResponse {
        upload_id: upload_id.clone(),
        status: UploadStatus::Completed,
        progress_percentage: 100.0,
        message: "Video upload completed successfully".to_string(),
        estimated_completion: None,
        current_operation: Some("Announcing to P2P network".to_string()),
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(progress),
        message: None,
        errors: None,
    }))
}

/// Delete video from IPFS
pub async fn delete_video(
    State((_, controller)): State<(Arc<AudioUploadController>, Arc<VideoUploadController>)>,
    Path(video_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // TODO: Get IPFS hash from database using video_id
    let ipfs_hash = format!("QmVideoHash{}", video_id);
    
    match controller.video_storage.delete_video(&ipfs_hash).await {
        Ok(_) => {
            Ok(Json(ApiResponse {
                success: true,
                data: Some(()),
                message: Some("Video deleted from IPFS successfully".to_string()),
                errors: None,
            }))
        },
        Err(e) => {
            Ok(Json(ApiResponse {
                success: false,
                data: None,
                message: Some("Failed to delete video".to_string()),
                errors: Some(vec![e.to_string()]),
            }))
        }
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Validate video file before upload
fn validate_video_upload(
    file_size: u64,
    filename: &str,
    max_size: u64,
) -> Result<(), StatusCode> {
    // Check file size
    if file_size > max_size {
        return Err(StatusCode::PAYLOAD_TOO_LARGE);
    }

    // Check file extension
    let extension = std::path::Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    let valid_extensions = ["mp4", "avi", "mov", "mkv", "webm", "flv"];
    if !valid_extensions.contains(&extension.to_lowercase().as_str()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(())
}

/// Parse video quality from string
fn parse_video_quality(quality_str: &str) -> Option<VideoQuality> {
    match quality_str.to_lowercase().as_str() {
        "low" => Some(VideoQuality::Low),
        "medium" => Some(VideoQuality::Medium),
        "high" => Some(VideoQuality::High),
        "ultra" => Some(VideoQuality::Ultra),
        _ => None,
    }
} 