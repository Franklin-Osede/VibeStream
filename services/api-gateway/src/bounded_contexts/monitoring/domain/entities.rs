use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Métricas de rendimiento P2P
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PMetrics {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub active_peers: u32,
    pub total_connections: u32,
    pub successful_transfers: u64,
    pub failed_transfers: u64,
    pub average_latency_ms: f64,
    pub bandwidth_utilization_mbps: f64,
    pub content_availability_score: f64,
    pub network_health_score: f64,
}

/// Métricas de streaming de video
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoStreamingMetrics {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
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

/// Métricas de almacenamiento IPFS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPFSStorageMetrics {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub total_files_stored: u64,
    pub total_storage_used_gb: f64,
    pub ipfs_node_status: String,
    pub peer_connections: u32,
    pub content_replication_factor: f64,
    pub pin_operations: u64,
    pub unpin_operations: u64,
    pub garbage_collection_runs: u32,
    pub storage_efficiency_percentage: f64,
}

/// Métricas de pagos y transacciones
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMetrics {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub total_transactions: u64,
    pub successful_transactions: u64,
    pub failed_transactions: u64,
    pub total_volume_usd: f64,
    pub average_transaction_amount: f64,
    pub stripe_transactions: u64,
    pub paypal_transactions: u64,
    pub coinbase_transactions: u64,
    pub payment_success_rate: f64,
    pub average_processing_time_ms: f64,
}

/// Métricas de usuarios y actividad
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserActivityMetrics {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub active_users: u32,
    pub total_users: u64,
    pub new_registrations: u32,
    pub user_sessions: u64,
    pub average_session_duration_minutes: f64,
    pub content_uploads: u64,
    pub content_downloads: u64,
    pub playlist_creations: u64,
    pub social_interactions: u64,
    pub user_retention_rate: f64,
}

/// Métricas de sistema y recursos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub cpu_usage_percentage: f64,
    pub memory_usage_percentage: f64,
    pub disk_usage_percentage: f64,
    pub network_bandwidth_mbps: f64,
    pub active_connections: u32,
    pub request_rate_per_second: f64,
    pub error_rate_percentage: f64,
    pub response_time_ms: f64,
    pub database_connections: u32,
    pub cache_hit_rate: f64,
}

/// Alertas del sistema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemAlert {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub details: serde_json::Value,
    pub resolved: bool,
    pub resolved_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    HighCpuUsage,
    HighMemoryUsage,
    LowDiskSpace,
    NetworkLatency,
    P2PConnectionFailure,
    VideoStreamingError,
    PaymentFailure,
    DatabaseConnectionError,
    IPFSNodeOffline,
    SecurityThreat,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Configuración de monitoreo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub metrics_collection_interval_seconds: u64,
    pub alert_check_interval_seconds: u64,
    pub retention_days: u32,
    pub enabled_metrics: Vec<MetricType>,
    pub alert_thresholds: AlertThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    P2PMetrics,
    VideoStreamingMetrics,
    IPFSStorageMetrics,
    PaymentMetrics,
    UserActivityMetrics,
    SystemMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub cpu_usage_threshold: f64,
    pub memory_usage_threshold: f64,
    pub disk_usage_threshold: f64,
    pub network_latency_threshold_ms: f64,
    pub error_rate_threshold: f64,
    pub p2p_connection_threshold: u32,
    pub payment_failure_threshold: f64,
}

/// Dashboard de monitoreo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringDashboard {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub widgets: Vec<DashboardWidget>,
    pub refresh_interval_seconds: u64,
    pub is_public: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardWidget {
    pub id: Uuid,
    pub widget_type: WidgetType,
    pub title: String,
    pub position: WidgetPosition,
    pub size: WidgetSize,
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WidgetType {
    MetricCard,
    LineChart,
    BarChart,
    PieChart,
    Gauge,
    Table,
    AlertList,
    StatusIndicator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetPosition {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetSize {
    pub width: u32,
    pub height: u32,
}

/// Reporte de rendimiento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceReport {
    pub id: Uuid,
    pub generated_at: DateTime<Utc>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub summary: PerformanceSummary,
    pub detailed_metrics: DetailedMetrics,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSummary {
    pub overall_health_score: f64,
    pub system_status: String,
    pub critical_alerts: u32,
    pub performance_trend: String,
    pub top_issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedMetrics {
    pub p2p_metrics: Option<P2PMetrics>,
    pub video_metrics: Option<VideoStreamingMetrics>,
    pub storage_metrics: Option<IPFSStorageMetrics>,
    pub payment_metrics: Option<PaymentMetrics>,
    pub user_metrics: Option<UserActivityMetrics>,
    pub system_metrics: Option<SystemMetrics>,
} 