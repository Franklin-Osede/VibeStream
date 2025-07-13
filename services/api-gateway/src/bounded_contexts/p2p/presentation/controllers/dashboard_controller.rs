use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, Json},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::p2p::application::services::analytics_service::P2PAnalyticsService;
use crate::bounded_contexts::p2p::domain::repositories::analytics_repository::AnalyticsError;

/// Controlador para el dashboard de monitoreo P2P
pub struct P2PDashboardController<S> {
    analytics_service: Arc<P2PAnalyticsService<S>>,
}

impl<S> P2PDashboardController<S> {
    pub fn new(analytics_service: Arc<P2PAnalyticsService<S>>) -> Self {
        Self { analytics_service }
    }
}

impl<S> P2PDashboardController<S>
where
    S: crate::bounded_contexts::p2p::domain::repositories::analytics_repository::P2PAnalyticsRepository + 'static,
{
    /// Servir el dashboard HTML
    pub async fn serve_dashboard() -> Html<&'static str> {
        Html(include_str!("../../../../../../dashboard/index.html"))
    }

    /// Obtener métricas en tiempo real para el dashboard
    pub async fn get_realtime_metrics(
        State(controller): State<Arc<Self>>,
        Query(params): Query<DashboardParams>,
    ) -> Result<Json<DashboardMetricsResponse>, (StatusCode, Json<ErrorResponse>)> {
        println!("📊 GET /api/p2p/dashboard/realtime-metrics");
        
        let hours = params.hours.unwrap_or(24);
        let end_time = Utc::now();
        let start_time = end_time - chrono::Duration::hours(hours as i64);
        
        match controller.analytics_service.get_aggregated_stats(start_time, end_time).await {
            Ok(stats) => {
                let response = DashboardMetricsResponse {
                    timestamp: Utc::now(),
                    period_hours: hours,
                    overview: DashboardOverview {
                        total_sessions: stats.total_sessions,
                        total_streaming_hours: stats.total_streaming_hours,
                        average_connection_quality: format!("{:?}", stats.average_connection_quality),
                        average_latency_ms: stats.average_latency_ms,
                        average_bandwidth_mbps: stats.average_bandwidth_mbps,
                        success_rate_percent: stats.success_rate_percent,
                        peak_concurrent_users: stats.peak_concurrent_users,
                    },
                    system_health: SystemHealth {
                        cpu_usage_percent: stats.average_cpu_usage_percent,
                        memory_usage_mb: stats.average_memory_usage_mb,
                        network_throughput_mbps: stats.average_bandwidth_mbps,
                        error_rate_percent: 100.0 - stats.success_rate_percent,
                    },
                    p2p_metrics: P2PMetrics {
                        active_peers: stats.peak_concurrent_users,
                        total_connections: stats.total_sessions,
                        average_latency_ms: stats.average_latency_ms,
                        bandwidth_utilization_mbps: stats.average_bandwidth_mbps,
                        content_availability_score: 95.0, // TODO: Calculate from actual data
                        network_health_score: stats.success_rate_percent / 100.0,
                    },
                    streaming_metrics: StreamingMetrics {
                        active_streams: stats.peak_concurrent_users,
                        total_streams: stats.total_sessions,
                        average_quality: format!("{:?}", stats.average_connection_quality),
                        buffering_events: 0, // TODO: Calculate from actual data
                        quality_switches: 0, // TODO: Calculate from actual data
                        chunk_download_success_rate: stats.success_rate_percent / 100.0,
                        average_chunk_download_time_ms: stats.average_latency_ms,
                        transcoding_jobs_active: 0, // TODO: Calculate from actual data
                        transcoding_jobs_completed: 0, // TODO: Calculate from actual data
                        transcoding_jobs_failed: 0, // TODO: Calculate from actual data
                    },
                };
                Ok(Json(response))
            }
            Err(e) => {
                println!("❌ Error getting dashboard metrics: {}", e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("Failed to get dashboard metrics: {}", e),
                    }),
                ))
            }
        }
    }

    /// Obtener alertas del sistema
    pub async fn get_system_alerts(
        State(_controller): State<Arc<Self>>,
    ) -> Result<Json<SystemAlertsResponse>, (StatusCode, Json<ErrorResponse>)> {
        println!("🚨 GET /api/p2p/dashboard/alerts");
        
        // TODO: Implementar lógica real de alertas
        let alerts = vec![
            SystemAlert {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: Utc::now(),
                alert_type: "HighLatency".to_string(),
                severity: "Medium".to_string(),
                message: "Average latency is above threshold".to_string(),
                details: serde_json::json!({
                    "current_latency_ms": 150,
                    "threshold_ms": 100
                }),
                resolved: false,
            },
            SystemAlert {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: Utc::now() - chrono::Duration::minutes(30),
                alert_type: "LowBandwidth".to_string(),
                severity: "Low".to_string(),
                message: "Bandwidth utilization is below optimal".to_string(),
                details: serde_json::json!({
                    "current_bandwidth_mbps": 2.5,
                    "optimal_mbps": 5.0
                }),
                resolved: true,
            },
        ];

        let response = SystemAlertsResponse {
            total_alerts: alerts.len() as u32,
            critical_alerts: alerts.iter().filter(|a| a.severity == "Critical").count() as u32,
            active_alerts: alerts.iter().filter(|a| !a.resolved).count() as u32,
            alerts,
        };

        Ok(Json(response))
    }

    /// Obtener gráficos de tendencias
    pub async fn get_trend_charts(
        State(controller): State<Arc<Self>>,
        Query(params): Query<TrendChartParams>,
    ) -> Result<Json<TrendChartsResponse>, (StatusCode, Json<ErrorResponse>)> {
        println!("📈 GET /api/p2p/dashboard/trends");
        
        let hours = params.hours.unwrap_or(24);
        let end_time = Utc::now();
        let start_time = end_time - chrono::Duration::hours(hours as i64);
        
        // TODO: Implementar lógica real de tendencias
        // Por ahora, generamos datos de ejemplo
        let mut latency_trend = Vec::new();
        let mut bandwidth_trend = Vec::new();
        let mut sessions_trend = Vec::new();
        
        for i in 0..hours {
            let timestamp = end_time - chrono::Duration::hours((hours - i) as i64);
            latency_trend.push(TrendPoint {
                timestamp,
                value: 50.0 + (i as f64 * 2.0) + (rand::random::<f64>() * 20.0),
            });
            bandwidth_trend.push(TrendPoint {
                timestamp,
                value: 5.0 + (i as f64 * 0.1) + (rand::random::<f64>() * 2.0),
            });
            sessions_trend.push(TrendPoint {
                timestamp,
                value: 100.0 + (i as f64 * 5.0) + (rand::random::<f64>() * 50.0),
            });
        }

        let response = TrendChartsResponse {
            latency_trend,
            bandwidth_trend,
            sessions_trend,
            success_rate_trend: vec![
                TrendPoint { timestamp: start_time, value: 95.0 },
                TrendPoint { timestamp: end_time, value: 98.0 },
            ],
        };

        Ok(Json(response))
    }
}

