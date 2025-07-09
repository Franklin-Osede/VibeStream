pub mod postgres_repository;
pub mod in_memory_repository;

use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use crate::bounded_contexts::fractional_ownership::domain::{
    repository::OwnershipContractRepository,
    aggregates::OwnershipContract,
    entities::{ShareTransaction, RevenueDistribution},
};
use crate::shared::domain::errors::AppError;

// Implementaciones para Arc<T> para resolver problemas de trait bounds
#[async_trait]
impl<T> OwnershipContractRepository for Arc<T>
where
    T: OwnershipContractRepository + Send + Sync,
{
    async fn save(&self, contract: &OwnershipContract) -> Result<(), AppError> {
        (**self).save(contract).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<OwnershipContract>, AppError> {
        (**self).find_by_id(id).await
    }

    async fn find_by_song_id(&self, song_id: Uuid) -> Result<Option<OwnershipContract>, AppError> {
        (**self).find_by_song_id(song_id).await
    }

    async fn find_by_artist_id(&self, artist_id: Uuid) -> Result<Vec<OwnershipContract>, AppError> {
        (**self).find_by_artist_id(artist_id).await
    }

    async fn find_active_contracts(&self) -> Result<Vec<OwnershipContract>, AppError> {
        (**self).find_active_contracts().await
    }

    async fn find_user_portfolio(&self, user_id: Uuid) -> Result<Vec<ShareTransaction>, AppError> {
        (**self).find_user_portfolio(user_id).await
    }

    async fn save_transaction(&self, transaction: &ShareTransaction) -> Result<(), AppError> {
        (**self).save_transaction(transaction).await
    }

    async fn save_distribution(&self, distribution: &RevenueDistribution) -> Result<(), AppError> {
        (**self).save_distribution(distribution).await
    }
}

// Implementaciones para &T para resolver problemas de trait bounds
#[async_trait]
impl<T> OwnershipContractRepository for &T
where
    T: OwnershipContractRepository + Send + Sync,
{
    async fn save(&self, contract: &OwnershipContract) -> Result<(), AppError> {
        (**self).save(contract).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<OwnershipContract>, AppError> {
        (**self).find_by_id(id).await
    }

    async fn find_by_song_id(&self, song_id: Uuid) -> Result<Option<OwnershipContract>, AppError> {
        (**self).find_by_song_id(song_id).await
    }

    async fn find_by_artist_id(&self, artist_id: Uuid) -> Result<Vec<OwnershipContract>, AppError> {
        (**self).find_by_artist_id(artist_id).await
    }

    async fn find_active_contracts(&self) -> Result<Vec<OwnershipContract>, AppError> {
        (**self).find_active_contracts().await
    }

    async fn find_user_portfolio(&self, user_id: Uuid) -> Result<Vec<ShareTransaction>, AppError> {
        (**self).find_user_portfolio(user_id).await
    }

    async fn save_transaction(&self, transaction: &ShareTransaction) -> Result<(), AppError> {
        (**self).save_transaction(transaction).await
    }

    async fn save_distribution(&self, distribution: &RevenueDistribution) -> Result<(), AppError> {
        (**self).save_distribution(distribution).await
    }
} 