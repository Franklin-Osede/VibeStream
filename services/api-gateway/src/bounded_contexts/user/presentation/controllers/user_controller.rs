// User Context REST Controller
// This module contains REST API endpoints for user operations using Axum

use axum::{
    extract::{Path, Query, State, Json},
    http::StatusCode,
    response::Json as ResponseJson,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::bounded_contexts::user::application::{
    handlers::{
        CreateUserCommand, UpdateUserCommand, FollowUserCommand,
        GetUserQuery,
        UserCommandHandler, UserQueryHandler,
    },
    services::UserApplicationService,
};
use crate::shared::infrastructure::database::postgres::PostgresUserRepository;

// Type alias para simplificar el estado
type UserAppService = UserApplicationService<PostgresUserRepository>;

// Request/Response DTOs
#[derive(Debug, Deserialize)]
pub struct RegisterUserRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub confirm_password: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub terms_accepted: bool,
    pub marketing_emails_consent: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct RegisterUserResponse {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub token: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub credential: String, // email or username
    pub password: String,
    pub remember_me: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub token: String,
    pub refresh_token: Option<String>,
    pub expires_in: u64,
    pub user_role: String,
    pub tier: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub profile_image_url: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub social_links: Option<HashMap<String, String>>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct UserProfileResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub cover_url: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub social_links: HashMap<String, String>,
    pub is_public: bool,
    pub tier: String,
    pub role: String,
    pub is_verified: bool,
    pub is_active: bool,
    pub wallet_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct UserStatsResponse {
    pub user_id: Uuid,
    pub total_listening_time_minutes: u64,
    pub total_listening_hours: f64,
    pub total_songs_listened: u64,
    pub total_rewards_earned: f64,
    pub current_listening_streak: u32,
    pub longest_listening_streak: u32,
    pub total_investments: f64,
    pub investment_count: u32,
    pub nfts_owned: u32,
    pub campaigns_participated: u32,
    pub tier_points: u32,
    pub achievements_unlocked: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct FollowUserRequest {
    pub follow: bool, // true = follow, false = unfollow
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
    pub confirm_new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct LinkWalletRequest {
    pub wallet_address: String,
    pub signature: Option<String>,
    pub message: Option<String>,
}

// Use the query parameters directly without defining a new struct
#[derive(Debug, Deserialize)]
pub struct UserSearchQuery {
    pub q: Option<String>,
    pub tier: Option<String>,
    pub role: Option<String>,
    pub is_verified: Option<bool>,
    pub is_active: Option<bool>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub users: Vec<UserSummaryResponse>,
    pub pagination: PaginationResponse,
}

#[derive(Debug, Serialize)]
pub struct UserSummaryResponse {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub tier: String,
    pub role: String,
    pub is_verified: bool,
    pub is_active: bool,
    pub tier_points: u32,
    pub total_rewards: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PaginationResponse {
    pub page: u32,
    pub page_size: u32,
    pub total_count: u64,
    pub total_pages: u32,
    pub has_next_page: bool,
    pub has_previous_page: bool,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub errors: Option<Vec<String>>,
}

// REST Endpoints

/// POST /api/v1/users/register
/// Register new user account
#[axum::debug_handler]
pub async fn register_user(
    State(user_service): State<UserAppService>,
    Json(request): Json<RegisterUserRequest>,
) -> Result<Json<ApiResponse<RegisterUserResponse>>, StatusCode> {
    // Validate request
    if request.password != request.confirm_password {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Las contraseñas no coinciden".to_string()),
            errors: None,
        }));
    }

    if !request.terms_accepted {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Debe aceptar los términos y condiciones".to_string()),
            errors: None,
        }));
    }

    // Create command using the correct structure
    let command = CreateUserCommand {
        email: request.email.clone(),
        username: request.username.clone(),
        password: request.password.clone(),
        display_name: request.display_name.clone(),
        bio: request.bio.clone(),
    };

    match user_service.handle_create_user(command).await {
        Ok(result) => {
            let response = RegisterUserResponse {
                user_id: result.id,
                username: result.username,
                email: result.email,
                display_name: result.display_name,
                token: "mock_jwt_token".to_string(), // TODO: Generate real JWT
                created_at: result.created_at,
            };

            Ok(Json(ApiResponse {
                success: true,
                data: Some(response),
                message: Some("Usuario registrado exitosamente".to_string()),
                errors: None,
            }))
        }
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some(e.to_string()),
            errors: None,
        })),
    }
}

