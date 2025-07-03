use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use async_trait::async_trait;
use uuid::Uuid;

use crate::shared::domain::errors::AppError;
use crate::shared::domain::repositories::RepoResult;
use crate::bounded_contexts::{
    music::domain::value_objects::{SongId, ArtistId},
    user::domain::value_objects::UserId,
    fractional_ownership::domain::{
        repository::{OwnershipContractRepository, ShareRepository},
        value_objects::{OwnershipContractId, ShareId},
        aggregates::{OwnershipContractAggregate, OwnershipAnalytics},
        entities::FractionalShare,
    },
};

/// In-memory implementation of OwnershipContractRepository for testing
#[derive(Debug, Clone)]
pub struct InMemoryOwnershipContractRepository {
    contracts: Arc<RwLock<HashMap<Uuid, OwnershipContractAggregate>>>,
    shares: Arc<RwLock<HashMap<Uuid, FractionalShare>>>,
}

impl InMemoryOwnershipContractRepository {
    pub fn new() -> Self {
        Self {
            contracts: Arc::new(RwLock::new(HashMap::new())),
            shares: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl OwnershipContractRepository for InMemoryOwnershipContractRepository {
    async fn save(&self, aggregate: &OwnershipContractAggregate) -> RepoResult<()> {
        let mut contracts = self.contracts.write().unwrap();
        contracts.insert(aggregate.id().value(), aggregate.clone());
        Ok(())
    }

    async fn update(&self, aggregate: &OwnershipContractAggregate) -> RepoResult<()> {
        let mut contracts = self.contracts.write().unwrap();
        contracts.insert(aggregate.id().value(), aggregate.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &OwnershipContractId) -> RepoResult<Option<OwnershipContractAggregate>> {
        let contracts = self.contracts.read().unwrap();
        Ok(contracts.get(&id.value()).cloned())
    }

    async fn find_by_song_id(&self, song_id: &SongId) -> RepoResult<Vec<OwnershipContractAggregate>> {
        let contracts = self.contracts.read().unwrap();
        let results: Vec<OwnershipContractAggregate> = contracts
            .values()
            .filter(|contract| contract.song_id().value() == song_id.value())
            .cloned()
            .collect();
        Ok(results)
    }

    async fn find_by_artist_id(&self, artist_id: &ArtistId) -> RepoResult<Vec<OwnershipContractAggregate>> {
        let contracts = self.contracts.read().unwrap();
        let results: Vec<OwnershipContractAggregate> = contracts
            .values()
            .filter(|contract| contract.artist_id().value() == artist_id.value())
            .cloned()
            .collect();
        Ok(results)
    }

    async fn find_active_contracts(&self) -> RepoResult<Vec<OwnershipContractAggregate>> {
        let contracts = self.contracts.read().unwrap();
        let results: Vec<OwnershipContractAggregate> = contracts
            .values()
            .filter(|contract| contract.is_active())
            .cloned()
            .collect();
        Ok(results)
    }

    async fn find_contracts_with_user_shares(&self, user_id: &UserId) -> RepoResult<Vec<OwnershipContractAggregate>> {
        // This would require checking shares table in real implementation
        let contracts = self.contracts.read().unwrap();
        Ok(contracts.values().cloned().collect())
    }

    async fn find_by_status(&self, status: &str) -> RepoResult<Vec<OwnershipContractAggregate>> {
        let contracts = self.contracts.read().unwrap();
        let results: Vec<OwnershipContractAggregate> = contracts
            .values()
            .filter(|contract| contract.status().to_string() == status)
            .cloned()
            .collect();
        Ok(results)
    }

    async fn exists_for_song(&self, song_id: &SongId) -> RepoResult<bool> {
        let contracts = self.contracts.read().unwrap();
        let exists = contracts
            .values()
            .any(|contract| contract.song_id().value() == song_id.value());
        Ok(exists)
    }

    async fn delete(&self, id: &OwnershipContractId) -> RepoResult<()> {
        let mut contracts = self.contracts.write().unwrap();
        contracts.remove(&id.value());
        Ok(())
    }

    async fn get_contract_analytics(&self, id: &OwnershipContractId) -> RepoResult<Option<OwnershipAnalytics>> {
        // Mock analytics data
        if self.find_by_id(id).await?.is_some() {
            Ok(Some(OwnershipAnalytics::default()))
        } else {
            Ok(None)
        }
    }

    async fn find_paginated(&self, offset: u32, limit: u32) -> RepoResult<(Vec<OwnershipContractAggregate>, u64)> {
        let contracts = self.contracts.read().unwrap();
        let all_contracts: Vec<OwnershipContractAggregate> = contracts.values().cloned().collect();
        let total_count = all_contracts.len() as u64;
        
        let start = offset as usize;
        let end = std::cmp::min(start + limit as usize, all_contracts.len());
        let page_contracts = if start < all_contracts.len() {
            all_contracts[start..end].to_vec()
        } else {
            Vec::new()
        };
        
        Ok((page_contracts, total_count))
    }

    async fn find_by_completion_range(&self, min_percentage: f64, max_percentage: f64) -> RepoResult<Vec<OwnershipContractAggregate>> {
        let contracts = self.contracts.read().unwrap();
        let results: Vec<OwnershipContractAggregate> = contracts
            .values()
            .filter(|contract| {
                let completion = contract.completion_percentage();
                completion >= min_percentage && completion <= max_percentage
            })
            .cloned()
            .collect();
        Ok(results)
    }

    async fn get_total_market_value(&self) -> RepoResult<f64> {
        let contracts = self.contracts.read().unwrap();
        let total_value: f64 = contracts
            .values()
            .map(|contract| contract.total_value())
            .sum();
        Ok(total_value)
    }
}

/// In-memory implementation of ShareRepository for testing
#[derive(Debug, Clone)]
pub struct InMemoryShareRepository {
    shares: Arc<RwLock<HashMap<Uuid, FractionalShare>>>,
}

impl InMemoryShareRepository {
    pub fn new() -> Self {
        Self {
            shares: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl ShareRepository for InMemoryShareRepository {
    async fn find_by_id(&self, id: &ShareId) -> RepoResult<Option<FractionalShare>> {
        let shares = self.shares.read().unwrap();
        Ok(shares.get(&id.value()).cloned())
    }

    async fn find_by_owner(&self, user_id: &UserId) -> RepoResult<Vec<FractionalShare>> {
        let shares = self.shares.read().unwrap();
        let results: Vec<FractionalShare> = shares
            .values()
            .filter(|share| share.owner_id().value() == user_id.value())
            .cloned()
            .collect();
        Ok(results)
    }

    async fn find_by_contract(&self, contract_id: &OwnershipContractId) -> RepoResult<Vec<FractionalShare>> {
        let shares = self.shares.read().unwrap();
        let results: Vec<FractionalShare> = shares
            .values()
            .filter(|share| share.contract_id().value() == contract_id.value())
            .cloned()
            .collect();
        Ok(results)
    }

    async fn find_tradeable_shares(&self) -> RepoResult<Vec<FractionalShare>> {
        let shares = self.shares.read().unwrap();
        let results: Vec<FractionalShare> = shares
            .values()
            .filter(|share| share.is_tradeable())
            .cloned()
            .collect();
        Ok(results)
    }

    async fn find_by_song(&self, song_id: &SongId) -> RepoResult<Vec<FractionalShare>> {
        let shares = self.shares.read().unwrap();
        let results: Vec<FractionalShare> = shares
            .values()
            .filter(|share| share.song_id().value() == song_id.value())
            .cloned()
            .collect();
        Ok(results)
    }

    async fn find_locked_shares(&self) -> RepoResult<Vec<FractionalShare>> {
        let shares = self.shares.read().unwrap();
        let results: Vec<FractionalShare> = shares
            .values()
            .filter(|share| share.is_locked())
            .cloned()
            .collect();
        Ok(results)
    }

    async fn update(&self, share: &FractionalShare) -> RepoResult<()> {
        let mut shares = self.shares.write().unwrap();
        shares.insert(share.id().value(), share.clone());
        Ok(())
    }

    async fn get_user_portfolio_value(&self, user_id: &UserId) -> RepoResult<f64> {
        let shares = self.shares.read().unwrap();
        let total_value: f64 = shares
            .values()
            .filter(|share| share.owner_id().value() == user_id.value())
            .map(|share| share.current_market_value().value())
            .sum();
        Ok(total_value)
    }

    async fn find_shares_vesting_soon(&self, days: u32) -> RepoResult<Vec<FractionalShare>> {
        let shares = self.shares.read().unwrap();
        let cutoff_date = chrono::Utc::now() + chrono::Duration::days(days as i64);
        
        let results: Vec<FractionalShare> = shares
            .values()
            .filter(|share| {
                if let Some(locked_until) = share.locked_until() {
                    locked_until <= cutoff_date
                } else {
                    false
                }
            })
            .cloned()
            .collect();
        Ok(results)
    }
} 