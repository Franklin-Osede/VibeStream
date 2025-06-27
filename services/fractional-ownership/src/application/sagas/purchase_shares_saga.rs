use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Estado de un Saga
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SagaState {
    NotStarted,
    InProgress,
    Completed,
    Failed,
    Compensating,
    Compensated,
}

/// Resultado de ejecutar un paso del Saga
#[derive(Debug, Clone)]
pub enum SagaStepResult {
    Success(serde_json::Value),
    Failure(String),
    Retry,
}

/// Definición de un paso del Saga
#[async_trait]
pub trait SagaStep: Send + Sync {
    /// Ejecutar el paso
    async fn execute(&self, context: &mut SagaContext) -> SagaStepResult;
    
    /// Compensar el paso (rollback)
    async fn compensate(&self, context: &mut SagaContext) -> SagaStepResult;
    
    /// Nombre del paso para logging
    fn step_name(&self) -> &str;
    
    /// ¿Es un paso crítico que no se puede fallar?
    fn is_critical(&self) -> bool {
        false
    }
}

/// Contexto compartido entre pasos del Saga
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SagaContext {
    pub saga_id: Uuid,
    pub correlation_id: Uuid,
    pub user_id: Uuid,
    pub fractional_song_id: Uuid,
    pub shares_quantity: u32,
    pub total_amount: f64,
    pub blockchain_tx_hash: Option<String>,
    pub nft_token_id: Option<Uuid>,
    pub payment_id: Option<Uuid>,
    pub step_data: HashMap<String, serde_json::Value>,
    pub started_at: DateTime<Utc>,
    pub completed_steps: Vec<String>,
    pub failed_steps: Vec<String>,
}

impl SagaContext {
    pub fn new(
        user_id: Uuid,
        fractional_song_id: Uuid,
        shares_quantity: u32,
        total_amount: f64,
    ) -> Self {
        Self {
            saga_id: Uuid::new_v4(),
            correlation_id: Uuid::new_v4(),
            user_id,
            fractional_song_id,
            shares_quantity,
            total_amount,
            blockchain_tx_hash: None,
            nft_token_id: None,
            payment_id: None,
            step_data: HashMap::new(),
            started_at: Utc::now(),
            completed_steps: Vec::new(),
            failed_steps: Vec::new(),
        }
    }

    pub fn set_step_data(&mut self, key: String, value: serde_json::Value) {
        self.step_data.insert(key, value);
    }

    pub fn get_step_data(&self, key: &str) -> Option<&serde_json::Value> {
        self.step_data.get(key)
    }

    pub fn mark_step_completed(&mut self, step_name: String) {
        if !self.completed_steps.contains(&step_name) {
            self.completed_steps.push(step_name);
        }
    }

    pub fn mark_step_failed(&mut self, step_name: String) {
        if !self.failed_steps.contains(&step_name) {
            self.failed_steps.push(step_name);
        }
    }
}

/// Saga para compra de acciones fraccionadas
pub struct PurchaseSharesSaga {
    steps: Vec<Box<dyn SagaStep>>,
    state: SagaState,
    context: SagaContext,
    current_step: usize,
    retry_count: HashMap<String, u32>,
    max_retries: u32,
}

impl PurchaseSharesSaga {
    pub fn new(context: SagaContext) -> Self {
        Self {
            steps: Vec::new(),
            state: SagaState::NotStarted,
            context,
            current_step: 0,
            retry_count: HashMap::new(),
            max_retries: 3,
        }
    }

    pub fn add_step(mut self, step: Box<dyn SagaStep>) -> Self {
        self.steps.push(step);
        self
    }

    /// Ejecutar el Saga completo
    pub async fn execute(&mut self) -> Result<SagaContext, SagaError> {
        self.state = SagaState::InProgress;
        
        // Ejecutar cada paso secuencialmente
        for (index, step) in self.steps.iter().enumerate() {
            self.current_step = index;
            
            match self.execute_step_with_retry(step).await {
                SagaStepResult::Success(data) => {
                    self.context.set_step_data(
                        format!("step_{}_result", index),
                        data,
                    );
                    self.context.mark_step_completed(step.step_name().to_string());
                    tracing::info!(
                        saga_id = %self.context.saga_id,
                        step = step.step_name(),
                        "Saga step completed successfully"
                    );
                }
                SagaStepResult::Failure(error) => {
                    self.context.mark_step_failed(step.step_name().to_string());
                    self.state = SagaState::Failed;
                    
                    tracing::error!(
                        saga_id = %self.context.saga_id,
                        step = step.step_name(),
                        error = %error,
                        "Saga step failed, starting compensation"
                    );
                    
                    // Compensar pasos completados
                    self.compensate().await?;
                    
                    return Err(SagaError::StepFailed {
                        step_name: step.step_name().to_string(),
                        error,
                    });
                }
                SagaStepResult::Retry => {
                    // Ya manejado en execute_step_with_retry
                    unreachable!("Retry should be handled in execute_step_with_retry");
                }
            }
        }

        self.state = SagaState::Completed;
        tracing::info!(
            saga_id = %self.context.saga_id,
            duration_ms = (Utc::now() - self.context.started_at).num_milliseconds(),
            "Saga completed successfully"
        );

        Ok(self.context.clone())
    }

