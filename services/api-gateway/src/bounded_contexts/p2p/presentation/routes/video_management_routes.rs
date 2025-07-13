use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::bounded_contexts::p2p::application::services::VideoManagementService;
use crate::bounded_contexts::p2p::domain::entities::video_stream::{
    VideoStreamId, VideoQuality, ConnectionQuality
};

/// Video management routes
pub fn video_management_routes() -> Router<Arc<VideoManagementService>> {
    Router::new()
        .route("/upload", post(upload_video))
        .route("/streams", get(get_active_streams))
        .route("/streams/:stream_id", get(get_stream))
        .route("/streams/:stream_id/start", post(start_stream))
        .route("/streams/:stream_id/stop", post(stop_stream))
        .route("/streams/:stream_id/join", post(join_stream))
        .route("/streams/:stream_id/leave", post(leave_stream))
        .route("/streams/:stream_id/stats", get(get_stream_stats))
        .route("/streams/:stream_id/metadata", get(get_video_metadata))
        .route("/streams/:stream_id", delete(delete_stream))
        .route("/transcoding/:job_id", get(get_transcoding_progress))
        .route("/quality/recommend", post(get_recommended_quality))
}

/// Upload video
async fn upload_video(
    State(service): State<Arc<VideoManagementService>>,
    Json(request): Json<UploadVideoRequest>,
) -> Result<Json<UploadVideoResponse>, (StatusCode, String)> {
    let result = service.upload_video(
        request.title,
        request.artist_id,
        request.video_path,
        request.description,
        request.is_live,
    ).await
    .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(Json(UploadVideoResponse {
        stream_id: result.stream_id,
        transcoding_job_id: result.transcoding_job_id,
        message: result.message,
    }))
}

/// Get active streams
async fn get_active_streams(
    State(service): State<Arc<VideoManagementService>>,
) -> Result<Json<Vec<StreamResponse>>, (StatusCode, String)> {
    let streams = service.get_active_streams().await;
    
    let responses: Vec<StreamResponse> = streams.into_iter().map(|stream| {
        StreamResponse {
            stream_id: stream.id.to_string(),
            title: stream.title,
            description: stream.description,
            artist_id: stream.artist_id.to_string(),
            duration_seconds: stream.duration_seconds,
            is_live: stream.is_live,
            current_viewers: stream.current_viewers,
            max_viewers: stream.max_viewers,
            status: format!("{:?}", stream.status),
            created_at: stream.created_at,
        }
    }).collect();

    Ok(Json(responses))
}

/// Get stream information
async fn get_stream(
    State(service): State<Arc<VideoManagementService>>,
    Path(stream_id): Path<String>,
) -> Result<Json<StreamResponse>, (StatusCode, String)> {
    let stream_id = VideoStreamId::from_uuid(
        Uuid::parse_str(&stream_id)
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid stream ID: {}", e)))?
    );

    // TODO: Implement get_stream method in service
    Err((StatusCode::NOT_IMPLEMENTED, "Get stream not implemented".to_string()))
}

/// Start stream
async fn start_stream(
    State(service): State<Arc<VideoManagementService>>,
    Path(stream_id): Path<String>,
) -> Result<Json<StartStreamResponse>, (StatusCode, String)> {
    let stream_id = VideoStreamId::from_uuid(
        Uuid::parse_str(&stream_id)
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid stream ID: {}", e)))?
    );

    service.start_stream(&stream_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(StartStreamResponse {
        message: "Stream started successfully".to_string(),
    }))
}

/// Stop stream
async fn stop_stream(
    State(service): State<Arc<VideoManagementService>>,
    Path(stream_id): Path<String>,
) -> Result<Json<StopStreamResponse>, (StatusCode, String)> {
    let stream_id = VideoStreamId::from_uuid(
        Uuid::parse_str(&stream_id)
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid stream ID: {}", e)))?
    );

    service.stop_stream(&stream_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(StopStreamResponse {
        message: "Stream stopped successfully".to_string(),
    }))
}

