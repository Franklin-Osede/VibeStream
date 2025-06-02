use sea_orm::entity::prelude::*;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "royalty_payments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub song_nft_id: Uuid,
    pub amount: f64,
    pub paid_at: DateTimeWithTimeZone,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::song_nft::Entity",
        from = "Column::SongNftId",
        to = "super::song_nft::Column::Id"
    )]
    SongNft,
}

impl Related<super::song_nft::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SongNft.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize)]
pub struct CreateRoyaltyPayment {
    pub song_nft_id: Uuid,
    pub amount: f64,
}

impl Model {
    pub async fn create(
        db: &DatabaseConnection,
        data: CreateRoyaltyPayment,
    ) -> Result<Self, DbErr> {
        let now = Utc::now();
        let payment = ActiveModel {
            id: Set(Uuid::new_v4()),
            song_nft_id: Set(data.song_nft_id),
            amount: Set(data.amount),
            paid_at: Set(now.into()),
            created_at: Set(now.into()),
            updated_at: Set(now.into()),
        };

        payment.insert(db).await
    }

    pub async fn get_nft(&self, db: &DatabaseConnection) -> Result<super::song_nft::Model, DbErr> {
        self.find_related(super::song_nft::Entity).one(db).await?
            .ok_or(DbErr::Custom("NFT not found".to_string()))
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
            contract::{self, CreateContract},
            song_nft::{self, CreateSongNft},
        },
    };
    use rust_decimal_macros::dec;

    #[tokio::test]
    async fn test_create_royalty_payment() {
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
            username: "artist4".to_string(),
            email: "artist4@example.com".to_string(),
            password: "password123".to_string(),
            wallet_address: Some("0x123...".to_string()),
        })
        .await
        .expect("Failed to create user");

        let artist = artist::Model::create(&db, CreateArtist {
            user_id: user.id,
            name: "Artist Four".to_string(),
            bio: None,
            profile_image_url: None,
        })
        .await
        .expect("Failed to create artist");

        let song = song::Model::create(&db, CreateSong {
            title: "Royalty Song".to_string(),
            artist_id: artist.id,
            duration_seconds: 300,
            genre: Some("Jazz".to_string()),
            ipfs_hash: "QmHash789...".to_string(),
            cover_art_url: None,
        })
        .await
        .expect("Failed to create song");

        let contract = contract::Model::create(&db, CreateContract {
            address: "0xContract456...".to_string(),
            name: "VibeStream Royalties".to_string(),
            symbol: "VBROY".to_string(),
            chain_id: 1,
        })
        .await
        .expect("Failed to create contract");

        let nft = song_nft::Model::create(&db, CreateSongNft {
            song_id: song.id,
            contract_id: contract.id,
            token_id: 1,
            royalty_percentage: dec!(5.0),
            owner_address: "0xOwner456...".to_string(),
        })
        .await
        .expect("Failed to create song NFT");

        // Create royalty payment
        let payment_data = CreateRoyaltyPayment {
            song_nft_id: nft.id,
            amount: 0.5,
        };

        let payment = Model::create(&db, payment_data)
            .await
            .expect("Failed to create royalty payment");

        assert_eq!(payment.song_nft_id, nft.id);
        assert_eq!(payment.amount, 0.5);
    }
} 