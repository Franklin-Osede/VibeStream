use crate::domain::entities::{FractionalSong, ShareOwnership, ShareTransaction, TransactionStatus};
use crate::domain::value_objects::{OwnershipPercentage, SharePrice, RevenueAmount};
use crate::domain::events::{SharePurchased, RevenueDistributed, ShareTransferred};
use crate::domain::errors::FractionalOwnershipError;
use uuid::Uuid;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Aggregate Root que encapsula toda la lógica de participaciones fraccionadas
/// Mantiene consistencia transaccional entre FractionalSong, ShareOwnership y ShareTransaction
#[derive(Debug, Clone)]
pub struct FractionalOwnershipAggregate {
    fractional_song: FractionalSong,
    ownerships: HashMap<Uuid, ShareOwnership>, // user_id -> ShareOwnership
    pending_transactions: HashMap<Uuid, ShareTransaction>, // transaction_id -> ShareTransaction
    // Cambiamos eventos a Vec<String> para serialización, los eventos reales se crean cuando se necesiten
    uncommitted_events: Vec<String>, // Event types que han ocurrido
}

impl FractionalOwnershipAggregate {
    /// Crear un nuevo aggregate para una canción fraccionada
    pub fn new(fractional_song: FractionalSong) -> Self {
        FractionalOwnershipAggregate {
            fractional_song,
            ownerships: HashMap::new(),
            pending_transactions: HashMap::new(),
            uncommitted_events: Vec::new(),
        }
    }

    /// Cargar aggregate existente con datos de repositorio
    pub fn load(
        fractional_song: FractionalSong,
        ownerships: Vec<ShareOwnership>,
        transactions: Vec<ShareTransaction>,
    ) -> Self {
        let ownerships_map: HashMap<Uuid, ShareOwnership> = ownerships
            .into_iter()
            .map(|ownership| (ownership.user_id(), ownership))
            .collect();

        let pending_transactions_map: HashMap<Uuid, ShareTransaction> = transactions
            .into_iter()
            .filter(|t| !t.is_finalized())
            .map(|transaction| (transaction.id(), transaction))
            .collect();

        FractionalOwnershipAggregate {
            fractional_song,
            ownerships: ownerships_map,
            pending_transactions: pending_transactions_map,
            uncommitted_events: Vec::new(),
        }
    }

    // Getters
    pub fn fractional_song(&self) -> &FractionalSong {
        &self.fractional_song
    }

    pub fn ownerships(&self) -> &HashMap<Uuid, ShareOwnership> {
        &self.ownerships
    }

    pub fn pending_transactions(&self) -> &HashMap<Uuid, ShareTransaction> {
        &self.pending_transactions
    }

    pub fn uncommitted_events(&self) -> &Vec<String> {
        &self.uncommitted_events
    }

    /// Limpiar eventos después de ser procesados
    pub fn clear_events(&mut self) {
        self.uncommitted_events.clear();
    }

    /// Lógica de dominio: Comprar acciones de la canción
    pub fn purchase_shares(
        &mut self,
        buyer_id: Uuid,
        shares_quantity: u32,
    ) -> Result<Uuid, FractionalOwnershipError> {
        // Validar que hay acciones disponibles
        if shares_quantity > self.fractional_song.available_shares() {
            return Err(FractionalOwnershipError::InsufficientShares);
        }

        // Crear transacción de compra
        let transaction = ShareTransaction::new_purchase(
            buyer_id,
            self.fractional_song.id(),
            shares_quantity,
            self.fractional_song.share_price().clone(),
        )?;

        let transaction_id = transaction.id();

        // Reservar acciones
        self.fractional_song.reserve_shares(shares_quantity)?;

        // Guardar transacción pendiente
        self.pending_transactions.insert(transaction_id, transaction);

        Ok(transaction_id)
    }

