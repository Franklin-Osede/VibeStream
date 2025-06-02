pub mod auth;
pub mod users;
pub mod artists;
pub mod songs;
pub mod playlists;
pub mod nfts;
pub mod royalties;

pub use auth::{login, register};
pub use users::get_current_user;
pub use artists::get_artist;
pub use songs::{get_song, create_song};
pub use playlists::{get_playlist, create_playlist, add_song};
pub use nfts::{get_nft, create_nft}; 