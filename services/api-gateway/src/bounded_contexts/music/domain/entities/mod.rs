pub mod song;
pub mod album;
pub mod playlist;
pub mod artist;
pub mod genre_stats;

// Re-export main entities
pub use song::{Song, SongMetadata, SongTrendingScore};
pub use album::{Album, AlbumTrack, AlbumMetadata};
pub use playlist::{Playlist, PlaylistTrack, PlaylistMetadata};
pub use artist::{Artist, ArtistProfile, ArtistStats};
pub use genre_stats::{GenreStats, GenrePopularity}; 