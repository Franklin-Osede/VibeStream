use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::bounded_contexts::p2p::presentation::controllers::analytics_controller::P2PAnalyticsController;

/// Crear router para rutas de analíticas P2P
pub fn create_analytics_routes<S>(analytics_controller: Arc<P2PAnalyticsController<S>>) -> Router
where
    S: crate::bounded_contexts::p2p::domain::repositories::analytics_repository::P2PAnalyticsRepository + 'static,
{
    Router::new()
        // Obtener analíticas de sesión
        .route(
            "/session/:session_id",
            get(P2PAnalyticsController::get_session_analytics),
        )
        // Obtener analíticas de usuario
        .route(
            "/user/:user_id",
            get(P2PAnalyticsController::get_user_analytics),
        )
        // Obtener estadísticas agregadas
        .route(
            "/stats",
            get(P2PAnalyticsController::get_aggregated_stats),
        )
        // Registrar métricas de conexión
        .route(
            "/connection-metrics",
            post(P2PAnalyticsController::record_connection_metrics),
        )
        // Registrar métricas de streaming
        .route(
            "/streaming-metrics",
            post(P2PAnalyticsController::record_streaming_metrics),
        )
        // Generar reporte de rendimiento
        .route(
            "/performance-report/:user_id",
            get(P2PAnalyticsController::generate_performance_report),
        )
        .with_state(analytics_controller)
} 