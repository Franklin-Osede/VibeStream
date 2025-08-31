// =============================================================================
// SIMPLIFIED ROUTER - Separado por contexto para reducir complejidad
// =============================================================================
// 
// Este módulo implementa un router simplificado que separa las responsabilidades
// por contexto, eliminando el acoplamiento excesivo.

use axum::{routing::get, Router};
use std::sync::Arc;
use sqlx::PgPool;

// Import shared infrastructure
use crate::shared::infrastructure::app_state::{AppState, AppStateFactory};
use crate::bounded_contexts::music::presentation::controllers::{
    SongController, AlbumController, PlaylistController, ArtistController
};

// =============================================================================
// MAIN ROUTER CREATION
// =============================================================================

/// Crear el router principal simplificado
pub async fn create_app_router(db_pool: PgPool) -> Result<Router, Box<dyn std::error::Error>> {
    // Crear AppState base
    let app_state = AppState::new(
        "postgresql://vibestream:vibestream@localhost:5433/vibestream",
        "redis://localhost:6379"
    ).await
        .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)?;

    // Crear router base con middleware
    let router = Router::new()
        // =============================================================================
        // HEALTH & INFO ENDPOINTS
        // =============================================================================
        .route("/health", get(health_check))
        .route("/", get(api_info))
        .route("/api", get(api_info))
        .route("/api/v1", get(api_info))
        
        // =============================================================================
        // CONTEXT-SPECIFIC ROUTES
        // =============================================================================
        .nest("/api/v1/users", create_user_routes(app_state.clone()).await?)
        .nest("/api/v1/music", create_music_routes(app_state.clone()).await?)
        .nest("/api/v1/campaigns", create_campaign_routes(app_state.clone()).await?)
        .nest("/api/v1/listen-rewards", create_listen_reward_routes(app_state.clone()).await?)
        .nest("/api/v1/fan-ventures", create_fan_ventures_routes(app_state.clone()).await?)
        .nest("/api/v1/notifications", create_notification_routes(app_state.clone()).await?);

    Ok(router)
}

// =============================================================================
// HEALTH CHECK HANDLERS
// =============================================================================

async fn health_check() -> &'static str {
    "VibeStream API Gateway - All systems operational"
}

async fn api_info() -> &'static str {
    "VibeStream API v2.0 - Simplified Architecture with Event-Driven Design"
}

// =============================================================================
// CONTEXT-SPECIFIC ROUTE CREATORS
// =============================================================================

/// Crear rutas para el contexto de usuario
async fn create_user_routes(app_state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
    let user_state = AppStateFactory::create_user_state(app_state).await
        .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)?;
    
    let router = Router::new()
        .route("/", get(crate::bounded_contexts::user::presentation::controllers::user_controller::get_users))
        .route("/:id", get(crate::bounded_contexts::user::presentation::controllers::user_controller::get_user))
        .route("/register", axum::routing::post(crate::bounded_contexts::user::presentation::controllers::user_controller::register_user))
        .route("/login", axum::routing::post(crate::bounded_contexts::user::presentation::controllers::user_controller::login_user))
        .route("/:id/profile", axum::routing::put(crate::bounded_contexts::user::presentation::controllers::user_controller::update_user_profile))
        .route("/:id/follow", axum::routing::post(crate::bounded_contexts::user::presentation::controllers::user_controller::follow_user))
        .route("/:id/unfollow", axum::routing::post(crate::bounded_contexts::user::presentation::controllers::user_controller::unfollow_user))
        .route("/:id/followers", get(crate::bounded_contexts::user::presentation::controllers::user_controller::get_user_followers))
        .route("/:id/following", get(crate::bounded_contexts::user::presentation::controllers::user_controller::get_user_following))
        .route("/search", get(crate::bounded_contexts::user::presentation::controllers::user_controller::search_users))
        .with_state(user_state.user_repository);
    
    Ok(router)
}

