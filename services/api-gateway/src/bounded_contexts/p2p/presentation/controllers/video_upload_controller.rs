use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::Json,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use bytes::Bytes;

use crate::bounded_contexts::p2p::application::services::video_streaming_service::{
    VideoStreamingService, StreamConfig
};
use crate::bounded_contexts::p2p::infrastructure::P2PInfrastructureConfig;
use crate::bounded_contexts::p2p::domain::entities::video_stream::VideoQuality;

/// Video upload controller for P2P video streaming
pub struct VideoUploadController {
    streaming_service: Arc<VideoStreamingService>,
}

impl VideoUploadController {
    pub fn new(streaming_service: Arc<VideoStreamingService>) -> Self {
        Self { streaming_service }
    }
    
    /// Create routes for video upload
    pub fn routes(self) -> Router {
        Router::new()
            .route("/upload", post(Self::upload_video))
            .route("/upload/chunk", post(Self::upload_chunk))
            .route("/upload/complete", post(Self::complete_upload))
            .with_state(Arc::new(self))
    }
}

/// Upload video request
#[derive(Debug, Deserialize)]
pub struct UploadVideoRequest {
    pub title: String,
    pub description: Option<String>,
    pub stream_type: String,
    pub quality: String,
    pub format: String,
    pub bitrate: u32,
    pub resolution: String,
    pub fps: u32,
}

/// Upload video response
#[derive(Debug, Serialize)]
pub struct UploadVideoResponse {
    pub upload_id: String,
    pub stream_id: String,
    pub ipfs_url: String,
    pub status: String,
    pub message: String,
}

/// Upload chunk request
#[derive(Debug, Deserialize)]
pub struct UploadChunkRequest {
    pub upload_id: String,
    pub chunk_index: u32,
    pub chunk_data: Vec<u8>,
}

/// Upload chunk response
#[derive(Debug, Serialize)]
pub struct UploadChunkResponse {
    pub upload_id: String,
    pub chunk_index: u32,
    pub status: String,
    pub ipfs_hash: Option<String>,
}

/// Complete upload request
#[derive(Debug, Deserialize)]
pub struct CompleteUploadRequest {
    pub upload_id: String,
    pub total_chunks: u32,
    pub final_metadata: serde_json::Value,
}

/// Complete upload response
#[derive(Debug, Serialize)]
pub struct CompleteUploadResponse {
    pub upload_id: String,
    pub stream_id: String,
    pub ipfs_url: String,
    pub total_size: u64,
    pub available_qualities: Vec<String>,
    pub status: String,
}