/// POST /api/v1/users/login
/// User login
#[axum::debug_handler]
pub async fn login_user(
    State(_user_service): State<UserAppService>,
    Json(_request): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, StatusCode> {
    // TODO: Implement authentication logic
    let mock_response = LoginResponse {
        user_id: Uuid::new_v4(),
        username: "user123".to_string(),
        email: "user@example.com".to_string(),
        display_name: Some("Usuario Demo".to_string()),
        token: "mock_jwt_token".to_string(),
        refresh_token: Some("mock_refresh_token".to_string()),
        expires_in: 3600,
        user_role: "user".to_string(),
        tier: "bronze".to_string(),
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(mock_response),
        message: Some("Login exitoso".to_string()),
        errors: None,
    }))
}

/// GET /api/v1/users/{user_id}
/// Get user profile by ID
#[axum::debug_handler]
pub async fn get_user_profile(
    State(user_service): State<UserAppService>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<UserProfileResponse>>, StatusCode> {
    let query = GetUserQuery { user_id };
    
    match user_service.handle_get_user(query).await {
        Ok(user_response) => {
            let profile = UserProfileResponse {
                id: user_response.id,
                username: user_response.username,
                email: user_response.email,
                display_name: user_response.display_name,
                bio: user_response.bio,
                avatar_url: user_response.profile_image_url,
                cover_url: None, // TODO: Add cover_url to UserResponse
                location: None,  // TODO: Add location to UserResponse
                website: None,   // TODO: Add website to UserResponse
                social_links: HashMap::new(), // TODO: Add social_links to UserResponse
                is_public: true, // TODO: Add is_public to UserResponse
                tier: "bronze".to_string(), // TODO: Add tier to UserResponse
                role: "user".to_string(),   // TODO: Add role to UserResponse
                is_verified: false,         // TODO: Add is_verified to UserResponse
                is_active: true,            // TODO: Add is_active to UserResponse
                wallet_address: None,       // TODO: Add wallet_address to UserResponse
                created_at: user_response.created_at,
                updated_at: user_response.created_at, // TODO: Add updated_at to UserResponse
                last_login_at: Some(Utc::now()),      // TODO: Add last_login_at to UserResponse
            };

            Ok(Json(ApiResponse {
                success: true,
                data: Some(profile),
                message: None,
                errors: None,
            }))
        }
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some(e.to_string()),
            errors: None,
        })),
    }
}

/// PUT /api/v1/users/{user_id}
/// Update user profile
#[axum::debug_handler]
pub async fn update_user_profile(
    State(user_service): State<UserAppService>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let command = UpdateUserCommand {
        user_id,
        display_name: request.display_name.clone(),
        bio: request.bio.clone(),
        profile_image_url: request.profile_image_url.clone(),
    };

    match user_service.handle_update_user(command).await {
        Ok(_) => Ok(Json(ApiResponse {
            success: true,
            data: None,
            message: Some("Perfil actualizado exitosamente".to_string()),
            errors: None,
        })),
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some(e.to_string()),
            errors: None,
        })),
    }
}

/// GET /api/v1/users/{user_id}/stats
/// Get user statistics
#[axum::debug_handler]
pub async fn get_user_stats(
    State(_user_service): State<UserAppService>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<UserStatsResponse>>, StatusCode> {
    let mock_stats = UserStatsResponse {
        user_id,
        total_listening_time_minutes: 14520,
        total_listening_hours: 242.0,
        total_songs_listened: 1250,
        total_rewards_earned: 125.50,
        current_listening_streak: 7,
        longest_listening_streak: 21,
        total_investments: 500.0,
        investment_count: 3,
        nfts_owned: 8,
        campaigns_participated: 5,
        tier_points: 1200,
        achievements_unlocked: vec![
            "Primera canción".to_string(),
            "100 horas de música".to_string(),
            "Primer NFT".to_string(),
        ],
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(mock_stats),
        message: None,
        errors: None,
    }))
}

