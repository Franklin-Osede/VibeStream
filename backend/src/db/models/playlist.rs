use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "playlists", schema_name = "music")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub user_id: Uuid,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    pub is_public: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::user::Entity")]
    User,
    #[sea_orm(has_many = "super::playlist_song::Entity")]
    PlaylistSongs,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::playlist_song::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlaylistSongs.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize)]
pub struct CreatePlaylist {
    pub name: String,
    pub user_id: Uuid,
    pub description: Option<String>,
    pub is_public: Option<bool>,
}

impl Model {
    pub async fn create(
        db: &DatabaseConnection,
        data: CreatePlaylist,
    ) -> Result<Self, DbErr> {
        let playlist = ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(data.name),
            user_id: Set(data.user_id),
            description: Set(data.description),
            is_public: Set(data.is_public.unwrap_or(true)),
            ..Default::default()
        };

        playlist.insert(db).await
    }

    pub async fn get_songs(&self, db: &DatabaseConnection) -> Result<Vec<(super::playlist_song::Model, super::song::Model)>, DbErr> {
        let playlist_songs = self.find_related(super::playlist_song::Entity)
            .order_by_asc(super::playlist_song::Column::Position)
            .find_with_related(super::song::Entity)
            .all(db)
            .await?;

        Ok(playlist_songs.into_iter().filter_map(|(ps, songs)| {
            songs.first().map(|s| (ps, s.clone()))
        }).collect())
    }

    pub async fn add_song(
        &self,
        db: &DatabaseConnection,
        song_id: Uuid,
    ) -> Result<super::playlist_song::Model, DbErr> {
        // Get the next position
        let next_position = self.find_related(super::playlist_song::Entity)
            .all(db)
            .await?
            .len() as i32 + 1;

        super::playlist_song::Model::create(db, super::playlist_song::CreatePlaylistSong {
            playlist_id: self.id,
            song_id,
            position: next_position,
        }).await
    }

    pub async fn remove_song(
        &self,
        db: &DatabaseConnection,
        song_id: Uuid,
    ) -> Result<(), DbErr> {
        super::playlist_song::Entity::delete_many()
            .filter(
                super::playlist_song::Column::PlaylistId.eq(self.id)
                    .and(super::playlist_song::Column::SongId.eq(song_id))
            )
            .exec(db)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AppConfig, SecretsManager};
    use crate::db::{
        self,
        models::{
            user::{self, CreateUser},
            artist::{self, CreateArtist},
            song::{self, CreateSong},
        },
    };

    #[tokio::test]
    async fn test_playlist_operations() {
        let config = AppConfig::new().unwrap();
        let vault_client = config.init_vault_client().await.unwrap();
        let secrets = SecretsManager::new(
            std::sync::Arc::new(vault_client),
            config.vault.mount_path.clone(),
        );
        
        let db = db::create_connection(&config, &secrets)
            .await
            .expect("Failed to connect to database");

        // Create user and artist
        let user = user::Model::create(&db, CreateUser {
            username: "user1".to_string(),
            email: "user1@example.com".to_string(),
            password: "password123".to_string(),
            wallet_address: None,
        })
        .await
        .expect("Failed to create user");

        let artist = artist::Model::create(&db, CreateArtist {
            user_id: user.id,
            name: "Test Artist".to_string(),
            bio: None,
            profile_image_url: None,
        })
        .await
        .expect("Failed to create artist");

        // Create songs
        let song1 = song::Model::create(&db, CreateSong {
            title: "Song One".to_string(),
            artist_id: artist.id,
            duration_seconds: 180,
            genre: Some("Pop".to_string()),
            ipfs_hash: "QmHash1...".to_string(),
            cover_art_url: None,
        })
        .await
        .expect("Failed to create song");

        let song2 = song::Model::create(&db, CreateSong {
            title: "Song Two".to_string(),
            artist_id: artist.id,
            duration_seconds: 240,
            genre: Some("Rock".to_string()),
            ipfs_hash: "QmHash2...".to_string(),
            cover_art_url: None,
        })
        .await
        .expect("Failed to create song");

        // Create playlist
        let playlist_data = CreatePlaylist {
            name: "My Playlist".to_string(),
            user_id: user.id,
            description: Some("A test playlist".to_string()),
            is_public: Some(true),
        };

        let playlist = Model::create(&db, playlist_data)
            .await
            .expect("Failed to create playlist");

        assert_eq!(playlist.name, "My Playlist");
        assert_eq!(playlist.user_id, user.id);

        // Add songs to playlist
        let ps1 = playlist.add_song(&db, song1.id)
            .await
            .expect("Failed to add song 1");

        let ps2 = playlist.add_song(&db, song2.id)
            .await
            .expect("Failed to add song 2");

        assert_eq!(ps1.position, 1);
        assert_eq!(ps2.position, 2);

        // Get playlist songs
        let songs = playlist.get_songs(&db)
            .await
            .expect("Failed to get playlist songs");

        assert_eq!(songs.len(), 2);
        assert_eq!(songs[0].1.title, "Song One");
        assert_eq!(songs[1].1.title, "Song Two");

        // Remove a song
        playlist.remove_song(&db, song1.id)
            .await
            .expect("Failed to remove song");

        let remaining_songs = playlist.get_songs(&db)
            .await
            .expect("Failed to get remaining songs");

        assert_eq!(remaining_songs.len(), 1);
        assert_eq!(remaining_songs[0].1.title, "Song Two");
    }
} 