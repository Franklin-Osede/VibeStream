use axum::{
    extract::{Query, Path},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::shared::infrastructure::app_state::AppState;

// DTOs for API requests/responses
#[derive(Debug, Deserialize)]
pub struct UploadSongRequest {
    pub title: String,
    pub artist_id: String,
    pub duration_seconds: u32,
    pub genre: String,
    pub royalty_percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct UploadSongResponse {
    pub song_id: String,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct DiscoverMusicRequest {
    pub filter: Option<String>,
    pub genre: Option<String>,
    pub artist_id: Option<String>,
    pub search_query: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Clone)]
pub struct SongResponse {
    pub id: String,
    pub title: String,
    pub artist_id: String,
    pub duration_seconds: u32,
    pub genre: String,
    pub listen_count: u64,
    pub revenue_generated: f64,
    pub is_popular: bool,
    pub is_trending: bool,
}

#[derive(Debug, Serialize)]
pub struct DiscoverMusicResponse {
    pub songs: Vec<SongResponse>,
    pub total_count: usize,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
}

pub fn create_music_routes() -> Router<AppState> {
    Router::new()
        .route("/songs", post(upload_song))
        .route("/songs/discover", get(discover_music))
        .route("/songs/trending", get(get_trending))
        .route("/songs/recommendations/:user_id", get(get_recommendations))
        .route("/health", get(health_check))
}

// POST /songs - Upload a new song
async fn upload_song(
    Json(request): Json<UploadSongRequest>,
) -> Result<Json<UploadSongResponse>, StatusCode> {
    // Simple demo implementation
    let song_id = Uuid::new_v4().to_string();
    
    Ok(Json(UploadSongResponse {
        song_id,
        success: true,
        message: format!("Song '{}' uploaded successfully! (Demo mode)", request.title),
    }))
}

// GET /songs/discover - Discover music with filters
async fn discover_music(
    Query(params): Query<DiscoverMusicRequest>,
) -> Result<Json<DiscoverMusicResponse>, StatusCode> {
    // Demo data
    let demo_songs = get_demo_songs();
    
    // Apply simple filtering
    let filtered_songs: Vec<SongResponse> = match params.filter.as_deref() {
        Some("trending") => demo_songs.into_iter().filter(|s| s.is_trending).collect(),
        Some("popular") => demo_songs.into_iter().filter(|s| s.is_popular).collect(),
        Some("by_genre") => {
            if let Some(genre) = params.genre {
                demo_songs.into_iter().filter(|s| s.genre.to_lowercase() == genre.to_lowercase()).collect()
            } else {
                demo_songs
            }
        }
        Some("search") => {
            if let Some(query) = params.search_query {
                demo_songs.into_iter().filter(|s| 
                    s.title.to_lowercase().contains(&query.to_lowercase())
                ).collect()
            } else {
                demo_songs
            }
        }
        _ => demo_songs,
    };
    
    // Apply limit
    let limited_songs: Vec<SongResponse> = if let Some(limit) = params.limit {
        filtered_songs.into_iter().take(limit).collect()
    } else {
        filtered_songs
    };
    
    Ok(Json(DiscoverMusicResponse {
        total_count: limited_songs.len(),
        songs: limited_songs,
    }))
}

// GET /songs/trending - Get trending songs
async fn get_trending() -> Result<Json<DiscoverMusicResponse>, StatusCode> {
    let demo_songs = get_demo_songs();
    let trending_songs: Vec<SongResponse> = demo_songs.into_iter()
        .filter(|s| s.is_trending)
        .take(10)
        .collect();

    Ok(Json(DiscoverMusicResponse {
        songs: trending_songs.clone(),
        total_count: trending_songs.len(),
    }))
}

// GET /songs/recommendations/:user_id - Get personalized recommendations
async fn get_recommendations(
    Path(user_id): Path<String>,
) -> Result<Json<DiscoverMusicResponse>, StatusCode> {
    // Validate user_id format
    Uuid::parse_str(&user_id).map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let demo_songs = get_demo_songs();
    let recommended_songs: Vec<SongResponse> = demo_songs.into_iter()
        .filter(|s| s.is_popular || s.is_trending)
        .take(20)
        .collect();

    Ok(Json(DiscoverMusicResponse {
        songs: recommended_songs.clone(),
        total_count: recommended_songs.len(),
    }))
}

// GET /health - Health check
async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "music-catalog-demo".to_string(),
    })
}

// Demo data generator
fn get_demo_songs() -> Vec<SongResponse> {
    vec![
        SongResponse {
            id: "550e8400-e29b-41d4-a716-446655440001".to_string(),
            title: "Neon Dreams".to_string(),
            artist_id: "artist_001".to_string(),
            duration_seconds: 240,
            genre: "Electronic".to_string(),
            listen_count: 15420,
            revenue_generated: 1542.50,
            is_popular: true,
            is_trending: true,
        },
        SongResponse {
            id: "550e8400-e29b-41d4-a716-446655440002".to_string(),
            title: "Midnight Jazz".to_string(),
            artist_id: "artist_002".to_string(),
            duration_seconds: 180,
            genre: "Jazz".to_string(),
            listen_count: 8950,
            revenue_generated: 895.20,
            is_popular: true,
            is_trending: false,
        },
        SongResponse {
            id: "550e8400-e29b-41d4-a716-446655440003".to_string(),
            title: "Rock Anthem".to_string(),
            artist_id: "artist_003".to_string(),
            duration_seconds: 300,
            genre: "Rock".to_string(),
            listen_count: 23450,
            revenue_generated: 2345.75,
            is_popular: true,
            is_trending: true,
        },
        SongResponse {
            id: "550e8400-e29b-41d4-a716-446655440004".to_string(),
            title: "Acoustic Vibes".to_string(),
            artist_id: "artist_004".to_string(),
            duration_seconds: 200,
            genre: "Folk".to_string(),
            listen_count: 5230,
            revenue_generated: 523.40,
            is_popular: false,
            is_trending: false,
        },
        SongResponse {
            id: "550e8400-e29b-41d4-a716-446655440005".to_string(),
            title: "Bass Drop".to_string(),
            artist_id: "artist_005".to_string(),
            duration_seconds: 220,
            genre: "Electronic".to_string(),
            listen_count: 18900,
            revenue_generated: 1890.30,
            is_popular: true,
            is_trending: true,
        },
        SongResponse {
            id: "550e8400-e29b-41d4-a716-446655440006".to_string(),
            title: "Country Road".to_string(),
            artist_id: "artist_006".to_string(),
            duration_seconds: 280,
            genre: "Country".to_string(),
            listen_count: 12750,
            revenue_generated: 1275.60,
            is_popular: false,
            is_trending: false,
        },
    ]
} 