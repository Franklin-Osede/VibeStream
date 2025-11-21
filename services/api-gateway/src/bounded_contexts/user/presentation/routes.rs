// User Context REST Routes
// This module configures all REST API routes for user operations using Axum

use axum::{
    routing::{get, post, put, delete},
    Router,
    middleware,
};
use std::sync::Arc;

use super::controllers::user_controller::*;
use crate::bounded_contexts::user::application::services::UserApplicationService;
use crate::shared::infrastructure::database::postgres::PostgresUserRepository;
use crate::shared::infrastructure::auth::middleware::jwt_auth_middleware;

/// Configure user routes with comprehensive REST API endpoints
pub fn configure_user_routes(
    user_service: Arc<UserApplicationService<PostgresUserRepository>>,
) -> Router {
    // Rutas públicas (no requieren autenticación)
    let public_routes = Router::new()
        // Authentication & Registration (públicas)
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        // Search básico puede ser público (con limitaciones)
        .route("/search", get(search_users));
    
    // Rutas protegidas (requieren JWT)
    let protected_routes = Router::new()
        // User Profile Management
        .route("/:user_id", get(get_user_profile))
        .route("/:user_id", put(update_user_profile))
        .route("/:user_id", delete(delete_user))
        
        // User Statistics & Analytics
        .route("/:user_id/stats", get(get_user_stats))
        
        // Social Features
        .route("/:user_id/follow", post(follow_user))
        .route("/:user_id/followers", get(get_user_followers))
        .route("/:user_id/following", get(get_user_following))
        
        // Account Management
        .route("/:user_id/change-password", post(change_password))
        .route("/:user_id/link-wallet", post(link_wallet))
        
        // Admin Analytics
        .route("/analytics", get(get_user_analytics))
        
        // Aplicar middleware de autenticación a todas las rutas protegidas
        .layer(middleware::from_fn(jwt_auth_middleware));
    
    // Combinar rutas públicas y protegidas
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        // Set the user service as shared state
        .with_state((*user_service).clone())
}

/// Create User Context Routes with API version prefix
pub fn create_user_routes(
    user_service: Arc<UserApplicationService<PostgresUserRepository>>,
) -> Router {
    Router::new()
        .nest("/users", configure_user_routes(user_service))
}

