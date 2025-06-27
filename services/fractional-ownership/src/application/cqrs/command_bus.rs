use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

/// Trait para comandos
pub trait Command: Send + Sync + std::fmt::Debug {
    type Result: Send + Sync;
    
    fn command_id(&self) -> Uuid;
    fn command_type(&self) -> &'static str;
    fn correlation_id(&self) -> Option<Uuid> { None }
    fn causation_id(&self) -> Option<Uuid> { None }
}

/// Trait para manejadores de comandos
#[async_trait]
pub trait CommandHandler<TCommand: Command>: Send + Sync {
    async fn handle(&self, command: TCommand) -> Result<TCommand::Result, CommandError>;
}

/// Bus de comandos con middleware support
pub struct CommandBus {
    handlers: Arc<RwLock<HashMap<&'static str, Box<dyn CommandHandlerExecutor>>>>,
    middlewares: Vec<Box<dyn CommandMiddleware>>,
}

impl CommandBus {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
            middlewares: Vec::new(),
        }
    }

    /// Registrar un manejador de comando
    pub async fn register_handler<TCommand, THandler>(
        &self,
        handler: THandler,
    ) 
    where
        TCommand: Command + 'static,
        THandler: CommandHandler<TCommand> + 'static,
    {
        let mut handlers = self.handlers.write().await;
        let executor = Box::new(TypedCommandHandlerExecutor::new(handler));
        handlers.insert(std::any::type_name::<TCommand>(), executor);
    }

    /// Agregar middleware
    pub fn add_middleware<M: CommandMiddleware + 'static>(mut self, middleware: M) -> Self {
        self.middlewares.push(Box::new(middleware));
        self
    }

    /// Enviar comando
    pub async fn send<TCommand: Command + 'static>(
        &self,
        command: TCommand,
    ) -> Result<TCommand::Result, CommandError> {
        // Crear contexto de comando
        let mut context = CommandContext::new(
            command.command_id(),
            command.command_type(),
            command.correlation_id(),
            command.causation_id(),
        );

        // Ejecutar middlewares de entrada
        for middleware in &self.middlewares {
            middleware.before_handle(&mut context).await?;
        }

        // Buscar manejador
        let handlers = self.handlers.read().await;
        let handler = handlers.get(std::any::type_name::<TCommand>())
            .ok_or_else(|| CommandError::HandlerNotFound {
                command_type: command.command_type().to_string(),
            })?;

        // Ejecutar comando
        let result = handler.execute_command(Box::new(command)).await;

        // Ejecutar middlewares de salida
        for middleware in &self.middlewares {
            middleware.after_handle(&mut context, &result).await?;
        }

        // Convertir resultado
        match result {
            Ok(boxed_result) => {
                // UNSAFE: Sabemos que el tipo es correcto por el registro
                let typed_result = *boxed_result.downcast::<TCommand::Result>()
                    .map_err(|_| CommandError::TypeMismatch)?;
                Ok(typed_result)
            }
            Err(err) => Err(err),
        }
    }
}

/// Contexto de ejecución de comando
#[derive(Debug, Clone)]
pub struct CommandContext {
    pub command_id: Uuid,
    pub command_type: String,
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub metadata: HashMap<String, String>,
}

impl CommandContext {
    pub fn new(
        command_id: Uuid,
        command_type: &str,
        correlation_id: Option<Uuid>,
        causation_id: Option<Uuid>,
    ) -> Self {
        Self {
            command_id,
            command_type: command_type.to_string(),
            correlation_id,
            causation_id,
            started_at: chrono::Utc::now(),
            metadata: HashMap::new(),
        }
    }

    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
}

/// Middleware para comandos
#[async_trait]
pub trait CommandMiddleware: Send + Sync {
    async fn before_handle(&self, context: &mut CommandContext) -> Result<(), CommandError>;
    async fn after_handle(
        &self,
        context: &mut CommandContext,
        result: &Result<Box<dyn std::any::Any + Send>, CommandError>,
    ) -> Result<(), CommandError>;
}

/// Executor type-erased para manejadores
#[async_trait]
trait CommandHandlerExecutor: Send + Sync {
    async fn execute_command(
        &self,
        command: Box<dyn std::any::Any + Send>,
    ) -> Result<Box<dyn std::any::Any + Send>, CommandError>;
}

/// Implementación typed del executor
struct TypedCommandHandlerExecutor<TCommand: Command, THandler: CommandHandler<TCommand>> {
    handler: THandler,
    _phantom: std::marker::PhantomData<TCommand>,
}

