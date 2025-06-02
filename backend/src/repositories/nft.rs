use sea_orm::*;
use uuid::Uuid;
use async_trait::async_trait;
use crate::models::song_nft::{self, Model as SongNft};

#[async_trait]
pub trait NftRepository: Send + Sync {
    async fn find_by_id(&self, db: &DatabaseConnection, id: Uuid) -> Result<Option<SongNft>, DbErr>;
    async fn find_by_song_id(&self, db: &DatabaseConnection, song_id: Uuid) -> Result<Vec<SongNft>, DbErr>;
    async fn create(&self, db: &DatabaseConnection, nft: song_nft::ActiveModel) -> Result<SongNft, DbErr>;
}

pub struct NftRepositoryImpl;

#[async_trait]
impl NftRepository for NftRepositoryImpl {
    async fn find_by_id(&self, db: &DatabaseConnection, id: Uuid) -> Result<Option<SongNft>, DbErr> {
        song_nft::Entity::find_by_id(id).one(db).await
    }

    async fn find_by_song_id(&self, db: &DatabaseConnection, song_id: Uuid) -> Result<Vec<SongNft>, DbErr> {
        song_nft::Entity::find()
            .filter(song_nft::Column::SongId.eq(song_id))
            .all(db)
            .await
    }

    async fn create(&self, db: &DatabaseConnection, nft: song_nft::ActiveModel) -> Result<SongNft, DbErr> {
        nft.insert(db).await
    }
} 