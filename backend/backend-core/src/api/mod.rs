use axum::{
    Router,
    routing::get,
};
use std::sync::Arc;
use crate::AppState;

// TODO: Uncomment and implement these modules as needed
// mod auth;
// mod users;
// mod artists;
// mod songs;
// mod playlists;
// mod nfts;
// mod royalties;

pub fn create_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(|| async { "OK" }))
} 