use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::p2p::application::services::analytics_service::P2PAnalyticsService;
use crate::bounded_contexts::p2p::domain::entities::analytics::{
    P2PConnectionMetrics, StreamingMetrics, NetworkMetrics, SystemPerformanceMetrics,
    ConnectionQuality, VideoQuality, ConnectionType
};
use crate::bounded_contexts::p2p::domain::repositories::analytics_repository::AnalyticsError;

/// Controlador para analíticas P2P
pub struct P2PAnalyticsController<S> {
    analytics_service: Arc<P2PAnalyticsService<S>>,
}

impl<S> P2PAnalyticsController<S> {
    pub fn new(analytics_service: Arc<P2PAnalyticsService<S>>) -> Self {
        Self { analytics_service }
    }
}

impl<S> P2PAnalyticsController<S>
where
    S: crate::bounded_contexts::p2p::domain::repositories::analytics_repository::P2PAnalyticsRepository + 'static,
{
    /// Obtener analíticas de sesión
    pub async fn get_session_analytics(
        State(controller): State<Arc<Self>>,
        Path(session_id): Path<String>,
    ) -> Result<Json<SessionAnalyticsResponse>, (StatusCode, Json<ErrorResponse>)> {
        println!("📊 GET /api/p2p/analytics/session/{}", session_id);
        
        match controller.analytics_service.get_session_analytics(&session_id).await {
            Ok(Some(analytics)) => {
                let response = SessionAnalyticsResponse {
                    session_id: analytics.session_id,
                    user_id: analytics.user_id,
                    total_connections: analytics.connection_metrics.len() as u32,
                    total_streaming_sessions: analytics.streaming_metrics.len() as u32,
                    average_connection_quality: format!("{:?}", analytics.get_average_connection_quality()),
                    total_streaming_hours: analytics.get_total_streaming_duration() / 3600.0,
                    success_rate_percent: analytics.get_success_rate(),
                    created_at: analytics.created_at,
                    updated_at: analytics.updated_at,
                };
                Ok(Json(response))
            }
            Ok(None) => Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "Session analytics not found".to_string(),
                }),
            )),
            Err(e) => {
                println!("❌ Error getting session analytics: {}", e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Failed to get session analytics: {}", e),
                    }),
                ))
            }
        }
    }

    /// Obtener analíticas de usuario
    pub async fn get_user_analytics(
        State(controller): State<Arc<Self>>,
        Path(user_id): Path<String>,
    ) -> Result<Json<UserAnalyticsResponse>, (StatusCode, Json<ErrorResponse>)> {
        println!("👤 GET /api/p2p/analytics/user/{}", user_id);
        
        match controller.analytics_service.get_user_analytics(&user_id).await {
            Ok(analytics) => {
                let total_sessions = analytics.len() as u64;
                let total_streaming_hours: f64 = analytics
                    .iter()
                    .map(|a| a.get_total_streaming_duration())
                    .sum::<f64>() / 3600.0;

                let average_quality = if !analytics.is_empty() {
                    let qualities: Vec<ConnectionQuality> = analytics
                        .iter()
                        .map(|a| a.get_average_connection_quality())
                        .collect();
                    
                    let quality_scores: Vec<u8> = qualities
                        .iter()
                        .map(|q| match q {
                            ConnectionQuality::Excellent => 5,
                            ConnectionQuality::Good => 4,
                            ConnectionQuality::Fair => 3,
                            ConnectionQuality::Poor => 2,
                            ConnectionQuality::Unusable => 1,
                        })
                        .collect();
                    
                    let avg_score = quality_scores.iter().sum::<u8>() as f64 / quality_scores.len() as f64;
                    
                    match avg_score {
                        score if score >= 4.5 => "Excellent",
                        score if score >= 3.5 => "Good",
                        score if score >= 2.5 => "Fair",
                        score if score >= 1.5 => "Poor",
                        _ => "Unusable",
                    }.to_string()
                } else {
                    "Good".to_string()
                };

                let success_rate = if !analytics.is_empty() {
                    analytics.iter().map(|a| a.get_success_rate()).sum::<f64>() / analytics.len() as f64
                } else {
                    100.0
                };

                let response = UserAnalyticsResponse {
                    user_id,
                    total_sessions,
                    total_streaming_hours,
                    average_connection_quality: average_quality,
                    success_rate_percent: success_rate,
                    sessions: analytics.into_iter().map(|a| SessionSummary {
                        session_id: a.session_id,
                        created_at: a.created_at,
                        total_connections: a.connection_metrics.len() as u32,
                        total_streaming_sessions: a.streaming_metrics.len() as u32,
                        average_quality: format!("{:?}", a.get_average_connection_quality()),
                    }).collect(),
                };
                Ok(Json(response))
            }
            Err(e) => {
                println!("❌ Error getting user analytics: {}", e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Failed to get user analytics: {}", e),
                    }),
                ))
            }
        }
    }

    /// Obtener estadísticas agregadas
    pub async fn get_aggregated_stats(
        State(controller): State<Arc<Self>>,
        Query(params): Query<TimeRangeParams>,
    ) -> Result<Json<AggregatedStatsResponse>, (StatusCode, Json<ErrorResponse>)> {
        println!("📊 GET /api/p2p/analytics/stats");
        
        let end_time = Utc::now();
        let start_time = end_time - chrono::Duration::hours(params.hours.unwrap_or(24) as i64);
        
        match controller.analytics_service.get_aggregated_stats(start_time, end_time).await {
            Ok(stats) => {
                let response = AggregatedStatsResponse {
                    period_hours: params.hours.unwrap_or(24),
                    total_sessions: stats.total_sessions,
                    total_streaming_hours: stats.total_streaming_hours,
                    average_connection_quality: format!("{:?}", stats.average_connection_quality),
                    average_latency_ms: stats.average_latency_ms,
                    average_bandwidth_mbps: stats.average_bandwidth_mbps,
                    total_data_transferred_gb: stats.total_data_transferred_gb,
                    success_rate_percent: stats.success_rate_percent,
                    peak_concurrent_users: stats.peak_concurrent_users,
                    average_cpu_usage_percent: stats.average_cpu_usage_percent,
                    average_memory_usage_mb: stats.average_memory_usage_mb,
                    generated_at: Utc::now(),
                };
                Ok(Json(response))
            }
            Err(e) => {
                println!("❌ Error getting aggregated stats: {}", e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Failed to get aggregated stats: {}", e),
                    }),
                ))
            }
        }
    }

    /// Registrar métricas de conexión
    pub async fn record_connection_metrics(
        State(controller): State<Arc<Self>>,
        Json(payload): Json<RecordConnectionMetricsRequest>,
    ) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
        println!("📊 POST /api/p2p/analytics/connection-metrics");
        
        let metrics = P2PConnectionMetrics {
            connection_id: payload.connection_id,
            session_id: payload.session_id.clone(),
            peer_id: payload.peer_id,
            connection_type: payload.connection_type,
            latency_ms: payload.latency_ms,
            bandwidth_mbps: payload.bandwidth_mbps,
            packet_loss_percent: payload.packet_loss_percent,
            jitter_ms: payload.jitter_ms,
            connection_quality: payload.connection_quality,
            ice_connection_state: payload.ice_connection_state,
            dtls_transport_state: payload.dtls_transport_state,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        match controller.analytics_service.record_connection_metrics(
            &payload.session_id,
            &payload.user_id,
            metrics,
        ).await {
            Ok(_) => Ok(Json(SuccessResponse {
                message: "Connection metrics recorded successfully".to_string(),
            })),
            Err(e) => {
                println!("❌ Error recording connection metrics: {}", e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Failed to record connection metrics: {}", e),
                    }),
                ))
            }
        }
    }

    /// Registrar métricas de streaming
    pub async fn record_streaming_metrics(
        State(controller): State<Arc<Self>>,
        Json(payload): Json<RecordStreamingMetricsRequest>,
    ) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
        println!("🎬 POST /api/p2p/analytics/streaming-metrics");
        
        let metrics = StreamingMetrics {
            stream_id: payload.stream_id,
            content_id: payload.content_id,
            user_id: payload.user_id.clone(),
            quality_level: payload.quality_level,
            bitrate_kbps: payload.bitrate_kbps,
            frame_rate: payload.frame_rate,
            resolution_width: payload.resolution_width,
            resolution_height: payload.resolution_height,
            buffer_level_seconds: payload.buffer_level_seconds,
            dropped_frames: payload.dropped_frames,
            total_frames: payload.total_frames,
            adaptive_switches: payload.adaptive_switches,
            start_time: payload.start_time,
            end_time: payload.end_time,
            duration_seconds: payload.duration_seconds,
        };

        match controller.analytics_service.record_streaming_metrics(
            &payload.session_id,
            &payload.user_id,
            metrics,
        ).await {
            Ok(_) => Ok(Json(SuccessResponse {
                message: "Streaming metrics recorded successfully".to_string(),
            })),
            Err(e) => {
                println!("❌ Error recording streaming metrics: {}", e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Failed to record streaming metrics: {}", e),
                    }),
                ))
            }
        }
    }

    /// Generar reporte de rendimiento
    pub async fn generate_performance_report(
        State(controller): State<Arc<Self>>,
        Path(user_id): Path<String>,
        Query(params): Query<ReportParams>,
    ) -> Result<Json<PerformanceReportResponse>, (StatusCode, Json<ErrorResponse>)> {
        println!("📋 GET /api/p2p/analytics/performance-report/{}", user_id);
        
        let days = params.days.unwrap_or(7);
        
        match controller.analytics_service.generate_performance_report(&user_id, days).await {
            Ok(report) => {
                let response = PerformanceReportResponse {
                    user_id: report.user_id,
                    period_days: report.period_days,
                    total_sessions: report.total_sessions,
                    total_streaming_hours: report.total_streaming_hours,
                    average_connection_quality: format!("{:?}", report.average_connection_quality),
                    success_rate_percent: report.success_rate_percent,
                    generated_at: report.generated_at,
                };
                Ok(Json(response))
            }
            Err(e) => {
                println!("❌ Error generating performance report: {}", e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Failed to generate performance report: {}", e),
                    }),
                ))
            }
        }
    }
}