    /// Ejecutar un paso con reintentos
    async fn execute_step_with_retry(&mut self, step: &Box<dyn SagaStep>) -> SagaStepResult {
        let step_name = step.step_name();
        let retry_count = *self.retry_count.get(step_name).unwrap_or(&0);

        loop {
            match step.execute(&mut self.context).await {
                SagaStepResult::Success(data) => {
                    self.retry_count.remove(step_name);
                    return SagaStepResult::Success(data);
                }
                SagaStepResult::Failure(error) => {
                    if step.is_critical() {
                        return SagaStepResult::Failure(error);
                    }
                    
                    let current_retries = *self.retry_count.get(step_name).unwrap_or(&0);
                    if current_retries >= self.max_retries {
                        return SagaStepResult::Failure(format!(
                            "Step failed after {} retries: {}",
                            self.max_retries, error
                        ));
                    }
                    
                    self.retry_count.insert(step_name.to_string(), current_retries + 1);
                    
                    tracing::warn!(
                        saga_id = %self.context.saga_id,
                        step = step_name,
                        retry_count = current_retries + 1,
                        max_retries = self.max_retries,
                        error = %error,
                        "Retrying saga step"
                    );
                    
                    // Exponential backoff
                    let delay_ms = 1000 * (2_u64.pow(current_retries));
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                }
                SagaStepResult::Retry => {
                    let current_retries = *self.retry_count.get(step_name).unwrap_or(&0);
                    if current_retries >= self.max_retries {
                        return SagaStepResult::Failure(format!(
                            "Step exceeded max retries: {}",
                            self.max_retries
                        ));
                    }
                    
                    self.retry_count.insert(step_name.to_string(), current_retries + 1);
                    
                    // Shorter delay for explicit retries
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                }
            }
        }
    }

    /// Compensar pasos completados (en orden inverso)
    async fn compensate(&mut self) -> Result<(), SagaError> {
        self.state = SagaState::Compensating;
        
        // Compensar en orden inverso
        let completed_steps: Vec<_> = self.context.completed_steps.clone();
        
        for step_name in completed_steps.iter().rev() {
            // Encontrar el paso por nombre
            if let Some(step) = self.steps.iter().find(|s| s.step_name() == step_name) {
                match step.compensate(&mut self.context).await {
                    SagaStepResult::Success(_) => {
                        tracing::info!(
                            saga_id = %self.context.saga_id,
                            step = step_name,
                            "Compensation completed successfully"
                        );
                    }
                    SagaStepResult::Failure(error) => {
                        tracing::error!(
                            saga_id = %self.context.saga_id,
                            step = step_name,
                            error = %error,
                            "Compensation failed - manual intervention required"
                        );
                        
                        return Err(SagaError::CompensationFailed {
                            step_name: step_name.clone(),
                            error,
                        });
                    }
                    SagaStepResult::Retry => {
                        // Reintento de compensación
                        tracing::warn!(
                            saga_id = %self.context.saga_id,
                            step = step_name,
                            "Retrying compensation"
                        );
                        // TODO: Implementar lógica de reintentos para compensación
                    }
                }
            }
        }

        self.state = SagaState::Compensated;
        Ok(())
    }

    pub fn get_state(&self) -> &SagaState {
        &self.state
    }

    pub fn get_context(&self) -> &SagaContext {
        &self.context
    }
}

/// Errores del Saga
#[derive(Debug, thiserror::Error)]
pub enum SagaError {
    #[error("Saga step '{step_name}' failed: {error}")]
    StepFailed { step_name: String, error: String },
    
    #[error("Compensation failed for step '{step_name}': {error}")]
    CompensationFailed { step_name: String, error: String },
    
    #[error("Saga timeout after {timeout_ms}ms")]
    Timeout { timeout_ms: u64 },
    