/// Join stream
async fn join_stream(
    State(service): State<Arc<VideoManagementService>>,
    Path(stream_id): Path<String>,
    Json(request): Json<JoinStreamRequest>,
) -> Result<Json<JoinStreamResponse>, (StatusCode, String)> {
    let stream_id = VideoStreamId::from_uuid(
        Uuid::parse_str(&stream_id)
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid stream ID: {}", e)))?
    );

    let connection_quality = ConnectionQuality {
        latency_ms: request.latency_ms.unwrap_or(50),
        bandwidth_mbps: request.bandwidth_mbps.unwrap_or(10.0),
        packet_loss_percent: request.packet_loss_percent.unwrap_or(0.1),
        jitter_ms: request.jitter_ms.unwrap_or(5),
    };

    let viewer = service.join_stream(
        &stream_id,
        request.user_id,
        request.peer_id,
        connection_quality,
    ).await
    .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(Json(JoinStreamResponse {
        viewer_id: viewer.id.to_string(),
        quality: viewer.quality,
        message: "Joined stream successfully".to_string(),
    }))
}

/// Leave stream
async fn leave_stream(
    State(service): State<Arc<VideoManagementService>>,
    Path(stream_id): Path<String>,
    Json(request): Json<LeaveStreamRequest>,
) -> Result<Json<LeaveStreamResponse>, (StatusCode, String)> {
    let stream_id = VideoStreamId::from_uuid(
        Uuid::parse_str(&stream_id)
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid stream ID: {}", e)))?
    );

    service.leave_stream(&stream_id, request.user_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(LeaveStreamResponse {
        message: "Left stream successfully".to_string(),
    }))
}

/// Get stream statistics
async fn get_stream_stats(
    State(service): State<Arc<VideoManagementService>>,
    Path(stream_id): Path<String>,
) -> Result<Json<StreamStatsResponse>, (StatusCode, String)> {
    let stream_id = VideoStreamId::from_uuid(
        Uuid::parse_str(&stream_id)
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid stream ID: {}", e)))?
    );

    let stats = service.get_stream_stats(&stream_id).await
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;

    Ok(Json(StreamStatsResponse {
        stream_id: stats.stream_id.to_string(),
        total_viewers: stats.total_viewers,
        quality_distribution: stats.quality_distribution,
        average_latency_ms: stats.average_latency_ms,
        average_bandwidth_mbps: stats.average_bandwidth_mbps,
        stream_duration: stats.stream_duration,
        is_live: stats.is_live,
        status: format!("{:?}", stats.status),
    }))
}

/// Get video metadata
async fn get_video_metadata(
    State(service): State<Arc<VideoManagementService>>,
    Path(stream_id): Path<String>,
) -> Result<Json<VideoMetadataResponse>, (StatusCode, String)> {
    let stream_id = VideoStreamId::from_uuid(
        Uuid::parse_str(&stream_id)
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid stream ID: {}", e)))?
    );

    if let Some(metadata) = service.get_video_metadata(&stream_id).await {
        Ok(Json(VideoMetadataResponse {
            duration: metadata.duration,
            width: metadata.width,
            height: metadata.height,
            bitrate: metadata.bitrate,
            frame_rate: metadata.frame_rate,
            codec: metadata.codec,
            format: metadata.format,
        }))
    } else {
        Err((StatusCode::NOT_FOUND, "Video metadata not found".to_string()))
    }
}

/// Delete stream
async fn delete_stream(
    State(service): State<Arc<VideoManagementService>>,
    Path(stream_id): Path<String>,
) -> Result<Json<DeleteStreamResponse>, (StatusCode, String)> {
    let stream_id = VideoStreamId::from_uuid(
        Uuid::parse_str(&stream_id)
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid stream ID: {}", e)))?
    );

    service.delete_stream(&stream_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(DeleteStreamResponse {
        message: "Stream deleted successfully".to_string(),
    }))
}