impl<TCommand: Command, THandler: CommandHandler<TCommand>> 
    TypedCommandHandlerExecutor<TCommand, THandler> 
{
    fn new(handler: THandler) -> Self {
        Self {
            handler,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<TCommand: Command + 'static, THandler: CommandHandler<TCommand>> 
    CommandHandlerExecutor for TypedCommandHandlerExecutor<TCommand, THandler>
{
    async fn execute_command(
        &self,
        command: Box<dyn std::any::Any + Send>,
    ) -> Result<Box<dyn std::any::Any + Send>, CommandError> {
        let typed_command = *command.downcast::<TCommand>()
            .map_err(|_| CommandError::TypeMismatch)?;

        let result = self.handler.handle(typed_command).await?;
        Ok(Box::new(result))
    }
}

/// Errores del Command Bus
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Handler not found for command type: {command_type}")]
    HandlerNotFound { command_type: String },
    
    #[error("Type mismatch in command handling")]
    TypeMismatch,
    
    #[error("Validation error: {message}")]
    ValidationError { message: String },
    
    #[error("Business rule violation: {message}")]
    BusinessRuleViolation { message: String },
    
    #[error("Concurrency conflict: {message}")]
    ConcurrencyConflict { message: String },
    
    #[error("Infrastructure error: {message}")]
    InfrastructureError { message: String },
    
    #[error("Timeout: {message}")]
    Timeout { message: String },
}

// ============================================================================
// MIDDLEWARES ESPECÍFICOS
// ============================================================================

/// Middleware de logging
pub struct LoggingMiddleware;

#[async_trait]
impl CommandMiddleware for LoggingMiddleware {
    async fn before_handle(&self, context: &mut CommandContext) -> Result<(), CommandError> {
        tracing::info!(
            command_id = %context.command_id,
            command_type = %context.command_type,
            correlation_id = ?context.correlation_id,
            "Executing command"
        );
        Ok(())
    }

    async fn after_handle(
        &self,
        context: &mut CommandContext,
        result: &Result<Box<dyn std::any::Any + Send>, CommandError>,
    ) -> Result<(), CommandError> {
        let duration = chrono::Utc::now() - context.started_at;
        
        match result {
            Ok(_) => {
                tracing::info!(
                    command_id = %context.command_id,
                    command_type = %context.command_type,
                    duration_ms = duration.num_milliseconds(),
                    "Command executed successfully"
                );
            }
            Err(err) => {
                tracing::error!(
                    command_id = %context.command_id,
                    command_type = %context.command_type,
                    duration_ms = duration.num_milliseconds(),
                    error = %err,
                    "Command execution failed"
                );
            }
        }
        Ok(())
    }
}

/// Middleware de validación
pub struct ValidationMiddleware;

#[async_trait]
impl CommandMiddleware for ValidationMiddleware {
    async fn before_handle(&self, context: &mut CommandContext) -> Result<(), CommandError> {
        // Validaciones básicas
        if context.command_id.is_nil() {
            return Err(CommandError::ValidationError {
                message: "Command ID cannot be nil".to_string(),
            });
        }

        if context.command_type.is_empty() {
            return Err(CommandError::ValidationError {
                message: "Command type cannot be empty".to_string(),
            });
        }

        Ok(())
    }

    async fn after_handle(
        &self,
        _context: &mut CommandContext,
        _result: &Result<Box<dyn std::any::Any + Send>, CommandError>,
    ) -> Result<(), CommandError> {
        Ok(())
    }
}

/// Middleware de métricas
pub struct MetricsMiddleware {
    // TODO: Integrar con Prometheus
}

impl MetricsMiddleware {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl CommandMiddleware for MetricsMiddleware {
    async fn before_handle(&self, context: &mut CommandContext) -> Result<(), CommandError> {
        context.add_metadata("start_time".to_string(), context.started_at.to_rfc3339());
        Ok(())
    }

    async fn after_handle(
        &self,
        context: &mut CommandContext,
        result: &Result<Box<dyn std::any::Any + Send>, CommandError>,
    ) -> Result<(), CommandError> {
        let duration = chrono::Utc::now() - context.started_at;
        
        // TODO: Enviar métricas a Prometheus
        let status = if result.is_ok() { "success" } else { "error" };
        
        tracing::debug!(
            command_type = %context.command_type,
            duration_ms = duration.num_milliseconds(),
            status = status,
            "Command metrics recorded"
        );
        
        Ok(())
    }
}

// ============================================================================
// COMANDOS ESPECÍFICOS PARA FRACTIONAL OWNERSHIP
// ============================================================================

/// Comando para comprar acciones
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseSharesCommand {
    pub command_id: Uuid,
    pub correlation_id: Option<Uuid>,
    pub user_id: Uuid,
    pub fractional_song_id: Uuid,
    pub shares_quantity: u32,
    pub max_price_per_share: f64,
    pub auto_confirm: bool,
}

impl Command for PurchaseSharesCommand {
    type Result = PurchaseSharesResult;

    fn command_id(&self) -> Uuid {
        self.command_id
    }

    fn command_type(&self) -> &'static str {
        "PurchaseShares"
    }

    fn correlation_id(&self) -> Option<Uuid> {
        self.correlation_id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseSharesResult {
    pub transaction_id: Uuid,
    pub shares_purchased: u32,
    pub total_cost: f64,
    pub new_ownership_percentage: f64,
    pub status: String,
}

/// Comando para transferir acciones
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferSharesCommand {
    pub command_id: Uuid,
    pub correlation_id: Option<Uuid>,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub fractional_song_id: Uuid,
    pub shares_quantity: u32,
    pub price_per_share: f64,
}

impl Command for TransferSharesCommand {
    type Result = TransferSharesResult;

    fn command_id(&self) -> Uuid {
        self.command_id
    }

    fn command_type(&self) -> &'static str {
        "TransferShares"
    }

    fn correlation_id(&self) -> Option<Uuid> {
        self.correlation_id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferSharesResult {
    pub transaction_id: Uuid,
    pub shares_transferred: u32,
    pub total_amount: f64,
    pub status: String,
}

/// Comando para distribuir ingresos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributeRevenueCommand {
    pub command_id: Uuid,
    pub correlation_id: Option<Uuid>,
    pub fractional_song_id: Uuid,
    pub total_revenue: f64,
    pub revenue_source: String,
    pub distribution_date: chrono::DateTime<chrono::Utc>,
}

impl Command for DistributeRevenueCommand {
    type Result = DistributeRevenueResult;

    fn command_id(&self) -> Uuid {
        self.command_id
    }

    fn command_type(&self) -> &'static str {
        "DistributeRevenue"
    }

    fn correlation_id(&self) -> Option<Uuid> {
        self.correlation_id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributeRevenueResult {
    pub distribution_id: Uuid,
    pub total_distributed: f64,
    pub shareholders_count: u32,
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    // Mock handler para testing
    struct MockPurchaseSharesHandler {
        call_count: AtomicU32,
    }

    impl MockPurchaseSharesHandler {
        fn new() -> Self {
            Self {
                call_count: AtomicU32::new(0),
            }
        }

        fn get_call_count(&self) -> u32 {
            self.call_count.load(Ordering::SeqCst)
        }
    }

    #[async_trait]
    impl CommandHandler<PurchaseSharesCommand> for MockPurchaseSharesHandler {
        async fn handle(&self, command: PurchaseSharesCommand) -> Result<PurchaseSharesResult, CommandError> {
            self.call_count.fetch_add(1, Ordering::SeqCst);
            
            Ok(PurchaseSharesResult {
                transaction_id: Uuid::new_v4(),
                shares_purchased: command.shares_quantity,
                total_cost: command.shares_quantity as f64 * command.max_price_per_share,
                new_ownership_percentage: 10.0,
                status: "Completed".to_string(),
            })
        }
    }

    #[tokio::test]
    async fn should_register_and_execute_command_handler() {
        let bus = CommandBus::new()
            .add_middleware(LoggingMiddleware)
            .add_middleware(ValidationMiddleware);

        let handler = MockPurchaseSharesHandler::new();
        bus.register_handler::<PurchaseSharesCommand, _>(handler).await;

        let command = PurchaseSharesCommand {
            command_id: Uuid::new_v4(),
            correlation_id: None,
            user_id: Uuid::new_v4(),
            fractional_song_id: Uuid::new_v4(),
            shares_quantity: 100,
            max_price_per_share: 10.0,
            auto_confirm: true,
        };

        let result = bus.send(command).await.unwrap();
        assert_eq!(result.shares_purchased, 100);
        assert_eq!(result.total_cost, 1000.0);
        assert_eq!(result.status, "Completed");
    }

    #[tokio::test]
    async fn should_fail_when_handler_not_registered() {
        let bus = CommandBus::new();

        let command = PurchaseSharesCommand {
            command_id: Uuid::new_v4(),
            correlation_id: None,
            user_id: Uuid::new_v4(),
            fractional_song_id: Uuid::new_v4(),
            shares_quantity: 100,
            max_price_per_share: 10.0,
            auto_confirm: true,
        };

        let result = bus.send(command).await;
        assert!(matches!(result, Err(CommandError::HandlerNotFound { .. })));
    }

    #[tokio::test]
    async fn should_validate_command_id() {
        let bus = CommandBus::new()
            .add_middleware(ValidationMiddleware);

        let handler = MockPurchaseSharesHandler::new();
        bus.register_handler::<PurchaseSharesCommand, _>(handler).await;

        let command = PurchaseSharesCommand {
            command_id: Uuid::nil(), // Invalid ID
            correlation_id: None,
            user_id: Uuid::new_v4(),
            fractional_song_id: Uuid::new_v4(),
            shares_quantity: 100,
            max_price_per_share: 10.0,
            auto_confirm: true,
        };

        let result = bus.send(command).await;
        assert!(matches!(result, Err(CommandError::ValidationError { .. })));
    }
} 