    /// Lógica de dominio: Confirmar compra de acciones
    pub fn confirm_purchase(&mut self, transaction_id: Uuid) -> Result<(), FractionalOwnershipError> {
        let mut transaction = self.pending_transactions
            .remove(&transaction_id)
            .ok_or_else(|| FractionalOwnershipError::NotFound)?;

        // Completar transacción
        transaction.complete()?;

        let buyer_id = transaction.buyer_id()
            .ok_or_else(|| FractionalOwnershipError::ValidationError("ID de comprador faltante".to_string()))?;

        // Crear o actualizar ownership
        if let Some(existing_ownership) = self.ownerships.get_mut(&buyer_id) {
            // El usuario ya tiene acciones, agregar más
            self.update_existing_ownership(existing_ownership, transaction.shares_quantity())?;
        } else {
            // Nuevo ownership
            let ownership = ShareOwnership::new(
                buyer_id,
                self.fractional_song.id(),
                transaction.shares_quantity(),
                self.fractional_song.total_shares(),
                transaction.price_per_share().clone(),
            )?;
            self.ownerships.insert(buyer_id, ownership);
        }

        // Registrar evento
        self.uncommitted_events.push("SharePurchased".to_string());

        Ok(())
    }

    /// Lógica de dominio: Cancelar compra de acciones
    pub fn cancel_purchase(&mut self, transaction_id: Uuid) -> Result<(), FractionalOwnershipError> {
        let mut transaction = self.pending_transactions
            .remove(&transaction_id)
            .ok_or_else(|| FractionalOwnershipError::NotFound)?;

        // Cancelar transacción
        transaction.cancel()?;

        // Liberar acciones reservadas
        self.fractional_song.release_shares(transaction.shares_quantity())?;

        Ok(())
    }

    /// Lógica de dominio: Transferir acciones entre usuarios
    pub fn transfer_shares(
        &mut self,
        from_user_id: Uuid,
        to_user_id: Uuid,
        percentage: OwnershipPercentage,
        price_per_share: SharePrice,
    ) -> Result<ShareTransaction, FractionalOwnershipError> {
        // Validar que el vendedor tiene suficientes acciones
        let seller_ownership = self.ownerships.get(&from_user_id)
            .ok_or_else(|| FractionalOwnershipError::UserNotFound)?;

        if seller_ownership.percentage().as_f64() < percentage.as_f64() {
            return Err(FractionalOwnershipError::InsufficientShares);
        }

        // Crear transacción de transferencia
        let transaction = ShareTransaction::new_transfer(
            to_user_id,
            from_user_id,
            self.fractional_song.id(),
            percentage.as_f64() as u32, // TODO: mejorar conversión
            price_per_share,
        )?;

        // Registrar evento
        self.uncommitted_events.push("ShareTransferred".to_string());

        Ok(transaction)
    }

    /// Lógica de dominio: Distribuir ingresos
    pub fn distribute_revenue(&mut self, total_revenue: RevenueAmount, revenue_period: String) -> Result<HashMap<Uuid, RevenueAmount>, FractionalOwnershipError> {
        let mut distribution = HashMap::new();
        
        for (user_id, ownership) in &self.ownerships {
            let user_share = RevenueAmount::new(
                total_revenue.as_f64() * ownership.percentage().as_f64()
            )?;
            distribution.insert(*user_id, user_share);
        }

        // Registrar evento
        self.uncommitted_events.push("RevenueDistributed".to_string());

        Ok(distribution)
    }

    /// Crear nuevo agregado (método factory faltante)
    pub fn create_new(
        song_id: Uuid,
        artist_id: Uuid,
        song_title: String,
        total_shares: u32,
        initial_price_per_share: SharePrice,
        artist_reserved_percentage: OwnershipPercentage,
    ) -> Result<Self, FractionalOwnershipError> {
        let fractional_song = FractionalSong::new(
            song_id,
            artist_id,
            song_title,
            total_shares,
            initial_price_per_share,
        )?;

        let mut aggregate = Self::new(fractional_song);

        // Crear ownership inicial para el artista
        let artist_ownership = ShareOwnership::new(
            artist_id,
            song_id,
            (total_shares as f64 * artist_reserved_percentage.as_f64()) as u32,
            total_shares,
            initial_price_per_share,
        )?;

        aggregate.ownerships.insert(artist_id, artist_ownership);

        Ok(aggregate)
    }

    /// Obtener ownership de un usuario específico
    pub fn get_user_ownership(&self, user_id: Uuid) -> Option<&ShareOwnership> {
        self.ownerships.get(&user_id)
    }

