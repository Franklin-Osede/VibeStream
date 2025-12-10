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
use crate::shared::infrastructure::auth::{JwtService, PasswordService, AuthenticatedUser};
use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::user::domain::repository::UserRepository;
use crate::shared::infrastructure::clients::facial_recognition_client::VerifyFaceResponse;

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

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
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

#[derive(Debug, Deserialize)]
pub struct VerifyBiometricsRequest {
    pub image: String, // base64
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

/// Register a new user account
#[utoipa::path(
    post,
    path = "/api/v1/users/register",
    request_body = RegisterUserRequest,
    responses(
        (status = 201, description = "User registered successfully", body = ApiResponse<RegisterUserResponse>),
        (status = 400, description = "Invalid request data", body = ApiResponse<serde_json::Value>),
        (status = 409, description = "User already exists", body = ApiResponse<serde_json::Value>)
    ),
    tag = "users"
)]
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
            // Generate real JWT token
            let jwt_secret = crate::shared::infrastructure::auth::get_jwt_secret()
                .map_err(|e| {
                    eprintln!("JWT configuration error: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
            let jwt_service = JwtService::new(&jwt_secret)
                .map_err(|e| {
                    eprintln!("Error creating JWT service: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
            
            let token_pair = jwt_service.generate_token_pair(
                result.id,
                &result.username,
                &result.email,
                "user", // Default role
                "free", // Default tier
            ).map_err(|e| {
                eprintln!("Error generating token pair: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            let response = RegisterUserResponse {
                user_id: result.id,
                username: result.username,
                email: result.email,
                display_name: result.display_name,
                token: token_pair.access_token,
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

/// Authenticate user and get JWT tokens
#[utoipa::path(
    post,
    path = "/api/v1/users/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = ApiResponse<LoginResponse>),
        (status = 401, description = "Invalid credentials", body = ApiResponse<serde_json::Value>)
    ),
    tag = "users"
)]
#[axum::debug_handler]
pub async fn login_user(
    State(user_service): State<UserAppService>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, StatusCode> {
    // Find user by email or username
    let user = if request.credential.contains('@') {
        // Search by email
        user_service.find_user_by_email(&request.credential).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        // Search by username
        user_service.find_user_by_username(&request.credential).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    let user = user.ok_or(StatusCode::UNAUTHORIZED)?;
    
    // Verify password
    let is_valid = PasswordService::verify_password(&request.password, user.password_hash.value())
        .map_err(|e| {
            eprintln!("Error verifying password: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    if !is_valid {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Credenciales inválidas".to_string()),
            errors: None,
        }));
    }

    // Generate real JWT tokens
    let jwt_secret = crate::shared::infrastructure::auth::get_jwt_secret()
        .map_err(|e| {
            eprintln!("JWT configuration error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let jwt_service = JwtService::new(&jwt_secret)
        .map_err(|e| {
            eprintln!("Error creating JWT service: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    let role_str = format!("{}", user.role);
    let tier_str = format!("{}", user.tier);
    
    let token_pair = jwt_service.generate_token_pair(
        user.id.value(),
        user.username.value(),
        user.email.value(),
        &role_str,
        &tier_str,
    ).map_err(|e| {
        eprintln!("Error generating token pair: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let response = LoginResponse {
        user_id: user.id.value(),
        username: user.username.value().to_string(),
        email: user.email.value().to_string(),
        display_name: None, // TODO: Add display_name field to User entity
        token: token_pair.access_token,
        refresh_token: Some(token_pair.refresh_token),
        expires_in: token_pair.expires_in,
        user_role: role_str,
        tier: tier_str,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response),
        message: Some("Login exitoso".to_string()),
        errors: None,
    }))
}

/// Refresh access token using refresh token
#[utoipa::path(
    post,
    path = "/api/v1/users/refresh",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = ApiResponse<RefreshTokenResponse>),
        (status = 401, description = "Invalid refresh token", body = ApiResponse<serde_json::Value>)
    ),
    tag = "users"
)]
#[axum::debug_handler]
pub async fn refresh_token(
    Json(request): Json<RefreshTokenRequest>,
) -> Result<Json<ApiResponse<RefreshTokenResponse>>, StatusCode> {
    let jwt_secret = crate::shared::infrastructure::auth::get_jwt_secret()
        .map_err(|e| {
            eprintln!("JWT configuration error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let jwt_service = JwtService::new(&jwt_secret)
        .map_err(|e| {
            eprintln!("Error creating JWT service: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    // Validate refresh token
    let claims = jwt_service.validate_refresh_token(&request.refresh_token)
        .map_err(|e| {
            eprintln!("Error validating refresh token: {}", e);
            StatusCode::UNAUTHORIZED
        })?;
    
    // Parse user ID from claims
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    // Generate new token pair
    let token_pair = jwt_service.generate_token_pair(
        user_id,
        &claims.username,
        &claims.email,
        &claims.role,
        &claims.tier,
    ).map_err(|e| {
        eprintln!("Error generating token pair: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let response = RefreshTokenResponse {
        access_token: token_pair.access_token,
        refresh_token: token_pair.refresh_token,
        expires_in: token_pair.expires_in,
    };
    
    Ok(Json(ApiResponse {
        success: true,
        data: Some(response),
        message: Some("Token renovado exitosamente".to_string()),
        errors: None,
    }))
}

/// Get user profile by ID
/// 
/// Shows more information if you're viewing your own profile.
#[utoipa::path(
    get,
    path = "/api/v1/users/{user_id}",
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User profile", body = ApiResponse<UserProfileResponse>),
        (status = 404, description = "User not found", body = ApiResponse<serde_json::Value>)
    ),
    tag = "users"
)]
#[axum::debug_handler]
pub async fn get_user_profile(
    // Authenticated user (required because this is in protected routes)
    AuthenticatedUser { user_id: authenticated_user_id, .. }: AuthenticatedUser,
    State(user_service): State<UserAppService>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<UserProfileResponse>>, StatusCode> {
    // Check if user is viewing their own profile (for showing more info)
    let is_own_profile = authenticated_user_id == user_id;
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
                tier: user_response.tier,
                role: user_response.role, 
                is_verified: user_response.is_verified,
                is_active: user_response.is_active,
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
    AuthenticatedUser { user_id: authenticated_user_id, .. }: AuthenticatedUser,
    State(user_service): State<UserAppService>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // Validar que el usuario solo puede editar su propio perfil
    if authenticated_user_id != user_id {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Solo puedes editar tu propio perfil".to_string()),
            errors: None,
        }));
    }
    
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
/// 
/// Note: Users can only view their own stats unless they are admin.
#[axum::debug_handler]
pub async fn get_user_stats(
    AuthenticatedUser { user_id: authenticated_user_id, role, .. }: AuthenticatedUser,
    State(user_service): State<UserAppService>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<UserStatsResponse>>, StatusCode> {
    // Validar que el usuario solo puede ver sus propias estadísticas (o admin puede ver cualquiera)
    if authenticated_user_id != user_id && role != "admin" {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Solo puedes ver tus propias estadísticas".to_string()),
            errors: None,
        }));
    }
    let user_id_vo = crate::bounded_contexts::user::domain::value_objects::UserId::from_uuid(user_id);
    let stats = user_service.repository.get_user_stats(&user_id_vo).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let response = UserStatsResponse {
        user_id,
        total_listening_time_minutes: stats.total_listening_time_minutes,
        total_listening_hours: stats.total_listening_time_minutes as f64 / 60.0,
        total_songs_listened: stats.total_songs_listened,
        total_rewards_earned: stats.total_rewards_earned,
        current_listening_streak: stats.current_listening_streak,
        longest_listening_streak: stats.longest_listening_streak,
        total_investments: stats.total_investments,
        investment_count: stats.investment_count,
        nfts_owned: stats.nfts_owned,
        campaigns_participated: stats.campaigns_participated,
        tier_points: stats.tier_points,
        achievements_unlocked: stats.achievements_unlocked,
        created_at: stats.created_at,
        updated_at: stats.updated_at,
    };

    Ok(Json(ApiResponse {
        success: true,
        data: Some(response),
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
    AuthenticatedUser { user_id: follower_id, .. }: AuthenticatedUser,
    State(user_service): State<UserAppService>,
    Path(followee_id): Path<Uuid>,
    Json(request): Json<FollowUserRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // Validar que no se puede seguir a sí mismo
    if follower_id == followee_id {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("No puedes seguirte a ti mismo".to_string()),
            errors: None,
        }));
    }
    
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
    AuthenticatedUser { user_id, .. }: AuthenticatedUser,
    State(user_service): State<UserAppService>,
    Path(requested_user_id): Path<Uuid>,
    Json(request): Json<ChangePasswordRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // Validar que el usuario solo puede cambiar su propia contraseña
    if user_id != requested_user_id {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Solo puedes cambiar tu propia contraseña".to_string()),
            errors: None,
        }));
    }

    // Validar que las contraseñas coinciden
    if request.new_password != request.confirm_new_password {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Las nuevas contraseñas no coinciden".to_string()),
            errors: None,
        }));
    }

    // Validar longitud mínima de contraseña
    if request.new_password.len() < 8 {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("La contraseña debe tener al menos 8 caracteres".to_string()),
            errors: None,
        }));
    }

    // Buscar usuario
    let user_id_vo = crate::bounded_contexts::user::domain::value_objects::UserId::from_uuid(user_id);
    let user_aggregate = user_service.repository.find_by_id(&user_id_vo).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Verificar contraseña actual
    let is_valid = PasswordService::verify_password(
        &request.current_password,
        user_aggregate.user.password_hash.value()
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !is_valid {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("La contraseña actual es incorrecta".to_string()),
            errors: None,
        }));
    }

    // Hashear nueva contraseña
    let new_password_hash = PasswordService::hash_password(&request.new_password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Actualizar contraseña en el agregado
    let password_hash_vo = crate::bounded_contexts::user::domain::value_objects::PasswordHash::new(new_password_hash);
    let mut updated_aggregate = user_aggregate;
    updated_aggregate.user.update_password(password_hash_vo);

    // Guardar cambios
    user_service.repository.update(&updated_aggregate).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: None,
        message: Some("Contraseña cambiada exitosamente".to_string()),
        errors: None,
    }))
}