impl VideoUploadController {
    /// Upload complete video file
    async fn upload_video(
        State(controller): State<Arc<Self>>,
        mut multipart: Multipart,
    ) -> Result<Json<UploadVideoResponse>, StatusCode> {
        let mut file_data: Option<Bytes> = None;
        let mut file_name: Option<String> = None;
        let mut content_type: Option<String> = None;
        let mut title: Option<String> = None;
        let mut description: Option<String> = None;
        let mut stream_type: Option<String> = None;
        let mut quality: Option<String> = None;
        let mut format: Option<String> = None;
        let mut bitrate: Option<u32> = None;
        let mut resolution: Option<String> = None;
        let mut fps: Option<u32> = None;

        // Parse multipart form data
        while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
            let field_name = field.name().unwrap_or("").to_string();
            
            match field_name.as_str() {
                "file" => {
                    file_data = Some(field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?);
                }
                "filename" => {
                    file_name = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?);
                }
                "content_type" => {
                    content_type = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?);
                }
                "title" => {
                    title = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?);
                }
                "description" => {
                    description = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?);
                }
                "stream_type" => {
                    stream_type = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?);
                }
                "quality" => {
                    quality = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?);
                }
                "format" => {
                    format = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?);
                }
                "bitrate" => {
                    let bitrate_str = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                    bitrate = Some(bitrate_str.parse().map_err(|_| StatusCode::BAD_REQUEST)?);
                }
                "resolution" => {
                    resolution = Some(field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?);
                }
                "fps" => {
                    let fps_str = field.text().await.map_err(|_| StatusCode::BAD_REQUEST)?);
                    fps = Some(fps_str.parse().map_err(|_| StatusCode::BAD_REQUEST)?);
                }
                _ => {}
            }
        }

        // Validate required fields
        let file_data = file_data.ok_or(StatusCode::BAD_REQUEST)?;
        let file_name = file_name.ok_or(StatusCode::BAD_REQUEST)?;
        let content_type = content_type.ok_or(StatusCode::BAD_REQUEST)?;
        let title = title.ok_or(StatusCode::BAD_REQUEST)?;
        let stream_type = stream_type.unwrap_or_else(|| "video".to_string());
        let quality = quality.unwrap_or_else(|| "High".to_string());
        let format = format.unwrap_or_else(|| "mp4".to_string());
        let bitrate = bitrate.unwrap_or(5000);
        let resolution = resolution.unwrap_or_else(|| "1920x1080".to_string());
        let fps = fps.unwrap_or(30);

        // Parse video quality
        let video_quality = match quality.as_str() {
            "Low" => VideoQuality::Low,
            "Medium" => VideoQuality::Medium,
            "High" => VideoQuality::High,
            "Ultra" => VideoQuality::Ultra,
            _ => VideoQuality::High,
        };

        // Create stream configuration
        let stream_config = StreamConfig {
            stream_type,
            quality: video_quality,
            format,
            bitrate,
            resolution,
            fps,
        };

        // Upload video to IPFS and create stream
        match controller.streaming_service.upload_video_stream(
            file_data,
            &file_name,
            &content_type,
            stream_config,
        ).await {
            Ok(stream) => {
                let upload_id = Uuid::new_v4().to_string();
                
                let response = UploadVideoResponse {
                    upload_id,
                    stream_id: stream.id.to_string(),
                    ipfs_url: stream.video_id.clone(),
                    status: "success".to_string(),
                    message: format!("Video '{}' uploaded successfully to IPFS", title),
                };

                println!("✅ Video uploaded successfully: {}", stream.id.to_string());
                Ok(Json(response))
            }
            Err(e) => {
                println!("❌ Video upload failed: {}", e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    /// Upload video chunk (for large files)
    async fn upload_chunk(
        State(_controller): State<Arc<Self>>,
        Json(request): Json<UploadChunkRequest>,
    ) -> Result<Json<UploadChunkResponse>, StatusCode> {
        // For now, we'll return a mock response
        // In a real implementation, this would handle chunked uploads
        let response = UploadChunkResponse {
            upload_id: request.upload_id,
            chunk_index: request.chunk_index,
            status: "success".to_string(),
            ipfs_hash: Some(format!("QmChunk{}", Uuid::new_v4().to_string().replace("-", ""))),
        };

        Ok(Json(response))
    }

    /// Complete chunked upload
    async fn complete_upload(
        State(_controller): State<Arc<Self>>,
        Json(request): Json<CompleteUploadRequest>,
    ) -> Result<Json<CompleteUploadResponse>, StatusCode> {
        // For now, we'll return a mock response
        // In a real implementation, this would combine chunks and create the final stream
        let response = CompleteUploadResponse {
            upload_id: request.upload_id,
            stream_id: Uuid::new_v4().to_string(),
            ipfs_url: format!("ipfs://QmVideo{}", Uuid::new_v4().to_string().replace("-", "")),
            total_size: 0,
            available_qualities: vec!["Low".to_string(), "Medium".to_string(), "High".to_string()],
            status: "completed".to_string(),
        };

        Ok(Json(response))
    }
}

/// Create video upload routes
pub fn create_video_upload_routes(
    streaming_service: Arc<VideoStreamingService>,
) -> Router {
    let controller = VideoUploadController::new(streaming_service);
    Router::new()
        .nest("/video", controller.routes())
} 