    #[error("Invalid saga state: {current_state:?}")]
    InvalidState { current_state: SagaState },
}

/// Factory para crear Sagas de compra de acciones
pub struct PurchaseSharesSagaFactory;

impl PurchaseSharesSagaFactory {
    pub fn create_purchase_saga(
        user_id: Uuid,
        fractional_song_id: Uuid,
        shares_quantity: u32,
        total_amount: f64,
    ) -> PurchaseSharesSaga {
        let context = SagaContext::new(user_id, fractional_song_id, shares_quantity, total_amount);
        
        PurchaseSharesSaga::new(context)
            .add_step(Box::new(ValidateUserStep))
            .add_step(Box::new(ValidateFundsStep))
            .add_step(Box::new(LockSharesStep))
            .add_step(Box::new(ProcessPaymentStep))
            .add_step(Box::new(MintOwnershipNFTStep))
            .add_step(Box::new(UpdateDatabaseStep))
            .add_step(Box::new(NotifyUserStep))
    }
}

// Implementaciones de pasos específicos

pub struct ValidateUserStep;

#[async_trait]
impl SagaStep for ValidateUserStep {
    async fn execute(&self, context: &mut SagaContext) -> SagaStepResult {
        // TODO: Validar que el usuario existe y está verificado
        tracing::info!(user_id = %context.user_id, "Validating user");
        
        // Simular validación
        if context.user_id.is_nil() {
            return SagaStepResult::Failure("Invalid user ID".to_string());
        }
        
        SagaStepResult::Success(serde_json::json!({
            "user_validated": true,
            "validation_timestamp": Utc::now()
        }))
    }

    async fn compensate(&self, _context: &mut SagaContext) -> SagaStepResult {
        // Validación no necesita compensación
        SagaStepResult::Success(serde_json::json!({}))
    }

    fn step_name(&self) -> &str {
        "validate_user"
    }

    fn is_critical(&self) -> bool {
        true // Usuario debe ser válido
    }
}

pub struct ValidateFundsStep;

#[async_trait]
impl SagaStep for ValidateFundsStep {
    async fn execute(&self, context: &mut SagaContext) -> SagaStepResult {
        tracing::info!(
            user_id = %context.user_id,
            amount = context.total_amount,
            "Validating user funds"
        );
        
        // TODO: Verificar que el usuario tiene fondos suficientes
        SagaStepResult::Success(serde_json::json!({
            "funds_validated": true,
            "available_balance": context.total_amount * 2.0 // Simular balance
        }))
    }

    async fn compensate(&self, _context: &mut SagaContext) -> SagaStepResult {
        // Validación no necesita compensación
        SagaStepResult::Success(serde_json::json!({}))
    }

    fn step_name(&self) -> &str {
        "validate_funds"
    }

    fn is_critical(&self) -> bool {
        true
    }
}

pub struct LockSharesStep;

#[async_trait]
impl SagaStep for LockSharesStep {
    async fn execute(&self, context: &mut SagaContext) -> SagaStepResult {
        tracing::info!(
            fractional_song_id = %context.fractional_song_id,
            shares = context.shares_quantity,
            "Locking shares on smart contract"
        );
        
        // TODO: Llamar smart contract para reservar acciones
        let tx_hash = format!("0x{}", Uuid::new_v4().simple());
        context.blockchain_tx_hash = Some(tx_hash.clone());
        
        SagaStepResult::Success(serde_json::json!({
            "shares_locked": true,
            "blockchain_tx_hash": tx_hash,
            "shares_quantity": context.shares_quantity
        }))
    }

    async fn compensate(&self, context: &mut SagaContext) -> SagaStepResult {
        if let Some(tx_hash) = &context.blockchain_tx_hash {
            tracing::info!(
                tx_hash = %tx_hash,
                "Unlocking shares on smart contract"
            );
            
            // TODO: Llamar smart contract para liberar acciones
            SagaStepResult::Success(serde_json::json!({
                "shares_unlocked": true
            }))
        } else {
            SagaStepResult::Success(serde_json::json!({}))
        }
    }

    fn step_name(&self) -> &str {
        "lock_shares"
    }
}

pub struct ProcessPaymentStep;

#[async_trait]
impl SagaStep for ProcessPaymentStep {
    async fn execute(&self, context: &mut SagaContext) -> SagaStepResult {
        tracing::info!(
            user_id = %context.user_id,
            amount = context.total_amount,
            "Processing payment"
        );
        
        // TODO: Procesar pago
        let payment_id = Uuid::new_v4();
        context.payment_id = Some(payment_id);
        
        SagaStepResult::Success(serde_json::json!({
            "payment_processed": true,
            "payment_id": payment_id,
            "amount": context.total_amount
        }))
    }

