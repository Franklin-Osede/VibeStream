use crate::domain::aggregates::FractionalOwnershipAggregate;
use crate::domain::entities::{FractionalSong, ShareOwnership, ShareTransaction};
use crate::domain::errors::FractionalOwnershipError;
use crate::domain::value_objects::{OwnershipPercentage, SharePrice, RevenueAmount, SongId, UserId};
use crate::domain::repositories::FractionalOwnershipRepository;
use async_trait::async_trait;
use uuid::Uuid;
use std::sync::Arc;
use crate::application::dtos::{PurchaseSharesCommand, PurchaseSharesResult};
use crate::application::commands::PurchaseSharesCommand as Command;
use crate::domain::value_objects::OwnershipLimits;

/// Command Handler para PurchaseShares que integra con Command Bus
pub struct PurchaseSharesHandler<R: FractionalOwnershipRepository + Send + Sync + 'static> {
    repository: Arc<R>,
}

impl<R: FractionalOwnershipRepository + Send + Sync + 'static> PurchaseSharesHandler<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
}

// Implementation of CommandHandler trait for Command Bus integration
use vibestream_types::CommandHandler;

#[async_trait]
impl<R: FractionalOwnershipRepository + Send + Sync + 'static> CommandHandler<Command> for PurchaseSharesHandler<R> {
    type Output = PurchaseSharesResult;
    type Error = FractionalOwnershipError;

    async fn handle(&self, command: Command) -> Result<Self::Output, Self::Error> {
        let use_case = PurchaseSharesUseCase::new(self.repository.clone(), OwnershipLimits::default());
        
        // Convert Command to DTO format expected by use case
        let dto_command = PurchaseSharesCommand {
            fractional_song_id: command.fractional_song_id,
            buyer_id: command.buyer_id,
            shares_quantity: command.shares_quantity,
            auto_confirm: command.auto_confirm,
        };
        
        use_case.execute(dto_command).await
    }
}

/// Caso de uso: Comprar acciones de una canción fraccionada
/// 
/// Este caso de uso maneja todo el flujo de compra de acciones:
/// 1. Validar que el usuario puede comprar
/// 2. Crear la transacción de compra
/// 3. Confirmar la compra si el pago es exitoso
/// 4. Persistir los cambios
/// 5. Publicar eventos de dominio
pub struct PurchaseSharesUseCase<R: FractionalOwnershipRepository> {
    repository: Arc<R>,
    ownership_limits: OwnershipLimits,
}

impl<R: FractionalOwnershipRepository> PurchaseSharesUseCase<R> {
    pub fn new(repository: Arc<R>, ownership_limits: OwnershipLimits) -> Self {
        Self { 
            repository,
            ownership_limits,
        }
    }

