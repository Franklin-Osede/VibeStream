use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "royalty_payments")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub song_id: Uuid,
    pub artist_id: Uuid,
    pub nft_id: Option<Uuid>,
    pub amount: Decimal,
    pub currency: String,
    pub transaction_hash: Option<String>,
    pub status: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::song::Entity")]
    Song,
    #[sea_orm(belongs_to = "super::artist::Entity")]
    Artist,
    #[sea_orm(belongs_to = "super::song_nft::Entity")]
    SongNft,
}

impl Related<super::song::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Song.def()
    }
}

impl Related<super::artist::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Artist.def()
    }
}

impl Related<super::song_nft::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SongNft.def()
    }
}

impl ActiveModelBehavior for ActiveModel {} 