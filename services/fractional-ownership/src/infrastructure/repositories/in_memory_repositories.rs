// Repositorio en memoria para testing
use crate::domain::aggregates::FractionalOwnershipAggregate;
use crate::domain::entities::ShareOwnership;
use crate::domain::repositories::FractionalOwnershipRepository;
use crate::domain::errors::FractionalOwnershipError;
use crate::domain::value_objects::RevenueAmount;
use async_trait::async_trait;
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct InMemoryFractionalOwnershipRepository {
    aggregates: Arc<Mutex<HashMap<Uuid, FractionalOwnershipAggregate>>>,
}

impl InMemoryFractionalOwnershipRepository {
    pub fn new() -> Self {
        Self {
            aggregates: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_aggregate(&self, aggregate: FractionalOwnershipAggregate) {
        let mut aggregates = self.aggregates.lock().await;
        aggregates.insert(aggregate.fractional_song().id(), aggregate);
    }
}

#[async_trait]
impl FractionalOwnershipRepository for InMemoryFractionalOwnershipRepository {
    async fn get_by_id(&self, song_id: Uuid) -> Result<Option<FractionalOwnershipAggregate>, FractionalOwnershipError> {
        let aggregates = self.aggregates.lock().await;
        Ok(aggregates.get(&song_id).cloned())
    }

    async fn load_aggregate(&self, song_id: &Uuid) -> Result<Option<FractionalOwnershipAggregate>, FractionalOwnershipError> {
        let aggregates = self.aggregates.lock().await;
        Ok(aggregates.get(song_id).cloned())
    }

    async fn save_aggregate(&self, aggregate: &FractionalOwnershipAggregate) -> Result<(), FractionalOwnershipError> {
        let mut aggregates = self.aggregates.lock().await;
        aggregates.insert(aggregate.fractional_song().id(), aggregate.clone());
        Ok(())
    }

    async fn save(&self, aggregate: &FractionalOwnershipAggregate) -> Result<(), FractionalOwnershipError> {
        self.save_aggregate(aggregate).await
    }

    async fn delete(&self, song_id: Uuid) -> Result<(), FractionalOwnershipError> {
        let mut aggregates = self.aggregates.lock().await;
        aggregates.remove(&song_id);
        Ok(())
    }

    async fn find_by_artist_id(&self, artist_id: Uuid) -> Result<Vec<FractionalOwnershipAggregate>, FractionalOwnershipError> {
        let aggregates = self.aggregates.lock().await;
        let result = aggregates
            .values()
            .filter(|aggregate| aggregate.fractional_song().artist_id() == artist_id)
            .cloned()
            .collect();
        Ok(result)
    }

    async fn get_all_paginated(&self, page: u32, size: u32) -> Result<Vec<FractionalOwnershipAggregate>, FractionalOwnershipError> {
        let aggregates = self.aggregates.lock().await;
        let skip = (page * size) as usize;
        let take = size as usize;
        
        let result = aggregates
            .values()
            .skip(skip)
            .take(take)
            .cloned()
            .collect();
        Ok(result)
    }

    async fn get_user_ownerships(&self, user_id: &Uuid) -> Result<Vec<ShareOwnership>, FractionalOwnershipError> {
        let aggregates = self.aggregates.lock().await;
        let mut user_ownerships = Vec::new();
        
        for aggregate in aggregates.values() {
            if let Some(ownership) = aggregate.ownerships().get(user_id) {
                user_ownerships.push(ownership.clone());
            }
        }
        
        Ok(user_ownerships)
    }

    async fn get_user_revenue_for_song(&self, user_id: &Uuid, song_id: &Uuid) -> Result<Option<RevenueAmount>, FractionalOwnershipError> {
        let aggregates = self.aggregates.lock().await;
        
        if let Some(aggregate) = aggregates.get(song_id) {
            if let Some(ownership) = aggregate.ownerships().get(user_id) {
                return Ok(Some(ownership.total_earnings().clone()));
            }
        }
        
        Ok(None)
    }
} 