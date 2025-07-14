pub mod value_objects;
pub mod entities;
pub mod events;
pub mod aggregates;
pub mod repositories;
pub mod services;

// Re-export specific items to avoid naming conflicts
pub use value_objects::*;
pub use entities::{Song, SongMetadata, AlbumTrack, PlaylistTrack, Artist, ArtistProfile, ArtistStats, ArtistTier, GenreStats};
pub use events::*;
// Re-export specific aggregates to avoid conflicts with entities
pub use aggregates::MusicCatalogAggregate; 