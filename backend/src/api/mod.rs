use axum::Router;
use crate::AppState;

mod auth;
mod users;
mod artists;
mod songs;
mod playlists;
mod nfts;
mod royalties;

pub fn create_router() -> Router<AppState> {
    Router::new()
        // Auth routes
        .merge(auth::create_auth_router())
        // User routes
        .merge(users::create_user_router())
        // Artist routes
        .merge(artists::create_artist_router())
        // Song routes
        .merge(songs::create_song_router())
        // Playlist routes
        .merge(playlists::create_playlist_router())
        // NFT routes
        .merge(nfts::create_nft_router())
        // Royalty routes
        .merge(royalties::create_royalty_router())
} 