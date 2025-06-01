use sea_orm::DatabaseConnection;

pub mod user;
pub mod artist;
pub mod song;
pub mod playlist;
pub mod nft;
pub mod royalty;

// Re-export repository traits
pub use user::UserRepository;
pub use artist::ArtistRepository;
pub use song::SongRepository;
pub use playlist::PlaylistRepository;
pub use nft::NFTRepository;
pub use royalty::RoyaltyRepository;

// Repository factory trait
pub trait RepositoryProvider {
    fn user_repository(&self) -> &dyn UserRepository;
    fn artist_repository(&self) -> &dyn ArtistRepository;
    fn song_repository(&self) -> &dyn SongRepository;
    fn playlist_repository(&self) -> &dyn PlaylistRepository;
    fn nft_repository(&self) -> &dyn NFTRepository;
    fn royalty_repository(&self) -> &dyn RoyaltyRepository;
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

    fn nft_repository(&self) -> &dyn NFTRepository {
        unimplemented!()
    }

    fn royalty_repository(&self) -> &dyn RoyaltyRepository {
        unimplemented!()
    }
} 