/*
=== USER CONTEXT API DOCUMENTATION ===

🔐 Authentication & Registration:
POST   /api/v1/users/register         - Register new user
POST   /api/v1/users/login           - User login

👤 User Profile Management:
GET    /api/v1/users/{user_id}       - Get user profile
PUT    /api/v1/users/{user_id}       - Update user profile  
DELETE /api/v1/users/{user_id}       - Delete user account

📊 User Statistics:
GET    /api/v1/users/{user_id}/stats - Get user statistics

🔍 Search & Discovery:
GET    /api/v1/users/search?q=...    - Search users by criteria

👥 Social Features:
POST   /api/v1/users/{user_id}/follow   - Follow/unfollow user
GET    /api/v1/users/{user_id}/followers - Get user followers
GET    /api/v1/users/{user_id}/following - Get users being followed

🔧 Account Management:
POST   /api/v1/users/{user_id}/change-password - Change password
POST   /api/v1/users/{user_id}/link-wallet     - Link blockchain wallet

📈 Admin Analytics:
GET    /api/v1/users/analytics       - Get user analytics (admin only)

=== EXAMPLE REQUESTS ===

Register User:
POST /api/v1/users/register
{
  "email": "user@example.com",
  "username": "user123",
  "password": "securepass123",
  "confirm_password": "securepass123",
  "display_name": "Usuario Demo",
  "bio": "Amante de la música",
  "terms_accepted": true,
  "marketing_emails_consent": true
}

Login:
POST /api/v1/users/login
{
  "credential": "user@example.com",
  "password": "securepass123",
  "remember_me": true
}

Update Profile:
PUT /api/v1/users/{user_id}
{
  "display_name": "Nuevo Nombre",
  "bio": "Nueva biografía",
  "profile_image_url": "https://example.com/avatar.jpg",
  "location": "Madrid, España",
  "website": "https://example.com",
  "social_links": {
    "twitter": "https://twitter.com/user",
    "instagram": "https://instagram.com/user"
  },
  "is_public": true
}

Search Users:
GET /api/v1/users/search?q=user&tier=bronze&is_verified=true&page=0&page_size=20

Follow User:
POST /api/v1/users/{user_id}/follow
{
  "follow": true
}

Change Password:
POST /api/v1/users/{user_id}/change-password
{
  "current_password": "oldpass123",
  "new_password": "newpass123",
  "confirm_new_password": "newpass123"
}

Link Wallet:
POST /api/v1/users/{user_id}/link-wallet
{
  "wallet_address": "0x1234567890abcdef...",
  "signature": "0xsignature...",
  "message": "Link wallet to VibeStream account"
}

=== RESPONSE FORMATS ===

Success Response:
{
  "success": true,
  "data": { ... },
  "message": "Operación exitosa",
  "errors": null
}

Error Response:
{
  "success": false,
  "data": null,
  "message": "Error message",
  "errors": ["Detailed error 1", "Detailed error 2"]
}

User Profile Response:
{
  "success": true,
  "data": {
    "id": "uuid",
    "username": "user123",
    "email": "user@example.com",
    "display_name": "Usuario Demo",
    "bio": "Amante de la música",
    "avatar_url": "https://example.com/avatar.jpg",
    "cover_url": "https://example.com/cover.jpg",
    "location": "Madrid, España",
    "website": "https://example.com",
    "social_links": {},
    "is_public": true,
    "tier": "bronze",
    "role": "user",
    "is_verified": false,
    "is_active": true,
    "wallet_address": null,
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z",
    "last_login_at": "2024-01-01T00:00:00Z"
  }
}

User Stats Response:
{
  "success": true,
  "data": {
    "user_id": "uuid",
    "total_listening_time_minutes": 14520,
    "total_listening_hours": 242.0,
    "total_songs_listened": 1250,
    "total_rewards_earned": 125.50,
    "current_listening_streak": 7,
    "longest_listening_streak": 21,
    "total_investments": 500.0,
    "investment_count": 3,
    "nfts_owned": 8,
    "campaigns_participated": 5,
    "tier_points": 1200,
    "achievements_unlocked": ["Primera canción", "100 horas de música"],
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  }
}

User List Response:
{
  "success": true,
  "data": {
    "users": [
      {
        "id": "uuid",
        "username": "user1",
        "display_name": "Usuario 1",
        "avatar_url": "https://example.com/avatar1.jpg",
        "tier": "bronze",
        "role": "user",
        "is_verified": false,
        "is_active": true,
        "tier_points": 800,
        "total_rewards": 50.0,
        "created_at": "2024-01-01T00:00:00Z"
      }
    ],
    "pagination": {
      "page": 0,
      "page_size": 20,
      "total_count": 1,
      "total_pages": 1,
      "has_next_page": false,
      "has_previous_page": false
    }
  }
}

=== AUTHENTICATION ===

Most endpoints require JWT authentication via Authorization header:
Authorization: Bearer <jwt_token>

Public endpoints (no auth required):
- POST /api/v1/users/register
- POST /api/v1/users/login
- GET /api/v1/users/search (limited results)

Admin endpoints (admin role required):
- GET /api/v1/users/analytics
- DELETE /api/v1/users/{user_id} (if different user)

=== RATE LIMITING ===

- Authentication endpoints: 5 requests/minute
- Search endpoints: 100 requests/minute
- Profile endpoints: 200 requests/minute
- Social endpoints: 50 requests/minute

=== VALIDATION RULES ===

Username:
- 3-30 characters
- Alphanumeric + underscore only
- Must be unique

Email:
- Valid email format
- Must be unique

Password:
- Minimum 8 characters
- Must contain letters and numbers

Display Name:
- 1-100 characters
- Any characters allowed

Bio:
- Maximum 500 characters

*/ 