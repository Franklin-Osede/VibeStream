use sea_orm::*;
use uuid::Uuid;
use async_trait::async_trait;
use crate::models::playlist::{self, Model as Playlist};

#[async_trait]
pub trait PlaylistRepository: Send + Sync {
    async fn find_by_id(&self, db: &DatabaseConnection, id: Uuid) -> Result<Option<Playlist>, DbErr>;
    async fn find_by_user_id(&self, db: &DatabaseConnection, user_id: Uuid) -> Result<Vec<Playlist>, DbErr>;
    async fn create(&self, db: &DatabaseConnection, playlist: playlist::ActiveModel) -> Result<Playlist, DbErr>;
}

pub struct PlaylistRepositoryImpl;

#[async_trait]
impl PlaylistRepository for PlaylistRepositoryImpl {
    async fn find_by_id(&self, db: &DatabaseConnection, id: Uuid) -> Result<Option<Playlist>, DbErr> {
        playlist::Entity::find_by_id(id).one(db).await
    }

    async fn find_by_user_id(&self, db: &DatabaseConnection, user_id: Uuid) -> Result<Vec<Playlist>, DbErr> {
        playlist::Entity::find()
            .filter(playlist::Column::UserId.eq(user_id))
            .all(db)
            .await
    }

    async fn create(&self, db: &DatabaseConnection, playlist: playlist::ActiveModel) -> Result<Playlist, DbErr> {
        playlist.insert(db).await
    }
}

impl PlaylistRepositoryImpl {
    pub async fn find_public(db: &DatabaseConnection) -> Result<Vec<Playlist>, DbErr> {
        playlist::Entity::find()
            .filter(playlist::Column::IsPublic.eq(true))
            .all(db)
            .await
    }

    pub async fn update(db: &DatabaseConnection, playlist: playlist::ActiveModel) -> Result<Playlist, DbErr> {
        playlist.update(db).await
    }

    pub async fn delete(db: &DatabaseConnection, id: Uuid) -> Result<DeleteResult, DbErr> {
        playlist::Entity::delete_by_id(id).exec(db).await
    }
} 