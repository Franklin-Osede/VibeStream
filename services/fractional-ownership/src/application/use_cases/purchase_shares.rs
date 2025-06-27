use crate::domain::aggregates::FractionalOwnershipAggregate;
use crate::domain::entities::{FractionalSong, ShareOwnership, ShareTransaction};
use crate::domain::errors::FractionalOwnershipError;
use crate::domain::value_objects::{OwnershipPercentage, SharePrice, RevenueAmount};
use crate::domain::repositories::FractionalOwnershipRepository;
use async_trait::async_trait;
use uuid::Uuid;
use std::sync::Arc;
use crate::application::dtos::{PurchaseSharesCommand, PurchaseSharesResult};

/// Caso de uso: Comprar acciones de una canción fraccionada
/// 
/// Este caso de uso maneja todo el flujo de compra de acciones:
/// 1. Validar que el usuario puede comprar
/// 2. Crear la transacción de compra
/// 3. Confirmar la compra si el pago es exitoso
/// 4. Persistir los cambios
/// 5. Publicar eventos de dominio
pub struct PurchaseSharesUseCase<R: FractionalOwnershipRepository> {
    repository: R,
}

impl<R: FractionalOwnershipRepository> PurchaseSharesUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Ejecutar la compra de acciones
    pub async fn execute(&self, command: PurchaseSharesCommand) -> Result<PurchaseSharesResult, FractionalOwnershipError> {
        // 1. Obtener el aggregate de la canción fraccionada
        let mut aggregate = self.repository
            .get_by_id(command.fractional_song_id)
            .await?
            .ok_or_else(|| FractionalOwnershipError::ValidationError("Canción fraccionada no encontrada".to_string()))?;

        // 2. Validaciones de negocio adicionales
        self.validate_purchase_request(&aggregate, &command)?;

        // 3. Ejecutar la compra en el dominio
        let transaction_id = aggregate.purchase_shares(
            command.buyer_id,
            command.shares_quantity,
        )?;

        // 4. Si es compra inmediata, confirmar la transacción
        if command.auto_confirm {
            aggregate.confirm_purchase(transaction_id)?;
        }

        // 5. Verificar integridad del aggregate
        aggregate.verify_integrity()?;

        // 6. Persistir los cambios
        self.repository.save(&aggregate).await?;

        // 7. Crear resultado
        let result = PurchaseSharesResult {
            transaction_id,
            fractional_song_id: command.fractional_song_id,
            buyer_id: command.buyer_id,
            shares_purchased: command.shares_quantity,
            total_cost: aggregate.fractional_song().share_price().multiply_by_quantity(command.shares_quantity)?,
            new_ownership_percentage: self.calculate_new_ownership_percentage(&aggregate, command.buyer_id)?,
            transaction_status: if command.auto_confirm { "Completed".to_string() } else { "Pending".to_string() },
            remaining_available_shares: aggregate.fractional_song().available_shares(),
        };

        Ok(result)
    }

    /// Validaciones específicas para la compra
    fn validate_purchase_request(
        &self,
        aggregate: &FractionalOwnershipAggregate,
        command: &PurchaseSharesCommand,
    ) -> Result<(), FractionalOwnershipError> {
        // Validar cantidad mínima
        if command.shares_quantity == 0 {
            return Err(FractionalOwnershipError::ValidationError("Debe comprar al menos 1 acción".to_string()));
        }

        // Validar cantidad máxima por transacción (regla de negocio)
        if command.shares_quantity > 1000 {
            return Err(FractionalOwnershipError::BusinessRuleViolation(
                "No se pueden comprar más de 1000 acciones en una sola transacción".to_string()
            ));
        }

        // Validar que el usuario no exceda el límite de ownership (30%)
        let current_ownership = aggregate.get_user_ownership(command.buyer_id);
        let current_shares = current_ownership.map(|o| o.shares_owned()).unwrap_or(0);
        let total_shares_after = current_shares + command.shares_quantity;
        let ownership_percentage_after = (total_shares_after as f64 / aggregate.fractional_song().total_shares() as f64) * 100.0;

        if ownership_percentage_after > 30.0 {
            return Err(FractionalOwnershipError::BusinessRuleViolation(
                format!("Un usuario no puede poseer más del 30% de una canción. Ownership resultante: {:.2}%", ownership_percentage_after)
            ));
        }

        // Validar que no sea el mismo artista comprando sus propias acciones
        if command.buyer_id == aggregate.fractional_song().artist_id() {
            return Err(FractionalOwnershipError::BusinessRuleViolation(
                "El artista no puede comprar acciones de su propia canción".to_string()
            ));
        }

        Ok(())
    }

    /// Calcular el nuevo porcentaje de ownership después de la compra
    fn calculate_new_ownership_percentage(
        &self,
        aggregate: &FractionalOwnershipAggregate,
        user_id: Uuid,
    ) -> Result<f64, FractionalOwnershipError> {
        if let Some(ownership) = aggregate.get_user_ownership(user_id) {
            Ok(ownership.ownership_percentage().value())
        } else {
            Ok(0.0)
        }
    }
}

/// Caso de uso para confirmar una compra pendiente
pub struct ConfirmPurchaseUseCase<R: FractionalOwnershipRepository> {
    repository: R,
}

impl<R: FractionalOwnershipRepository> ConfirmPurchaseUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, transaction_id: Uuid, fractional_song_id: Uuid) -> Result<(), FractionalOwnershipError> {
        // 1. Obtener el aggregate
        let mut aggregate = self.repository
            .get_by_id(fractional_song_id)
            .await?
            .ok_or_else(|| FractionalOwnershipError::ValidationError("Canción fraccionada no encontrada".to_string()))?;