    /// Ejecutar la compra de acciones
    pub async fn execute(&self, command: PurchaseSharesCommand) -> Result<PurchaseSharesResult, FractionalOwnershipError> {
        // 1. Obtener el aggregate de la canción fraccionada
        let mut aggregate = self.repository
            .get_by_id(command.fractional_song_id)
            .await?
            .ok_or_else(|| FractionalOwnershipError::ValidationError("Fractional song not found".to_string()))?;

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

    /// Validar la solicitud de compra según las reglas de negocio
    fn validate_purchase_request(
        &self,
        aggregate: &FractionalOwnershipAggregate,
        command: &PurchaseSharesCommand,
    ) -> Result<(), FractionalOwnershipError> {
        // Validar cantidad mínima
        if command.shares_quantity == 0 {
            return Err(FractionalOwnershipError::ValidationError("Must purchase at least 1 share".to_string()));
        }

        // Validar cantidad de shares por transacción usando límites configurables
        self.ownership_limits.validate_shares_per_transaction(command.shares_quantity)?;

        // Validar que el usuario no exceda el límite de ownership configurado
        let current_ownership = aggregate.get_user_ownership(command.buyer_id);
        let current_shares = current_ownership.map(|o| o.shares_owned()).unwrap_or(0);
        let total_shares_after = current_shares + command.shares_quantity;
        let ownership_percentage_after = (total_shares_after as f64 / aggregate.fractional_song().total_shares() as f64) * 100.0;

        self.ownership_limits.validate_ownership_percentage(ownership_percentage_after)?;

        // Validar que no sea el mismo artista comprando sus propias acciones (si está configurado)
        let is_artist_purchase = command.buyer_id == aggregate.fractional_song().artist_id();
        self.ownership_limits.validate_artist_purchase(is_artist_purchase)?;

        Ok(())
    }

    /// Calcular el nuevo porcentaje de ownership después de la compra
    fn calculate_new_ownership_percentage(
        &self,
        aggregate: &FractionalOwnershipAggregate,
        user_id: Uuid,
    ) -> Result<f64, FractionalOwnershipError> {
        if let Some(ownership) = aggregate.get_user_ownership(user_id) {
            Ok(ownership.percentage().value())
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
    // Repositorio en memoria para testing directo
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use std::collections::HashMap;
    use async_trait::async_trait;
    
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

    async fn create_test_aggregate() -> FractionalOwnershipAggregate {
        let song_id = Uuid::new_v4();
        let artist_id = Uuid::new_v4();
        let share_price = SharePrice::new(10.0).unwrap();
        
        let fractional_song = FractionalSong::new(
            song_id,
            artist_id,
            "Test Song".to_string(),
            1000,
            share_price,
        ).unwrap();
        
        FractionalOwnershipAggregate::new(fractional_song, HashMap::new()).unwrap()
    }

    #[tokio::test]
    async fn purchase_shares_handler_should_work_with_command_bus() {
        let repository = Arc::new(InMemoryFractionalOwnershipRepository::new());
        let handler = PurchaseSharesHandler::new(repository.clone());
        
        // Crear aggregate de prueba
        let aggregate = create_test_aggregate().await;
        repository.add_aggregate(aggregate).await;
        
        let command = Command {
            fractional_song_id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            buyer_id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap(),
            shares_quantity: 100,
            auto_confirm: true,
        };
        
        let result = handler.handle(command).await;
        assert!(result.is_ok());
        
        let purchase_result = result.unwrap();
        assert_eq!(purchase_result.shares_purchased, 100);
        assert_eq!(purchase_result.transaction_status, "Completed");
    }

    #[tokio::test]
    async fn should_reject_excessive_ownership() {
        let repository = Arc::new(InMemoryFractionalOwnershipRepository::new());
        
        // Crear límites restrictivos (10% max ownership)
        let restrictive_limits = OwnershipLimits::restrictive();
        let use_case = PurchaseSharesUseCase::new(repository.clone(), restrictive_limits);
        
        let aggregate = create_test_aggregate().await;
        repository.add_aggregate(aggregate).await;
        
        // Usuario 1 compra 100 acciones (10%) - debería ser aceptado
        let command = PurchaseSharesCommand {
            fractional_song_id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            buyer_id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap(),
            shares_quantity: 100, // 10% de 1000 shares
            auto_confirm: true,
        };
        
        let result1 = use_case.execute(command).await;
        assert!(result1.is_ok());
        
        // Usuario 1 trata de comprar 50 acciones más (5% adicional = 15% total) - debería ser rechazado
        let command2 = PurchaseSharesCommand {
            fractional_song_id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            buyer_id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap(),
            shares_quantity: 50,
            auto_confirm: true,
        };
        
        let result2 = use_case.execute(command2).await;
        assert!(result2.is_err());
        
        if let Err(FractionalOwnershipError::OwnershipExceedsLimit { .. }) = result2 {
            // Expected error
        } else {
            panic!("Expected OwnershipExceedsLimit error");
        }
    }

    #[tokio::test]
    async fn should_reject_artist_buying_own_shares_when_restricted() {
        let repository = Arc::new(InMemoryFractionalOwnershipRepository::new());
        
        // Usar límites conservadores (artista NO puede comprar)
        let conservative_limits = OwnershipLimits::conservative();
        let use_case = PurchaseSharesUseCase::new(repository.clone(), conservative_limits);
        
        let aggregate = create_test_aggregate().await;
        repository.add_aggregate(aggregate).await;
        
        let command = PurchaseSharesCommand {
            fractional_song_id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            buyer_id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440002").unwrap(), // Este es el artist_id
            shares_quantity: 100,
            auto_confirm: true,
        };
        
        let result = use_case.execute(command).await;
        assert!(result.is_err());
        
        if let Err(FractionalOwnershipError::BusinessRuleViolation(msg)) = result {
            assert!(msg.contains("Artist cannot buy shares"));
        } else {
            panic!("Expected BusinessRuleViolation for artist purchase");
        }
    }

    #[tokio::test]
    async fn should_allow_artist_buying_own_shares_when_permitted() {
        let repository = Arc::new(InMemoryFractionalOwnershipRepository::new());
        
        // Usar límites liberales (artista SÍ puede comprar)
        let liberal_limits = OwnershipLimits::liberal();
        let use_case = PurchaseSharesUseCase::new(repository.clone(), liberal_limits);
        
        let aggregate = create_test_aggregate().await;
        repository.add_aggregate(aggregate).await;
        
        let command = PurchaseSharesCommand {
            fractional_song_id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            buyer_id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440002").unwrap(), // Este es el artist_id
            shares_quantity: 100,
            auto_confirm: true,
        };
        
        let result = use_case.execute(command).await;
        assert!(result.is_ok());
        
        let purchase_result = result.unwrap();
        assert_eq!(purchase_result.shares_purchased, 100);
    }

    #[tokio::test]
    async fn should_respect_different_ownership_limits() {
        // Probar solo los límites conservadores (30%) con valor de prueba más bajo
        let repository = Arc::new(InMemoryFractionalOwnershipRepository::new());
        let conservative_limits = OwnershipLimits::conservative();
        println!("Límite configurado: {}%", conservative_limits.max_individual_ownership_percentage());
        
        let use_case = PurchaseSharesUseCase::new(repository.clone(), conservative_limits);
        
        // IDs para referencias externas
        let external_song_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let artist_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440002").unwrap();
        
        // Crear canción simple
        let fractional_song = FractionalSong::new(
            external_song_id, // ID externo de la canción en el Music Context
            artist_id,
            "Test Song".to_string(),
            1000, // 1000 acciones totales
            SharePrice::new(10.0).unwrap(),
        ).unwrap();
        
        // Obtenemos el ID interno que realmente usará el repositorio
        let internal_song_id = fractional_song.id();
        println!("Canción fraccionada creada - ID externo: {}, ID interno: {}", external_song_id, internal_song_id);
        println!("Creando aggregate con {} acciones totales y {} disponibles", 
            fractional_song.total_shares(), fractional_song.available_shares());
        
        // Crear el aggregate con la canción
        let aggregate = FractionalOwnershipAggregate::new(
            fractional_song,
            HashMap::new()
        ).unwrap();
        
        // Guardar el aggregate en el repositorio
        repository.add_aggregate(aggregate).await;
        
        // Verificar que el aggregate fue guardado correctamente por su ID interno
        let saved_aggregate = repository.get_by_id(internal_song_id).await.unwrap();
        println!("¿El aggregate existe en el repositorio con ID interno? {}", saved_aggregate.is_some());
        
        // Intentar comprar 250 acciones (25%) - debería estar bien
        let shares_to_buy = 250; // 25% de 1000 acciones
        println!("Comprando {} acciones ({}%)", shares_to_buy, shares_to_buy as f64 / 10.0);
        
        let command = PurchaseSharesCommand {
            fractional_song_id: internal_song_id, // Usar el ID interno de la canción fraccionada
            buyer_id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap(),
            shares_quantity: shares_to_buy,
            auto_confirm: true,
        };
        
        let result = use_case.execute(command).await;
        
        if result.is_err() {
            println!("ERROR: {:?}", result.err().unwrap());
            panic!("Conservative limits should allow 25% ownership");
        } else {
            println!("ÉXITO: Se compraron las acciones correctamente");
            assert!(result.is_ok());
            let purchase_result = result.unwrap();
            println!("Nueva propiedad: {}%", purchase_result.new_ownership_percentage);
        }
    }

    #[tokio::test]
    async fn should_respect_shares_per_transaction_limits() {
        let repository = Arc::new(InMemoryFractionalOwnershipRepository::new());
        
        // Test con límites restrictivos (100 shares max por transacción)
        let restrictive_limits = OwnershipLimits::restrictive();
        let use_case = PurchaseSharesUseCase::new(repository.clone(), restrictive_limits);
        
        let aggregate = create_test_aggregate().await;
        repository.add_aggregate(aggregate).await;
        
        // 150 shares excede el límite de 100 por transacción
        let command_excess = PurchaseSharesCommand {
            fractional_song_id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            buyer_id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap(),
            shares_quantity: 150,
            auto_confirm: true,
        };
        
        let result_excess = use_case.execute(command_excess).await;
        assert!(result_excess.is_err());
        
        if let Err(FractionalOwnershipError::ValidationError(msg)) = result_excess {
            assert!(msg.contains("100"));
        } else {
            panic!("Expected ValidationError for exceeding shares per transaction limit");
        }
        
        // 100 shares exacto debería funcionar
        let command_ok = PurchaseSharesCommand {
            fractional_song_id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            buyer_id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap(),
            shares_quantity: 100,
            auto_confirm: true,
        };
        
        let result_ok = use_case.execute(command_ok).await;
        assert!(result_ok.is_ok());
    }
} 