/// Crear rutas para el contexto de música
async fn create_music_routes(app_state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
    let music_state = AppStateFactory::create_music_state(app_state).await
        .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)?;
    
    let router = Router::new()
        .route("/songs", get(SongController::get_songs))
        .route("/songs", axum::routing::post(SongController::create_song))
        .route("/songs/:id", get(SongController::get_song))
        .route("/songs/:id", axum::routing::put(SongController::update_song))
        .route("/songs/:id", axum::routing::delete(SongController::delete_song))
        .route("/songs/discover", get(SongController::discover_songs))
        .route("/songs/trending", get(SongController::get_trending_songs))
        .route("/songs/:id/like", axum::routing::post(SongController::like_song))
        .route("/songs/:id/unlike", axum::routing::post(SongController::unlike_song))
        .route("/songs/:id/share", axum::routing::post(SongController::share_song))
        .route("/albums", get(AlbumController::get_albums))
        .route("/albums", axum::routing::post(AlbumController::create_album))
        .route("/albums/:id", get(AlbumController::get_album))
        .route("/playlists", get(PlaylistController::get_playlists))
        .route("/playlists", axum::routing::post(PlaylistController::create_playlist))
        .route("/playlists/:id", get(PlaylistController::get_playlist))
        .route("/playlists/:id/songs", axum::routing::post(PlaylistController::add_song_to_playlist))
        .route("/playlists/:id/songs/:song_id", axum::routing::delete(PlaylistController::remove_song_from_playlist))
        .route("/artists", get(ArtistController::get_artists))
        .route("/artists/:id", get(ArtistController::get_artist))
        .route("/artists/:id/songs", get(ArtistController::get_artist_songs))
        .route("/artists/:id/albums", get(ArtistController::get_artist_albums))
        .with_state(music_state);
    
    Ok(router)
}

/// Crear rutas para el contexto de campañas
async fn create_campaign_routes(app_state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
    let campaign_state = AppStateFactory::create_campaign_state(app_state).await
        .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)?;
    
    let router = Router::new()
        .route("/", axum::routing::post(crate::bounded_contexts::campaign::presentation::controllers::CampaignController::create_campaign))
        .route("/", get(crate::bounded_contexts::campaign::presentation::controllers::CampaignController::list_campaigns))
        .route("/:id", get(crate::bounded_contexts::campaign::presentation::controllers::CampaignController::get_campaign))
        .route("/:id/activate", axum::routing::put(crate::bounded_contexts::campaign::presentation::controllers::CampaignController::activate_campaign))
        .route("/:id/pause", axum::routing::put(crate::bounded_contexts::campaign::presentation::controllers::CampaignController::pause_campaign))
        .route("/:id/resume", axum::routing::put(crate::bounded_contexts::campaign::presentation::controllers::CampaignController::resume_campaign))
        .route("/:id/end", axum::routing::put(crate::bounded_contexts::campaign::presentation::controllers::CampaignController::end_campaign))
        .route("/:id/purchase", axum::routing::post(crate::bounded_contexts::campaign::presentation::controllers::CampaignController::purchase_nft))
        .route("/:id/analytics", get(crate::bounded_contexts::campaign::presentation::controllers::CampaignController::get_campaign_analytics))
        .route("/trending", get(crate::bounded_contexts::campaign::presentation::controllers::CampaignController::get_trending_campaigns))
        .with_state(campaign_state);
    
    Ok(router)
}

/// Crear rutas para el contexto de listen rewards
async fn create_listen_reward_routes(app_state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
    let listen_reward_state = AppStateFactory::create_listen_reward_state(app_state).await
        .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)?;
    
    let router = Router::new()
        .route("/sessions/start", axum::routing::post(crate::bounded_contexts::listen_reward::presentation::controllers::listen_reward_controller::ListenRewardController::start_session))
        .route("/sessions/:session_id/complete", axum::routing::post(crate::bounded_contexts::listen_reward::presentation::controllers::listen_reward_controller::ListenRewardController::complete_session))
        .route("/sessions/:session_id", get(crate::bounded_contexts::listen_reward::presentation::controllers::listen_reward_controller::ListenRewardController::get_session_details))
        .route("/sessions/user/:user_id", get(crate::bounded_contexts::listen_reward::presentation::controllers::listen_reward_controller::ListenRewardController::get_user_sessions))
        .with_state(listen_reward_state.session_repository);
    
    Ok(router)
}

