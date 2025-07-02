// PostgreSQL Analytics Repository for Listen Reward Context
//
// This repository provides complex analytics queries using the database views
// and aggregation functions for reporting and business intelligence.

use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use super::{
    repository_traits::{
        RewardAnalyticsRepository, UserRewardHistory, ArtistRevenueAnalytics, 
        SongMetrics, PlatformStatistics, FraudMetrics, TopSong, RevenueTrend,
        GeographicMetric, TopArtist, FraudIndicator,
    },
    RepositoryResult, Pagination, RewardAnalytics,
};

// Estructuras para mapear las filas de base de datos
#[derive(sqlx::FromRow)]
struct UserRewardRow {
    user_id: Uuid,
    session_id: Uuid,
    song_id: Uuid,
    artist_id: Uuid,
    reward_amount: f64,
    quality_score: Option<f64>,
    listen_duration_seconds: Option<i32>,
    earned_at: DateTime<Utc>,
    transaction_hash: Option<String>,
}

#[derive(sqlx::FromRow)]
struct ArtistRevenueRow {
    artist_id: Uuid,
    total_revenue: f64,
    total_sessions: i64,
    unique_listeners: i64,
}

#[derive(sqlx::FromRow)]
struct SongMetricsRow {
    song_id: Uuid,
    total_listens: i64,
    unique_listeners: i64,
    total_rewards_paid: f64,
    average_listen_duration: f64,
    average_quality_score: Option<f64>,
    completion_rate: f64,
}

#[derive(sqlx::FromRow)]
struct PlatformStatsRow {
    total_sessions: Option<i64>,
    total_rewards_distributed: Option<f64>,
    unique_users: Option<i64>,
    unique_artists: Option<i64>,
    unique_songs: Option<i64>,
    average_session_duration: Option<f64>,
    total_zk_proofs_verified: Option<f64>,
    failed_verifications: Option<f64>,
}

#[derive(sqlx::FromRow)]
struct TopSongRow {
    song_id: Uuid,
    title: String,
    listen_count: i64,
    revenue: f64,
}

#[derive(sqlx::FromRow)]
struct TopArtistRow {
    artist_id: Uuid,
    name: String,
    revenue: f64,
    session_count: i64,
}

#[derive(sqlx::FromRow)]
struct AnalyticsAggregateRow {
    total_sessions: i64,
    total_rewards: f64,
    unique_users: i64,
    avg_session_duration: f64,
    avg_quality_score: Option<f64>,
}

pub struct PostgresRewardAnalyticsRepository {
    pool: PgPool,
}

impl PostgresRewardAnalyticsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Método auxiliar para convertir filas de base de datos a objetos de dominio
    fn map_user_reward_row(&self, row: UserRewardRow) -> UserRewardHistory {
        UserRewardHistory {
            session_id: row.session_id,
            song_id: row.song_id,
            artist_id: row.artist_id,
            reward_amount: row.reward_amount,
            quality_score: row.quality_score,
            listen_duration: row.listen_duration_seconds.map(|d| d as u32),
            earned_at: row.earned_at,
            transaction_hash: row.transaction_hash,
        }
    }

    fn map_revenue_trend(&self, rows: Vec<ArtistRevenueRow>) -> Vec<RevenueTrend> {
        rows.into_iter()
            .map(|row| RevenueTrend {
                date: Utc::now(), // Placeholder - this field needs adjustment
                revenue: row.total_revenue,
                session_count: row.total_sessions,
            })
            .collect()
    }

    fn map_top_song(&self, row: TopSongRow) -> TopSong {
        TopSong {
            song_id: row.song_id,
            title: row.title,
            listen_count: row.listen_count,
            revenue: row.revenue,
        }
    }

    fn map_top_artist(&self, row: TopArtistRow) -> TopArtist {
        TopArtist {
            artist_id: row.artist_id,
            name: row.name,
            revenue: row.revenue,
            session_count: row.session_count,
        }
    }
}