        // 2. Confirmar la compra
        aggregate.confirm_purchase(transaction_id)?;

        // 3. Verificar integridad
        aggregate.verify_integrity()?;

        // 4. Persistir
        self.repository.save(&aggregate).await?;

        Ok(())
    }
}

/// Caso de uso para cancelar una compra pendiente
pub struct CancelPurchaseUseCase<R: FractionalOwnershipRepository> {
    repository: R,
}

impl<R: FractionalOwnershipRepository> CancelPurchaseUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, transaction_id: Uuid, fractional_song_id: Uuid) -> Result<(), FractionalOwnershipError> {
        // 1. Obtener el aggregate
        let mut aggregate = self.repository
            .get_by_id(fractional_song_id)
            .await?
            .ok_or_else(|| FractionalOwnershipError::ValidationError("Canción fraccionada no encontrada".to_string()))?;

        // 2. Cancelar la compra
        aggregate.cancel_purchase(transaction_id)?;

        // 3. Persistir
        self.repository.save(&aggregate).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::FractionalSong;
    use crate::domain::value_objects::SharePrice;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    // Mock repository para testing
    pub struct MockFractionalOwnershipRepository {
        aggregates: Arc<Mutex<HashMap<Uuid, FractionalOwnershipAggregate>>>,
    }

    impl MockFractionalOwnershipRepository {
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
    impl FractionalOwnershipRepository for MockFractionalOwnershipRepository {
        async fn get_by_id(&self, song_id: Uuid) -> Result<Option<FractionalOwnershipAggregate>, FractionalOwnershipError> {
            let aggregates = self.aggregates.lock().await;
            Ok(aggregates.get(&song_id).cloned())
        }

        async fn save(&self, aggregate: &FractionalOwnershipAggregate) -> Result<(), FractionalOwnershipError> {
            let mut aggregates = self.aggregates.lock().await;
            aggregates.insert(aggregate.fractional_song().id(), aggregate.clone());
            Ok(())
        }

        async fn delete(&self, song_id: Uuid) -> Result<(), FractionalOwnershipError> {
            let mut aggregates = self.aggregates.lock().await;
            aggregates.remove(&song_id);
            Ok(())
        }

        async fn find_by_artist_id(&self, _artist_id: Uuid) -> Result<Vec<FractionalOwnershipAggregate>, FractionalOwnershipError> {
            Ok(Vec::new())
        }

        async fn get_all_paginated(&self, _page: u32, _size: u32) -> Result<Vec<FractionalOwnershipAggregate>, FractionalOwnershipError> {
            Ok(Vec::new())
        }
    }

    #[tokio::test]
    async fn should_purchase_shares_successfully() {
        // Setup
        let repository = MockFractionalOwnershipRepository::new();
        let use_case = PurchaseSharesUseCase::new(repository);

        let song_id = Uuid::new_v4();
        let artist_id = Uuid::new_v4();
        let buyer_id = Uuid::new_v4();

        let fractional_song = FractionalSong::new(
            song_id,
            artist_id,
            "Test Song".to_string(),
            1000,
            SharePrice::new(10.0).unwrap(),
        ).unwrap();

        let aggregate = FractionalOwnershipAggregate::new(fractional_song);
        use_case.repository.add_aggregate(aggregate).await;

        let command = PurchaseSharesCommand {
            fractional_song_id: song_id,
            buyer_id,
            shares_quantity: 100,
            auto_confirm: true,
        };

        // Execute
        let result = use_case.execute(command).await.unwrap();

        // Assert
        assert_eq!(result.shares_purchased, 100);
        assert_eq!(result.buyer_id, buyer_id);
        assert_eq!(result.new_ownership_percentage, 10.0);
        assert_eq!(result.transaction_status, "Completed");
        assert_eq!(result.remaining_available_shares, 900);
    }

    #[tokio::test]
    async fn should_reject_excessive_ownership() {
        // Setup
        let repository = MockFractionalOwnershipRepository::new();
        let use_case = PurchaseSharesUseCase::new(repository);

        let song_id = Uuid::new_v4();
        let artist_id = Uuid::new_v4();
        let buyer_id = Uuid::new_v4();

        let fractional_song = FractionalSong::new(
            song_id,
            artist_id,
            "Test Song".to_string(),
            1000,
            SharePrice::new(10.0).unwrap(),
        ).unwrap();

        let aggregate = FractionalOwnershipAggregate::new(fractional_song);
        use_case.repository.add_aggregate(aggregate).await;

        let command = PurchaseSharesCommand {
            fractional_song_id: song_id,
            buyer_id,
            shares_quantity: 400, // 40% > 30% límite
            auto_confirm: true,
        };

        // Execute & Assert
        let result = use_case.execute(command).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_reject_artist_buying_own_shares() {
        // Setup
        let repository = MockFractionalOwnershipRepository::new();
        let use_case = PurchaseSharesUseCase::new(repository);

        let song_id = Uuid::new_v4();
        let artist_id = Uuid::new_v4();

        let fractional_song = FractionalSong::new(
            song_id,
            artist_id,
            "Test Song".to_string(),
            1000,
            SharePrice::new(10.0).unwrap(),
        ).unwrap();

        let aggregate = FractionalOwnershipAggregate::new(fractional_song);
        use_case.repository.add_aggregate(aggregate).await;

        let command = PurchaseSharesCommand {
            fractional_song_id: song_id,
            buyer_id: artist_id, // Mismo artista
            shares_quantity: 100,
            auto_confirm: true,
        };

        // Execute & Assert
        let result = use_case.execute(command).await;
        assert!(result.is_err());
    }
} 