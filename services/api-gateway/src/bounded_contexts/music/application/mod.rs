pub mod commands;
pub mod queries;
// pub mod services; // Commented out since file doesn't exist
pub mod use_cases;

// Re-export only the commands and handlers that actually exist
pub use commands::{
    CreateSongCommand, UpdateSongCommand, DeleteSongCommand,
    RecordListenCommand,
    CreateSongHandler, UpdateSongHandler, DeleteSongHandler,
    RecordListenHandler,
};

pub use queries::{
    SearchSongsQuery, GetSongQuery, 
    SearchAlbumsQuery, GetAlbumQuery, 
    SearchPlaylistsQuery, GetPlaylistQuery, 
    GetTrendingSongsQuery, GetPopularSongsQuery,
    GetArtistSongsQuery,
};

pub use use_cases::{
    UploadSongCommand, UploadSongResult,
}; 