/// GET /api/v1/users/search
/// Search users
#[axum::debug_handler]
pub async fn search_users(
    State(user_service): State<UserAppService>,
    Query(search_query): Query<UserSearchQuery>,
) -> Result<Json<ApiResponse<UserListResponse>>, StatusCode> {
    let query = crate::bounded_contexts::user::application::handlers::SearchUsersQuery {
        search_text: search_query.q,
        limit: search_query.page_size,
        offset: search_query.page.map(|p| p * search_query.page_size.unwrap_or(20)),
    };
    
    match user_service.handle_search_users(query).await {
        Ok(user_responses) => {
            let users: Vec<UserSummaryResponse> = user_responses.into_iter().map(|user_response| UserSummaryResponse {
                id: user_response.id,
                username: user_response.username,
                display_name: user_response.display_name,
                avatar_url: user_response.profile_image_url,
                tier: "bronze".to_string(), // TODO: Add tier to UserResponse
                role: "user".to_string(),   // TODO: Add role to UserResponse
                is_verified: false,         // TODO: Add is_verified to UserResponse
                is_active: true,            // TODO: Add is_active to UserResponse
                tier_points: 800,           // TODO: Add tier_points to UserResponse
                total_rewards: 50.0,        // TODO: Add total_rewards to UserResponse
                created_at: user_response.created_at,
            }).collect();

            let pagination = PaginationResponse {
                page: search_query.page.unwrap_or(0),
                page_size: search_query.page_size.unwrap_or(20),
                total_count: users.len() as u64,
                total_pages: 1,
                has_next_page: false,
                has_previous_page: false,
            };

            let response = UserListResponse {
                users,
                pagination,
            };

            Ok(Json(ApiResponse {
                success: true,
                data: Some(response),
                message: None,
                errors: None,
            }))
        }
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some(e.to_string()),
            errors: None,
        })),
    }
}

/// POST /api/v1/users/{user_id}/follow
/// Follow/unfollow user
#[axum::debug_handler]
pub async fn follow_user(
    State(user_service): State<UserAppService>,
    Path(followee_id): Path<Uuid>,
    Json(request): Json<FollowUserRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // TODO: Extract follower_id from JWT token
    let follower_id = Uuid::new_v4(); // Mock for now
    
    let command = FollowUserCommand {
        follower_id,
        followee_id,
        follow: request.follow,
    };

    match user_service.handle_follow_user(command).await {
        Ok(_) => {
            let action = if request.follow { "seguido" } else { "dejado de seguir" };
            Ok(Json(ApiResponse {
                success: true,
                data: None,
                message: Some(format!("Usuario {} exitosamente", action)),
                errors: None,
            }))
        }
        Err(e) => Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some(e.to_string()),
            errors: None,
        })),
    }
}

/// POST /api/v1/users/{user_id}/change-password
/// Change user password
#[axum::debug_handler]
pub async fn change_password(
    State(_user_service): State<UserAppService>,
    Path(_user_id): Path<Uuid>,
    Json(request): Json<ChangePasswordRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // Validate passwords match
    if request.new_password != request.confirm_new_password {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Las nuevas contraseñas no coinciden".to_string()),
            errors: None,
        }));
    }

    // TODO: Implement password change logic
    Ok(Json(ApiResponse {
        success: true,
        data: None,
        message: Some("Contraseña cambiada exitosamente".to_string()),
        errors: None,
    }))
}

/// POST /api/v1/users/{user_id}/link-wallet
/// Link wallet to user account
#[axum::debug_handler]
pub async fn link_wallet(
    State(_user_service): State<UserAppService>,
    Path(_user_id): Path<Uuid>,
    Json(_request): Json<LinkWalletRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // TODO: Implement wallet linking logic with signature verification
    Ok(Json(ApiResponse {
        success: true,
        data: None,
        message: Some("Wallet vinculada exitosamente".to_string()),
        errors: None,
    }))
}

/// DELETE /api/v1/users/{user_id}
/// Delete user account
#[axum::debug_handler]
pub async fn delete_user(
    State(_user_service): State<UserAppService>,
    Path(_user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // TODO: Implement user deletion logic
    Ok(Json(ApiResponse {
        success: true,
        data: None,
        message: Some("Usuario eliminado exitosamente".to_string()),
        errors: None,
    }))
}

/// GET /api/v1/users/{user_id}/followers
/// Get user followers
#[axum::debug_handler]
pub async fn get_user_followers(
    State(_user_service): State<UserAppService>,
    Path(_user_id): Path<Uuid>,
    Query(_query): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<UserListResponse>>, StatusCode> {
    let mock_followers = vec![
        UserSummaryResponse {
            id: Uuid::new_v4(),
            username: "follower1".to_string(),
            display_name: Some("Seguidor 1".to_string()),
            avatar_url: Some("https://example.com/avatar1.jpg".to_string()),
            tier: "bronze".to_string(),
            role: "user".to_string(),
            is_verified: false,
            is_active: true,
            tier_points: 600,
            total_rewards: 30.0,
            created_at: Utc::now(),
        },
    ];

    let pagination = PaginationResponse {
        page: 0,
        page_size: 20,
        total_count: 1,
        total_pages: 1,
        has_next_page: false,
        has_previous_page: false,
    };

    let response = UserListResponse {
        users: mock_followers,
        pagination,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response),
        message: None,
        errors: None,
    }))
}

