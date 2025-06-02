use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "songs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub duration: i32,
    pub genre: Option<String>,
    pub file_url: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::artist::Entity")]
    Artist,
    #[sea_orm(has_many = "super::song_nft::Entity")]
    SongNfts,
    #[sea_orm(has_many = "super::royalty_payment::Entity")]
    RoyaltyPayments,
}

impl Related<super::artist::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Artist.def()
    }
}

impl Related<super::song_nft::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SongNfts.def()
    }
}

impl Related<super::royalty_payment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RoyaltyPayments.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..ActiveModelTrait::default()
        }
    }

    fn before_save(mut self, insert: bool) -> Result<Self, DbErr> {
        self.updated_at = Set(chrono::Utc::now());
        if insert {
            self.created_at = Set(chrono::Utc::now());
        }
        Ok(self)
    }
} 