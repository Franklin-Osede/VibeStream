use sea_orm::{DatabaseConnection, DbErr, ActiveValue::Set};
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::{
    db::models::royalty::{self, Model as Royalty, ActiveModel},
    error::AppError,
    repositories::royalty::RoyaltyRepository,
};

pub struct RoyaltyService {
    royalty_repository: Box<dyn RoyaltyRepository>,
}

impl RoyaltyService {
    pub fn new(royalty_repository: Box<dyn RoyaltyRepository>) -> Self {
        Self { royalty_repository }
    }

    pub async fn create_royalty_payment(
        &self,
        db: &DatabaseConnection,
        nft_id: Uuid,
        amount: Decimal,
        transaction_hash: String,
    ) -> Result<Royalty, AppError> {
        let royalty = ActiveModel {
            id: Set(Uuid::new_v4()),
            nft_id: Set(nft_id),
            amount: Set(amount),
            transaction_hash: Set(transaction_hash),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
        };

        self.royalty_repository
            .create(db, royalty)
            .await
            .map_err(AppError::DatabaseError)
    }

    pub async fn get_royalty_payment(
        &self,
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<Royalty>, AppError> {
        self.royalty_repository
            .find_by_id(db, id)
            .await
            .map_err(AppError::DatabaseError)
    }

    pub async fn get_nft_royalties(
        &self,
        db: &DatabaseConnection,
        nft_id: Uuid,
    ) -> Result<Vec<Royalty>, AppError> {
        self.royalty_repository
            .find_by_nft(db, nft_id)
            .await
            .map_err(AppError::DatabaseError)
    }

    pub async fn get_total_royalties(
        &self,
        db: &DatabaseConnection,
        nft_id: Uuid,
    ) -> Result<Decimal, AppError> {
        self.royalty_repository
            .get_total_amount(db, nft_id)
            .await
            .map_err(AppError::DatabaseError)
    }
} 