/// GET /api/v1/users/{user_id}/following
/// Get users that the user is following
#[axum::debug_handler]
pub async fn get_user_following(
    State(_user_service): State<UserAppService>,
    Path(_user_id): Path<Uuid>,
    Query(_query): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<UserListResponse>>, StatusCode> {
    let mock_following = vec![
        UserSummaryResponse {
            id: Uuid::new_v4(),
            username: "following1".to_string(),
            display_name: Some("Siguiendo 1".to_string()),
            avatar_url: Some("https://example.com/avatar1.jpg".to_string()),
            tier: "silver".to_string(),
            role: "artist".to_string(),
            is_verified: true,
            is_active: true,
            tier_points: 2000,
            total_rewards: 200.0,
            created_at: Utc::now(),
        },
    ];

    let pagination = PaginationResponse {
        page: 0,
        page_size: 20,
        total_count: 1,
        total_pages: 1,
        has_next_page: false,
        has_previous_page: false,
    };

    let response = UserListResponse {
        users: mock_following,
        pagination,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response),
        message: None,
        errors: None,
    }))
}

/// GET /api/v1/users/analytics
/// Get user analytics (admin only)
#[axum::debug_handler]
pub async fn get_user_analytics(
    State(_user_service): State<UserAppService>,
    Query(_query): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    // TODO: Implement analytics logic
    let mock_analytics = serde_json::json!({
        "total_users": 10500,
        "active_users": 8750,
        "verified_users": 3200,
        "users_with_wallets": 4800,
        "tier_distribution": {
            "bronze": 6000,
            "silver": 3000,
            "gold": 1200,
            "diamond": 300
        },
        "role_distribution": {
            "user": 9800,
            "artist": 650,
            "admin": 50
        },
        "registration_stats": [
            {
                "period": "2024-01-01T00:00:00Z",
                "count": 120,
                "verified_count": 45
            }
        ]
    });

    Ok(Json(ApiResponse {
        success: true,
        data: Some(mock_analytics),
        message: None,
        errors: None,
    }))
}

/// Create User Context Routes
/// 
/// Creates a complete Router for all user-related endpoints
pub fn create_user_routes() -> Router<UserAppService> {
    Router::new()
        // Authentication & Registration
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        
        // User Profile Management
        .route("/:user_id", get(get_user_profile))
        .route("/:user_id", put(update_user_profile))
        .route("/:user_id", delete(delete_user))
        
        // User Statistics & Analytics
        .route("/:user_id/stats", get(get_user_stats))
        
        // User Search & Discovery
        .route("/search", get(search_users))
        
        // Social Features
        .route("/:user_id/follow", post(follow_user))
        .route("/:user_id/followers", get(get_user_followers))
        .route("/:user_id/following", get(get_user_following))
        
        // Account Management
        .route("/:user_id/change-password", post(change_password))
        .route("/:user_id/link-wallet", post(link_wallet))
        
        // Admin Analytics
        .route("/analytics", get(get_user_analytics))
}

// =============================================================================
// FUNCIONES FALTANTES PARA COMPATIBILIDAD CON EL ROUTER
// =============================================================================

/// GET /api/v1/users - List users
pub async fn get_users(
    State(_user_service): State<UserAppService>,
) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
    // TODO: Implement actual user listing logic
    let users = vec![
        serde_json::json!({
            "user_id": Uuid::new_v4(),
            "username": "demo_user",
            "email": "demo@example.com",
            "display_name": "Demo User",
            "created_at": Utc::now()
        })
    ];
    
    Ok(ResponseJson(serde_json::json!({
        "users": users,
        "total": users.len()
    })))
}

/// GET /api/v1/users/:id - Get user by ID
pub async fn get_user(
    State(_user_service): State<UserAppService>,
    Path(user_id): Path<Uuid>,
) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
    // TODO: Implement actual user retrieval logic
    let user = serde_json::json!({
        "user_id": user_id,
        "username": "demo_user",
        "email": "demo@example.com",
        "display_name": "Demo User",
        "created_at": Utc::now()
    });
    
    Ok(ResponseJson(user))
}

/// POST /api/v1/users/:id/unfollow - Unfollow user
pub async fn unfollow_user(
    State(_user_service): State<UserAppService>,
    Path(user_id): Path<Uuid>,
) -> Result<ResponseJson<serde_json::Value>, (StatusCode, ResponseJson<serde_json::Value>)> {
    // TODO: Implement actual unfollow logic
    Ok(ResponseJson(serde_json::json!({
        "message": "User unfollowed successfully",
        "user_id": user_id
    })))
} 