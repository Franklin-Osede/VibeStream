use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use async_trait::async_trait;

use crate::shared::application::command::{Command, CommandHandler};
use crate::shared::domain::errors::AppError;

/// A trait-object friendly wrapper around any concrete `CommandHandler`.
#[async_trait]
trait ErasedCommandHandler: Send + Sync {
    async fn handle_boxed(&self, cmd: Box<dyn Any + Send>) -> Result<Box<dyn Any + Send>, AppError>;
}

/// Wrapper that bridges a typed `CommandHandler<C>` to `ErasedCommandHandler`.
struct HandlerWrapper<C, H>
where
    C: Command + 'static,
    H: CommandHandler<C> + Send + Sync + 'static,
{
    inner: H,
    _marker: std::marker::PhantomData<C>,
}

#[async_trait]
impl<C, H> ErasedCommandHandler for HandlerWrapper<C, H>
where
    C: Command + 'static,
    H: CommandHandler<C> + Send + Sync + 'static,
{
    async fn handle_boxed(&self, cmd: Box<dyn Any + Send>) -> Result<Box<dyn Any + Send>, AppError> {
        // Downcast the boxed command to the expected concrete type.
        let cmd = cmd
            .downcast::<C>()
            .map_err(|_| AppError::Internal("Invalid command type".into()))?;
        let cmd = *cmd; // move out of Box
        let output = self.inner.handle(cmd).await?;
        Ok(Box::new(output))
    }
}

/// In-memory Command Bus.
#[derive(Default)]
pub struct InMemoryCommandBus {
    handlers: RwLock<HashMap<TypeId, Arc<dyn ErasedCommandHandler>>>,
}

impl InMemoryCommandBus {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a concrete `CommandHandler<C>` for the given command type `C`.
    pub async fn register<C, H>(&self, handler: H)
    where
        C: Command + 'static,
        H: CommandHandler<C> + Send + Sync + 'static,
    {
        let mut map = self.handlers.write().await;
        let wrapper = HandlerWrapper::<C, H> {
            inner: handler,
            _marker: std::marker::PhantomData,
        };
        map.insert(TypeId::of::<C>(), Arc::new(wrapper));
    }

    /// Dispatches a command and returns the raw boxed output (`Any`).
    /// The caller is expected to downcast to the expected type.
    pub async fn dispatch<C>(&self, command: C) -> Result<Box<dyn Any + Send>, AppError>
    where
        C: Command + 'static,
    {
        let map = self.handlers.read().await;
        let handler = map
            .get(&TypeId::of::<C>())
            .ok_or_else(|| AppError::Internal("No handler registered for command".into()))?;
        handler.handle_boxed(Box::new(command)).await
    }
}

// ---------------------------------------------------------
// Tests
// ---------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::application::command::Command;

    #[derive(Debug)]
    struct SayHello;
    impl Command for SayHello {}

    struct SayHelloHandler;

    #[async_trait]
    impl CommandHandler<SayHello> for SayHelloHandler {
        type Output = String;
        async fn handle(&self, _cmd: SayHello) -> Result<Self::Output, AppError> {
            Ok("Hello!".to_string())
        }
    }

    #[tokio::test]
    async fn dispatch_executes_registered_handler() {
        let bus = InMemoryCommandBus::new();
        bus.register::<SayHello, _>(SayHelloHandler).await;

        let boxed_res = bus.dispatch(SayHello).await.unwrap();
        let res = *boxed_res.downcast::<String>().unwrap();
        assert_eq!(res, "Hello!");
    }
} 