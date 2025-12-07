//! OpenAPI Path Definitions
//! 
//! OpenAPI endpoint documentation using utoipa

use utoipa::path;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::openapi::{
    User, CreateUserRequest, LoginRequest, LoginResponse, 
    Song, CreateSongRequest, SongListResponse,
    Album, AlbumListResponse, CreateAlbumRequest, UpdateAlbumRequest,
    Playlist, PlaylistListResponse, CreatePlaylistRequest, AddSongToPlaylistRequest,
    Campaign, ApiError, ApiResponse
};

// =============================================================================
// USER ENDPOINTS
// =============================================================================

/// Register a new user account
#[utoipa::path(
    post,
    path = "/api/v1/users/register",
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
    path = "/api/v1/users/login",
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
    path = "/api/v1/users/refresh",
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
    path = "/api/v1/users/{user_id}",
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
    path = "/api/v1/music/songs",
    request_body = CreateSongRequest,
    responses(
        (status = 201, description = "Song created successfully", body = ApiResponse<Song>),
        (status = 400, description = "Invalid request data", body = ApiError)
    ),
    tag = "music"
)]
pub async fn _create_song_doc() {}

/// List songs with pagination and optional filters
#[utoipa::path(
    get,
    path = "/api/v1/music/songs",
    params(
        ("q" = Option<String>, Query, description = "Search query for song title"),
        ("genre" = Option<String>, Query, description = "Filter by genre"),
        ("artist_id" = Option<Uuid>, Query, description = "Filter by artist ID"),
        ("limit" = Option<usize>, Query, description = "Number of songs per page (default: 20, max: 100)"),
        ("offset" = Option<usize>, Query, description = "Number of songs to skip (default: 0)")
    ),
    responses(
        (status = 200, description = "List of songs", body = SongListResponse),
        (status = 400, description = "Invalid filter parameters", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "music"
)]
pub async fn _get_songs_doc() {}

/// Get song by ID
#[utoipa::path(
    get,
    path = "/api/v1/music/songs/{song_id}",
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

/// Update song by ID
#[utoipa::path(
    put,
    path = "/api/v1/music/songs/{song_id}",
    params(
        ("song_id" = Uuid, Path, description = "Song ID")
    ),
    request_body = CreateSongRequest,
    responses(
        (status = 200, description = "Song updated successfully", body = ApiResponse<Song>),
        (status = 404, description = "Song not found", body = ApiError),
        (status = 400, description = "Invalid request data", body = ApiError)
    ),
    tag = "music"
)]
pub async fn _update_song_doc() {}

/// Delete song by ID
#[utoipa::path(
    delete,
    path = "/api/v1/music/songs/{song_id}",
    params(
        ("song_id" = Uuid, Path, description = "Song ID")
    ),
    responses(
        (status = 200, description = "Song deleted successfully"),
        (status = 404, description = "Song not found", body = ApiError)
    ),
    tag = "music"
)]
pub async fn _delete_song_doc() {}

/// List albums with pagination
#[utoipa::path(
    get,
    path = "/api/v1/music/albums",
    params(
        ("limit" = Option<usize>, Query, description = "Number of albums per page (default: 20, max: 100)"),
        ("offset" = Option<usize>, Query, description = "Number of albums to skip (default: 0)")
    ),
    responses(
        (status = 200, description = "List of albums", body = AlbumListResponse),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "music"
)]
pub async fn _get_albums_doc() {}

/// Create a new album
#[utoipa::path(
    post,
    path = "/api/v1/music/albums",
    request_body = CreateAlbumRequest,
    responses(
        (status = 201, description = "Album created successfully", body = ApiResponse<Album>),
        (status = 400, description = "Invalid request data", body = ApiError)
    ),
    tag = "music"
)]
pub async fn _create_album_doc() {}

/// Get album by ID
#[utoipa::path(
    get,
    path = "/api/v1/music/albums/{album_id}",
    params(
        ("album_id" = Uuid, Path, description = "Album ID")
    ),
    responses(
        (status = 200, description = "Album details", body = ApiResponse<Album>),
        (status = 404, description = "Album not found", body = ApiError)
    ),
    tag = "music"
)]
pub async fn _get_album_doc() {}

