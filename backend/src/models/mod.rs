// Domain models
pub mod user;
pub mod artist;
pub mod song;
pub mod playlist;
pub mod nft;
pub mod royalty;

// Re-export common types
pub use user::User;
pub use artist::Artist;
pub use song::Song;
pub use playlist::Playlist;
pub use nft::{Contract, SongNFT};
pub use royalty::RoyaltyPayment; 