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
}

#[async_trait]
impl FractionalOwnershipRepository for InMemoryFractionalOwnershipRepository {
    async fn load_aggregate(&self, song_id: &Uuid) -> Result<Option<FractionalOwnershipAggregate>, FractionalOwnershipError> {
        let aggregates = self.aggregates.lock().await;
        Ok(aggregates.get(song_id).cloned())
    }

    async fn save_aggregate(&self, aggregate: &FractionalOwnershipAggregate) -> Result<(), FractionalOwnershipError> {
        let mut aggregates = self.aggregates.lock().await;
        aggregates.insert(aggregate.fractional_song().id(), aggregate.clone());
        Ok(())
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

    async fn get_user_revenue_for_song(&self, _user_id: &Uuid, _song_id: &Uuid) -> Result<Option<RevenueAmount>, FractionalOwnershipError> {
        // TODO: Implementar lógica de ingresos en memoria
        Ok(Some(RevenueAmount::new(0.0)?))
    }
} 