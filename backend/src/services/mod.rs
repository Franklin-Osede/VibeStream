pub mod auth;
pub mod user;
pub mod artist;
pub mod song;
pub mod playlist;
pub mod nft;
pub mod royalty;

// Re-export common types
pub use auth::AuthService;
pub use user::UserService;
pub use artist::ArtistService;
pub use song::SongService;
pub use playlist::PlaylistService;
pub use nft::NFTService;
pub use royalty::RoyaltyService; 