/// Get transcoding progress
async fn get_transcoding_progress(
    State(service): State<Arc<VideoManagementService>>,
    Path(job_id): Path<String>,
) -> Result<Json<TranscodingProgressResponse>, (StatusCode, String)> {
    if let Some(progress) = service.get_transcoding_progress(&job_id).await {
        Ok(Json(TranscodingProgressResponse {
            job_id: progress.job_id,
            status: format!("{:?}", progress.status),
            progress: progress.progress,
            created_at: progress.created_at,
            started_at: progress.started_at,
            completed_at: progress.completed_at,
        }))
    } else {
        Err((StatusCode::NOT_FOUND, "Transcoding job not found".to_string()))
    }
}

/// Get recommended quality
async fn get_recommended_quality(
    State(service): State<Arc<VideoManagementService>>,
    Json(request): Json<QualityRecommendationRequest>,
) -> Result<Json<QualityRecommendationResponse>, (StatusCode, String)> {
    let quality = service.get_recommended_quality(request.bandwidth_mbps, request.latency_ms);
    
    Ok(Json(QualityRecommendationResponse {
        recommended_quality: quality,
        resolution: quality.resolution(),
        bitrate: quality.bitrate(),
    }))
}

// Request/Response types

#[derive(Debug, Deserialize)]
struct UploadVideoRequest {
    title: String,
    artist_id: Uuid,
    video_path: String,
    description: Option<String>,
    is_live: bool,
}

#[derive(Debug, Serialize)]
struct UploadVideoResponse {
    stream_id: String,
    transcoding_job_id: Option<String>,
    message: String,
}

#[derive(Debug, Serialize)]
struct StreamResponse {
    stream_id: String,
    title: String,
    description: Option<String>,
    artist_id: String,
    duration_seconds: u32,
    is_live: bool,
    current_viewers: u32,
    max_viewers: u32,
    status: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
struct StartStreamResponse {
    message: String,
}

#[derive(Debug, Serialize)]
struct StopStreamResponse {
    message: String,
}

#[derive(Debug, Deserialize)]
struct JoinStreamRequest {
    user_id: Uuid,
    peer_id: String,
    latency_ms: Option<u32>,
    bandwidth_mbps: Option<f32>,
    packet_loss_percent: Option<f32>,
    jitter_ms: Option<u32>,
}

#[derive(Debug, Serialize)]
struct JoinStreamResponse {
    viewer_id: String,
    quality: VideoQuality,
    message: String,
}

#[derive(Debug, Deserialize)]
struct LeaveStreamRequest {
    user_id: Uuid,
}

#[derive(Debug, Serialize)]
struct LeaveStreamResponse {
    message: String,
}

#[derive(Debug, Serialize)]
struct StreamStatsResponse {
    stream_id: String,
    total_viewers: u32,
    quality_distribution: std::collections::HashMap<VideoQuality, u32>,
    average_latency_ms: u32,
    average_bandwidth_mbps: f32,
    stream_duration: u32,
    is_live: bool,
    status: String,
}

#[derive(Debug, Serialize)]
struct VideoMetadataResponse {
    duration: f64,
    width: u32,
    height: u32,
    bitrate: u32,
    frame_rate: f32,
    codec: String,
    format: String,
}

#[derive(Debug, Serialize)]
struct DeleteStreamResponse {
    message: String,
}

#[derive(Debug, Serialize)]
struct TranscodingProgressResponse {
    job_id: String,
    status: String,
    progress: f32,
    created_at: chrono::DateTime<chrono::Utc>,
    started_at: Option<chrono::DateTime<chrono::Utc>>,
    completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize)]
struct QualityRecommendationRequest {
    bandwidth_mbps: f32,
    latency_ms: u32,
}

#[derive(Debug, Serialize)]
struct QualityRecommendationResponse {
    recommended_quality: VideoQuality,
    resolution: (u32, u32),
    bitrate: u32,
} 