/// Crear rutas para el contexto de fan ventures
async fn create_fan_ventures_routes(app_state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
    let fan_ventures_state = AppStateFactory::create_fan_ventures_state(app_state).await
        .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)?;
    
    let router = Router::new()
        .route("/ventures", axum::routing::post(crate::bounded_contexts::fan_ventures::presentation::controllers::FanVenturesController::create_venture))
        .route("/ventures", get(crate::bounded_contexts::fan_ventures::presentation::controllers::FanVenturesController::get_ventures))
        .route("/ventures/:id", get(crate::bounded_contexts::fan_ventures::presentation::controllers::FanVenturesController::get_venture))
        .route("/ventures/:id/invest", axum::routing::post(crate::bounded_contexts::fan_ventures::presentation::controllers::FanVenturesController::invest_in_venture))
        .route("/ventures/:id/benefits", get(crate::bounded_contexts::fan_ventures::presentation::controllers::FanVenturesController::get_venture_benefits))
        .route("/ventures/:id/benefits/:benefit_id/deliver", axum::routing::post(crate::bounded_contexts::fan_ventures::presentation::controllers::FanVenturesController::deliver_benefit))
        .route("/investments/user/:user_id", get(crate::bounded_contexts::fan_ventures::presentation::controllers::FanVenturesController::get_user_investments))
        .route("/analytics/venture/:id", get(crate::bounded_contexts::fan_ventures::presentation::controllers::FanVenturesController::get_venture_analytics))
        .with_state(fan_ventures_state);
    
    Ok(router)
}

/// Crear rutas para el contexto de notificaciones
async fn create_notification_routes(app_state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
    let notification_state = AppStateFactory::create_notification_state(app_state).await
        .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)?;
    
    let router = Router::new()
        .route("/", axum::routing::post(crate::bounded_contexts::notifications::presentation::controllers::NotificationController::create_notification))
        .route("/user/:user_id", get(crate::bounded_contexts::notifications::presentation::controllers::NotificationController::get_user_notifications))
        .route("/:id", get(crate::bounded_contexts::notifications::presentation::controllers::NotificationController::get_notification))
        .route("/:id/read", axum::routing::put(crate::bounded_contexts::notifications::presentation::controllers::NotificationController::mark_as_read))
        .route("/:id/archive", axum::routing::put(crate::bounded_contexts::notifications::presentation::controllers::NotificationController::mark_as_archived))
        .route("/:id", axum::routing::delete(crate::bounded_contexts::notifications::presentation::controllers::NotificationController::delete_notification))
        .route("/user/:user_id/read-all", axum::routing::put(crate::bounded_contexts::notifications::presentation::controllers::NotificationController::mark_all_as_read))
        .route("/user/:user_id/preferences", get(crate::bounded_contexts::notifications::presentation::controllers::NotificationController::get_preferences))
        .route("/user/:user_id/preferences", axum::routing::put(crate::bounded_contexts::notifications::presentation::controllers::NotificationController::update_preferences))
        .route("/user/:user_id/summary", get(crate::bounded_contexts::notifications::presentation::controllers::NotificationController::get_notification_summary))
        .with_state(notification_state);
    
    Ok(router)
}

// =============================================================================
// MIDDLEWARE CONFIGURATION
// =============================================================================

use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;

/// Aplicar middleware al router
pub fn apply_middleware(router: Router) -> Router {
    router
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        )
        .layer(TraceLayer::new_for_http())
}

// =============================================================================
// DOCUMENTATION
// =============================================================================

pub const API_DOCUMENTATION: &str = r#"
# VibeStream API v2.0 - Simplified Architecture

## Architecture Improvements

### ✅ Event-Driven Communication
- Domain events for cross-context communication
- Eliminated centralized orchestrator
- Reduced coupling between contexts

### ✅ Context-Specific State Management
- Each context manages its own state
- Simplified AppState with minimal dependencies
- Better separation of concerns

### ✅ Simplified Router Structure
- Separated routes by context
- Reduced complexity from 502 lines to manageable modules
- Clear responsibility boundaries

## API Endpoints

### 👤 User Context (/api/v1/users)
- User registration, authentication, and profile management
- Social features: following, followers
- User search and discovery

### 🎵 Music Context (/api/v1/music)
- Song upload, discovery, and analytics
- Album and playlist management
- Artist profiles and content management

### 🎯 Campaign Context (/api/v1/campaigns)
- Marketing campaign creation and management
- NFT purchasing and distribution
- Campaign analytics and performance tracking

### 🎧 Listen Reward Context (/api/v1/listen-rewards)
- Listen tracking and verification
- Reward calculation and distribution
- User behavior analytics

### 💎 Fan Ventures Context (/api/v1/fan-ventures)
- Investment and trading platform
- Venture creation and management
- Benefit delivery and tracking

### 🔔 Notifications Context (/api/v1/notifications)
- User notification management
- Preference settings
- Notification delivery and tracking

## Architecture Patterns

✅ Domain-Driven Design (DDD)
✅ Event-Driven Architecture
✅ Command Query Responsibility Segregation (CQRS)
✅ Repository Pattern
✅ Clean Architecture layers
✅ Context-Specific State Management
✅ Simplified Dependency Injection
"#; 