    /// Obtener todos los propietarios con sus porcentajes
    pub fn get_ownership_breakdown(&self) -> Vec<(Uuid, f64, u32)> {
        self.ownerships
            .iter()
            .map(|(user_id, ownership)| {
                (*user_id, ownership.ownership_percentage().value(), ownership.shares_owned())
            })
            .collect()
    }

    /// Calcular valor total de mercado de la canción
    pub fn calculate_market_value(&self) -> RevenueAmount {
        let total_value = self.fractional_song.share_price().value() * self.fractional_song.total_shares() as f64;
        RevenueAmount::new(total_value).unwrap_or_else(|_| RevenueAmount::new(0.0).unwrap())
    }

    /// Verificar integridad del aggregate
    pub fn verify_integrity(&self) -> Result<(), FractionalOwnershipError> {
        // Verificar que la suma de acciones owned no exceda el total
        let total_owned_shares: u32 = self.ownerships.values()
            .map(|ownership| ownership.shares_owned())
            .sum();

        let total_available_and_owned = self.fractional_song.available_shares() + total_owned_shares;

        if total_available_and_owned != self.fractional_song.total_shares() {
            return Err(FractionalOwnershipError::BusinessRuleViolation(
                format!("Inconsistencia en acciones: Total={}, Disponibles={}, Poseídas={}",
                        self.fractional_song.total_shares(),
                        self.fractional_song.available_shares(),
                        total_owned_shares)
            ));
        }

        // Verificar que los porcentajes de ownership sumen correctamente
        let total_percentage: f64 = self.ownerships.values()
            .map(|ownership| ownership.ownership_percentage().value())
            .sum();

        let expected_percentage = (total_owned_shares as f64 / self.fractional_song.total_shares() as f64) * 100.0;

        if (total_percentage - expected_percentage).abs() > 0.01 {
            return Err(FractionalOwnershipError::BusinessRuleViolation(
                format!("Inconsistencia en porcentajes: Calculado={:.2}%, Esperado={:.2}%",
                        total_percentage, expected_percentage)
            ));
        }

        Ok(())
    }

    /// Funciones auxiliares privadas
    fn update_existing_ownership(&mut self, ownership: &mut ShareOwnership, additional_shares: u32) -> Result<(), FractionalOwnershipError> {
        let new_total_shares = ownership.shares_owned() + additional_shares;
        
        // Crear nuevo ownership con los shares actualizados
        let new_ownership = ShareOwnership::new(
            ownership.user_id(),
            ownership.fractional_song_id(),
            new_total_shares,
            self.fractional_song.total_shares(),
            ownership.purchase_price().clone(),
        )?;

        *ownership = new_ownership;
        Ok(())
    }

