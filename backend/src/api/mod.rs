pub mod auth;
pub mod users;
pub mod artists;
pub mod songs;
pub mod playlists;
pub mod nfts;
pub mod royalties;

use axum::{
    routing::{get, post},
    Router,
};

pub fn create_router() -> Router {
    Router::new()
        // Auth routes
        .route("/auth/login", post(auth::login))
        .route("/auth/register", post(auth::register))
        
        // User routes
        .route("/users/me", get(users::get_current_user))
        
        // Artist routes
        .route("/artists", get(artists::list_artists))
        .route("/artists", post(artists::create_artist))
        .route("/artists/:id", get(artists::get_artist))
        
        // Song routes
        .route("/songs", get(songs::list_songs))
        .route("/songs", post(songs::create_song))
        .route("/songs/:id", get(songs::get_song))
        
        // Playlist routes
        .route("/playlists", get(playlists::list_playlists))
        .route("/playlists", post(playlists::create_playlist))
        .route("/playlists/:id", get(playlists::get_playlist))
        .route("/playlists/:id/songs", post(playlists::add_song))
        
        // NFT routes
        .route("/nfts", get(nfts::list_nfts))
        .route("/nfts", post(nfts::create_nft))
        .route("/nfts/:id", get(nfts::get_nft))
        
        // Royalty routes
        .route("/royalties", get(royalties::list_payments))
} 