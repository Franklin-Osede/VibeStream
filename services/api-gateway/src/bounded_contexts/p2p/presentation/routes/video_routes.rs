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

use crate::bounded_contexts::p2p::application::services::VideoStreamingService;
use crate::bounded_contexts::p2p::domain::entities::video_stream::{
    VideoStreamId, VideoQuality, ConnectionQuality
};

/// Video streaming routes
pub fn video_routes() -> Router<Arc<VideoStreamingService>> {
    Router::new()
        .route("/streams", post(create_stream))
        .route("/streams/:stream_id", get(get_stream))
        .route("/streams/:stream_id", delete(stop_stream))
        .route("/streams/:stream_id/join", post(join_stream))
        .route("/streams/:stream_id/leave", post(leave_stream))
        .route("/streams/:stream_id/stats", get(get_stream_stats))
        .route("/streams/:stream_id/chunks/:chunk_id", get(get_chunk))
        .route("/streams/:stream_id/quality", post(update_quality))
}

/// Create new video stream
async fn create_stream(
    State(service): State<Arc<VideoStreamingService>>,
    Json(request): Json<CreateStreamRequest>,
) -> Result<Json<CreateStreamResponse>, (StatusCode, String)> {
    let stream_id = service.start_stream(
        request.title,
        request.artist_id,
        request.video_url,
        request.duration_seconds,
        request.is_live,
    ).await
    .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    Ok(Json(CreateStreamResponse {
        stream_id: stream_id.to_string(),
        message: "Stream created successfully".to_string(),
    }))
}

/// Get stream information
async fn get_stream(
    State(service): State<Arc<VideoStreamingService>>,
    Path(stream_id): Path<String>,
) -> Result<Json<StreamResponse>, (StatusCode, String)> {
    let stream_id = VideoStreamId::from_uuid(
        Uuid::parse_str(&stream_id)
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid stream ID: {}", e)))?
    );

    // TODO: Implement get_stream method in service
    Err((StatusCode::NOT_IMPLEMENTED, "Get stream not implemented".to_string()))
}

/// Stop video stream
async fn stop_stream(
    State(service): State<Arc<VideoStreamingService>>,
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

/// Join video stream as viewer
async fn join_stream(
    State(service): State<Arc<VideoStreamingService>>,
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

/// Leave video stream
async fn leave_stream(
    State(service): State<Arc<VideoStreamingService>>,
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
    State(service): State<Arc<VideoStreamingService>>,
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

/// Get video chunk
async fn get_chunk(
    State(service): State<Arc<VideoStreamingService>>,
    Path((stream_id, chunk_id)): Path<(String, String)>,
    Query(params): Query<GetChunkParams>,
) -> Result<Json<ChunkResponse>, (StatusCode, String)> {
    let stream_id = VideoStreamId::from_uuid(
        Uuid::parse_str(&stream_id)
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid stream ID: {}", e)))?
    );

    let chunk_id = crate::bounded_contexts::p2p::domain::entities::video_stream::VideoChunkId::from_uuid(
        Uuid::parse_str(&chunk_id)
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid chunk ID: {}", e)))?
    );

    let chunk = service.get_chunk(&stream_id, &chunk_id, &params.peer_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    match chunk {
        Some(chunk) => Ok(Json(ChunkResponse {
            chunk_id: chunk.id.0.to_string(),
            data: chunk.data,
            timestamp: chunk.timestamp,
            duration: chunk.duration,
            quality: chunk.quality,
        })),
        None => Err((StatusCode::NOT_FOUND, "Chunk not available".to_string())),
    }
}

/// Update stream quality
async fn update_quality(
    State(service): State<Arc<VideoStreamingService>>,
    Path(stream_id): Path<String>,
    Json(request): Json<UpdateQualityRequest>,
) -> Result<Json<UpdateQualityResponse>, (StatusCode, String)> {
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

    service.update_viewer_quality(&stream_id, request.user_id, connection_quality).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(UpdateQualityResponse {
        message: "Quality updated successfully".to_string(),
    }))
}

// Request/Response types

#[derive(Debug, Deserialize)]
struct CreateStreamRequest {
    title: String,
    artist_id: Uuid,
    video_url: String,
    duration_seconds: u32,
    is_live: bool,
}

#[derive(Debug, Serialize)]
struct CreateStreamResponse {
    stream_id: String,
    message: String,
}

#[derive(Debug, Serialize)]
struct StreamResponse {
    stream_id: String,
    title: String,
    status: String,
    current_viewers: u32,
    max_viewers: u32,
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

#[derive(Debug, Deserialize)]
struct GetChunkParams {
    peer_id: String,
}

#[derive(Debug, Serialize)]
struct ChunkResponse {
    chunk_id: String,
    data: Vec<u8>,
    timestamp: u64,
    duration: u32,
    quality: VideoQuality,
}

#[derive(Debug, Deserialize)]
struct UpdateQualityRequest {
    user_id: Uuid,
    latency_ms: Option<u32>,
    bandwidth_mbps: Option<f32>,
    packet_loss_percent: Option<f32>,
    jitter_ms: Option<u32>,
}

#[derive(Debug, Serialize)]
struct UpdateQualityResponse {
    message: String,
} 