    fn reduce_user_ownership(&mut self, user_id: Uuid, shares_to_reduce: u32) -> Result<(), FractionalOwnershipError> {
        let ownership = self.ownerships.get_mut(&user_id)
            .ok_or_else(|| FractionalOwnershipError::ValidationError("Usuario no posee acciones".to_string()))?;

        if ownership.shares_owned() < shares_to_reduce {
            return Err(FractionalOwnershipError::BusinessRuleViolation(
                "No se pueden reducir más acciones de las que posee".to_string()
            ));
        }

        let new_shares = ownership.shares_owned() - shares_to_reduce;

        if new_shares == 0 {
            // Remover ownership completamente
            self.ownerships.remove(&user_id);
        } else {
            // Actualizar ownership con menos acciones
            let new_ownership = ShareOwnership::new(
                user_id,
                ownership.fractional_song_id(),
                new_shares,
                self.fractional_song.total_shares(),
                ownership.purchase_price().clone(),
            )?;
            self.ownerships.insert(user_id, new_ownership);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::FractionalSong;

    fn create_test_fractional_song() -> FractionalSong {
        let song_id = Uuid::new_v4();
        let artist_id = Uuid::new_v4();
        let share_price = SharePrice::new(10.0).unwrap();

        FractionalSong::new(
            song_id,
            artist_id,
            "Test Song".to_string(),
            1000,
            share_price,
        ).unwrap()
    }

    #[test]
    fn should_create_aggregate_correctly() {
        let fractional_song = create_test_fractional_song();
        let aggregate = FractionalOwnershipAggregate::new(fractional_song.clone());

        assert_eq!(aggregate.fractional_song().id(), fractional_song.id());
        assert_eq!(aggregate.ownerships().len(), 0);
        assert_eq!(aggregate.pending_transactions().len(), 0);
    }

    #[test]
    fn should_purchase_shares_successfully() {
        let fractional_song = create_test_fractional_song();
        let mut aggregate = FractionalOwnershipAggregate::new(fractional_song);

        let buyer_id = Uuid::new_v4();
        let transaction_id = aggregate.purchase_shares(buyer_id, 100).unwrap();

        // Verificar que se reservaron las acciones
        assert_eq!(aggregate.fractional_song().available_shares(), 900);
        assert_eq!(aggregate.pending_transactions().len(), 1);
        assert!(aggregate.pending_transactions().contains_key(&transaction_id));
    }

    #[test]
    fn should_confirm_purchase_and_create_ownership() {
        let fractional_song = create_test_fractional_song();
        let mut aggregate = FractionalOwnershipAggregate::new(fractional_song);

        let buyer_id = Uuid::new_v4();
        let transaction_id = aggregate.purchase_shares(buyer_id, 100).unwrap();
        aggregate.confirm_purchase(transaction_id).unwrap();

        // Verificar ownership creado
        assert_eq!(aggregate.ownerships().len(), 1);
        let ownership = aggregate.get_user_ownership(buyer_id).unwrap();
        assert_eq!(ownership.shares_owned(), 100);
        assert_eq!(ownership.ownership_percentage().value(), 10.0);

        // Verificar evento generado
        assert_eq!(aggregate.uncommitted_events().len(), 1);
        assert_eq!(aggregate.uncommitted_events()[0], "SharePurchased");
    }

    #[test]
    fn should_distribute_revenue_proportionally() {
        let fractional_song = create_test_fractional_song();
        let mut aggregate = FractionalOwnershipAggregate::new(fractional_song);

        // Crear dos propietarios
        let buyer1_id = Uuid::new_v4();
        let buyer2_id = Uuid::new_v4();

        // Primer comprador: 100 acciones (10%)
        let transaction1_id = aggregate.purchase_shares(buyer1_id, 100).unwrap();
        aggregate.confirm_purchase(transaction1_id).unwrap();

        // Segundo comprador: 200 acciones (20%)
        let transaction2_id = aggregate.purchase_shares(buyer2_id, 200).unwrap();
        aggregate.confirm_purchase(transaction2_id).unwrap();

        // Distribuir $1000 en ingresos
        let revenue = RevenueAmount::new(1000.0).unwrap();
        aggregate.distribute_revenue(revenue, "2024".to_string()).unwrap();

        // Verificar distribución proporcional
        let ownership1 = aggregate.get_user_ownership(buyer1_id).unwrap();
        let ownership2 = aggregate.get_user_ownership(buyer2_id).unwrap();

        // Buyer1 (10%) debe recibir $100
        assert_eq!(ownership1.total_earnings().value(), 100.0);
        // Buyer2 (20%) debe recibir $200
        assert_eq!(ownership2.total_earnings().value(), 200.0);
    }

    #[test]
    fn should_maintain_integrity() {
        let fractional_song = create_test_fractional_song();
        let mut aggregate = FractionalOwnershipAggregate::new(fractional_song);

        let buyer_id = Uuid::new_v4();
        let transaction_id = aggregate.purchase_shares(buyer_id, 100).unwrap();
        aggregate.confirm_purchase(transaction_id).unwrap();

        // Verificar integridad del aggregate
        assert!(aggregate.verify_integrity().is_ok());
    }

    #[test]
    fn should_reject_excessive_share_purchase() {
        let fractional_song = create_test_fractional_song();
        let mut aggregate = FractionalOwnershipAggregate::new(fractional_song);

        let buyer_id = Uuid::new_v4();
        let result = aggregate.purchase_shares(buyer_id, 1001);

        assert!(result.is_err());
    }
} 