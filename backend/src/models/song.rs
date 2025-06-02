use sea_orm::entity::prelude::*;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "songs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub title: String,
    pub artist_id: Uuid,
    pub duration: i32,
    pub genre: Option<String>,
    pub release_date: DateTimeWithTimeZone,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::artist::Entity",
        from = "Column::ArtistId",
        to = "super::artist::Column::Id"
    )]
    Artist,
    #[sea_orm(has_many = "super::song_nft::Entity")]
    SongNfts,
    #[sea_orm(has_many = "super::playlist_song::Entity")]
    PlaylistSongs,
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

impl Related<super::playlist_song::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlaylistSongs.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize)]
pub struct CreateSong {
    pub title: String,
    pub artist_id: Uuid,
    pub duration_seconds: i32,
    pub genre: Option<String>,
}

impl Model {
    pub async fn create(
        db: &DatabaseConnection,
        data: CreateSong,
    ) -> Result<Self, DbErr> {
        let now = Utc::now();
        let song = ActiveModel {
            id: Set(Uuid::new_v4()),
            title: Set(data.title),
            artist_id: Set(data.artist_id),
            duration: Set(data.duration_seconds),
            genre: Set(data.genre),
            release_date: Set(now.into()),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
            ..Default::default()
        };

        song.insert(db).await
    }

    pub async fn get_artist(&self, db: &DatabaseConnection) -> Result<super::artist::Model, DbErr> {
        self.find_related(super::artist::Entity).one(db).await?
            .ok_or(DbErr::Custom("Artist not found".to_string()))
    }

    pub async fn get_nfts(&self, db: &DatabaseConnection) -> Result<Vec<super::song_nft::Model>, DbErr> {
        self.find_related(super::song_nft::Entity).all(db).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AppConfig, SecretsManager};
    use crate::db::{self, models::{user::{self, CreateUser}, artist::{self, CreateArtist}}};

    #[tokio::test]
    async fn test_create_song() {
        let config = AppConfig::new().unwrap();
        let vault_client = config.init_vault_client().await.unwrap();
        let secrets = SecretsManager::new(
            std::sync::Arc::new(vault_client),
            config.vault.mount_path.clone(),
        );
        
        let db = db::create_connection(&config, &secrets)
            .await
            .expect("Failed to connect to database");

        // Create user and artist first
        let user_data = CreateUser {
            username: "artist2".to_string(),
            email: "artist2@example.com".to_string(),
            password: "password123".to_string(),
            wallet_address: Some("0x123...".to_string()),
        };

        let user = user::Model::create(&db, user_data)
            .await
            .expect("Failed to create user");

        let artist_data = CreateArtist {
            user_id: user.id,
            name: "Artist Two".to_string(),
            bio: Some("Another great musician".to_string()),
            profile_image_url: None,
        };

        let artist = artist::Model::create(&db, artist_data)
            .await
            .expect("Failed to create artist");

        // Create a song
        let song_data = CreateSong {
            title: "My First Song".to_string(),
            artist_id: artist.id,
            duration_seconds: 180,
            genre: Some("Pop".to_string()),
        };

        let song = Model::create(&db, song_data)
            .await
            .expect("Failed to create song");

        assert_eq!(song.title, "My First Song");
        assert_eq!(song.artist_id, artist.id);
        assert_eq!(song.duration, 180);
        assert_eq!(song.genre.unwrap(), "Pop");
    }
} 