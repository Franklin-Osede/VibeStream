use std::sync::Arc;
use chrono::{DateTime, Utc};
use crate::bounded_contexts::p2p::domain::entities::analytics::{
    P2PAnalyticsAggregate, P2PConnectionMetrics, StreamingMetrics,
    NetworkMetrics, SystemPerformanceMetrics, ConnectionQuality
};
use crate::bounded_contexts::p2p::domain::repositories::analytics_repository::{
    P2PAnalyticsRepository, AggregatedStats, AnalyticsError
};

/// Servicio de aplicación para analíticas P2P
pub struct P2PAnalyticsService<R: P2PAnalyticsRepository> {
    repository: Arc<R>,
}

impl<R: P2PAnalyticsRepository> P2PAnalyticsService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// Registrar métricas de conexión P2P
    pub async fn record_connection_metrics(
        &self,
        session_id: &str,
        user_id: &str,
        metrics: P2PConnectionMetrics,
    ) -> Result<(), AnalyticsError> {
        println!("📊 Registrando métricas de conexión P2P: {} -> {}", session_id, metrics.peer_id);
        
        // Obtener o crear agregado de analíticas
        let mut analytics = match self.repository.find_by_session(session_id).await? {
            Some(agg) => agg,
            None => P2PAnalyticsAggregate::new(session_id.to_string(), user_id.to_string()),
        };

        // Agregar métricas de conexión
        analytics.add_connection_metric(metrics);

        // Guardar en repositorio
        self.repository.save_analytics(&analytics).await?;
        
        println!("✅ Métricas de conexión registradas exitosamente");
        Ok(())
    }

    /// Registrar métricas de streaming
    pub async fn record_streaming_metrics(
        &self,
        session_id: &str,
        user_id: &str,
        metrics: StreamingMetrics,
    ) -> Result<(), AnalyticsError> {
        println!("🎬 Registrando métricas de streaming: {} - {}", session_id, metrics.content_id);
        
        // Obtener o crear agregado de analíticas
        let mut analytics = match self.repository.find_by_session(session_id).await? {
            Some(agg) => agg,
            None => P2PAnalyticsAggregate::new(session_id.to_string(), user_id.to_string()),
        };

        // Agregar métricas de streaming
        analytics.add_streaming_metric(metrics);

        // Guardar en repositorio
        self.repository.save_analytics(&analytics).await?;
        
        println!("✅ Métricas de streaming registradas exitosamente");
        Ok(())
    }

    /// Actualizar métricas de red
    pub async fn update_network_metrics(
        &self,
        session_id: &str,
        user_id: &str,
        metrics: NetworkMetrics,
    ) -> Result<(), AnalyticsError> {
        println!("🌐 Actualizando métricas de red P2P: {} - {}", session_id, metrics.peer_id);
        
        // Obtener o crear agregado de analíticas
        let mut analytics = match self.repository.find_by_session(session_id).await? {
            Some(agg) => agg,
            None => P2PAnalyticsAggregate::new(session_id.to_string(), user_id.to_string()),
        };

        // Actualizar métricas de red
        analytics.update_network_metrics(metrics);

        // Guardar en repositorio
        self.repository.save_analytics(&analytics).await?;
        
        println!("✅ Métricas de red actualizadas exitosamente");
        Ok(())
    }

    /// Actualizar métricas del sistema
    pub async fn update_system_metrics(
        &self,
        session_id: &str,
        user_id: &str,
        metrics: SystemPerformanceMetrics,
    ) -> Result<(), AnalyticsError> {
        println!("⚙️ Actualizando métricas del sistema: {} - CPU: {:.1}%", session_id, metrics.cpu_usage_percent);
        
        // Obtener o crear agregado de analíticas
        let mut analytics = match self.repository.find_by_session(session_id).await? {
            Some(agg) => agg,
            None => P2PAnalyticsAggregate::new(session_id.to_string(), user_id.to_string()),
        };

        // Actualizar métricas del sistema
        analytics.update_system_metrics(metrics);

        // Guardar en repositorio
        self.repository.save_analytics(&analytics).await?;
        
        println!("✅ Métricas del sistema actualizadas exitosamente");
        Ok(())
    }

    /// Obtener analíticas de sesión
    pub async fn get_session_analytics(&self, session_id: &str) -> Result<Option<P2PAnalyticsAggregate>, AnalyticsError> {
        println!("📈 Obteniendo analíticas de sesión: {}", session_id);
        self.repository.find_by_session(session_id).await
    }

    /// Obtener analíticas de usuario
    pub async fn get_user_analytics(&self, user_id: &str) -> Result<Vec<P2PAnalyticsAggregate>, AnalyticsError> {
        println!("👤 Obteniendo analíticas de usuario: {}", user_id);
        self.repository.find_by_user(user_id).await
    }

    /// Obtener estadísticas agregadas
    pub async fn get_aggregated_stats(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<AggregatedStats, AnalyticsError> {
        println!("📊 Obteniendo estadísticas agregadas: {} - {}", start_time, end_time);
        self.repository.get_aggregated_stats(start_time, end_time).await
    }

    /// Obtener métricas de rendimiento del sistema
    pub async fn get_system_performance(&self, hours: u32) -> Result<Vec<SystemPerformanceMetrics>, AnalyticsError> {
        println!("⚙️ Obteniendo métricas de rendimiento del sistema (últimas {} horas)", hours);
        self.repository.get_system_performance_metrics(hours).await
    }

    /// Obtener conexiones por calidad
    pub async fn get_connections_by_quality(
        &self,
        quality: ConnectionQuality,
        limit: Option<u32>,
    ) -> Result<Vec<P2PConnectionMetrics>, AnalyticsError> {
        println!("🔗 Obteniendo conexiones con calidad: {:?}", quality);
        self.repository.find_connections_by_quality(quality, limit).await
    }

    /// Generar reporte de rendimiento
    pub async fn generate_performance_report(
        &self,
        user_id: &str,
        days: u32,
    ) -> Result<PerformanceReport, AnalyticsError> {
        println!("📋 Generando reporte de rendimiento para usuario: {} (últimos {} días)", user_id, days);
        
        let end_time = Utc::now();
        let start_time = end_time - chrono::Duration::days(days as i64);
        
        let user_analytics = self.get_user_analytics(user_id).await?;
        let filtered_analytics: Vec<_> = user_analytics
            .into_iter()
            .filter(|a| a.created_at >= start_time && a.created_at <= end_time)
            .collect();

        let total_sessions = filtered_analytics.len() as u64;
        let total_streaming_hours: f64 = filtered_analytics
            .iter()
            .map(|a| a.get_total_streaming_duration())
            .sum::<f64>() / 3600.0; // Convertir segundos a horas

        let average_quality = if !filtered_analytics.is_empty() {
            let qualities: Vec<ConnectionQuality> = filtered_analytics
                .iter()
                .map(|a| a.get_average_connection_quality())
                .collect();
            
            // Calcular calidad promedio
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
        } else {
            ConnectionQuality::Good
        };

        let success_rate = if !filtered_analytics.is_empty() {
            let total_success_rate: f64 = filtered_analytics
                .iter()
                .map(|a| a.get_success_rate())
                .sum();
            total_success_rate / filtered_analytics.len() as f64
        } else {
            100.0
        };

        Ok(PerformanceReport {
            user_id: user_id.to_string(),
            period_days: days,
            total_sessions,
            total_streaming_hours,
            average_connection_quality: average_quality,
            success_rate_percent: success_rate,
            generated_at: Utc::now(),
        })
    }

    /// Limpiar analíticas antiguas
    pub async fn cleanup_old_analytics(&self, days_to_keep: u32) -> Result<u64, AnalyticsError> {
        println!("🧹 Limpiando analíticas antiguas (manteniendo {} días)", days_to_keep);
        let deleted_count = self.repository.cleanup_old_analytics(days_to_keep).await?;
        println!("✅ Eliminadas {} entradas de analíticas antiguas", deleted_count);
        Ok(deleted_count)
    }
}

/// Reporte de rendimiento de usuario
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PerformanceReport {
    pub user_id: String,
    pub period_days: u32,
    pub total_sessions: u64,
    pub total_streaming_hours: f64,
    pub average_connection_quality: ConnectionQuality,
    pub success_rate_percent: f64,
    pub generated_at: DateTime<Utc>,
} 