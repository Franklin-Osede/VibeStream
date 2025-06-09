use sea_orm::entity::prelude::*;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::types::DateTimeWithTimeZone;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "artists")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub bio: Option<String>,
    pub profile_image: Option<String>,
    pub verified: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::user::Entity")]
    User,
    #[sea_orm(has_many = "super::song::Entity")]
    Song,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::song::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Song.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize)]
pub struct CreateArtist {
    pub user_id: Uuid,
    pub name: String,
    pub bio: Option<String>,
    pub profile_image_url: Option<String>,
}

impl Model {
    pub async fn create(
        db: &DatabaseConnection,
        data: CreateArtist,
    ) -> Result<Self, DbErr> {
        let artist = ActiveModel {
            id: Set(data.user_id),
            name: Set(data.name),
            bio: Set(data.bio),
            profile_image: Set(data.profile_image_url),
            verified: Set(false),
            ..Default::default()
        };

        artist.insert(db).await
    }

    pub async fn verify(&mut self, db: &DatabaseConnection) -> Result<Self, DbErr> {
        let mut artist: ActiveModel = self.clone().into();
        artist.verified = Set(true);
        artist.update(db).await
    }

    pub async fn get_songs(&self, db: &DatabaseConnection) -> Result<Vec<super::song::Model>, DbErr> {
        self.find_related(super::song::Entity).all(db).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AppConfig, SecretsManager};
    use crate::db::{self, models::user::{self, CreateUser}};

    #[tokio::test]
    async fn test_create_artist() {
        let config = AppConfig::new().unwrap();
        let vault_client = config.init_vault_client().await.unwrap();
        let secrets = SecretsManager::new(
            std::sync::Arc::new(vault_client),
            config.vault.mount_path.clone(),
        );
        
        let db = db::create_connection(&config, &secrets)
            .await
            .expect("Failed to connect to database");

        // First create a user
        let user_data = CreateUser {
            username: "artist1".to_string(),
            email: "artist1@example.com".to_string(),
            password: "password123".to_string(),
            wallet_address: Some("0x123...".to_string()),
        };

        let user = user::Model::create(&db, user_data)
            .await
            .expect("Failed to create user");

        // Then create an artist profile
        let artist_data = CreateArtist {
            user_id: user.id,
            name: "Artist One".to_string(),
            bio: Some("A great musician".to_string()),
            profile_image_url: Some("https://example.com/image.jpg".to_string()),
        };

        let artist = Model::create(&db, artist_data)
            .await
            .expect("Failed to create artist");

        assert_eq!(artist.name, "Artist One");
        assert_eq!(artist.bio.unwrap(), "A great musician");
        assert!(!artist.verified);
    }
} 