#[async_trait]
impl RewardAnalyticsRepository for PostgresRewardAnalyticsRepository {
    async fn get_analytics(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> RepositoryResult<RewardAnalytics> {
        // Consulta agregada para estadísticas básicas
        let query = r#"
            SELECT 
                COUNT(*) as total_sessions,
                COALESCE(SUM(final_reward_tokens), 0) as total_rewards,
                COUNT(DISTINCT user_id) as unique_users,
                COALESCE(AVG(listen_duration_seconds), 0) as avg_session_duration,
                AVG(quality_score) as avg_quality_score
            FROM listen_sessions 
            WHERE started_at >= $1 AND started_at <= $2
        "#;

        let row = sqlx::query_as::<_, AnalyticsAggregateRow>(query)
            .bind(start)
            .bind(end)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        Ok(RewardAnalytics {
            total_sessions: row.total_sessions,
            total_rewards_distributed: row.total_rewards,
            unique_users: row.unique_users,
            unique_songs: 0, // Placeholder
            average_session_duration: row.avg_session_duration,
            average_reward_per_session: if row.total_sessions > 0 { row.total_rewards / row.total_sessions as f64 } else { 0.0 },
            total_zk_proofs_verified: 0, // Placeholder
            failed_verifications: 0, // Placeholder
            period_start: start,
            period_end: end,
        })
    }

    async fn get_user_reward_history(
        &self,
        user_id: Uuid,
        pagination: &Pagination,
    ) -> RepositoryResult<Vec<UserRewardHistory>> {
        let query = r#"
            SELECT 
                user_id,
                id as session_id,
                song_id,
                artist_id,
                COALESCE(final_reward_tokens, 0) as reward_amount,
                quality_score,
                listen_duration_seconds,
                COALESCE(completed_at, started_at) as earned_at,
                NULL::text as transaction_hash
            FROM listen_sessions 
            WHERE user_id = $1 
            ORDER BY started_at DESC 
            LIMIT $2 OFFSET $3
        "#;

        let rows = sqlx::query_as::<_, UserRewardRow>(query)
            .bind(user_id)
            .bind(pagination.limit)
            .bind(pagination.offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let history: Vec<UserRewardHistory> = rows.into_iter().map(|row| {
            UserRewardHistory {
                session_id: row.session_id,
                song_id: row.song_id,
                artist_id: row.artist_id,
                reward_amount: row.reward_amount,
                quality_score: row.quality_score,
                listen_duration: row.listen_duration_seconds.map(|d| d as u32),
                earned_at: row.earned_at,
                transaction_hash: row.transaction_hash,
            }
        }).collect();

        Ok(history)
    }

    async fn get_artist_revenue(
        &self,
        artist_id: Uuid,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> RepositoryResult<ArtistRevenueAnalytics> {
        // Estadísticas básicas del artista
        let stats_query = r#"
            SELECT 
                artist_id,
                COALESCE(SUM(final_reward_tokens), 0) as total_revenue,
                COUNT(*) as total_sessions,
                COUNT(DISTINCT user_id) as unique_listeners
            FROM listen_sessions 
            WHERE artist_id = $1 AND started_at >= $2 AND started_at <= $3
            GROUP BY artist_id
        "#;

        let artist_stats = sqlx::query_as::<_, ArtistRevenueRow>(stats_query)
            .bind(artist_id)
            .bind(start)
            .bind(end)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        if let Some(stats) = artist_stats {
            // Top songs para este artista
            let top_songs_query = r#"
                SELECT 
                    song_id,
                    'Unknown Song' as title,
                    COUNT(*) as listen_count,
                    COALESCE(SUM(final_reward_tokens), 0) as revenue
                FROM listen_sessions 
                WHERE artist_id = $1 AND started_at >= $2 AND started_at <= $3
                GROUP BY song_id
                ORDER BY revenue DESC
                LIMIT 10
            "#;

            let song_rows = sqlx::query_as::<_, TopSongRow>(top_songs_query)
                .bind(artist_id)
                .bind(start)
                .bind(end)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| format!("Database error: {}", e))?;

            let top_songs: Vec<TopSong> = song_rows.into_iter().map(|row| {
                TopSong {
                    song_id: row.song_id,
                    title: row.title,
                    listen_count: row.listen_count,
                    revenue: row.revenue,
                }
            }).collect();

            // Tendencia de ingresos por día
            let trend_query = r#"
                SELECT 
                    DATE_TRUNC('day', started_at) as date,
                    COUNT(*) as session_count
                FROM listen_sessions 
                WHERE artist_id = $1 AND started_at >= $2 AND started_at <= $3
                GROUP BY DATE_TRUNC('day', started_at)
                ORDER BY date
            "#;

            let trend_rows = sqlx::query(trend_query)
                .bind(artist_id)
                .bind(start)
                .bind(end)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| format!("Database error: {}", e))?;

            let revenue_trend: Vec<RevenueTrend> = trend_rows.into_iter().map(|row| {
                RevenueTrend {
                    date: row.get("date"),
                    session_count: row.get("session_count"),
                    revenue: 0.0, // Placeholder - podríamos calcular esto con otra consulta
                }
            }).collect();

            Ok(ArtistRevenueAnalytics {
                artist_id: stats.artist_id,
                total_revenue: stats.total_revenue,
                total_sessions: stats.total_sessions,
                unique_listeners: stats.unique_listeners,
                top_songs,
                revenue_trend,
                period_start: start,
                period_end: end,
            })
        } else {
            // Retornar analíticas vacías si no hay datos
            Ok(ArtistRevenueAnalytics {
                artist_id,
                total_revenue: 0.0,
                total_sessions: 0,
                unique_listeners: 0,
                top_songs: Vec::new(),
                revenue_trend: Vec::new(),
                period_start: start,
                period_end: end,
            })
        }
    }

    async fn get_song_metrics(
        &self,
        song_id: Uuid,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> RepositoryResult<SongMetrics> {
        let query = r#"
            SELECT 
                song_id,
                COUNT(*) as total_listens,
                COUNT(DISTINCT user_id) as unique_listeners,
                COALESCE(SUM(final_reward_tokens), 0) as total_rewards_paid,
                COALESCE(AVG(listen_duration_seconds), 0) as average_listen_duration,
                AVG(quality_score) as average_quality_score,
                CASE 
                    WHEN COUNT(*) > 0 THEN 
                        (COUNT(CASE WHEN status = 'completed' THEN 1 END)::float / COUNT(*)::float) * 100
                    ELSE 0 
                END as completion_rate
            FROM listen_sessions 
            WHERE song_id = $1 AND started_at >= $2 AND started_at <= $3
            GROUP BY song_id
        "#;

        let result = sqlx::query_as::<_, SongMetricsRow>(query)
            .bind(song_id)
            .bind(start)
            .bind(end)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        if let Some(r) = result {
            Ok(SongMetrics {
                song_id: r.song_id,
                total_listens: r.total_listens,
                unique_listeners: r.unique_listeners,
                total_rewards_paid: r.total_rewards_paid,
                average_listen_duration: r.average_listen_duration,
                average_quality_score: r.average_quality_score,
                completion_rate: r.completion_rate,
                listener_geography: Vec::new(), // Se puede implementar más tarde
            })
        } else {
            Ok(SongMetrics {
                song_id,
                total_listens: 0,
                unique_listeners: 0,
                total_rewards_paid: 0.0,
                average_listen_duration: 0.0,
                average_quality_score: None,
                completion_rate: 0.0,
                listener_geography: Vec::new(),
            })
        }
    }

    async fn get_platform_statistics(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> RepositoryResult<PlatformStatistics> {
        let stats_query = r#"
            SELECT 
                COUNT(*) as total_sessions,
                COALESCE(SUM(final_reward_tokens), 0) as total_rewards_distributed,
                COUNT(DISTINCT user_id) as unique_users,
                COUNT(DISTINCT artist_id) as unique_artists,
                COUNT(DISTINCT song_id) as unique_songs,
                COALESCE(AVG(listen_duration_seconds), 0) as average_session_duration,
                COUNT(CASE WHEN status = 'verified' THEN 1 END)::float as total_zk_proofs_verified,
                COUNT(CASE WHEN status = 'failed' THEN 1 END)::float as failed_verifications
            FROM listen_sessions 
            WHERE started_at >= $1 AND started_at <= $2
        "#;

        let result = sqlx::query_as::<_, PlatformStatsRow>(stats_query)
            .bind(start)
            .bind(end)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        // Top artists
        let top_artists_query = r#"
            SELECT 
                artist_id,
                'Unknown Artist' as name,
                COALESCE(SUM(final_reward_tokens), 0) as revenue,
                COUNT(*) as session_count
            FROM listen_sessions 
            WHERE started_at >= $1 AND started_at <= $2
            GROUP BY artist_id
            ORDER BY revenue DESC
            LIMIT 10
        "#;

        let artist_rows = sqlx::query_as::<_, TopArtistRow>(top_artists_query)
            .bind(start)
            .bind(end)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let top_performing_artists: Vec<TopArtist> = artist_rows.into_iter().map(|row| {
            TopArtist {
                artist_id: row.artist_id,
                name: row.name,
                revenue: row.revenue,
                session_count: row.session_count,
            }
        }).collect();

        let total_verified = result.total_zk_proofs_verified.unwrap_or(0.0);
        let total_failed = result.failed_verifications.unwrap_or(0.0);
        let success_rate = if total_verified + total_failed > 0.0 {
            (total_verified / (total_verified + total_failed)) * 100.0
        } else {
            0.0
        };

        Ok(PlatformStatistics {
            total_sessions: result.total_sessions.unwrap_or(0),
            total_rewards_distributed: result.total_rewards_distributed.unwrap_or(0.0),
            unique_users: result.unique_users.unwrap_or(0),
            unique_artists: result.unique_artists.unwrap_or(0),
            unique_songs: result.unique_songs.unwrap_or(0),
            average_session_duration: result.average_session_duration.unwrap_or(0.0),
            zk_proof_success_rate: success_rate,
            daily_active_users: result.unique_users.unwrap_or(0), // Simplificación
            top_performing_artists,
            reward_pool_utilization: 75.0, // Placeholder
        })
    }

    async fn get_fraud_metrics(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> RepositoryResult<FraudMetrics> {
        let query = r#"
            SELECT 
                COUNT(CASE WHEN status = 'failed' THEN 1 END) as failed_verifications,
                COUNT(CASE WHEN quality_score < 0.3 THEN 1 END) as suspicious_patterns,
                COUNT(*) as total_sessions
            FROM listen_sessions 
            WHERE started_at >= $1 AND started_at <= $2
        "#;

        let result = sqlx::query(query)
            .bind(start)
            .bind(end)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let failed_verifications: i64 = result.get("failed_verifications");
        let suspicious_patterns: i64 = result.get("suspicious_patterns");
        let total_sessions: i64 = result.get("total_sessions");

        let fraud_rate = if total_sessions > 0 {
            ((failed_verifications + suspicious_patterns) as f64 / total_sessions as f64) * 100.0
        } else {
            0.0
        };

        let fraud_indicators = vec![
            FraudIndicator {
                indicator_type: "zk_proof_failure".to_string(),
                count: failed_verifications,
                description: "Failed ZK proof verification".to_string(),
            },
            FraudIndicator {
                indicator_type: "low_quality_score".to_string(),
                count: suspicious_patterns,
                description: "Suspiciously low quality scores".to_string(),
            },
        ];

        Ok(FraudMetrics {
            total_fraud_attempts: failed_verifications + suspicious_patterns,
            failed_zk_verifications: failed_verifications,
            suspicious_patterns,
            blocked_sessions: failed_verifications, // Simplificación
            fraud_rate_percentage: fraud_rate,
            top_fraud_indicators: fraud_indicators,
        })
    }
} 