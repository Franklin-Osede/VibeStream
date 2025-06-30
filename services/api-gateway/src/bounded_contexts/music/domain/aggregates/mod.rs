// Aggregates will be implemented here
// For now, the Song entity contains most business logic 

pub mod music_catalog_aggregate;

pub use music_catalog_aggregate::{MusicCatalogAggregate, Album, Playlist, CatalogStats}; 