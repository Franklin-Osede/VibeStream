use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::bounded_contexts::music::domain::entities::Song;
use crate::bounded_contexts::music::domain::value_objects::{
    SongId, ArtistId, AlbumId, PlaylistId, SongTitle, AlbumTitle, Genre, 
    RoyaltyPercentage, SongDuration, ListenCount
};
use crate::shared::domain::events::DomainEvent;
use crate::bounded_contexts::music::domain::events::{
    SongUploaded, AlbumCreated, PlaylistCreated
};

// Business configuration - easily changeable
pub struct CatalogLimits {
    pub max_songs_per_artist: usize,
    pub max_songs_per_album: usize,
    pub max_songs_per_playlist: usize,
}

impl Default for CatalogLimits {
    fn default() -> Self {
        Self {
            max_songs_per_artist: 1000,    // Configurable: could be 5000 for premium artists
            max_songs_per_album: 50,       // Industry standard
            max_songs_per_playlist: 1000,  // User experience limit
        }
    }
}

impl CatalogLimits {
    pub fn new(max_songs_per_artist: usize, max_songs_per_album: usize, max_songs_per_playlist: usize) -> Self {
        Self {
            max_songs_per_artist,
            max_songs_per_album,
            max_songs_per_playlist,
        }
    }

    // Preset configurations for different artist tiers
    pub fn indie_artist() -> Self {
        Self::new(500, 30, 500)
    }

    pub fn professional_artist() -> Self {
        Self::new(2000, 50, 1000)
    }

    pub fn major_label_artist() -> Self {
        Self::new(5000, 100, 2000)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
    id: AlbumId,
    title: AlbumTitle,
    artist_id: ArtistId,
    song_ids: Vec<SongId>,
    created_at: DateTime<Utc>,
}

impl Album {
    pub fn new(title: AlbumTitle, artist_id: ArtistId) -> Self {
        Self {
            id: AlbumId::new(),
            title,
            artist_id,
            song_ids: Vec::new(),
            created_at: Utc::now(),
        }
    }

    pub fn id(&self) -> &AlbumId {
        &self.id
    }

    pub fn title(&self) -> &AlbumTitle {
        &self.title
    }

    pub fn artist_id(&self) -> &ArtistId {
        &self.artist_id
    }

    pub fn song_ids(&self) -> &[SongId] {
        &self.song_ids
    }

    pub fn add_song(&mut self, song_id: SongId) -> Result<(), String> {
        if self.song_ids.contains(&song_id) {
            return Err("Song already exists in album".to_string());
        }
        if self.song_ids.len() >= 50 {
            return Err("Album cannot have more than 50 songs".to_string());
        }
        self.song_ids.push(song_id);
        Ok(())
    }

    pub fn remove_song(&mut self, song_id: &SongId) -> Result<(), String> {
        if let Some(pos) = self.song_ids.iter().position(|id| id == song_id) {
            self.song_ids.remove(pos);
            Ok(())
        } else {
            Err("Song not found in album".to_string())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    id: Uuid,
    name: String,
    user_id: Uuid,
    song_ids: Vec<SongId>,
    is_public: bool,
    created_at: DateTime<Utc>,
}

impl Playlist {
    pub fn new(name: String, user_id: Uuid, is_public: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            user_id,
            song_ids: Vec::new(),
            is_public,
            created_at: Utc::now(),
        }
    }

    pub fn add_song(&mut self, song_id: SongId) -> Result<(), String> {
        if self.song_ids.contains(&song_id) {
            return Err("Song already exists in playlist".to_string());
        }
        if self.song_ids.len() >= 1000 {
            return Err("Playlist cannot have more than 1000 songs".to_string());
        }
        self.song_ids.push(song_id);
        Ok(())
    }
}

#[derive(Debug)]
pub struct MusicCatalogAggregate {
    songs: HashMap<SongId, Song>,
    albums: HashMap<AlbumId, Album>,
    playlists: HashMap<Uuid, Playlist>,
    uncommitted_events: Vec<Box<dyn DomainEvent>>,
}

impl MusicCatalogAggregate {
    pub fn new() -> Self {
        Self {
            songs: HashMap::new(),
            albums: HashMap::new(),
            playlists: HashMap::new(),
            uncommitted_events: Vec::new(),
        }
    }

