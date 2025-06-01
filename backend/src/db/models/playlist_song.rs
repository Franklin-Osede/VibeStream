use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "playlist_songs", schema_name = "music")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub playlist_id: Uuid,
    #[sea_orm(primary_key)]
    pub song_id: Uuid,
    pub position: i32,
    pub added_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::playlist::Entity")]
    Playlist,
    #[sea_orm(belongs_to = "super::song::Entity")]
    Song,
}

impl Related<super::playlist::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Playlist.def()
    }
}

impl Related<super::song::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Song.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize)]
pub struct CreatePlaylistSong {
    pub playlist_id: Uuid,
    pub song_id: Uuid,
    pub position: i32,
}

impl Model {
    pub async fn create(
        db: &DatabaseConnection,
        data: CreatePlaylistSong,
    ) -> Result<Self, DbErr> {
        let playlist_song = ActiveModel {
            playlist_id: Set(data.playlist_id),
            song_id: Set(data.song_id),
            position: Set(data.position),
            added_at: Set(chrono::Utc::now().into()),
        };

        playlist_song.insert(db).await
    }

    pub async fn reorder(
        db: &DatabaseConnection,
        playlist_id: Uuid,
        song_id: Uuid,
        new_position: i32,
    ) -> Result<(), DbErr> {
        let mut playlist_song: ActiveModel = Self::find_by_id((playlist_id, song_id))
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Playlist song not found".to_string()))?
            .into();

        let old_position = playlist_song.position.as_ref().clone();
        playlist_song.position = Set(new_position);

        // Update positions of other songs
        if new_position > old_position {
            Entity::update_many()
                .filter(
                    Column::PlaylistId.eq(playlist_id)
                        .and(Column::Position.gt(old_position))
                        .and(Column::Position.lte(new_position))
                )
                .col_expr(Column::Position, Expr::col(Column::Position).sub(1))
                .exec(db)
                .await?;
        } else {
            Entity::update_many()
                .filter(
                    Column::PlaylistId.eq(playlist_id)
                        .and(Column::Position.lt(old_position))
                        .and(Column::Position.gte(new_position))
                )
                .col_expr(Column::Position, Expr::col(Column::Position).add(1))
                .exec(db)
                .await?;
        }

        playlist_song.update(db).await?;
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
            playlist::{self, CreatePlaylist},
        },
    };

    #[tokio::test]
    async fn test_playlist_song_operations() {
        let config = AppConfig::new().unwrap();
        let vault_client = config.init_vault_client().await.unwrap();
        let secrets = SecretsManager::new(
            std::sync::Arc::new(vault_client),
            config.vault.mount_path.clone(),
        );
        
        let db = db::create_connection(&config, &secrets)
            .await
            .expect("Failed to connect to database");

        // Create prerequisites
        let user = user::Model::create(&db, CreateUser {
            username: "user2".to_string(),
            email: "user2@example.com".to_string(),
            password: "password123".to_string(),
            wallet_address: None,
        })
        .await
        .expect("Failed to create user");

        let artist = artist::Model::create(&db, CreateArtist {
            user_id: user.id,
            name: "Test Artist 2".to_string(),
            bio: None,
            profile_image_url: None,
        })
        .await
        .expect("Failed to create artist");

        let song1 = song::Model::create(&db, CreateSong {
            title: "Test Song 1".to_string(),
            artist_id: artist.id,
            duration_seconds: 180,
            genre: None,
            ipfs_hash: "QmHash1...".to_string(),
            cover_art_url: None,
        })
        .await
        .expect("Failed to create song 1");

        let song2 = song::Model::create(&db, CreateSong {
            title: "Test Song 2".to_string(),
            artist_id: artist.id,
            duration_seconds: 200,
            genre: None,
            ipfs_hash: "QmHash2...".to_string(),
            cover_art_url: None,
        })
        .await
        .expect("Failed to create song 2");

        let playlist = playlist::Model::create(&db, CreatePlaylist {
            name: "Test Playlist".to_string(),
            user_id: user.id,
            description: None,
            is_public: None,
        })
        .await
        .expect("Failed to create playlist");

        // Test creating playlist songs
        let ps1 = Model::create(&db, CreatePlaylistSong {
            playlist_id: playlist.id,
            song_id: song1.id,
            position: 1,
        })
        .await
        .expect("Failed to create playlist song 1");

        let ps2 = Model::create(&db, CreatePlaylistSong {
            playlist_id: playlist.id,
            song_id: song2.id,
            position: 2,
        })
        .await
        .expect("Failed to create playlist song 2");

        assert_eq!(ps1.position, 1);
        assert_eq!(ps2.position, 2);

        // Test reordering
        Model::reorder(&db, playlist.id, song2.id, 1)
            .await
            .expect("Failed to reorder songs");

        let updated_ps1 = Model::find_by_id((playlist.id, song1.id))
            .one(db)
            .await
            .expect("Failed to find playlist song 1")
            .expect("Playlist song 1 not found");

        let updated_ps2 = Model::find_by_id((playlist.id, song2.id))
            .one(db)
            .await
            .expect("Failed to find playlist song 2")
            .expect("Playlist song 2 not found");

        assert_eq!(updated_ps1.position, 2);
        assert_eq!(updated_ps2.position, 1);
    }
} 