use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Métricas de conexión P2P
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PConnectionMetrics {
    pub connection_id: String,
    pub session_id: String,
    pub peer_id: String,
    pub connection_type: ConnectionType,
    pub latency_ms: u32,
    pub bandwidth_mbps: f64,
    pub packet_loss_percent: f64,
    pub jitter_ms: u32,
    pub connection_quality: ConnectionQuality,
    pub ice_connection_state: String,
    pub dtls_transport_state: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Tipo de conexión P2P
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConnectionType {
    WebRTC,
    WebSocket,
    Direct,
    Relay,
}

/// Calidad de conexión
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConnectionQuality {
    Excellent,
    Good,
    Fair,
    Poor,
    Unusable,
}

/// Métricas de streaming de contenido
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingMetrics {
    pub stream_id: String,
    pub content_id: String,
    pub user_id: String,
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

/// Calidad de video
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VideoQuality {
    UltraHD,
    FullHD,
    HD,
    SD,
    Low,
}

/// Métricas de red P2P
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    pub peer_id: String,
    pub total_data_sent_bytes: u64,
    pub total_data_received_bytes: u64,
    pub active_connections: u32,
    pub max_connections: u32,
    pub connection_errors: u32,
    pub successful_transfers: u32,
    pub failed_transfers: u32,
    pub average_transfer_speed_mbps: f64,
    pub timestamp: DateTime<Utc>,
}

/// Métricas de rendimiento del sistema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemPerformanceMetrics {
    pub cpu_usage_percent: f64,
    pub memory_usage_mb: u64,
    pub network_throughput_mbps: f64,
    pub active_streams: u32,
    pub total_peers: u32,
    pub average_latency_ms: f64,
    pub error_rate_percent: f64,
    pub timestamp: DateTime<Utc>,
}

/// Agregado de analíticas P2P
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PAnalyticsAggregate {
    pub id: String,
    pub session_id: String,
    pub user_id: String,
    pub connection_metrics: Vec<P2PConnectionMetrics>,
    pub streaming_metrics: Vec<StreamingMetrics>,
    pub network_metrics: NetworkMetrics,
    pub system_metrics: SystemPerformanceMetrics,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl P2PAnalyticsAggregate {
    pub fn new(session_id: String, user_id: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            session_id,
            user_id,
            connection_metrics: Vec::new(),
            streaming_metrics: Vec::new(),
            network_metrics: NetworkMetrics {
                peer_id: user_id.clone(),
                total_data_sent_bytes: 0,
                total_data_received_bytes: 0,
                active_connections: 0,
                max_connections: 10,
                connection_errors: 0,
                successful_transfers: 0,
                failed_transfers: 0,
                average_transfer_speed_mbps: 0.0,
                timestamp: Utc::now(),
            },
            system_metrics: SystemPerformanceMetrics {
                cpu_usage_percent: 0.0,
                memory_usage_mb: 0,
                network_throughput_mbps: 0.0,
                active_streams: 0,
                total_peers: 0,
                average_latency_ms: 0.0,
                error_rate_percent: 0.0,
                timestamp: Utc::now(),
            },
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn add_connection_metric(&mut self, metric: P2PConnectionMetrics) {
        self.connection_metrics.push(metric);
        self.updated_at = Utc::now();
    }

    pub fn add_streaming_metric(&mut self, metric: StreamingMetrics) {
        self.streaming_metrics.push(metric);
        self.updated_at = Utc::now();
    }

    pub fn update_network_metrics(&mut self, metrics: NetworkMetrics) {
        self.network_metrics = metrics;
        self.updated_at = Utc::now();
    }

    pub fn update_system_metrics(&mut self, metrics: SystemPerformanceMetrics) {
        self.system_metrics = metrics;
        self.updated_at = Utc::now();
    }

    pub fn get_average_connection_quality(&self) -> ConnectionQuality {
        if self.connection_metrics.is_empty() {
            return ConnectionQuality::Good;
        }

        let quality_scores: Vec<u8> = self.connection_metrics
            .iter()
            .map(|m| match m.connection_quality {
                ConnectionQuality::Excellent => 5,
                ConnectionQuality::Good => 4,
                ConnectionQuality::Fair => 3,
                ConnectionQuality::Poor => 2,
                ConnectionQuality::Unusable => 1,
            })
            .collect();

        let average_score = quality_scores.iter().sum::<u8>() as f64 / quality_scores.len() as f64;

        match average_score {
            score if score >= 4.5 => ConnectionQuality::Excellent,
            score if score >= 3.5 => ConnectionQuality::Good,
            score if score >= 2.5 => ConnectionQuality::Fair,
            score if score >= 1.5 => ConnectionQuality::Poor,
            _ => ConnectionQuality::Unusable,
        }
    }

    pub fn get_total_streaming_duration(&self) -> f64 {
        self.streaming_metrics
            .iter()
            .map(|m| m.duration_seconds)
            .sum()
    }

    pub fn get_success_rate(&self) -> f64 {
        let total_transfers = self.network_metrics.successful_transfers + self.network_metrics.failed_transfers;
        if total_transfers == 0 {
            return 100.0;
        }
        (self.network_metrics.successful_transfers as f64 / total_transfers as f64) * 100.0
    }
} 