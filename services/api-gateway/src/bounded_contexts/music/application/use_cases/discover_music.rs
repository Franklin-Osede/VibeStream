use uuid::Uuid;

use crate::bounded_contexts::music::domain::{
    MusicCatalogAggregate, Song, SongMetadata, Genre, ArtistId
};

#[derive(Debug)]
pub struct DiscoverMusicQuery {
    pub filter: DiscoveryFilter,
    pub limit: Option<usize>,
}

#[derive(Debug)]
pub enum DiscoveryFilter {
    Trending,
    Popular,
    ByGenre(String),
    ByArtist(Uuid),
    Search(String),
}

#[derive(Debug)]
pub struct DiscoverMusicResult {
    pub songs: Vec<SongMetadata>,
    pub total_count: usize,
}

pub struct DiscoverMusicUseCase {
    // Repository will be injected here later
}

impl DiscoverMusicUseCase {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn execute(&self, query: DiscoverMusicQuery) -> Result<DiscoverMusicResult, String> {
        // In real implementation, this would load from repository
        let catalog = MusicCatalogAggregate::new();
        
        let songs = match query.filter {
            DiscoveryFilter::Trending => {
                catalog.get_trending_songs()
            },
            DiscoveryFilter::Popular => {
                catalog.get_popular_songs()
            },
            DiscoveryFilter::ByGenre(genre_str) => {
                let genre = Genre::new(genre_str.clone())
                    .map_err(|e| format!("Invalid genre: {}", e))?;
                catalog.get_songs_by_genre(&genre)
            },
            DiscoveryFilter::ByArtist(artist_uuid) => {
                let artist_id = ArtistId::from_uuid(artist_uuid);
                catalog.get_artist_songs(&artist_id)
            },
            DiscoveryFilter::Search(query_text) => {
                catalog.search_songs_by_title(&query_text)
            },
        };

        // Apply limit if specified
        let limited_songs: Vec<&Song> = if let Some(limit) = query.limit {
            songs.into_iter().take(limit).collect()
        } else {
            songs
        };

        // Convert to metadata
        let song_metadata: Vec<SongMetadata> = limited_songs
            .into_iter()
            .map(|song| song.get_metadata())
            .collect();

        let total_count = song_metadata.len();

        Ok(DiscoverMusicResult {
            songs: song_metadata,
            total_count,
        })
    }

    // New method for personalized recommendations
    pub async fn get_personalized_recommendations(&self, user_id: Uuid, limit: Option<usize>) -> Result<Vec<Song>, String> {
        // In a real implementation, this would use ML/AI to generate personalized recommendations
        // For now, we'll return trending songs as a simple recommendation
        let catalog = MusicCatalogAggregate::new();
        let trending_songs = catalog.get_trending_songs();
        
        let limited_songs: Vec<Song> = if let Some(limit) = limit {
            trending_songs.into_iter().take(limit).cloned().collect()
        } else {
            trending_songs.into_iter().cloned().collect()
        };
        
        Ok(limited_songs)
    }

    pub async fn get_new_releases(&self) -> Result<DiscoverMusicResult, String> {
        // In real implementation, this would filter by recent upload dates
        let query = DiscoverMusicQuery {
            filter: DiscoveryFilter::Popular,
            limit: Some(50),
        };
        
        self.execute(query).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_discover_trending_music() {
        let use_case = DiscoverMusicUseCase::new();
        
        let query = DiscoverMusicQuery {
            filter: DiscoveryFilter::Trending,
            limit: Some(10),
        };

        let result = use_case.execute(query).await;
        assert!(result.is_ok());
        
        let discovery_result = result.unwrap();
        assert_eq!(discovery_result.total_count, 10);
    }

    #[tokio::test]
    async fn test_discover_by_genre() {
        let use_case = DiscoverMusicUseCase::new();
        
        let query = DiscoverMusicQuery {
            filter: DiscoveryFilter::ByGenre("rock".to_string()),
            limit: None,
        };

        let result = use_case.execute(query).await;
        assert!(result.is_ok());
        
        let discovery_result = result.unwrap();
        assert!(discovery_result.total_count > 0);
    }

    #[tokio::test]
    async fn test_discover_invalid_genre() {
        let use_case = DiscoverMusicUseCase::new();
        
        let query = DiscoverMusicQuery {
            filter: DiscoveryFilter::ByGenre("invalid_genre".to_string()),
            limit: None,
        };

        let result = use_case.execute(query).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid genre"));
    }

    #[tokio::test]
    async fn test_search_songs() {
        let use_case = DiscoverMusicUseCase::new();
        
        let query = DiscoverMusicQuery {
            filter: DiscoveryFilter::Search("test".to_string()),
            limit: Some(5),
        };

        let result = use_case.execute(query).await;
        assert!(result.is_ok());
        
        let discovery_result = result.unwrap();
        assert!(discovery_result.total_count <= 5);
    }

    #[tokio::test]
    async fn test_get_recommendations() {
        let use_case = DiscoverMusicUseCase::new();
        let user_id = Uuid::new_v4();

        let result = use_case.get_personalized_recommendations(user_id, Some(20)).await;
        assert!(result.is_ok());
        
        let recommendations = result.unwrap();
        assert!(recommendations.len() <= 20);
    }
} 