// Request DTOs
#[derive(Debug, Deserialize)]
pub struct DashboardParams {
    pub hours: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct TrendChartParams {
    pub hours: Option<u32>,
    pub metric_type: Option<String>,
}

// Response DTOs
#[derive(Debug, Serialize)]
pub struct DashboardMetricsResponse {
    pub timestamp: DateTime<Utc>,
    pub period_hours: u32,
    pub overview: DashboardOverview,
    pub system_health: SystemHealth,
    pub p2p_metrics: P2PMetrics,
    pub streaming_metrics: StreamingMetrics,
}

#[derive(Debug, Serialize)]
pub struct DashboardOverview {
    pub total_sessions: u64,
    pub total_streaming_hours: f64,
    pub average_connection_quality: String,
    pub average_latency_ms: f64,
    pub average_bandwidth_mbps: f64,
    pub success_rate_percent: f64,
    pub peak_concurrent_users: u32,
}

#[derive(Debug, Serialize)]
pub struct SystemHealth {
    pub cpu_usage_percent: f64,
    pub memory_usage_mb: u64,
    pub network_throughput_mbps: f64,
    pub error_rate_percent: f64,
}

#[derive(Debug, Serialize)]
pub struct P2PMetrics {
    pub active_peers: u32,
    pub total_connections: u64,
    pub average_latency_ms: f64,
    pub bandwidth_utilization_mbps: f64,
    pub content_availability_score: f64,
    pub network_health_score: f64,
}

#[derive(Debug, Serialize)]
pub struct StreamingMetrics {
    pub active_streams: u32,
    pub total_streams: u64,
    pub average_quality: String,
    pub buffering_events: u64,
    pub quality_switches: u64,
    pub chunk_download_success_rate: f64,
    pub average_chunk_download_time_ms: f64,
    pub transcoding_jobs_active: u32,
    pub transcoding_jobs_completed: u64,
    pub transcoding_jobs_failed: u64,
}

#[derive(Debug, Serialize)]
pub struct SystemAlertsResponse {
    pub total_alerts: u32,
    pub critical_alerts: u32,
    pub active_alerts: u32,
    pub alerts: Vec<SystemAlert>,
}

#[derive(Debug, Serialize)]
pub struct SystemAlert {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub alert_type: String,
    pub severity: String,
    pub message: String,
    pub details: serde_json::Value,
    pub resolved: bool,
}

#[derive(Debug, Serialize)]
pub struct TrendChartsResponse {
    pub latency_trend: Vec<TrendPoint>,
    pub bandwidth_trend: Vec<TrendPoint>,
    pub sessions_trend: Vec<TrendPoint>,
    pub success_rate_trend: Vec<TrendPoint>,
}

#[derive(Debug, Serialize)]
pub struct TrendPoint {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
} 