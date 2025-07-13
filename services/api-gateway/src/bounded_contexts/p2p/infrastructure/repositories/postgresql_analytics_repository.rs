use async_trait::async_trait;
use sqlx::{PgPool, Row};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::bounded_contexts::p2p::domain::entities::analytics::{
    P2PAnalyticsAggregate, P2PConnectionMetrics, StreamingMetrics,
    NetworkMetrics, SystemPerformanceMetrics, ConnectionQuality
};
use crate::bounded_contexts::p2p::domain::repositories::analytics_repository::{
    P2PAnalyticsRepository, AggregatedStats, AnalyticsError
};

/// Repositorio PostgreSQL para analíticas P2P
pub struct PostgreSQLP2PAnalyticsRepository {
    pool: PgPool,
}

impl PostgreSQLP2PAnalyticsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Crear tablas si no existen
    pub async fn create_tables(&self) -> Result<(), AnalyticsError> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS p2p_analytics_aggregates (
                id VARCHAR(255) PRIMARY KEY,
                session_id VARCHAR(255) NOT NULL,
                user_id VARCHAR(255) NOT NULL,
                network_metrics JSONB NOT NULL,
                system_metrics JSONB NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITH TIME ZONE NOT NULL
            )
            "#
        ).execute(&self.pool).await.map_err(|e| AnalyticsError::Database(e.to_string()))?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS p2p_connection_metrics (
                id VARCHAR(255) PRIMARY KEY,
                analytics_id VARCHAR(255) NOT NULL,
                connection_id VARCHAR(255) NOT NULL,
                session_id VARCHAR(255) NOT NULL,
                peer_id VARCHAR(255) NOT NULL,
                connection_type VARCHAR(50) NOT NULL,
                latency_ms INTEGER NOT NULL,
                bandwidth_mbps DOUBLE PRECISION NOT NULL,
                packet_loss_percent DOUBLE PRECISION NOT NULL,
                jitter_ms INTEGER NOT NULL,
                connection_quality VARCHAR(50) NOT NULL,
                ice_connection_state VARCHAR(255) NOT NULL,
                dtls_transport_state VARCHAR(255) NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
                FOREIGN KEY (analytics_id) REFERENCES p2p_analytics_aggregates(id) ON DELETE CASCADE
            )
            "#
        ).execute(&self.pool).await.map_err(|e| AnalyticsError::Database(e.to_string()))?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS p2p_streaming_metrics (
                id VARCHAR(255) PRIMARY KEY,
                analytics_id VARCHAR(255) NOT NULL,
                stream_id VARCHAR(255) NOT NULL,
                content_id VARCHAR(255) NOT NULL,
                user_id VARCHAR(255) NOT NULL,
                quality_level VARCHAR(50) NOT NULL,
                bitrate_kbps INTEGER NOT NULL,
                frame_rate REAL NOT NULL,
                resolution_width INTEGER NOT NULL,
                resolution_height INTEGER NOT NULL,
                buffer_level_seconds REAL NOT NULL,
                dropped_frames INTEGER NOT NULL,
                total_frames INTEGER NOT NULL,
                adaptive_switches INTEGER NOT NULL,
                start_time TIMESTAMP WITH TIME ZONE NOT NULL,
                end_time TIMESTAMP WITH TIME ZONE,
                duration_seconds DOUBLE PRECISION NOT NULL,
                FOREIGN KEY (analytics_id) REFERENCES p2p_analytics_aggregates(id) ON DELETE CASCADE
            )
            "#
        ).execute(&self.pool).await.map_err(|e| AnalyticsError::Database(e.to_string()))?;

        // Crear índices para optimizar consultas
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_p2p_analytics_session_id ON p2p_analytics_aggregates(session_id)")
            .execute(&self.pool).await.map_err(|e| AnalyticsError::Database(e.to_string()))?;
        
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_p2p_analytics_user_id ON p2p_analytics_aggregates(user_id)")
            .execute(&self.pool).await.map_err(|e| AnalyticsError::Database(e.to_string()))?;
        
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_p2p_analytics_created_at ON p2p_analytics_aggregates(created_at)")
            .execute(&self.pool).await.map_err(|e| AnalyticsError::Database(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl P2PAnalyticsRepository for PostgreSQLP2PAnalyticsRepository {
    async fn save_analytics(&self, analytics: &P2PAnalyticsAggregate) -> Result<(), AnalyticsError> {
        let mut transaction = self.pool.begin().await.map_err(|e| AnalyticsError::Database(e.to_string()))?;

        // Guardar agregado principal
        sqlx::query(
            r#"
            INSERT INTO p2p_analytics_aggregates (id, session_id, user_id, network_metrics, system_metrics, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                network_metrics = EXCLUDED.network_metrics,
                system_metrics = EXCLUDED.system_metrics,
                updated_at = EXCLUDED.updated_at
            "#
        )
        .bind(&analytics.id)
        .bind(&analytics.session_id)
        .bind(&analytics.user_id)
        .bind(serde_json::to_value(&analytics.network_metrics).map_err(|e| AnalyticsError::Serialization(e.to_string()))?)
        .bind(serde_json::to_value(&analytics.system_metrics).map_err(|e| AnalyticsError::Serialization(e.to_string()))?)
        .bind(analytics.created_at)
        .bind(analytics.updated_at)
        .execute(&mut *transaction)
        .await
        .map_err(|e| AnalyticsError::Database(e.to_string()))?;

        // Guardar métricas de conexión
        for metric in &analytics.connection_metrics {
            sqlx::query(
                r#"
                INSERT INTO p2p_connection_metrics (
                    id, analytics_id, connection_id, session_id, peer_id, connection_type,
                    latency_ms, bandwidth_mbps, packet_loss_percent, jitter_ms,
                    connection_quality, ice_connection_state, dtls_transport_state,
                    created_at, updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
                ON CONFLICT (id) DO UPDATE SET
                    latency_ms = EXCLUDED.latency_ms,
                    bandwidth_mbps = EXCLUDED.bandwidth_mbps,
                    packet_loss_percent = EXCLUDED.packet_loss_percent,
                    jitter_ms = EXCLUDED.jitter_ms,
                    connection_quality = EXCLUDED.connection_quality,
                    ice_connection_state = EXCLUDED.ice_connection_state,
                    dtls_transport_state = EXCLUDED.dtls_transport_state,
                    updated_at = EXCLUDED.updated_at
                "#
            )
            .bind(Uuid::new_v4().to_string())
            .bind(&analytics.id)
            .bind(&metric.connection_id)
            .bind(&metric.session_id)
            .bind(&metric.peer_id)
            .bind(format!("{:?}", metric.connection_type))
            .bind(metric.latency_ms as i32)
            .bind(metric.bandwidth_mbps)
            .bind(metric.packet_loss_percent)
            .bind(metric.jitter_ms as i32)
            .bind(format!("{:?}", metric.connection_quality))
            .bind(&metric.ice_connection_state)
            .bind(&metric.dtls_transport_state)
            .bind(metric.created_at)
            .bind(metric.updated_at)
            .execute(&mut *transaction)
            .await
            .map_err(|e| AnalyticsError::Database(e.to_string()))?;
        }

        // Guardar métricas de streaming
        for metric in &analytics.streaming_metrics {
            sqlx::query(
                r#"
                INSERT INTO p2p_streaming_metrics (
                    id, analytics_id, stream_id, content_id, user_id, quality_level,
                    bitrate_kbps, frame_rate, resolution_width, resolution_height,
                    buffer_level_seconds, dropped_frames, total_frames, adaptive_switches,
                    start_time, end_time, duration_seconds
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
                ON CONFLICT (id) DO UPDATE SET
                    bitrate_kbps = EXCLUDED.bitrate_kbps,
                    frame_rate = EXCLUDED.frame_rate,
                    buffer_level_seconds = EXCLUDED.buffer_level_seconds,
                    dropped_frames = EXCLUDED.dropped_frames,
                    total_frames = EXCLUDED.total_frames,
                    adaptive_switches = EXCLUDED.adaptive_switches,
                    end_time = EXCLUDED.end_time,
                    duration_seconds = EXCLUDED.duration_seconds
                "#
            )
            .bind(Uuid::new_v4().to_string())
            .bind(&analytics.id)
            .bind(&metric.stream_id)
            .bind(&metric.content_id)
            .bind(&metric.user_id)
            .bind(format!("{:?}", metric.quality_level))
            .bind(metric.bitrate_kbps as i32)
            .bind(metric.frame_rate)
            .bind(metric.resolution_width as i32)
            .bind(metric.resolution_height as i32)
            .bind(metric.buffer_level_seconds)
            .bind(metric.dropped_frames as i32)
            .bind(metric.total_frames as i32)
            .bind(metric.adaptive_switches as i32)
            .bind(metric.start_time)
            .bind(metric.end_time)
            .bind(metric.duration_seconds)
            .execute(&mut *transaction)
            .await
            .map_err(|e| AnalyticsError::Database(e.to_string()))?;
        }

        transaction.commit().await.map_err(|e| AnalyticsError::Database(e.to_string()))?;
        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<P2PAnalyticsAggregate>, AnalyticsError> {
        let row = sqlx::query(
            "SELECT * FROM p2p_analytics_aggregates WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AnalyticsError::Database(e.to_string()))?;

        if let Some(row) = row {
            let analytics = self.build_analytics_from_row(row).await?;
            Ok(Some(analytics))
        } else {
            Ok(None)
        }
    }

    async fn find_by_session(&self, session_id: &str) -> Result<Option<P2PAnalyticsAggregate>, AnalyticsError> {
        let row = sqlx::query(
            "SELECT * FROM p2p_analytics_aggregates WHERE session_id = $1"
        )
        .bind(session_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AnalyticsError::Database(e.to_string()))?;

        if let Some(row) = row {
            let analytics = self.build_analytics_from_row(row).await?;
            Ok(Some(analytics))
        } else {
            Ok(None)
        }
    }

    async fn find_by_user(&self, user_id: &str) -> Result<Vec<P2PAnalyticsAggregate>, AnalyticsError> {
        let rows = sqlx::query(
            "SELECT * FROM p2p_analytics_aggregates WHERE user_id = $1 ORDER BY created_at DESC"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AnalyticsError::Database(e.to_string()))?;

        let mut analytics = Vec::new();
        for row in rows {
            let agg = self.build_analytics_from_row(row).await?;
            analytics.push(agg);
        }

        Ok(analytics)
    }

    async fn find_by_time_range(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<P2PAnalyticsAggregate>, AnalyticsError> {
        let rows = sqlx::query(
            "SELECT * FROM p2p_analytics_aggregates WHERE created_at BETWEEN $1 AND $2 ORDER BY created_at DESC"
        )
        .bind(start_time)
        .bind(end_time)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AnalyticsError::Database(e.to_string()))?;

        let mut analytics = Vec::new();
        for row in rows {
            let agg = self.build_analytics_from_row(row).await?;
            analytics.push(agg);
        }

        Ok(analytics)
    }

    async fn find_connections_by_quality(
        &self,
        quality: ConnectionQuality,
        limit: Option<u32>,
    ) -> Result<Vec<P2PConnectionMetrics>, AnalyticsError> {
        let mut query = sqlx::query(
            "SELECT * FROM p2p_connection_metrics WHERE connection_quality = $1 ORDER BY created_at DESC"
        )
        .bind(format!("{:?}", quality));

        if let Some(limit) = limit {
            query = query.bind(limit as i64);
        }

        let rows = query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AnalyticsError::Database(e.to_string()))?;

        let mut metrics = Vec::new();
        for row in rows {
            let metric = P2PConnectionMetrics {
                connection_id: row.try_get("connection_id").map_err(|e| AnalyticsError::Database(e.to_string()))?,
                session_id: row.try_get("session_id").map_err(|e| AnalyticsError::Database(e.to_string()))?,
                peer_id: row.try_get("peer_id").map_err(|e| AnalyticsError::Database(e.to_string()))?,
                connection_type: match row.try_get::<&str, _>("connection_type").map_err(|e| AnalyticsError::Database(e.to_string()))? {
                    "WebRTC" => crate::bounded_contexts::p2p::domain::entities::analytics::ConnectionType::WebRTC,
                    "WebSocket" => crate::bounded_contexts::p2p::domain::entities::analytics::ConnectionType::WebSocket,
                    "Direct" => crate::bounded_contexts::p2p::domain::entities::analytics::ConnectionType::Direct,
                    "Relay" => crate::bounded_contexts::p2p::domain::entities::analytics::ConnectionType::Relay,
                    _ => crate::bounded_contexts::p2p::domain::entities::analytics::ConnectionType::WebRTC,
                },
                latency_ms: row.try_get::<i32, _>("latency_ms").map_err(|e| AnalyticsError::Database(e.to_string()))? as u32,
                bandwidth_mbps: row.try_get("bandwidth_mbps").map_err(|e| AnalyticsError::Database(e.to_string()))?,
                packet_loss_percent: row.try_get("packet_loss_percent").map_err(|e| AnalyticsError::Database(e.to_string()))?,
                jitter_ms: row.try_get::<i32, _>("jitter_ms").map_err(|e| AnalyticsError::Database(e.to_string()))? as u32,
                connection_quality: match row.try_get::<&str, _>("connection_quality").map_err(|e| AnalyticsError::Database(e.to_string()))? {
                    "Excellent" => ConnectionQuality::Excellent,
                    "Good" => ConnectionQuality::Good,
                    "Fair" => ConnectionQuality::Fair,
                    "Poor" => ConnectionQuality::Poor,
                    "Unusable" => ConnectionQuality::Unusable,
                    _ => ConnectionQuality::Good,
                },
                ice_connection_state: row.try_get("ice_connection_state").map_err(|e| AnalyticsError::Database(e.to_string()))?,
                dtls_transport_state: row.try_get("dtls_transport_state").map_err(|e| AnalyticsError::Database(e.to_string()))?,
                created_at: row.try_get("created_at").map_err(|e| AnalyticsError::Database(e.to_string()))?,
                updated_at: row.try_get("updated_at").map_err(|e| AnalyticsError::Database(e.to_string()))?,
            };
            metrics.push(metric);
        }

        Ok(metrics)
    }

    async fn get_system_performance_metrics(
        &self,
        hours: u32,
    ) -> Result<Vec<SystemPerformanceMetrics>, AnalyticsError> {
        let cutoff_time = Utc::now() - chrono::Duration::hours(hours as i64);
        
        let rows = sqlx::query(
            "SELECT system_metrics FROM p2p_analytics_aggregates WHERE created_at >= $1 ORDER BY created_at DESC"
        )
        .bind(cutoff_time)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AnalyticsError::Database(e.to_string()))?;

        let mut metrics = Vec::new();
        for row in rows {
            let metrics_json: serde_json::Value = row.try_get("system_metrics").map_err(|e| AnalyticsError::Database(e.to_string()))?;
            let metric: SystemPerformanceMetrics = serde_json::from_value(metrics_json)
                .map_err(|e| AnalyticsError::Serialization(e.to_string()))?;
            metrics.push(metric);
        }

        Ok(metrics)
    }

    async fn get_aggregated_stats(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<AggregatedStats, AnalyticsError> {
        // Obtener estadísticas básicas
        let row = sqlx::query(
            r#"
            SELECT 
                COUNT(*) as total_sessions,
                AVG(CAST(system_metrics->>'cpu_usage_percent' AS DOUBLE PRECISION)) as avg_cpu,
                AVG(CAST(system_metrics->>'memory_usage_mb' AS DOUBLE PRECISION)) as avg_memory
            FROM p2p_analytics_aggregates 
            WHERE created_at BETWEEN $1 AND $2
            "#
        )
        .bind(start_time)
        .bind(end_time)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AnalyticsError::Database(e.to_string()))?;

        let total_sessions: i64 = row.try_get("total_sessions").map_err(|e| AnalyticsError::Database(e.to_string()))?;
        let avg_cpu: Option<f64> = row.try_get("avg_cpu").map_err(|e| AnalyticsError::Database(e.to_string()))?;
        let avg_memory: Option<f64> = row.try_get("avg_memory").map_err(|e| AnalyticsError::Database(e.to_string()))?;

        // Obtener estadísticas de conexiones
        let connection_stats = sqlx::query(
            r#"
            SELECT 
                AVG(latency_ms) as avg_latency,
                AVG(bandwidth_mbps) as avg_bandwidth,
                COUNT(*) as total_connections
            FROM p2p_connection_metrics cm
            JOIN p2p_analytics_aggregates aa ON cm.analytics_id = aa.id
            WHERE aa.created_at BETWEEN $1 AND $2
            "#
        )
        .bind(start_time)
        .bind(end_time)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AnalyticsError::Database(e.to_string()))?;

        let avg_latency: Option<f64> = connection_stats.try_get("avg_latency").map_err(|e| AnalyticsError::Database(e.to_string()))?;
        let avg_bandwidth: Option<f64> = connection_stats.try_get("avg_bandwidth").map_err(|e| AnalyticsError::Database(e.to_string()))?;

        // Calcular calidad promedio de conexión
        let quality_distribution = sqlx::query(
            r#"
            SELECT connection_quality, COUNT(*) as count
            FROM p2p_connection_metrics cm
            JOIN p2p_analytics_aggregates aa ON cm.analytics_id = aa.id
            WHERE aa.created_at BETWEEN $1 AND $2
            GROUP BY connection_quality
            "#
        )
        .bind(start_time)
        .bind(end_time)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AnalyticsError::Database(e.to_string()))?;

        let mut total_connections = 0;
        let mut quality_scores = Vec::new();
        
        for row in quality_distribution {
            let quality: String = row.try_get("connection_quality").map_err(|e| AnalyticsError::Database(e.to_string()))?;
            let count: i64 = row.try_get("count").map_err(|e| AnalyticsError::Database(e.to_string()))?;
            
            let score = match quality.as_str() {
                "Excellent" => 5,
                "Good" => 4,
                "Fair" => 3,
                "Poor" => 2,
                "Unusable" => 1,
                _ => 3,
            };
            
            for _ in 0..count {
                quality_scores.push(score);
            }
            total_connections += count;
        }

        let average_connection_quality = if !quality_scores.is_empty() {
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

        Ok(AggregatedStats {
            total_sessions: total_sessions as u64,
            total_streaming_hours: 0.0, // TODO: Calculate from streaming metrics
            average_connection_quality,
            average_latency_ms: avg_latency.unwrap_or(0.0),
            average_bandwidth_mbps: avg_bandwidth.unwrap_or(0.0),
            total_data_transferred_gb: 0.0, // TODO: Calculate from network metrics
            success_rate_percent: 95.0, // TODO: Calculate from actual data
            peak_concurrent_users: 0, // TODO: Calculate from actual data
            average_cpu_usage_percent: avg_cpu.unwrap_or(0.0),
            average_memory_usage_mb: avg_memory.unwrap_or(0.0) as u64,
        })
    }

    async fn cleanup_old_analytics(&self, days_to_keep: u32) -> Result<u64, AnalyticsError> {
        let cutoff_date = Utc::now() - chrono::Duration::days(days_to_keep as i64);
        
        let result = sqlx::query(
            "DELETE FROM p2p_analytics_aggregates WHERE created_at < $1"
        )
        .bind(cutoff_date)
        .execute(&self.pool)
        .await
        .map_err(|e| AnalyticsError::Database(e.to_string()))?;

        Ok(result.rows_affected())
    }
}

impl PostgreSQLP2PAnalyticsRepository {
    async fn build_analytics_from_row(&self, row: sqlx::postgres::PgRow) -> Result<P2PAnalyticsAggregate, AnalyticsError> {
        let id: String = row.try_get("id").map_err(|e| AnalyticsError::Database(e.to_string()))?;
        let session_id: String = row.try_get("session_id").map_err(|e| AnalyticsError::Database(e.to_string()))?;
        let user_id: String = row.try_get("user_id").map_err(|e| AnalyticsError::Database(e.to_string()))?;
        let created_at: DateTime<Utc> = row.try_get("created_at").map_err(|e| AnalyticsError::Database(e.to_string()))?;
        let updated_at: DateTime<Utc> = row.try_get("updated_at").map_err(|e| AnalyticsError::Database(e.to_string()))?;

        // Obtener métricas de conexión
        let connection_rows = sqlx::query(
            "SELECT * FROM p2p_connection_metrics WHERE analytics_id = $1"
        )
        .bind(&id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AnalyticsError::Database(e.to_string()))?;

        let mut connection_metrics = Vec::new();
        for conn_row in connection_rows {
            let metric = P2PConnectionMetrics {
                connection_id: conn_row.try_get("connection_id").map_err(|e| AnalyticsError::Database(e.to_string()))?,
                session_id: conn_row.try_get("session_id").map_err(|e| AnalyticsError::Database(e.to_string()))?,
                peer_id: conn_row.try_get("peer_id").map_err(|e| AnalyticsError::Database(e.to_string()))?,
                connection_type: match conn_row.try_get::<&str, _>("connection_type").map_err(|e| AnalyticsError::Database(e.to_string()))? {
                    "WebRTC" => crate::bounded_contexts::p2p::domain::entities::analytics::ConnectionType::WebRTC,
                    "WebSocket" => crate::bounded_contexts::p2p::domain::entities::analytics::ConnectionType::WebSocket,
                    "Direct" => crate::bounded_contexts::p2p::domain::entities::analytics::ConnectionType::Direct,
                    "Relay" => crate::bounded_contexts::p2p::domain::entities::analytics::ConnectionType::Relay,
                    _ => crate::bounded_contexts::p2p::domain::entities::analytics::ConnectionType::WebRTC,
                },
                latency_ms: conn_row.try_get::<i32, _>("latency_ms").map_err(|e| AnalyticsError::Database(e.to_string()))? as u32,
                bandwidth_mbps: conn_row.try_get("bandwidth_mbps").map_err(|e| AnalyticsError::Database(e.to_string()))?,
                packet_loss_percent: conn_row.try_get("packet_loss_percent").map_err(|e| AnalyticsError::Database(e.to_string()))?,
                jitter_ms: conn_row.try_get::<i32, _>("jitter_ms").map_err(|e| AnalyticsError::Database(e.to_string()))? as u32,
                connection_quality: match conn_row.try_get::<&str, _>("connection_quality").map_err(|e| AnalyticsError::Database(e.to_string()))? {
                    "Excellent" => ConnectionQuality::Excellent,
                    "Good" => ConnectionQuality::Good,
                    "Fair" => ConnectionQuality::Fair,
                    "Poor" => ConnectionQuality::Poor,
                    "Unusable" => ConnectionQuality::Unusable,
                    _ => ConnectionQuality::Good,
                },
                ice_connection_state: conn_row.try_get("ice_connection_state").map_err(|e| AnalyticsError::Database(e.to_string()))?,
                dtls_transport_state: conn_row.try_get("dtls_transport_state").map_err(|e| AnalyticsError::Database(e.to_string()))?,
                created_at: conn_row.try_get("created_at").map_err(|e| AnalyticsError::Database(e.to_string()))?,
                updated_at: conn_row.try_get("updated_at").map_err(|e| AnalyticsError::Database(e.to_string()))?,
            };
            connection_metrics.push(metric);
        }

        // Obtener métricas de streaming
        let streaming_rows = sqlx::query(
            "SELECT * FROM p2p_streaming_metrics WHERE analytics_id = $1"
        )
        .bind(&id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AnalyticsError::Database(e.to_string()))?;

        let mut streaming_metrics = Vec::new();
        for stream_row in streaming_rows {
            let metric = StreamingMetrics {
                stream_id: stream_row.try_get("stream_id").map_err(|e| AnalyticsError::Database(e.to_string()))?,
                content_id: stream_row.try_get("content_id").map_err(|e| AnalyticsError::Database(e.to_string()))?,
                user_id: stream_row.try_get("user_id").map_err(|e| AnalyticsError::Database(e.to_string()))?,
                quality_level: match stream_row.try_get::<&str, _>("quality_level").map_err(|e| AnalyticsError::Database(e.to_string()))? {
                    "UltraHD" => crate::bounded_contexts::p2p::domain::entities::analytics::VideoQuality::UltraHD,
                    "FullHD" => crate::bounded_contexts::p2p::domain::entities::analytics::VideoQuality::FullHD,
                    "HD" => crate::bounded_contexts::p2p::domain::entities::analytics::VideoQuality::HD,
                    "SD" => crate::bounded_contexts::p2p::domain::entities::analytics::VideoQuality::SD,
                    "Low" => crate::bounded_contexts::p2p::domain::entities::analytics::VideoQuality::Low,
                    _ => crate::bounded_contexts::p2p::domain::entities::analytics::VideoQuality::HD,
                },
                bitrate_kbps: stream_row.try_get::<i32, _>("bitrate_kbps").map_err(|e| AnalyticsError::Database(e.to_string()))? as u32,
                frame_rate: stream_row.try_get("frame_rate").map_err(|e| AnalyticsError::Database(e.to_string()))?,
                resolution_width: stream_row.try_get::<i32, _>("resolution_width").map_err(|e| AnalyticsError::Database(e.to_string()))? as u32,
                resolution_height: stream_row.try_get::<i32, _>("resolution_height").map_err(|e| AnalyticsError::Database(e.to_string()))? as u32,
                buffer_level_seconds: stream_row.try_get("buffer_level_seconds").map_err(|e| AnalyticsError::Database(e.to_string()))?,
                dropped_frames: stream_row.try_get::<i32, _>("dropped_frames").map_err(|e| AnalyticsError::Database(e.to_string()))? as u32,
                total_frames: stream_row.try_get::<i32, _>("total_frames").map_err(|e| AnalyticsError::Database(e.to_string()))? as u32,
                adaptive_switches: stream_row.try_get::<i32, _>("adaptive_switches").map_err(|e| AnalyticsError::Database(e.to_string()))? as u32,
                start_time: stream_row.try_get("start_time").map_err(|e| AnalyticsError::Database(e.to_string()))?,
                end_time: stream_row.try_get("end_time").map_err(|e| AnalyticsError::Database(e.to_string()))?,
                duration_seconds: stream_row.try_get("duration_seconds").map_err(|e| AnalyticsError::Database(e.to_string()))?,
            };
            streaming_metrics.push(metric);
        }

        // Obtener métricas de red y sistema desde JSON
        let network_metrics_json: serde_json::Value = row.try_get("network_metrics").map_err(|e| AnalyticsError::Database(e.to_string()))?;
        let system_metrics_json: serde_json::Value = row.try_get("system_metrics").map_err(|e| AnalyticsError::Database(e.to_string()))?;

        let network_metrics: NetworkMetrics = serde_json::from_value(network_metrics_json)
            .map_err(|e| AnalyticsError::Serialization(e.to_string()))?;
        let system_metrics: SystemPerformanceMetrics = serde_json::from_value(system_metrics_json)
            .map_err(|e| AnalyticsError::Serialization(e.to_string()))?;

        Ok(P2PAnalyticsAggregate {
            id,
            session_id,
            user_id,
            connection_metrics,
            streaming_metrics,
            network_metrics,
            system_metrics,
            created_at,
            updated_at,
        })
    }
} 