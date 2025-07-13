pub mod analytics_routes;
pub mod video_management_routes;
pub mod video_routes;

pub use analytics_routes::*;
pub use video_management_routes::*;
pub use video_routes::*;

use axum::Router;
use std::sync::Arc;

/// Crear router principal para todas las rutas P2P
pub fn create_p2p_routes<S>(
    analytics_controller: Arc<crate::bounded_contexts::p2p::presentation::controllers::P2PAnalyticsController<S>>,
    video_streaming_service: Arc<crate::bounded_contexts::p2p::application::services::VideoStreamingService>,
    video_management_service: Arc<crate::bounded_contexts::p2p::application::services::VideoManagementService>,
) -> Router
where
    S: crate::bounded_contexts::p2p::domain::repositories::analytics_repository::P2PAnalyticsRepository + 'static,
{
    Router::new()
        .nest("/analytics", create_analytics_routes(analytics_controller))
        .nest("/video", video_routes().with_state(video_streaming_service))
        .nest("/video-management", create_video_management_routes(video_management_service))
} 