pub mod song;
pub mod album;
pub mod playlist;
pub mod artist;
pub mod genre_stats;

// Re-export main entities and value objects
pub use song::{Song, SongMetadata};
pub use album::{Album, AlbumTrack};
pub use playlist::{Playlist, PlaylistTrack};
pub use artist::{Artist, ArtistProfile, ArtistStats, ArtistTier};
pub use genre_stats::{GenreStats}; 