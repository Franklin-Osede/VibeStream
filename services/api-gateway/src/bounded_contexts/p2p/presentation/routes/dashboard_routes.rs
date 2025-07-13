use axum::{
    routing::{get},
    Router,
};
use std::sync::Arc;

use crate::bounded_contexts::p2p::presentation::controllers::dashboard_controller::P2PDashboardController;

/// Crear router para rutas del dashboard P2P
pub fn create_dashboard_routes<S>(dashboard_controller: Arc<P2PDashboardController<S>>) -> Router
where
    S: crate::bounded_contexts::p2p::domain::repositories::analytics_repository::P2PAnalyticsRepository + 'static,
{
    Router::new()
        // Servir el dashboard HTML
        .route(
            "/",
            get(P2PDashboardController::serve_dashboard),
        )
        // Obtener métricas en tiempo real
        .route(
            "/realtime-metrics",
            get(P2PDashboardController::get_realtime_metrics),
        )
        // Obtener alertas del sistema
        .route(
            "/alerts",
            get(P2PDashboardController::get_system_alerts),
        )
        // Obtener gráficos de tendencias
        .route(
            "/trends",
            get(P2PDashboardController::get_trend_charts),
        )
        .with_state(dashboard_controller)
} 