/// POST /api/v1/users/biometrics/verify
/// Verify user biometrics (face)
#[utoipa::path(
    post,
    path = "/api/v1/users/biometrics/verify",
    request_body = VerifyBiometricsRequest,
    responses(
        (status = 200, description = "Verification successful", body = ApiResponse<VerifyFaceResponse>),
        (status = 503, description = "Service unavailable", body = ApiResponse<serde_json::Value>)
    ),
    tag = "users"
)]
#[axum::debug_handler]
pub async fn verify_biometrics(
    AuthenticatedUser { user_id, .. }: AuthenticatedUser,
    State(user_service): State<UserAppService>,
    Json(request): Json<VerifyBiometricsRequest>,
) -> Result<Json<ApiResponse<VerifyFaceResponse>>, StatusCode> {
    let client = user_service.facial_client.as_ref()
        .ok_or_else(|| {
             eprintln!("Facial client not configured");
             StatusCode::SERVICE_UNAVAILABLE
        })?;

    let result = client.verify_face(user_id, request.image).await
        .map_err(|e| {
            eprintln!("Error verifying face: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(result),
        message: Some("Biometric verification completed".to_string()),
        errors: None,
    }))
}

/// POST /api/v1/users/{user_id}/link-wallet
/// Link wallet to user account
#[axum::debug_handler]
pub async fn link_wallet(
    AuthenticatedUser { user_id, .. }: AuthenticatedUser,
    State(user_service): State<UserAppService>,
    Path(requested_user_id): Path<Uuid>,
    Json(request): Json<LinkWalletRequest>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // Validar que el usuario solo puede vincular su propia wallet
    if user_id != requested_user_id {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Solo puedes vincular tu propia wallet".to_string()),
            errors: None,
        }));
    }

    // Validar formato de wallet address
    let wallet_address_vo = crate::bounded_contexts::user::domain::value_objects::WalletAddress::new(
        request.wallet_address.clone()
    ).map_err(|e| {
        StatusCode::BAD_REQUEST
    })?;

    // TODO: Verificar firma de la wallet
    // Por ahora solo validamos el formato, pero en producción deberíamos:
    // 1. Verificar que la firma corresponde al mensaje
    // 2. Verificar que la wallet address corresponde a la firma
    // 3. Verificar que el mensaje es el esperado
    
    // Buscar usuario
    let user_id_vo = crate::bounded_contexts::user::domain::value_objects::UserId::from_uuid(user_id);
    let mut user_aggregate = user_service.repository.find_by_id(&user_id_vo).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Vincular wallet
    user_aggregate.link_wallet(wallet_address_vo)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Guardar cambios
    user_service.repository.update(&user_aggregate).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: None,
        message: Some("Wallet vinculada exitosamente".to_string()),
        errors: None,
    }))
}