    async fn compensate(&self, context: &mut SagaContext) -> SagaStepResult {
        if let Some(payment_id) = context.payment_id {
            tracing::info!(
                payment_id = %payment_id,
                "Refunding payment"
            );
            
            // TODO: Reembolsar pago
            SagaStepResult::Success(serde_json::json!({
                "payment_refunded": true
            }))
        } else {
            SagaStepResult::Success(serde_json::json!({}))
        }
    }

    fn step_name(&self) -> &str {
        "process_payment"
    }
}

pub struct MintOwnershipNFTStep;

#[async_trait]
impl SagaStep for MintOwnershipNFTStep {
    async fn execute(&self, context: &mut SagaContext) -> SagaStepResult {
        tracing::info!(
            user_id = %context.user_id,
            fractional_song_id = %context.fractional_song_id,
            "Minting ownership NFT"
        );
        
        // TODO: Mint NFT de ownership
        let nft_token_id = Uuid::new_v4();
        context.nft_token_id = Some(nft_token_id);
        
        SagaStepResult::Success(serde_json::json!({
            "nft_minted": true,
            "nft_token_id": nft_token_id,
            "shares_represented": context.shares_quantity
        }))
    }

    async fn compensate(&self, context: &mut SagaContext) -> SagaStepResult {
        if let Some(nft_token_id) = context.nft_token_id {
            tracing::info!(
                nft_token_id = %nft_token_id,
                "Burning ownership NFT"
            );
            
            // TODO: Burn NFT
            SagaStepResult::Success(serde_json::json!({
                "nft_burned": true
            }))
        } else {
            SagaStepResult::Success(serde_json::json!({}))
        }
    }

    fn step_name(&self) -> &str {
        "mint_ownership_nft"
    }
}

pub struct UpdateDatabaseStep;

#[async_trait]
impl SagaStep for UpdateDatabaseStep {
    async fn execute(&self, context: &mut SagaContext) -> SagaStepResult {
        tracing::info!(
            user_id = %context.user_id,
            fractional_song_id = %context.fractional_song_id,
            "Updating database records"
        );
        
        // TODO: Actualizar base de datos
        SagaStepResult::Success(serde_json::json!({
            "database_updated": true,
            "ownership_recorded": true
        }))
    }

    async fn compensate(&self, context: &mut SagaContext) -> SagaStepResult {
        tracing::info!(
            user_id = %context.user_id,
            "Reverting database changes"
        );
        
        // TODO: Revertir cambios en BD
        SagaStepResult::Success(serde_json::json!({
            "database_reverted": true
        }))
    }

    fn step_name(&self) -> &str {
        "update_database"
    }
}

pub struct NotifyUserStep;

#[async_trait]
impl SagaStep for NotifyUserStep {
    async fn execute(&self, context: &mut SagaContext) -> SagaStepResult {
        tracing::info!(
            user_id = %context.user_id,
            "Notifying user of successful purchase"
        );
        
        // TODO: Enviar notificación al usuario
        SagaStepResult::Success(serde_json::json!({
            "user_notified": true,
            "notification_sent_at": Utc::now()
        }))
    }

    async fn compensate(&self, context: &mut SagaContext) -> SagaStepResult {
        tracing::info!(
            user_id = %context.user_id,
            "Notifying user of purchase failure"
        );
        
        // TODO: Enviar notificación de fallo
        SagaStepResult::Success(serde_json::json!({
            "failure_notification_sent": true
        }))
    }

    fn step_name(&self) -> &str {
        "notify_user"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_execute_successful_saga() {
        let user_id = Uuid::new_v4();
        let song_id = Uuid::new_v4();
        
        let mut saga = PurchaseSharesSagaFactory::create_purchase_saga(
            user_id,
            song_id,
            100,
            1000.0,
        );

        let result = saga.execute().await;
        assert!(result.is_ok());
        assert_eq!(*saga.get_state(), SagaState::Completed);

        let context = result.unwrap();
        assert_eq!(context.shares_quantity, 100);
        assert_eq!(context.total_amount, 1000.0);
        assert!(context.blockchain_tx_hash.is_some());
        assert!(context.nft_token_id.is_some());
        assert!(context.payment_id.is_some());
    }

    #[tokio::test]
    async fn should_compensate_on_failure() {
        // TODO: Test con step que falla para verificar compensación
    }
} 