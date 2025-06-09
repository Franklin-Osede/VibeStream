use sea_orm::entity::prelude::*;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::types::DateTimeWithTimeZone;
use crate::utils::{hash_password, verify_password};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[sea_orm(column_type = "Text")]
    pub password_hash: String,
    pub wallet_address: Option<String>,
    pub is_artist: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::playlist::Entity")]
    Playlist,
    #[sea_orm(has_many = "super::artist::Entity")]
    Artist,
}

impl Related<super::playlist::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Playlist.def()
    }
}

impl Related<super::artist::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Artist.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub wallet_address: Option<String>,
}

impl Model {
    pub async fn create(
        db: &DatabaseConnection,
        data: CreateUser,
    ) -> Result<Self, DbErr> {
        let password_hash = hash_password(&data.password)
            .map_err(|e| DbErr::Custom(format!("Failed to hash password: {}", e)))?;

        let user = ActiveModel {
            id: Set(Uuid::new_v4()),
            username: Set(data.username),
            email: Set(data.email),
            password_hash: Set(password_hash),
            wallet_address: Set(data.wallet_address),
            ..Default::default()
        };

        user.insert(db).await
    }

    pub async fn verify_password(&self, password: &str) -> bool {
        verify_password(password, &self.password_hash)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AppConfig, SecretsManager};
    use crate::db;

    #[tokio::test]
    async fn test_create_user() {
        let config = AppConfig::new().unwrap();
        let vault_client = config.init_vault_client().await.unwrap();
        let secrets = SecretsManager::new(
            std::sync::Arc::new(vault_client),
            config.vault.mount_path.clone(),
        );
        
        let db = db::create_connection(&config, &secrets)
            .await
            .expect("Failed to connect to database");

        let user_data = CreateUser {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            wallet_address: Some("0x123...".to_string()),
        };

        let user = Model::create(&db, user_data)
            .await
            .expect("Failed to create user");

        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
        assert!(user.verify_password("password123").await);
    }
} 