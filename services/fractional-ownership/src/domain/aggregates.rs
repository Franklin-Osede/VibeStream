use crate::domain::entities::{FractionalSong, ShareOwnership, ShareTransaction};
use crate::domain::value_objects::{OwnershipPercentage, SharePrice, RevenueAmount};
use crate::domain::errors::FractionalOwnershipError;
use uuid::Uuid;
use std::collections::HashMap;

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
    pub fn new(fractional_song: FractionalSong, ownerships: HashMap<Uuid, ShareOwnership>) -> Result<Self, FractionalOwnershipError> {
        let aggregate = FractionalOwnershipAggregate {
            fractional_song,
            ownerships,
            pending_transactions: HashMap::new(),
            uncommitted_events: Vec::new(),
        };
        
        // Verificar integridad al crear
        aggregate.verify_integrity()?;
        Ok(aggregate)
    }

    /// Cargar aggregate existente con datos de repositorio
    pub fn load(
        fractional_song: FractionalSong,
        ownerships: Vec<ShareOwnership>,
        transactions: Vec<ShareTransaction>,
    ) -> Result<Self, FractionalOwnershipError> {
        let ownerships_map: HashMap<Uuid, ShareOwnership> = ownerships
            .into_iter()
            .map(|ownership| (ownership.user_id(), ownership))
            .collect();

        let pending_transactions_map: HashMap<Uuid, ShareTransaction> = transactions
            .into_iter()
            .filter(|t| !t.is_finalized())
            .map(|transaction| (transaction.id(), transaction))
            .collect();

        let mut aggregate = Self::new(fractional_song, ownerships_map)?;
        aggregate.pending_transactions = pending_transactions_map;
        
        Ok(aggregate)
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
        if self.ownerships.contains_key(&buyer_id) {
            // El usuario ya tiene acciones, agregar más
            self.update_existing_ownership(buyer_id, transaction.shares_quantity())?;
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
    pub fn distribute_revenue(&mut self, total_revenue: RevenueAmount, _revenue_period: String) -> Result<HashMap<Uuid, RevenueAmount>, FractionalOwnershipError> {
        let mut distribution = HashMap::new();
        
        for (user_id, ownership) in &self.ownerships {
            let user_share = RevenueAmount::new(
                total_revenue.as_f64() * (ownership.percentage().value() / 100.0)
            )?;
            distribution.insert(*user_id, user_share);
        }

        // Registrar evento
        self.uncommitted_events.push("RevenueDistributed".to_string());

        Ok(distribution)
    }

    /// Factory method para crear nuevo aggregate
    pub fn create_new(
        song_id: Uuid,
        artist_id: Uuid,
        song_title: String,
        total_shares: u32,
        initial_price_per_share: SharePrice,
        artist_reserved_percentage: OwnershipPercentage,
    ) -> Result<Self, FractionalOwnershipError> {
        let artist_reserved_shares = ((total_shares as f64) * (artist_reserved_percentage.as_f64() / 100.0)) as u32;
        
        let fractional_song = FractionalSong::new_with_artist_control(
            Uuid::new_v4(),
            song_id,
            artist_id,
            song_title,
            total_shares,
            artist_reserved_shares,
            artist_reserved_percentage.as_f64() / 100.0, // Convert to 0.0-1.0 range
            initial_price_per_share,
        )?;

        let aggregate = Self::new(fractional_song, HashMap::new())?;
        Ok(aggregate)
    }

    /// Obtener ownership de un usuario específico
    pub fn get_user_ownership(&self, user_id: Uuid) -> Option<&ShareOwnership> {
        self.ownerships.get(&user_id)
    }

    /// Obtener resumen de ownership
    pub fn get_ownership_breakdown(&self) -> Vec<(Uuid, f64, u32)> {
        self.ownerships
            .iter()
            .map(|(user_id, ownership)| {
                (*user_id, ownership.percentage().value(), ownership.shares_owned())
            })
            .collect()
    }

    /// Calcular valor total de mercado
    pub fn calculate_market_value(&self) -> RevenueAmount {
        let total_value = self.fractional_song.total_shares() as f64 * self.fractional_song.share_price().as_f64();
        RevenueAmount::new(total_value).unwrap_or_else(|_| RevenueAmount::new(0.0).unwrap())
    }

    /// Verificar integridad del aggregate
    pub fn verify_integrity(&self) -> Result<(), FractionalOwnershipError> {
        // Verificar que la suma de porcentajes no exceda 100%
        let total_ownership_percentage: f64 = self.ownerships
            .values()
            .map(|ownership| ownership.percentage().value())
            .sum();

        if total_ownership_percentage > 100.1 { // Pequeña tolerancia para errores de punto flotante
            return Err(FractionalOwnershipError::BusinessRuleViolation(
                format!("Total ownership percentage exceeds 100%: {}", total_ownership_percentage)
            ));
        }

        // Verificar que las acciones disponibles + vendidas = total
        let sold_shares = self.ownerships
            .values()
            .map(|ownership| ownership.shares_owned())
            .sum::<u32>();

        if sold_shares + self.fractional_song.available_shares() != self.fractional_song.total_shares() {
            return Err(FractionalOwnershipError::BusinessRuleViolation(
                format!("Share count mismatch: sold {} + available {} != total {}", 
                        sold_shares, self.fractional_song.available_shares(), self.fractional_song.total_shares())
            ));
        }

        Ok(())
    }

    /// Actualizar ownership existente agregando más acciones
    fn update_existing_ownership(&mut self, user_id: Uuid, additional_shares: u32) -> Result<(), FractionalOwnershipError> {
        if let Some(ownership) = self.ownerships.get_mut(&user_id) {
            let new_total_shares = ownership.shares_owned() + additional_shares;
            let new_percentage = OwnershipPercentage::new(
                (new_total_shares as f64 / self.fractional_song.total_shares() as f64) * 100.0
            )?;

            // Crear nuevo ownership con las acciones actualizadas
            let updated_ownership = ShareOwnership::new(
                user_id,
                self.fractional_song.id(),
                new_total_shares,
                self.fractional_song.total_shares(),
                ownership.purchase_price().clone(),
            )?;

            self.ownerships.insert(user_id, updated_ownership);
        }
        Ok(())
    }

    /// Reducir ownership del usuario (para transferencias)
    fn reduce_user_ownership(&mut self, user_id: Uuid, shares_to_reduce: u32) -> Result<(), FractionalOwnershipError> {
        if let Some(ownership) = self.ownerships.get_mut(&user_id) {
            if ownership.shares_owned() < shares_to_reduce {
                return Err(FractionalOwnershipError::InsufficientShares);
            }

            let new_total_shares = ownership.shares_owned() - shares_to_reduce;
            
            if new_total_shares == 0 {
                // Remover ownership completamente
                self.ownerships.remove(&user_id);
            } else {
                // Actualizar ownership
                let new_percentage = OwnershipPercentage::new(
                    (new_total_shares as f64 / self.fractional_song.total_shares() as f64) * 100.0
                )?;

                let updated_ownership = ShareOwnership::new(
                    user_id,
                    self.fractional_song.id(),
                    new_total_shares,
                    self.fractional_song.total_shares(),
                    ownership.purchase_price().clone(),
                )?;

                self.ownerships.insert(user_id, updated_ownership);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_fractional_song() -> FractionalSong {
        FractionalSong::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "Test Song".to_string(),
            1000,
            SharePrice::new(10.0).unwrap(),
        ).unwrap()
    }

    #[test]
    fn should_create_aggregate_correctly() {
        let song = create_test_fractional_song();
        let aggregate = FractionalOwnershipAggregate::new(song, HashMap::new()).unwrap();
        
        assert_eq!(aggregate.fractional_song().total_shares(), 1000);
        assert_eq!(aggregate.ownerships().len(), 0);
    }

    #[test]
    fn should_purchase_shares_successfully() {
        let song = create_test_fractional_song();
        let mut aggregate = FractionalOwnershipAggregate::new(song, HashMap::new()).unwrap();
        let buyer_id = Uuid::new_v4();
        
        let transaction_id = aggregate.purchase_shares(buyer_id, 100).unwrap();
        aggregate.confirm_purchase(transaction_id).unwrap();
        
        let ownership = aggregate.get_user_ownership(buyer_id).unwrap();
        assert_eq!(ownership.shares_owned(), 100);
        assert_eq!(ownership.percentage().value(), 10.0);
    }

    #[test]
    fn should_distribute_revenue_proportionally() {
        let song = create_test_fractional_song();
        let mut aggregate = FractionalOwnershipAggregate::new(song, HashMap::new()).unwrap();
        
        let buyer1 = Uuid::new_v4();
        let buyer2 = Uuid::new_v4();
        
        // Usuario 1 compra 300 acciones (30%)
        let tx1 = aggregate.purchase_shares(buyer1, 300).unwrap();
        aggregate.confirm_purchase(tx1).unwrap();
        
        // Usuario 2 compra 200 acciones (20%)
        let tx2 = aggregate.purchase_shares(buyer2, 200).unwrap();
        aggregate.confirm_purchase(tx2).unwrap();
        
        // Distribuir $1000 en ingresos
        let total_revenue = RevenueAmount::new(1000.0).unwrap();
        let distribution = aggregate.distribute_revenue(total_revenue, "Q1 2024".to_string()).unwrap();
        
        // Usuario 1 debería recibir $300 (30%)
        assert_eq!(distribution.get(&buyer1).unwrap().amount(), 300.0);
        // Usuario 2 debería recibir $200 (20%)
        assert_eq!(distribution.get(&buyer2).unwrap().amount(), 200.0);
    }

    #[test]
    fn should_maintain_integrity() {
        let song = create_test_fractional_song();
        let mut aggregate = FractionalOwnershipAggregate::new(song, HashMap::new()).unwrap();
        
        let buyer = Uuid::new_v4();
        let tx = aggregate.purchase_shares(buyer, 500).unwrap();
        aggregate.confirm_purchase(tx).unwrap();
        
        // La integridad debería mantenerse
        assert!(aggregate.verify_integrity().is_ok());
        
        // Verificar que shares disponibles + vendidas = total
        assert_eq!(aggregate.fractional_song().available_shares() + 500, 1000);
    }

    #[test]
    fn should_reject_excessive_share_purchase() {
        let song = create_test_fractional_song();
        let mut aggregate = FractionalOwnershipAggregate::new(song, HashMap::new()).unwrap();
        let buyer = Uuid::new_v4();
        
        // Intentar comprar más acciones de las disponibles
        let result = aggregate.purchase_shares(buyer, 1500);
        assert!(result.is_err());
    }
} 