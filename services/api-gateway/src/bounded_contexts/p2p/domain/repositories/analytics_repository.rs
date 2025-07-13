use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::bounded_contexts::p2p::domain::entities::analytics::{
    P2PAnalyticsAggregate, P2PConnectionMetrics, StreamingMetrics, 
    NetworkMetrics, SystemPerformanceMetrics, ConnectionQuality
};

#[async_trait]
pub trait P2PAnalyticsRepository: Send + Sync {
    /// Guardar métricas de analíticas P2P
    async fn save_analytics(&self, analytics: &P2PAnalyticsAggregate) -> Result<(), AnalyticsError>;
    
    /// Obtener analíticas por ID
    async fn find_by_id(&self, id: &str) -> Result<Option<P2PAnalyticsAggregate>, AnalyticsError>;
    
    /// Obtener analíticas por sesión
    async fn find_by_session(&self, session_id: &str) -> Result<Option<P2PAnalyticsAggregate>, AnalyticsError>;
    
    /// Obtener analíticas por usuario
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<P2PAnalyticsAggregate>, AnalyticsError>;
    
    /// Obtener analíticas en un rango de tiempo
    async fn find_by_time_range(
        &self, 
        start_time: DateTime<Utc>, 
        end_time: DateTime<Utc>
    ) -> Result<Vec<P2PAnalyticsAggregate>, AnalyticsError>;
    
    /// Obtener métricas de conexión por calidad
    async fn find_connections_by_quality(
        &self, 
        quality: ConnectionQuality,
        limit: Option<u32>
    ) -> Result<Vec<P2PConnectionMetrics>, AnalyticsError>;
    
    /// Obtener métricas de rendimiento del sistema
    async fn get_system_performance_metrics(
        &self,
        hours: u32
    ) -> Result<Vec<SystemPerformanceMetrics>, AnalyticsError>;
    
    /// Obtener estadísticas agregadas
    async fn get_aggregated_stats(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>
    ) -> Result<AggregatedStats, AnalyticsError>;
    
    /// Eliminar analíticas antiguas
    async fn cleanup_old_analytics(&self, days_to_keep: u32) -> Result<u64, AnalyticsError>;
}

/// Estadísticas agregadas de analíticas P2P
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedStats {
    pub total_sessions: u64,
    pub total_streaming_hours: f64,
    pub average_connection_quality: ConnectionQuality,
    pub average_latency_ms: f64,
    pub average_bandwidth_mbps: f64,
    pub total_data_transferred_gb: f64,
    pub success_rate_percent: f64,
    pub peak_concurrent_users: u32,
    pub average_cpu_usage_percent: f64,
    pub average_memory_usage_mb: u64,
}

/// Errores del repositorio de analíticas
#[derive(Debug, thiserror::Error)]
pub enum AnalyticsError {
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Analytics not found")]
    NotFound,
    
    #[error("Invalid time range")]
    InvalidTimeRange,
    
    #[error("Storage error: {0}")]
    Storage(String),
}

impl From<sqlx::Error> for AnalyticsError {
    fn from(err: sqlx::Error) -> Self {
        AnalyticsError::Database(err.to_string())
    }
}

impl From<serde_json::Error> for AnalyticsError {
    fn from(err: serde_json::Error) -> Self {
        AnalyticsError::Serialization(err.to_string())
    }
} 