    // Song operations
    pub fn upload_song(
        &mut self,
        title: SongTitle,
        artist_id: ArtistId,
        duration: SongDuration,
        genre: Genre,
        royalty_percentage: RoyaltyPercentage,
    ) -> Result<SongId, String> {
        // Domain validation: Artist can't have more than 1000 songs
        let artist_song_count = self.songs.values()
            .filter(|song| song.artist_id() == &artist_id)
            .count();
        
        if artist_song_count >= 1000 {
            return Err("Artist cannot have more than 1000 songs".to_string());
        }

        let song = Song::new(title.clone(), artist_id.clone(), duration.clone(), genre.clone(), royalty_percentage);
        let song_id = song.id().clone();

        // Create domain event
        let event = Box::new(SongUploaded {
            song_id: song_id.clone(),
            artist_id: artist_id.clone(),
            title: title.clone(),
            genre: genre.clone(),
            duration_seconds: duration.seconds(),
            uploaded_at: Utc::now(),
            metadata: crate::shared::domain::events::EventMetadata::new(),
        });

        self.songs.insert(song_id.clone(), song);
        self.uncommitted_events.push(event);

        Ok(song_id)
    }

    pub fn get_song(&self, song_id: &SongId) -> Option<&Song> {
        self.songs.get(song_id)
    }

    pub fn get_song_mut(&mut self, song_id: &SongId) -> Option<&mut Song> {
        self.songs.get_mut(song_id)
    }

    pub fn record_listen(&mut self, song_id: &SongId, listener_id: Uuid, duration_seconds: u32) -> Result<(), String> {
        if let Some(song) = self.songs.get_mut(song_id) {
            let event = song.record_listen(listener_id, duration_seconds)?;
            self.uncommitted_events.push(event);
            Ok(())
        } else {
            Err("Song not found".to_string())
        }
    }

    // Album operations
    pub fn create_album(
        &mut self,
        title: AlbumTitle,
        artist_id: ArtistId,
        song_ids: Vec<SongId>,
    ) -> Result<AlbumId, String> {
        // Validate all songs exist and belong to the artist
        for song_id in &song_ids {
            if let Some(song) = self.songs.get(song_id) {
                if song.artist_id() != &artist_id {
                    return Err(format!("Song {} does not belong to artist", song_id));
                }
            } else {
                return Err(format!("Song {} not found", song_id));
            }
        }

        let mut album = Album::new(title, artist_id.clone());
        for song_id in song_ids.clone() {
            album.add_song(song_id)?;
        }

        let album_id = album.id().clone();

        // Create domain event
        let event = Box::new(AlbumCreated {
            album_id: album_id.clone(),
            artist_id,
            title: album.title().value().to_string(),
            song_ids,
            created_at: Utc::now(),
            metadata: crate::shared::domain::events::EventMetadata::new(),
        });

        self.albums.insert(album_id.clone(), album);
        self.uncommitted_events.push(event);

        Ok(album_id)
    }

    pub fn get_album(&self, album_id: &AlbumId) -> Option<&Album> {
        self.albums.get(album_id)
    }

    // Playlist operations
    pub fn create_playlist(
        &mut self,
        name: String,
        user_id: Uuid,
        song_ids: Vec<SongId>,
        is_public: bool,
    ) -> Result<Uuid, String> {
        // Validate all songs exist
        for song_id in &song_ids {
            if !self.songs.contains_key(song_id) {
                return Err(format!("Song {} not found", song_id));
            }
        }

        let mut playlist = Playlist::new(name.clone(), user_id, is_public);
        for song_id in song_ids.clone() {
            playlist.add_song(song_id)?;
        }

        let playlist_id = playlist.id;

        // Create domain event
        let event = Box::new(PlaylistCreated {
            playlist_id: PlaylistId::from_uuid(playlist_id),
            user_id,
            name,
            song_ids,
            created_at: Utc::now(),
            metadata: crate::shared::domain::events::EventMetadata::new(),
        });

        self.playlists.insert(playlist_id, playlist);
        self.uncommitted_events.push(event);

        Ok(playlist_id)
    }

    // Discovery and search
    pub fn get_trending_songs(&self) -> Vec<&Song> {
        let mut songs: Vec<&Song> = self.songs.values()
            .filter(|song| song.is_trending())
            .collect();
        
        songs.sort_by(|a, b| b.listen_count().value().cmp(&a.listen_count().value()));
        songs.into_iter().take(50).collect()
    }

