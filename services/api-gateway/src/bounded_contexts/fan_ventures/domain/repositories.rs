use async_trait::async_trait;
use uuid::Uuid;
use crate::bounded_contexts::fan_ventures::domain::entities::ArtistVenture;

#[async_trait]
pub trait ArtistVentureRepository: Send + Sync {
    async fn create(&self, venture: &ArtistVenture) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn find_by_id(&self, venture_id: &Uuid) -> Result<Option<ArtistVenture>, Box<dyn std::error::Error + Send + Sync>>;
    async fn find_by_artist(&self, artist_id: &Uuid) -> Result<Vec<ArtistVenture>, Box<dyn std::error::Error + Send + Sync>>;
    async fn update(&self, venture: &ArtistVenture) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn delete(&self, venture_id: &Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn find_all_active(&self) -> Result<Vec<ArtistVenture>, Box<dyn std::error::Error + Send + Sync>>;
}
