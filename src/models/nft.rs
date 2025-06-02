use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "nfts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub song_id: Uuid,
    pub contract_id: Uuid,
    pub token_id: i64,
    pub owner_id: Uuid,
    pub total_supply: i32,
    pub royalty_percentage: f64,
    pub description: Option<String>,
    pub metadata: Option<Value>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::song::Entity")]
    Song,
    #[sea_orm(belongs_to = "super::contract::Entity")]
    Contract,
    #[sea_orm(belongs_to = "super::user::Entity")]
    Owner,
    #[sea_orm(has_many = "super::royalty_payment::Entity")]
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

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Owner.def()
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