    pub fn get_popular_songs(&self) -> Vec<&Song> {
        let mut songs: Vec<&Song> = self.songs.values()
            .filter(|song| song.is_popular())
            .collect();
        
        songs.sort_by(|a, b| b.listen_count().value().cmp(&a.listen_count().value()));
        songs.into_iter().take(100).collect()
    }

    pub fn search_songs_by_title(&self, query: &str) -> Vec<&Song> {
        let query_lower = query.to_lowercase();
        self.songs.values()
            .filter(|song| song.title().value().to_lowercase().contains(&query_lower))
            .collect()
    }

    pub fn get_songs_by_genre(&self, genre: &Genre) -> Vec<&Song> {
        self.songs.values()
            .filter(|song| song.genre() == genre)
            .collect()
    }

    pub fn get_artist_songs(&self, artist_id: &ArtistId) -> Vec<&Song> {
        self.songs.values()
            .filter(|song| song.artist_id() == artist_id)
            .collect()
    }

    // Event handling
    pub fn get_uncommitted_events(&self) -> &[Box<dyn DomainEvent>] {
        &self.uncommitted_events
    }

    pub fn mark_events_as_committed(&mut self) {
        self.uncommitted_events.clear();
    }

    // Analytics
    pub fn get_catalog_stats(&self) -> CatalogStats {
        let total_songs = self.songs.len();
        let total_albums = self.albums.len();
        let total_playlists = self.playlists.len();
        let total_listens: u64 = self.songs.values()
            .map(|song| song.listen_count().value())
            .sum();

        CatalogStats {
            total_songs,
            total_albums,
            total_playlists,
            total_listens,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CatalogStats {
    pub total_songs: usize,
    pub total_albums: usize,
    pub total_playlists: usize,
    pub total_listens: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_aggregate() -> MusicCatalogAggregate {
        MusicCatalogAggregate::new()
    }

    #[test]
    fn test_upload_song() {
        let mut catalog = create_test_aggregate();
        
        let song_id = catalog.upload_song(
            SongTitle::new("Test Song".to_string()).unwrap(),
            ArtistId::new(),
            SongDuration::new(180).unwrap(),
            Genre::new("rock".to_string()).unwrap(),
            RoyaltyPercentage::new(70.0).unwrap(),
        ).unwrap();

        assert!(catalog.get_song(&song_id).is_some());
        assert_eq!(catalog.get_uncommitted_events().len(), 1);
    }

    #[test]
    fn test_create_album() {
        let mut catalog = create_test_aggregate();
        let artist_id = ArtistId::new();
        
        // Upload songs first
        let song1 = catalog.upload_song(
            SongTitle::new("Song 1".to_string()).unwrap(),
            artist_id.clone(),
            SongDuration::new(180).unwrap(),
            Genre::new("rock".to_string()).unwrap(),
            RoyaltyPercentage::new(70.0).unwrap(),
        ).unwrap();

        let song2 = catalog.upload_song(
            SongTitle::new("Song 2".to_string()).unwrap(),
            artist_id.clone(),
            SongDuration::new(200).unwrap(),
            Genre::new("rock".to_string()).unwrap(),
            RoyaltyPercentage::new(70.0).unwrap(),
        ).unwrap();

        // Create album
        let album_id = catalog.create_album(
            AlbumTitle::new("Test Album".to_string()).unwrap(),
            artist_id,
            vec![song1, song2],
        ).unwrap();

        assert!(catalog.get_album(&album_id).is_some());
        assert_eq!(catalog.get_album(&album_id).unwrap().song_ids().len(), 2);
    }

    #[test]
    fn test_trending_songs() {
        let mut catalog = create_test_aggregate();
        let artist_id = ArtistId::new();
        
        let song_id = catalog.upload_song(
            SongTitle::new("Trending Song".to_string()).unwrap(),
            artist_id,
            SongDuration::new(180).unwrap(),
            Genre::new("pop".to_string()).unwrap(),
            RoyaltyPercentage::new(70.0).unwrap(),
        ).unwrap();

        // Simulate many listens to make it trending
        for _ in 0..100 {
            let _ = catalog.record_listen(&song_id, Uuid::new_v4(), 90);
        }

        let trending = catalog.get_trending_songs();
        assert!(!trending.is_empty());
    }
} 