// Request/Response types

#[derive(Debug, Deserialize)]
pub struct TimeRangeParams {
    pub hours: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ReportParams {
    pub days: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct RecordConnectionMetricsRequest {
    pub session_id: String,
    pub user_id: String,
    pub connection_id: String,
    pub peer_id: String,
    pub connection_type: ConnectionType,
    pub latency_ms: u32,
    pub bandwidth_mbps: f64,
    pub packet_loss_percent: f64,
    pub jitter_ms: u32,
    pub connection_quality: ConnectionQuality,
    pub ice_connection_state: String,
    pub dtls_transport_state: String,
}

#[derive(Debug, Deserialize)]
pub struct RecordStreamingMetricsRequest {
    pub session_id: String,
    pub user_id: String,
    pub stream_id: String,
    pub content_id: String,
    pub quality_level: VideoQuality,
    pub bitrate_kbps: u32,
    pub frame_rate: f32,
    pub resolution_width: u32,
    pub resolution_height: u32,
    pub buffer_level_seconds: f32,
    pub dropped_frames: u32,
    pub total_frames: u32,
    pub adaptive_switches: u32,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_seconds: f64,
}

#[derive(Debug, Serialize)]
pub struct SessionAnalyticsResponse {
    pub session_id: String,
    pub user_id: String,
    pub total_connections: u32,
    pub total_streaming_sessions: u32,
    pub average_connection_quality: String,
    pub total_streaming_hours: f64,
    pub success_rate_percent: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UserAnalyticsResponse {
    pub user_id: String,
    pub total_sessions: u64,
    pub total_streaming_hours: f64,
    pub average_connection_quality: String,
    pub success_rate_percent: f64,
    pub sessions: Vec<SessionSummary>,
}

#[derive(Debug, Serialize)]
pub struct SessionSummary {
    pub session_id: String,
    pub created_at: DateTime<Utc>,
    pub total_connections: u32,
    pub total_streaming_sessions: u32,
    pub average_quality: String,
}

#[derive(Debug, Serialize)]
pub struct AggregatedStatsResponse {
    pub period_hours: u32,
    pub total_sessions: u64,
    pub total_streaming_hours: f64,
    pub average_connection_quality: String,
    pub average_latency_ms: f64,
    pub average_bandwidth_mbps: f64,
    pub total_data_transferred_gb: f64,
    pub success_rate_percent: f64,
    pub peak_concurrent_users: u32,
    pub average_cpu_usage_percent: f64,
    pub average_memory_usage_mb: u64,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PerformanceReportResponse {
    pub user_id: String,
    pub period_days: u32,
    pub total_sessions: u64,
    pub total_streaming_hours: f64,
    pub average_connection_quality: String,
    pub success_rate_percent: f64,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
} 