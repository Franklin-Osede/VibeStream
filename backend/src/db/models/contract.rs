use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "contracts", schema_name = "blockchain")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub chain_id: i32,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::song_nft::Entity")]
    SongNfts,
}

impl Related<super::song_nft::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SongNfts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize)]
pub struct CreateContract {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub chain_id: i32,
}

impl Model {
    pub async fn create(
        db: &DatabaseConnection,
        data: CreateContract,
    ) -> Result<Self, DbErr> {
        let contract = ActiveModel {
            id: Set(Uuid::new_v4()),
            address: Set(data.address),
            name: Set(data.name),
            symbol: Set(data.symbol),
            chain_id: Set(data.chain_id),
            ..Default::default()
        };

        contract.insert(db).await
    }

    pub async fn get_nfts(&self, db: &DatabaseConnection) -> Result<Vec<super::song_nft::Model>, DbErr> {
        self.find_related(super::song_nft::Entity).all(db).await
    }

    pub async fn find_by_address(db: &DatabaseConnection, address: &str) -> Result<Option<Self>, DbErr> {
        Entity::find()
            .filter(Column::Address.eq(address))
            .one(db)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AppConfig, SecretsManager};
    use crate::db;

    #[tokio::test]
    async fn test_create_contract() {
        let config = AppConfig::new().unwrap();
        let vault_client = config.init_vault_client().await.unwrap();
        let secrets = SecretsManager::new(
            std::sync::Arc::new(vault_client),
            config.vault.mount_path.clone(),
        );
        
        let db = db::create_connection(&config, &secrets)
            .await
            .expect("Failed to connect to database");

        let contract_data = CreateContract {
            address: "0xContract789...".to_string(),
            name: "VibeStream Collection".to_string(),
            symbol: "VIBE".to_string(),
            chain_id: 1,
        };

        let contract = Model::create(&db, contract_data)
            .await
            .expect("Failed to create contract");

        assert_eq!(contract.address, "0xContract789...");
        assert_eq!(contract.name, "VibeStream Collection");
        assert_eq!(contract.symbol, "VIBE");
        assert_eq!(contract.chain_id, 1);

        // Test find by address
        let found_contract = Model::find_by_address(&db, "0xContract789...")
            .await
            .expect("Failed to find contract")
            .expect("Contract not found");

        assert_eq!(found_contract.id, contract.id);
    }
} 