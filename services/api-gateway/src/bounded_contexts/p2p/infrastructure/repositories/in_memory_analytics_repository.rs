use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use crate::bounded_contexts::p2p::domain::entities::analytics::{
    P2PAnalyticsAggregate, P2PConnectionMetrics, StreamingMetrics,
    NetworkMetrics, SystemPerformanceMetrics, ConnectionQuality
};
use crate::bounded_contexts::p2p::domain::repositories::analytics_repository::{
    P2PAnalyticsRepository, AggregatedStats, AnalyticsError
};

/// Repositorio en memoria para analíticas P2P
pub struct InMemoryP2PAnalyticsRepository {
    analytics: Arc<RwLock<HashMap<String, P2PAnalyticsAggregate>>>,
    connection_metrics: Arc<RwLock<Vec<P2PConnectionMetrics>>>,
    system_metrics: Arc<RwLock<Vec<SystemPerformanceMetrics>>>,
}

impl InMemoryP2PAnalyticsRepository {
    pub fn new() -> Self {
        Self {
            analytics: Arc::new(RwLock::new(HashMap::new())),
            connection_metrics: Arc::new(RwLock::new(Vec::new())),
            system_metrics: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[async_trait::async_trait]
impl P2PAnalyticsRepository for InMemoryP2PAnalyticsRepository {
    async fn save_analytics(&self, analytics: &P2PAnalyticsAggregate) -> Result<(), AnalyticsError> {
        let mut storage = self.analytics.write().await;
        storage.insert(analytics.id.clone(), analytics.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<P2PAnalyticsAggregate>, AnalyticsError> {
        let storage = self.analytics.read().await;
        Ok(storage.get(id).cloned())
    }

    async fn find_by_session(&self, session_id: &str) -> Result<Option<P2PAnalyticsAggregate>, AnalyticsError> {
        let storage = self.analytics.read().await;
        Ok(storage.values().find(|a| a.session_id == session_id).cloned())
    }

    async fn find_by_user(&self, user_id: &str) -> Result<Vec<P2PAnalyticsAggregate>, AnalyticsError> {
        let storage = self.analytics.read().await;
        Ok(storage.values()
            .filter(|a| a.user_id == user_id)
            .cloned()
            .collect())
    }

    async fn find_by_time_range(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<P2PAnalyticsAggregate>, AnalyticsError> {
        let storage = self.analytics.read().await;
        Ok(storage.values()
            .filter(|a| a.created_at >= start_time && a.created_at <= end_time)
            .cloned()
            .collect())
    }

    async fn find_connections_by_quality(
        &self,
        quality: ConnectionQuality,
        limit: Option<u32>,
    ) -> Result<Vec<P2PConnectionMetrics>, AnalyticsError> {
        let mut connections = self.connection_metrics.read().await;
        let filtered: Vec<_> = connections
            .iter()
            .filter(|c| c.connection_quality == quality)
            .cloned()
            .collect();

        if let Some(limit) = limit {
            Ok(filtered.into_iter().take(limit as usize).collect())
        } else {
            Ok(filtered)
        }
    }

    async fn get_system_performance_metrics(
        &self,
        hours: u32,
    ) -> Result<Vec<SystemPerformanceMetrics>, AnalyticsError> {
        let cutoff_time = Utc::now() - chrono::Duration::hours(hours as i64);
        let metrics = self.system_metrics.read().await;
        
        Ok(metrics
            .iter()
            .filter(|m| m.timestamp >= cutoff_time)
            .cloned()
            .collect())
    }

    async fn get_aggregated_stats(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<AggregatedStats, AnalyticsError> {
        let analytics = self.find_by_time_range(start_time, end_time).await?;
        
        if analytics.is_empty() {
            return Ok(AggregatedStats {
                total_sessions: 0,
                total_streaming_hours: 0.0,
                average_connection_quality: ConnectionQuality::Good,
                average_latency_ms: 0.0,
                average_bandwidth_mbps: 0.0,
                total_data_transferred_gb: 0.0,
                success_rate_percent: 100.0,
                peak_concurrent_users: 0,
                average_cpu_usage_percent: 0.0,
                average_memory_usage_mb: 0,
            });
        }

        let total_sessions = analytics.len() as u64;
        let total_streaming_hours: f64 = analytics
            .iter()
            .map(|a| a.get_total_streaming_duration())
            .sum::<f64>() / 3600.0;

        let average_quality = {
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
                score if score >= 4.5 => ConnectionQuality::Excellent,
                score if score >= 3.5 => ConnectionQuality::Good,
                score if score >= 2.5 => ConnectionQuality::Fair,
                score if score >= 1.5 => ConnectionQuality::Poor,
                _ => ConnectionQuality::Unusable,
            }
        };

        let average_latency_ms: f64 = analytics
            .iter()
            .flat_map(|a| &a.connection_metrics)
            .map(|c| c.latency_ms as f64)
            .sum::<f64>() / analytics.iter().map(|a| a.connection_metrics.len()).sum::<usize>().max(1) as f64;

        let average_bandwidth_mbps: f64 = analytics
            .iter()
            .flat_map(|a| &a.connection_metrics)
            .map(|c| c.bandwidth_mbps)
            .sum::<f64>() / analytics.iter().map(|a| a.connection_metrics.len()).sum::<usize>().max(1) as f64;

        let total_data_transferred_gb: f64 = analytics
            .iter()
            .map(|a| {
                (a.network_metrics.total_data_sent_bytes + a.network_metrics.total_data_received_bytes) as f64
            })
            .sum::<f64>() / (1024.0 * 1024.0 * 1024.0); // Convertir bytes a GB

        let success_rate_percent: f64 = analytics
            .iter()
            .map(|a| a.get_success_rate())
            .sum::<f64>() / analytics.len() as f64;

        let peak_concurrent_users = analytics.len() as u32;

        let average_cpu_usage_percent: f64 = analytics
            .iter()
            .map(|a| a.system_metrics.cpu_usage_percent)
            .sum::<f64>() / analytics.len() as f64;

        let average_memory_usage_mb: u64 = analytics
            .iter()
            .map(|a| a.system_metrics.memory_usage_mb)
            .sum::<u64>() / analytics.len() as u64;

        Ok(AggregatedStats {
            total_sessions,
            total_streaming_hours,
            average_connection_quality: average_quality,
            average_latency_ms,
            average_bandwidth_mbps,
            total_data_transferred_gb,
            success_rate_percent,
            peak_concurrent_users,
            average_cpu_usage_percent,
            average_memory_usage_mb,
        })
    }

    async fn cleanup_old_analytics(&self, days_to_keep: u32) -> Result<u64, AnalyticsError> {
        let cutoff_time = Utc::now() - chrono::Duration::days(days_to_keep as i64);
        let mut storage = self.analytics.write().await;
        
        let initial_count = storage.len();
        storage.retain(|_, analytics| analytics.created_at >= cutoff_time);
        let final_count = storage.len();
        
        Ok((initial_count - final_count) as u64)
    }
} 