use sea_orm::*;
use uuid::Uuid;
use async_trait::async_trait;
use crate::models::song::{self, Model as Song};

#[async_trait]
pub trait SongRepository: Send + Sync {
    async fn find_by_id(&self, db: &DatabaseConnection, id: Uuid) -> Result<Option<Song>, DbErr>;
    async fn find_by_artist_id(&self, db: &DatabaseConnection, artist_id: Uuid) -> Result<Vec<Song>, DbErr>;
    async fn create(&self, db: &DatabaseConnection, song: song::ActiveModel) -> Result<Song, DbErr>;
}

pub struct SongRepositoryImpl;

#[async_trait]
impl SongRepository for SongRepositoryImpl {
    async fn find_by_id(&self, db: &DatabaseConnection, id: Uuid) -> Result<Option<Song>, DbErr> {
        song::Entity::find_by_id(id).one(db).await
    }

    async fn find_by_artist_id(&self, db: &DatabaseConnection, artist_id: Uuid) -> Result<Vec<Song>, DbErr> {
        song::Entity::find()
            .filter(song::Column::ArtistId.eq(artist_id))
            .all(db)
            .await
    }

    async fn create(&self, db: &DatabaseConnection, song: song::ActiveModel) -> Result<Song, DbErr> {
        song.insert(db).await
    }
} 