use sea_orm::*;
use uuid::Uuid;
use async_trait::async_trait;
use crate::models::artist::{self, Model as Artist};

#[async_trait]
pub trait ArtistRepository: Send + Sync {
    async fn find_by_id(&self, db: &DatabaseConnection, id: Uuid) -> Result<Option<Artist>, DbErr>;
    async fn find_by_user_id(&self, db: &DatabaseConnection, user_id: Uuid) -> Result<Option<Artist>, DbErr>;
    async fn find_all(&self, db: &DatabaseConnection) -> Result<Vec<Artist>, DbErr>;
}

pub struct ArtistRepositoryImpl;

#[async_trait]
impl ArtistRepository for ArtistRepositoryImpl {
    async fn find_by_id(&self, db: &DatabaseConnection, id: Uuid) -> Result<Option<Artist>, DbErr> {
        artist::Entity::find_by_id(id).one(db).await
    }

    async fn find_by_user_id(&self, db: &DatabaseConnection, user_id: Uuid) -> Result<Option<Artist>, DbErr> {
        artist::Entity::find()
            .filter(artist::Column::Id.eq(user_id))
            .one(db)
            .await
    }

    async fn find_all(&self, db: &DatabaseConnection) -> Result<Vec<Artist>, DbErr> {
        artist::Entity::find().all(db).await
    }
} 