use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sea_orm::Set;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "song_nfts", schema_name = "blockchain")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub song_id: Uuid,
    pub contract_id: Uuid,
    pub token_id: i64,
    pub owner_address: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::song::Entity",
        from = "Column::SongId",
        to = "super::song::Column::Id"
    )]
    Song,
    #[sea_orm(
        belongs_to = "super::contract::Entity",
        from = "Column::ContractId",
        to = "super::contract::Column::Id"
    )]
    Contract,
    #[sea_orm(
        has_many = "super::royalty_payment::Entity",
        from = "Column::Id",
        to = "super::royalty_payment::Column::SongNftId"
    )]
    RoyaltyPayments,
}

impl Related<super::song::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Song.def()
    }
}

impl Related<super::contract::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contract.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize)]
pub struct CreateSongNft {
    pub song_id: Uuid,
    pub contract_id: Uuid,
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
            contract_id: Set(data.contract_id),
            token_id: Set(data.token_id),
            owner_address: Set(data.owner_address),
            created_at: Set(chrono::Utc::now().into()),
            updated_at: Set(chrono::Utc::now().into()),
        };

        nft.insert(db).await
    }

    pub async fn get_song(&self, db: &DatabaseConnection) -> Result<super::song::Model, DbErr> {
        self.find_related(super::song::Entity).one(db).await?
            .ok_or(DbErr::Custom("Song not found".to_string()))
    }

    pub async fn get_contract(&self, db: &DatabaseConnection) -> Result<super::contract::Model, DbErr> {
        self.find_related(super::contract::Entity).one(db).await?
            .ok_or(DbErr::Custom("Contract not found".to_string()))
    }
} 