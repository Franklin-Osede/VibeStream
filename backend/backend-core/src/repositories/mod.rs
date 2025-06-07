use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub mod user;
pub mod artist;
pub mod song;
pub mod playlist;
pub mod nft;

pub use user::*;
pub use artist::*;
pub use song::*;
pub use playlist::*;
pub use nft::*;

// Repository factory trait
pub trait RepositoryProvider {
    fn user_repository(&self) -> &dyn UserRepository;
    fn artist_repository(&self) -> &dyn ArtistRepository;
    fn song_repository(&self) -> &dyn SongRepository;
    fn playlist_repository(&self) -> &dyn PlaylistRepository;
    fn nft_repository(&self) -> &dyn NftRepository;
}

// Default implementation using Sea-ORM
pub struct SeaORMRepositoryProvider {
    db: DatabaseConnection,
}

impl SeaORMRepositoryProvider {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl RepositoryProvider for SeaORMRepositoryProvider {
    fn user_repository(&self) -> &dyn UserRepository {
        // TODO: Implement concrete repositories
        unimplemented!()
    }

    fn artist_repository(&self) -> &dyn ArtistRepository {
        unimplemented!()
    }

    fn song_repository(&self) -> &dyn SongRepository {
        unimplemented!()
    }

    fn playlist_repository(&self) -> &dyn PlaylistRepository {
        unimplemented!()
    }

    fn nft_repository(&self) -> &dyn NftRepository {
        unimplemented!()
    }
}

pub struct Repositories {
    pub user: Arc<dyn UserRepository>,
    pub artist: Arc<dyn ArtistRepository>,
    pub song: Arc<dyn SongRepository>,
    pub playlist: Arc<dyn PlaylistRepository>,
    pub nft: Arc<dyn NftRepository>,
}

impl Repositories {
    pub fn new(
        db: DatabaseConnection,
    ) -> Self {
        Self {
            user: Arc::new(UserRepositoryImpl),
            artist: Arc::new(ArtistRepositoryImpl),
            song: Arc::new(SongRepositoryImpl),
            playlist: Arc::new(PlaylistRepositoryImpl),
            nft: Arc::new(NftRepositoryImpl),
        }
    }
} 