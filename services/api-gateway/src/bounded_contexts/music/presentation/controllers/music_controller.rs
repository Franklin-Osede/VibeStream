use axum::{
    extract::{Query, Path},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::bounded_contexts::music::application::use_cases::{
    UploadSongUseCase, UploadSongCommand,
    DiscoverMusicUseCase, DiscoverMusicQuery, DiscoveryFilter
};

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

// Music Controller
pub struct MusicController;

impl MusicController {
    pub fn routes() -> Router<crate::AppState> {
        Router::new()
            .route("/songs", post(Self::upload_song))
            .route("/songs/discover", get(Self::discover_music))
            .route("/songs/trending", get(Self::get_trending))
            .route("/songs/recommendations/:user_id", get(Self::get_recommendations))
            .route("/health", get(Self::health_check))
    }

    // POST /songs - Upload a new song
    async fn upload_song(
        Json(request): Json<UploadSongRequest>,
    ) -> Result<Json<UploadSongResponse>, StatusCode> {
        let use_case = UploadSongUseCase::new();

        let artist_id = Uuid::parse_str(&request.artist_id)
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        let command = UploadSongCommand {
            title: request.title,
            artist_id,
            duration_seconds: request.duration_seconds,
            genre: request.genre,
            royalty_percentage: request.royalty_percentage,
        };

        match use_case.execute(command).await {
            Ok(result) => Ok(Json(UploadSongResponse {
                song_id: result.song_id.value().to_string(),
                success: result.success,
                message: result.message,
            })),
            Err(error) => {
                eprintln!("Upload song error: {}", error);
                Err(StatusCode::BAD_REQUEST)
            }
        }
    }

    // GET /songs/discover - Discover music with filters
    async fn discover_music(
        Query(params): Query<DiscoverMusicRequest>,
    ) -> Result<Json<DiscoverMusicResponse>, StatusCode> {
        let use_case = DiscoverMusicUseCase::new();

        let filter = match params.filter.as_deref() {
            Some("trending") => DiscoveryFilter::Trending,
            Some("popular") => DiscoveryFilter::Popular,
            Some("by_genre") => {
                if let Some(genre) = params.genre {
                    DiscoveryFilter::ByGenre(genre)
                } else {
                    return Err(StatusCode::BAD_REQUEST);
                }
            }
            Some("by_artist") => {
                if let Some(artist_id) = params.artist_id {
                    let uuid = Uuid::parse_str(&artist_id)
                        .map_err(|_| StatusCode::BAD_REQUEST)?;
                    DiscoveryFilter::ByArtist(uuid)
                } else {
                    return Err(StatusCode::BAD_REQUEST);
                }
            }
            Some("search") => {
                if let Some(query) = params.search_query {
                    DiscoveryFilter::Search(query)
                } else {
                    return Err(StatusCode::BAD_REQUEST);
                }
            }
            _ => DiscoveryFilter::Popular, // Default
        };

        let query = DiscoverMusicQuery {
            filter,
            limit: params.limit,
        };

        match use_case.execute(query).await {
            Ok(result) => {
                let songs: Vec<SongResponse> = result.songs.into_iter()
                    .map(|song| SongResponse {
                        id: song.id.value().to_string(),
                        title: song.title,
                        artist_id: song.artist_id.value().to_string(),
                        duration_seconds: song.duration_seconds,
                        genre: song.genre,
                        listen_count: song.listen_count,
                        revenue_generated: song.revenue_generated,
                        is_popular: song.is_popular,
                        is_trending: song.is_trending,
                    })
                    .collect();

                Ok(Json(DiscoverMusicResponse {
                    songs,
                    total_count: result.total_count,
                }))
            }
            Err(error) => {
                eprintln!("Discover music error: {}", error);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    // GET /songs/trending - Get trending songs
    async fn get_trending() -> Result<Json<DiscoverMusicResponse>, StatusCode> {
        let use_case = DiscoverMusicUseCase::new();

        let query = DiscoverMusicQuery {
            filter: DiscoveryFilter::Trending,
            limit: Some(50),
        };

        match use_case.execute(query).await {
            Ok(result) => {
                let songs: Vec<SongResponse> = result.songs.into_iter()
                    .map(|song| SongResponse {
                        id: song.id.value().to_string(),
                        title: song.title,
                        artist_id: song.artist_id.value().to_string(),
                        duration_seconds: song.duration_seconds,
                        genre: song.genre,
                        listen_count: song.listen_count,
                        revenue_generated: song.revenue_generated,
                        is_popular: song.is_popular,
                        is_trending: song.is_trending,
                    })
                    .collect();

                Ok(Json(DiscoverMusicResponse {
                    songs,
                    total_count: result.total_count,
                }))
            }
            Err(error) => {
                eprintln!("Get trending error: {}", error);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    // GET /songs/recommendations/:user_id - Get personalized recommendations
    async fn get_recommendations(
        Path(user_id): Path<String>,
    ) -> Result<Json<DiscoverMusicResponse>, StatusCode> {
        let use_case = DiscoverMusicUseCase::new();

        let user_uuid = Uuid::parse_str(&user_id)
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        match use_case.get_personalized_recommendations(user_uuid, Some(30)).await {
            Ok(songs) => {
                let song_responses: Vec<SongResponse> = songs.into_iter()
                    .map(|song| {
                        let metadata = song.get_metadata();
                        SongResponse {
                            id: metadata.id.value().to_string(),
                            title: metadata.title,
                            artist_id: metadata.artist_id.value().to_string(),
                            duration_seconds: metadata.duration_seconds,
                            genre: metadata.genre,
                            listen_count: metadata.listen_count,
                            revenue_generated: metadata.revenue_generated,
                            is_popular: metadata.is_popular,
                            is_trending: metadata.is_trending,
                        }
                    })
                    .collect();

                Ok(Json(DiscoverMusicResponse {
                    songs: song_responses.clone(),
                    total_count: song_responses.len(),
                }))
            }
            Err(error) => {
                eprintln!("Get recommendations error: {}", error);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }

    // GET /health - Health check
    async fn health_check() -> Json<HealthResponse> {
        Json(HealthResponse {
            status: "healthy".to_string(),
            service: "music-catalog".to_string(),
        })
    }
}

// Function to create music routes (needed for main.rs)
pub fn create_music_routes() -> Router<crate::AppState> {
    MusicController::routes()
} 