// =============================================================================
// USER GATEWAY - GESTIÓN DE USUARIOS INDEPENDIENTE
// =============================================================================
//
// Este gateway maneja todas las operaciones relacionadas con usuarios:
// - Registro y autenticación
// - Perfiles y preferencias
// - Relaciones sociales (followers/following)
// - Búsqueda y descubrimiento

use axum::{
    Router,
    routing::get,
    response::Json as ResponseJson,
};
use serde_json::json;
use std::sync::Arc;
use crate::shared::infrastructure::app_state::{AppState, AppStateFactory};
use crate::bounded_contexts::user::application::services::UserApplicationService;
use crate::shared::infrastructure::database::postgres::PostgresUserRepository;
use crate::bounded_contexts::user::presentation::routes::configure_user_routes;

// =============================================================================
// GATEWAY CREATION
// =============================================================================

/// Crear el gateway de usuario con todas las rutas y middleware
pub async fn create_user_gateway(app_state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
    // Crear UserAppState desde AppState usando el factory
    let user_app_state = AppStateFactory::create_user_state(app_state).await?;
    
    // Crear UserApplicationService con el repositorio
    let user_service = Arc::new(UserApplicationService::new(
        user_app_state.user_repository.clone()
    ));
    
    // Configurar rutas reales usando los controllers
    let user_routes = configure_user_routes(user_service);
    
    // Crear router principal con health/info + rutas reales
    let router = Router::new()
        // =============================================================================
        // HEALTH & INFO ENDPOINTS (mantener estos)
        // =============================================================================
        .route("/health", get(health_check))
        .route("/info", get(gateway_info))
        
        // =============================================================================
        // USER ROUTES REALES (conectados a controllers)
        // =============================================================================
        .nest("/", user_routes);

    Ok(router)
}

// =============================================================================
// HEALTH & INFO HANDLERS
// =============================================================================

async fn health_check() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "status": "healthy",
        "service": "user-gateway",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn gateway_info() -> ResponseJson<serde_json::Value> {
    ResponseJson(json!({
        "gateway": "user",
        "description": "User management and social features",
        "endpoints": {
            "health": "/health",
            "users": "/",
            "register": "/register",
            "login": "/login",
            "profiles": "/:id/profile",
            "social": "/:id/follow, /:id/followers",
            "search": "/search, /discover",
            "admin": "/admin/users"
        }
    }))
}

// =============================================================================
// NOTA: Los handlers reales están en user_controller.rs
// Estos handlers TODO fueron reemplazados por controllers reales
// =============================================================================
