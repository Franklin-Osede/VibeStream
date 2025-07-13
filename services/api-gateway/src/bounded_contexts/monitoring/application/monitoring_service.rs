use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::bounded_contexts::monitoring::domain::entities::*;
use crate::bounded_contexts::monitoring::domain::repositories::MonitoringRepository;
use crate::bounded_contexts::monitoring::infrastructure::collectors::*;

/// Servicio principal de monitoreo del sistema
pub struct MonitoringService {
    repository: Arc<dyn MonitoringRepository>,
    collectors: Arc<MonitoringCollectors>,
    config: MonitoringConfig,
    metrics_cache: Arc<RwLock<HashMap<String, serde_json::Value>>>,
}

/// Coleccionadores de métricas para diferentes componentes
pub struct MonitoringCollectors {
    pub p2p_collector: Arc<P2PMetricsCollector>,
    pub video_collector: Arc<VideoStreamingMetricsCollector>,
    pub storage_collector: Arc<IPFSStorageMetricsCollector>,
    pub payment_collector: Arc<PaymentMetricsCollector>,
    pub user_collector: Arc<UserActivityMetricsCollector>,
    pub system_collector: Arc<SystemMetricsCollector>,
}

impl MonitoringService {
    pub fn new(
        repository: Arc<dyn MonitoringRepository>,
        collectors: MonitoringCollectors,
        config: MonitoringConfig,
    ) -> Self {
        Self {
            repository,
            collectors: Arc::new(collectors),
            config,
            metrics_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Recolectar todas las métricas del sistema
    pub async fn collect_all_metrics(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 Recolectando métricas del sistema...");

        // Recolectar métricas P2P
        if self.config.enabled_metrics.contains(&MetricType::P2PMetrics) {
            let p2p_metrics = self.collectors.p2p_collector.collect_metrics().await?;
            self.repository.save_p2p_metrics(&p2p_metrics).await?;
            self.cache_metrics("p2p", &p2p_metrics).await;
        }

        // Recolectar métricas de video streaming
        if self.config.enabled_metrics.contains(&MetricType::VideoStreamingMetrics) {
            let video_metrics = self.collectors.video_collector.collect_metrics().await?;
            self.repository.save_video_metrics(&video_metrics).await?;
            self.cache_metrics("video", &video_metrics).await;
        }

        // Recolectar métricas de almacenamiento IPFS
        if self.config.enabled_metrics.contains(&MetricType::IPFSStorageMetrics) {
            let storage_metrics = self.collectors.storage_collector.collect_metrics().await?;
            self.repository.save_storage_metrics(&storage_metrics).await?;
            self.cache_metrics("storage", &storage_metrics).await;
        }

        // Recolectar métricas de pagos
        if self.config.enabled_metrics.contains(&MetricType::PaymentMetrics) {
            let payment_metrics = self.collectors.payment_collector.collect_metrics().await?;
            self.repository.save_payment_metrics(&payment_metrics).await?;
            self.cache_metrics("payment", &payment_metrics).await;
        }

        // Recolectar métricas de usuarios
        if self.config.enabled_metrics.contains(&MetricType::UserActivityMetrics) {
            let user_metrics = self.collectors.user_collector.collect_metrics().await?;
            self.repository.save_user_metrics(&user_metrics).await?;
            self.cache_metrics("user", &user_metrics).await;
        }

        // Recolectar métricas del sistema
        if self.config.enabled_metrics.contains(&MetricType::SystemMetrics) {
            let system_metrics = self.collectors.system_collector.collect_metrics().await?;
            self.repository.save_system_metrics(&system_metrics).await?;
            self.cache_metrics("system", &system_metrics).await;
        }

        println!("✅ Métricas recolectadas exitosamente");
        Ok(())
    }

    /// Verificar alertas basadas en las métricas actuales
    pub async fn check_alerts(&self) -> Result<Vec<SystemAlert>, Box<dyn std::error::Error>> {
        println!("🚨 Verificando alertas del sistema...");

        let mut alerts = Vec::new();
        let thresholds = &self.config.alert_thresholds;

        // Verificar métricas del sistema
        if let Some(system_metrics) = self.get_latest_system_metrics().await? {
            // Alerta por uso alto de CPU
            if system_metrics.cpu_usage_percentage > thresholds.cpu_usage_threshold {
                alerts.push(SystemAlert {
                    id: Uuid::new_v4(),
                    timestamp: Utc::now(),
                    alert_type: AlertType::HighCpuUsage,
                    severity: AlertSeverity::High,
                    message: format!("CPU usage is {}% (threshold: {}%)", 
                        system_metrics.cpu_usage_percentage, thresholds.cpu_usage_threshold),
                    details: serde_json::json!({
                        "current_usage": system_metrics.cpu_usage_percentage,
                        "threshold": thresholds.cpu_usage_threshold
                    }),
                    resolved: false,
                    resolved_at: None,
                });
            }

            // Alerta por uso alto de memoria
            if system_metrics.memory_usage_percentage > thresholds.memory_usage_threshold {
                alerts.push(SystemAlert {
                    id: Uuid::new_v4(),
                    timestamp: Utc::now(),
                    alert_type: AlertType::HighMemoryUsage,
                    severity: AlertSeverity::High,
                    message: format!("Memory usage is {}% (threshold: {}%)", 
                        system_metrics.memory_usage_percentage, thresholds.memory_usage_threshold),
                    details: serde_json::json!({
                        "current_usage": system_metrics.memory_usage_percentage,
                        "threshold": thresholds.memory_usage_threshold
                    }),
                    resolved: false,
                    resolved_at: None,
                });
            }

            // Alerta por uso alto de disco
            if system_metrics.disk_usage_percentage > thresholds.disk_usage_threshold {
                alerts.push(SystemAlert {
                    id: Uuid::new_v4(),
                    timestamp: Utc::now(),
                    alert_type: AlertType::LowDiskSpace,
                    severity: AlertSeverity::Critical,
                    message: format!("Disk usage is {}% (threshold: {}%)", 
                        system_metrics.disk_usage_percentage, thresholds.disk_usage_threshold),
                    details: serde_json::json!({
                        "current_usage": system_metrics.disk_usage_percentage,
                        "threshold": thresholds.disk_usage_threshold
                    }),
                    resolved: false,
                    resolved_at: None,
                });
            }

            // Alerta por alta tasa de errores
            if system_metrics.error_rate_percentage > thresholds.error_rate_threshold {
                alerts.push(SystemAlert {
                    id: Uuid::new_v4(),
                    timestamp: Utc::now(),
                    alert_type: AlertType::Custom("HighErrorRate".to_string()),
                    severity: AlertSeverity::High,
                    message: format!("Error rate is {}% (threshold: {}%)", 
                        system_metrics.error_rate_percentage, thresholds.error_rate_threshold),
                    details: serde_json::json!({
                        "current_rate": system_metrics.error_rate_percentage,
                        "threshold": thresholds.error_rate_threshold
                    }),
                    resolved: false,
                    resolved_at: None,
                });
            }
        }

        // Verificar métricas P2P
        if let Some(p2p_metrics) = self.get_latest_p2p_metrics().await? {
            if p2p_metrics.active_peers < thresholds.p2p_connection_threshold {
                alerts.push(SystemAlert {
                    id: Uuid::new_v4(),
                    timestamp: Utc::now(),
                    alert_type: AlertType::P2PConnectionFailure,
                    severity: AlertSeverity::Medium,
                    message: format!("Active P2P peers: {} (threshold: {})", 
                        p2p_metrics.active_peers, thresholds.p2p_connection_threshold),
                    details: serde_json::json!({
                        "active_peers": p2p_metrics.active_peers,
                        "threshold": thresholds.p2p_connection_threshold
                    }),
                    resolved: false,
                    resolved_at: None,
                });
            }
        }

        // Verificar métricas de pagos
        if let Some(payment_metrics) = self.get_latest_payment_metrics().await? {
            if payment_metrics.payment_success_rate < thresholds.payment_failure_threshold {
                alerts.push(SystemAlert {
                    id: Uuid::new_v4(),
                    timestamp: Utc::now(),
                    alert_type: AlertType::PaymentFailure,
                    severity: AlertSeverity::Critical,
                    message: format!("Payment success rate is {}% (threshold: {}%)", 
                        payment_metrics.payment_success_rate * 100.0, 
                        thresholds.payment_failure_threshold * 100.0),
                    details: serde_json::json!({
                        "success_rate": payment_metrics.payment_success_rate,
                        "threshold": thresholds.payment_failure_threshold
                    }),
                    resolved: false,
                    resolved_at: None,
                });
            }
        }

        // Guardar alertas en el repositorio
        for alert in &alerts {
            self.repository.save_alert(alert).await?;
        }

        println!("✅ Verificación de alertas completada. {} alertas generadas", alerts.len());
        Ok(alerts)
    }

    /// Generar reporte de rendimiento
    pub async fn generate_performance_report(
        &self,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<PerformanceReport, Box<dyn std::error::Error>> {
        println!("📋 Generando reporte de rendimiento...");

        // Obtener métricas del período
        let p2p_metrics = self.repository.get_p2p_metrics_in_period(period_start, period_end).await?;
        let video_metrics = self.repository.get_video_metrics_in_period(period_start, period_end).await?;
        let storage_metrics = self.repository.get_storage_metrics_in_period(period_start, period_end).await?;
        let payment_metrics = self.repository.get_payment_metrics_in_period(period_start, period_end).await?;
        let user_metrics = self.repository.get_user_metrics_in_period(period_start, period_end).await?;
        let system_metrics = self.repository.get_system_metrics_in_period(period_start, period_end).await?;

        // Calcular resumen
        let summary = self.calculate_performance_summary(
            &p2p_metrics,
            &video_metrics,
            &storage_metrics,
            &payment_metrics,
            &user_metrics,
            &system_metrics,
        ).await?;

        // Generar recomendaciones
        let recommendations = self.generate_recommendations(
            &p2p_metrics,
            &video_metrics,
            &storage_metrics,
            &payment_metrics,
            &user_metrics,
            &system_metrics,
        ).await?;

        let report = PerformanceReport {
            id: Uuid::new_v4(),
            generated_at: Utc::now(),
            period_start,
            period_end,
            summary,
            detailed_metrics: DetailedMetrics {
                p2p_metrics: p2p_metrics.last().cloned(),
                video_metrics: video_metrics.last().cloned(),
                storage_metrics: storage_metrics.last().cloned(),
                payment_metrics: payment_metrics.last().cloned(),
                user_metrics: user_metrics.last().cloned(),
                system_metrics: system_metrics.last().cloned(),
            },
            recommendations,
        };

        // Guardar reporte
        self.repository.save_performance_report(&report).await?;

        println!("✅ Reporte de rendimiento generado exitosamente");
        Ok(report)
    }

    /// Obtener dashboard de monitoreo
    pub async fn get_dashboard(&self, dashboard_id: Uuid) -> Result<MonitoringDashboard, Box<dyn std::error::Error>> {
        self.repository.get_dashboard(dashboard_id).await
    }

    /// Crear o actualizar dashboard
    pub async fn save_dashboard(&self, dashboard: &MonitoringDashboard) -> Result<(), Box<dyn std::error::Error>> {
        self.repository.save_dashboard(dashboard).await
    }

    /// Obtener métricas en tiempo real
    pub async fn get_realtime_metrics(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let cache = self.metrics_cache.read().await;
        Ok(serde_json::json!({
            "timestamp": Utc::now(),
            "metrics": cache.clone()
        }))
    }

    // Métodos auxiliares privados

    async fn cache_metrics(&self, key: &str, metrics: &impl serde::Serialize) {
        let mut cache = self.metrics_cache.write().await;
        cache.insert(key.to_string(), serde_json::to_value(metrics).unwrap_or_default());
    }

    async fn get_latest_system_metrics(&self) -> Result<Option<SystemMetrics>, Box<dyn std::error::Error>> {
        self.repository.get_latest_system_metrics().await
    }

    async fn get_latest_p2p_metrics(&self) -> Result<Option<P2PMetrics>, Box<dyn std::error::Error>> {
        self.repository.get_latest_p2p_metrics().await
    }

    async fn get_latest_payment_metrics(&self) -> Result<Option<PaymentMetrics>, Box<dyn std::error::Error>> {
        self.repository.get_latest_payment_metrics().await
    }

    async fn calculate_performance_summary(
        &self,
        p2p_metrics: &[P2PMetrics],
        video_metrics: &[VideoStreamingMetrics],
        storage_metrics: &[IPFSStorageMetrics],
        payment_metrics: &[PaymentMetrics],
        user_metrics: &[UserActivityMetrics],
        system_metrics: &[SystemMetrics],
    ) -> Result<PerformanceSummary, Box<dyn std::error::Error>> {
        // Calcular puntuación de salud general
        let mut health_score = 100.0;

        // Evaluar métricas del sistema
        if let Some(latest_system) = system_metrics.last() {
            if latest_system.cpu_usage_percentage > 80.0 { health_score -= 20.0; }
            if latest_system.memory_usage_percentage > 80.0 { health_score -= 20.0; }
            if latest_system.error_rate_percentage > 5.0 { health_score -= 30.0; }
        }

        // Evaluar métricas P2P
        if let Some(latest_p2p) = p2p_metrics.last() {
            if latest_p2p.network_health_score < 0.7 { health_score -= 15.0; }
        }

        // Evaluar métricas de pagos
        if let Some(latest_payment) = payment_metrics.last() {
            if latest_payment.payment_success_rate < 0.95 { health_score -= 25.0; }
        }

        health_score = health_score.max(0.0);

        // Determinar estado del sistema
        let system_status = if health_score >= 80.0 { "Healthy" }
        else if health_score >= 60.0 { "Warning" }
        else if health_score >= 40.0 { "Critical" }
        else { "Emergency" };

        // Contar alertas críticas
        let critical_alerts = self.repository.get_critical_alerts_count().await?;

        // Determinar tendencia de rendimiento
        let performance_trend = self.calculate_performance_trend(system_metrics).await?;

        // Identificar problemas principales
        let top_issues = self.identify_top_issues(
            p2p_metrics,
            video_metrics,
            storage_metrics,
            payment_metrics,
            user_metrics,
            system_metrics,
        ).await?;

        Ok(PerformanceSummary {
            overall_health_score: health_score,
            system_status: system_status.to_string(),
            critical_alerts,
            performance_trend,
            top_issues,
        })
    }

    async fn calculate_performance_trend(&self, system_metrics: &[SystemMetrics]) -> Result<String, Box<dyn std::error::Error>> {
        if system_metrics.len() < 2 {
            return Ok("Insufficient data".to_string());
        }

        let recent = &system_metrics[system_metrics.len() - 1];
        let previous = &system_metrics[system_metrics.len() - 2];

        let cpu_change = recent.cpu_usage_percentage - previous.cpu_usage_percentage;
        let memory_change = recent.memory_usage_percentage - previous.memory_usage_percentage;
        let error_change = recent.error_rate_percentage - previous.error_rate_percentage;

        if cpu_change > 10.0 || memory_change > 10.0 || error_change > 2.0 {
            Ok("Declining".to_string())
        } else if cpu_change < -5.0 && memory_change < -5.0 && error_change < -1.0 {
            Ok("Improving".to_string())
        } else {
            Ok("Stable".to_string())
        }
    }

    async fn identify_top_issues(
        &self,
        _p2p_metrics: &[P2PMetrics],
        _video_metrics: &[VideoStreamingMetrics],
        _storage_metrics: &[IPFSStorageMetrics],
        _payment_metrics: &[PaymentMetrics],
        _user_metrics: &[UserActivityMetrics],
        system_metrics: &[SystemMetrics],
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut issues = Vec::new();

        if let Some(latest) = system_metrics.last() {
            if latest.cpu_usage_percentage > 80.0 {
                issues.push("High CPU usage detected".to_string());
            }
            if latest.memory_usage_percentage > 80.0 {
                issues.push("High memory usage detected".to_string());
            }
            if latest.error_rate_percentage > 5.0 {
                issues.push("High error rate detected".to_string());
            }
            if latest.response_time_ms > 1000.0 {
                issues.push("Slow response times detected".to_string());
            }
        }

        if issues.is_empty() {
            issues.push("No critical issues detected".to_string());
        }

        Ok(issues)
    }

    async fn generate_recommendations(
        &self,
        _p2p_metrics: &[P2PMetrics],
        _video_metrics: &[VideoStreamingMetrics],
        _storage_metrics: &[IPFSStorageMetrics],
        _payment_metrics: &[PaymentMetrics],
        _user_metrics: &[UserActivityMetrics],
        system_metrics: &[SystemMetrics],
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut recommendations = Vec::new();

        if let Some(latest) = system_metrics.last() {
            if latest.cpu_usage_percentage > 70.0 {
                recommendations.push("Consider scaling up CPU resources or optimizing code".to_string());
            }
            if latest.memory_usage_percentage > 70.0 {
                recommendations.push("Consider increasing memory allocation or implementing caching".to_string());
            }
            if latest.error_rate_percentage > 3.0 {
                recommendations.push("Investigate and fix error sources to improve reliability".to_string());
            }
            if latest.response_time_ms > 500.0 {
                recommendations.push("Optimize database queries and implement caching to reduce response times".to_string());
            }
        }

        if recommendations.is_empty() {
            recommendations.push("System is performing well. Continue monitoring for any changes".to_string());
        }

        Ok(recommendations)
    }
} 