//! OpenAPI Path Definitions
//! 
//! Documentación de endpoints para OpenAPI usando utoipa

use utoipa::path;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::openapi::{
    User, CreateUserRequest, LoginRequest, LoginResponse, 
    Song, CreateSongRequest, Campaign, ApiError, ApiResponse
};

// =============================================================================
// USER ENDPOINTS
// =============================================================================

/// Register a new user account
#[utoipa::path(
    post,
    path = "/users/register",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User registered successfully", body = ApiResponse<User>),
        (status = 400, description = "Invalid request data", body = ApiError),
        (status = 409, description = "User already exists", body = ApiError)
    ),
    tag = "users"
)]
pub async fn _register_user_doc() {}

/// Authenticate user and get JWT tokens
#[utoipa::path(
    post,
    path = "/users/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = ApiResponse<LoginResponse>),
        (status = 401, description = "Invalid credentials", body = ApiError)
    ),
    tag = "users"
)]
pub async fn _login_user_doc() {}

/// Refresh access token using refresh token
#[utoipa::path(
    post,
    path = "/users/refresh",
    request_body(content = RefreshTokenRequest, description = "Refresh token request"),
    responses(
        (status = 200, description = "Token refreshed successfully", body = RefreshTokenResponse),
        (status = 401, description = "Invalid refresh token", body = ApiError)
    ),
    tag = "users"
)]
pub async fn _refresh_token_doc() {}

/// Get user profile by ID
#[utoipa::path(
    get,
    path = "/users/{user_id}",
    params(
        ("user_id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User profile", body = ApiResponse<User>),
        (status = 404, description = "User not found", body = ApiError)
    ),
    tag = "users"
)]
pub async fn _get_user_profile_doc() {}

// =============================================================================
// MUSIC ENDPOINTS
// =============================================================================

/// Create a new song
#[utoipa::path(
    post,
    path = "/music/songs",
    request_body = CreateSongRequest,
    responses(
        (status = 201, description = "Song created successfully", body = ApiResponse<Song>),
        (status = 400, description = "Invalid request data", body = ApiError)
    ),
    tag = "music"
)]
pub async fn _create_song_doc() {}

/// Get song by ID
#[utoipa::path(
    get,
    path = "/music/songs/{song_id}",
    params(
        ("song_id" = Uuid, Path, description = "Song ID")
    ),
    responses(
        (status = 200, description = "Song details", body = ApiResponse<Song>),
        (status = 404, description = "Song not found", body = ApiError)
    ),
    tag = "music"
)]
pub async fn _get_song_doc() {}

// =============================================================================
// CAMPAIGN ENDPOINTS
// =============================================================================

/// Create a new campaign
#[utoipa::path(
    post,
    path = "/campaigns",
    request_body = Campaign,
    responses(
        (status = 201, description = "Campaign created successfully", body = ApiResponse<Campaign>),
        (status = 400, description = "Invalid request data", body = ApiError)
    ),
    tag = "campaigns"
)]
pub async fn _create_campaign_doc() {}

// =============================================================================
// ADDITIONAL TYPES FOR DOCUMENTATION
// =============================================================================

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