/// DELETE /api/v1/users/{user_id}
/// Delete user account (soft delete)
#[axum::debug_handler]
pub async fn delete_user(
    AuthenticatedUser { user_id, role, .. }: AuthenticatedUser,
    State(user_service): State<UserAppService>,
    Path(requested_user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    // Validar que el usuario solo puede eliminar su propia cuenta (o admin)
    if user_id != requested_user_id && role != "admin" {
        return Ok(Json(ApiResponse {
            success: false,
            data: None,
            message: Some("Solo puedes eliminar tu propia cuenta".to_string()),
            errors: None,
        }));
    }

    // Buscar usuario
    let user_id_vo = crate::bounded_contexts::user::domain::value_objects::UserId::from_uuid(requested_user_id);
    let mut user_aggregate = user_service.repository.find_by_id(&user_id_vo).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Soft delete: desactivar usuario en lugar de eliminar
    user_aggregate.user.deactivate();

    // Guardar cambios
    user_service.repository.update(&user_aggregate).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: None,
        message: Some("Cuenta eliminada exitosamente".to_string()),
        errors: None,
    }))
}

/// GET /api/v1/users/{user_id}/followers
/// Get user followers
#[axum::debug_handler]
pub async fn get_user_followers(
    State(user_service): State<UserAppService>,
    Path(user_id): Path<Uuid>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<UserListResponse>>, StatusCode> {
    let page = query.get("page")
        .and_then(|p| p.parse::<u32>().ok())
        .unwrap_or(0);
    let page_size = query.get("page_size")
        .and_then(|p| p.parse::<u32>().ok())
        .unwrap_or(20);

    let user_id_vo = crate::bounded_contexts::user::domain::value_objects::UserId::from_uuid(user_id);
    let followers_summaries = user_service.repository.get_followers(&user_id_vo, page, page_size).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let users: Vec<UserSummaryResponse> = followers_summaries.into_iter().map(|summary| {
        UserSummaryResponse {
            id: summary.id.value(),
            username: summary.username.value().to_string(),
            display_name: summary.display_name.clone(),
            avatar_url: summary.avatar_url.clone(),
            tier: format!("{}", summary.tier),
            role: format!("{}", summary.role),
            is_verified: summary.is_verified,
            is_active: summary.is_active,
            tier_points: summary.tier_points.max(0) as u32,
            total_rewards: summary.total_rewards,
            created_at: summary.created_at,
        }
    }).collect();

    let total_count = users.len() as u64; // TODO: Get actual count from repository
    let total_pages = (total_count as f64 / page_size as f64).ceil() as u32;

    let pagination = PaginationResponse {
        page,
        page_size,
        total_count,
        total_pages,
        has_next_page: page < total_pages.saturating_sub(1),
        has_previous_page: page > 0,
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

/// GET /api/v1/users/{user_id}/following
/// Get users that the user is following
#[axum::debug_handler]
pub async fn get_user_following(
    State(user_service): State<UserAppService>,
    Path(user_id): Path<Uuid>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<UserListResponse>>, StatusCode> {
    let page = query.get("page")
        .and_then(|p| p.parse::<u32>().ok())
        .unwrap_or(0);
    let page_size = query.get("page_size")
        .and_then(|p| p.parse::<u32>().ok())
        .unwrap_or(20);

    let user_id_vo = crate::bounded_contexts::user::domain::value_objects::UserId::from_uuid(user_id);
    let following_summaries = user_service.repository.get_following(&user_id_vo, page, page_size).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let users: Vec<UserSummaryResponse> = following_summaries.into_iter().map(|summary| {
        UserSummaryResponse {
            id: summary.id.value(),
            username: summary.username.value().to_string(),
            display_name: summary.display_name.clone(),
            avatar_url: summary.avatar_url.clone(),
            tier: format!("{}", summary.tier),
            role: format!("{}", summary.role),
            is_verified: summary.is_verified,
            is_active: summary.is_active,
            tier_points: summary.tier_points.max(0) as u32,
            total_rewards: summary.total_rewards,
            created_at: summary.created_at,
        }
    }).collect();

    let total_count = users.len() as u64; // TODO: Get actual count from repository
    let total_pages = (total_count as f64 / page_size as f64).ceil() as u32;

    let pagination = PaginationResponse {
        page,
        page_size,
        total_count,
        total_pages,
        has_next_page: page < total_pages.saturating_sub(1),
        has_previous_page: page > 0,
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
        .route("/refresh", post(refresh_token))
        
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