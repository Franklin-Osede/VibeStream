use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sea_orm::Set;
use crate::types::DateTimeWithTimeZone;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "song_nfts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub song_id: Uuid,
    pub token_id: i64,
    pub owner_address: String,
    pub metadata_uri: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::song::Entity")]
    Song,
}

impl Related<super::song::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Song.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize)]
pub struct CreateSongNft {
    pub song_id: Uuid,
    pub token_id: i64,
    pub owner_address: String,
}

impl Model {
    pub async fn create(
        db: &DatabaseConnection,
        data: CreateSongNft,
    ) -> Result<Self, DbErr> {
        let nft = ActiveModel {
            id: Set(Uuid::new_v4()),
            song_id: Set(data.song_id),
            token_id: Set(data.token_id),
            owner_address: Set(data.owner_address),
            metadata_uri: Set("".to_string()),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
        };

        nft.insert(db).await
    }

    pub async fn get_song(&self, db: &DatabaseConnection) -> Result<super::song::Model, DbErr> {
        self.find_related(super::song::Entity).one(db).await?
            .ok_or(DbErr::Custom("Song not found".to_string()))
    }
} 