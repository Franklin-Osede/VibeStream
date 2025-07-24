// Re-export implementations from parent directory
pub use super::postgres_repository;
pub use super::in_memory_repository;

use std::sync::Arc;
use async_trait::async_trait;
use crate::bounded_contexts::fractional_ownership::domain::{
    repository::OwnershipContractRepository,
    aggregates::{OwnershipContractAggregate, OwnershipAnalytics},
    value_objects::OwnershipContractId,
};
use crate::bounded_contexts::music::domain::value_objects::{SongId, ArtistId};
use crate::bounded_contexts::user::domain::value_objects::UserId;
use crate::shared::domain::repositories::RepoResult;

// Implementaciones para Arc<T> para resolver problemas de trait bounds
#[async_trait]
impl<T> OwnershipContractRepository for Arc<T>
where
    T: OwnershipContractRepository + Send + Sync,
{
    async fn save(&self, aggregate: &OwnershipContractAggregate) -> RepoResult<()> {
        (**self).save(aggregate).await
    }

    async fn update(&self, aggregate: &OwnershipContractAggregate) -> RepoResult<()> {
        (**self).update(aggregate).await
    }

    async fn find_by_id(&self, id: &OwnershipContractId) -> RepoResult<Option<OwnershipContractAggregate>> {
        (**self).find_by_id(id).await
    }

    async fn find_by_song_id(&self, song_id: &SongId) -> RepoResult<Vec<OwnershipContractAggregate>> {
        (**self).find_by_song_id(song_id).await
    }

    async fn find_by_artist_id(&self, artist_id: &ArtistId) -> RepoResult<Vec<OwnershipContractAggregate>> {
        (**self).find_by_artist_id(artist_id).await
    }

    async fn find_active_contracts(&self) -> RepoResult<Vec<OwnershipContractAggregate>> {
        (**self).find_active_contracts().await
    }

    async fn find_contracts_with_user_shares(&self, user_id: &UserId) -> RepoResult<Vec<OwnershipContractAggregate>> {
        (**self).find_contracts_with_user_shares(user_id).await
    }

    async fn find_by_status(&self, status: &str) -> RepoResult<Vec<OwnershipContractAggregate>> {
        (**self).find_by_status(status).await
    }

    async fn exists_for_song(&self, song_id: &SongId) -> RepoResult<bool> {
        (**self).exists_for_song(song_id).await
    }

    async fn delete(&self, id: &OwnershipContractId) -> RepoResult<()> {
        (**self).delete(id).await
    }

    async fn get_contract_analytics(&self, id: &OwnershipContractId) -> RepoResult<Option<OwnershipAnalytics>> {
        (**self).get_contract_analytics(id).await
    }

    async fn find_paginated(&self, offset: u32, limit: u32) -> RepoResult<(Vec<OwnershipContractAggregate>, u64)> {
        (**self).find_paginated(offset, limit).await
    }

    async fn find_by_completion_range(&self, min_percentage: f64, max_percentage: f64) -> RepoResult<Vec<OwnershipContractAggregate>> {
        (**self).find_by_completion_range(min_percentage, max_percentage).await
    }

    async fn get_total_market_value(&self) -> RepoResult<f64> {
        (**self).get_total_market_value().await
    }
}

// Implementaciones para &T para resolver problemas de trait bounds
#[async_trait]
impl<T> OwnershipContractRepository for &T
where
    T: OwnershipContractRepository + Send + Sync,
{
    async fn save(&self, aggregate: &OwnershipContractAggregate) -> RepoResult<()> {
        (**self).save(aggregate).await
    }

    async fn update(&self, aggregate: &OwnershipContractAggregate) -> RepoResult<()> {
        (**self).update(aggregate).await
    }

    async fn find_by_id(&self, id: &OwnershipContractId) -> RepoResult<Option<OwnershipContractAggregate>> {
        (**self).find_by_id(id).await
    }

    async fn find_by_song_id(&self, song_id: &SongId) -> RepoResult<Vec<OwnershipContractAggregate>> {
        (**self).find_by_song_id(song_id).await
    }

    async fn find_by_artist_id(&self, artist_id: &ArtistId) -> RepoResult<Vec<OwnershipContractAggregate>> {
        (**self).find_by_artist_id(artist_id).await
    }

    async fn find_active_contracts(&self) -> RepoResult<Vec<OwnershipContractAggregate>> {
        (**self).find_active_contracts().await
    }

    async fn find_contracts_with_user_shares(&self, user_id: &UserId) -> RepoResult<Vec<OwnershipContractAggregate>> {
        (**self).find_contracts_with_user_shares(user_id).await
    }

    async fn find_by_status(&self, status: &str) -> RepoResult<Vec<OwnershipContractAggregate>> {
        (**self).find_by_status(status).await
    }

    async fn exists_for_song(&self, song_id: &SongId) -> RepoResult<bool> {
        (**self).exists_for_song(song_id).await
    }

    async fn delete(&self, id: &OwnershipContractId) -> RepoResult<()> {
        (**self).delete(id).await
    }

    async fn get_contract_analytics(&self, id: &OwnershipContractId) -> RepoResult<Option<OwnershipAnalytics>> {
        (**self).get_contract_analytics(id).await
    }

    async fn find_paginated(&self, offset: u32, limit: u32) -> RepoResult<(Vec<OwnershipContractAggregate>, u64)> {
        (**self).find_paginated(offset, limit).await
    }

    async fn find_by_completion_range(&self, min_percentage: f64, max_percentage: f64) -> RepoResult<Vec<OwnershipContractAggregate>> {
        (**self).find_by_completion_range(min_percentage, max_percentage).await
    }

    async fn get_total_market_value(&self) -> RepoResult<f64> {
        (**self).get_total_market_value().await
    }
} 