/// Update album by ID
#[utoipa::path(
    put,
    path = "/api/v1/music/albums/{album_id}",
    params(
        ("album_id" = Uuid, Path, description = "Album ID")
    ),
    request_body = UpdateAlbumRequest,
    responses(
        (status = 200, description = "Album updated successfully", body = ApiResponse<Album>),
        (status = 400, description = "Invalid request data", body = ApiError),
        (status = 404, description = "Album not found", body = ApiError)
    ),
    tag = "music"
)]
pub async fn _update_album_doc() {}

/// Delete album by ID
#[utoipa::path(
    delete,
    path = "/api/v1/music/albums/{album_id}",
    params(
        ("album_id" = Uuid, Path, description = "Album ID")
    ),
    responses(
        (status = 200, description = "Album deleted successfully"),
        (status = 404, description = "Album not found", body = ApiError)
    ),
    tag = "music"
)]
pub async fn _delete_album_doc() {}

/// List playlists with pagination
#[utoipa::path(
    get,
    path = "/api/v1/music/playlists",
    params(
        ("limit" = Option<usize>, Query, description = "Number of playlists per page (default: 20, max: 100)"),
        ("offset" = Option<usize>, Query, description = "Number of playlists to skip (default: 0)")
    ),
    responses(
        (status = 200, description = "List of playlists", body = PlaylistListResponse),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tag = "music"
)]
pub async fn _get_playlists_doc() {}

/// Create a new playlist
#[utoipa::path(
    post,
    path = "/api/v1/music/playlists",
    request_body = CreatePlaylistRequest,
    responses(
        (status = 201, description = "Playlist created successfully", body = ApiResponse<Playlist>),
        (status = 400, description = "Invalid request data", body = ApiError)
    ),
    tag = "music"
)]
pub async fn _create_playlist_doc() {}

/// Get playlist by ID
#[utoipa::path(
    get,
    path = "/api/v1/music/playlists/{playlist_id}",
    params(
        ("playlist_id" = Uuid, Path, description = "Playlist ID")
    ),
    responses(
        (status = 200, description = "Playlist details", body = ApiResponse<Playlist>),
        (status = 404, description = "Playlist not found", body = ApiError)
    ),
    tag = "music"
)]
pub async fn _get_playlist_doc() {}

/// Add song to playlist
#[utoipa::path(
    post,
    path = "/api/v1/music/playlists/{playlist_id}/songs",
    params(
        ("playlist_id" = Uuid, Path, description = "Playlist ID")
    ),
    request_body = AddSongToPlaylistRequest,
    responses(
        (status = 200, description = "Song added to playlist successfully"),
        (status = 400, description = "Invalid request data", body = ApiError),
        (status = 403, description = "Forbidden - only playlist owner can add songs", body = ApiError),
        (status = 404, description = "Playlist not found", body = ApiError),
        (status = 401, description = "Unauthorized - authentication required", body = ApiError)
    ),
    tag = "music",
    security(
        ("bearer" = [])
    )
)]
pub async fn _add_song_to_playlist_doc() {}

/// Remove song from playlist
#[utoipa::path(
    delete,
    path = "/api/v1/music/playlists/{playlist_id}/songs/{song_id}",
    params(
        ("playlist_id" = Uuid, Path, description = "Playlist ID"),
        ("song_id" = Uuid, Path, description = "Song ID")
    ),
    responses(
        (status = 200, description = "Song removed from playlist successfully"),
        (status = 403, description = "Forbidden - only playlist owner can remove songs", body = ApiError),
        (status = 404, description = "Playlist or song not found", body = ApiError),
        (status = 401, description = "Unauthorized - authentication required", body = ApiError)
    ),
    tag = "music",
    security(
        ("bearer" = [])
    )
)]
pub async fn _remove_song_from_playlist_doc() {}

// =============================================================================
// CAMPAIGN ENDPOINTS
// =============================================================================

/// Create a new campaign
#[utoipa::path(
    post,
    path = "/api/v1/campaigns",
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

