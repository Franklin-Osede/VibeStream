// =============================================================================
// REDIS STREAMS EVENT BUS IMPLEMENTATION
// =============================================================================
// 
// Implementación persistente del Event Bus usando Redis Streams.
// Reemplaza el InMemoryEventBus para producción.

use std::sync::Arc;
use std::collections::HashMap;
use async_trait::async_trait;
use redis::Client as RedisClient;
use tokio::sync::RwLock;
use tracing::{info, error, warn};

use crate::shared::domain::errors::AppError;
use super::{EventBus, EventHandler, DomainEvent};

/// Stream name para eventos de dominio en Redis
const DOMAIN_EVENTS_STREAM: &str = "vibestream:domain-events";

/// Consumer group name para procesar eventos
const CONSUMER_GROUP_NAME: &str = "vibestream-event-handlers";

/// Consumer name para este worker
const CONSUMER_NAME: &str = "api-gateway-worker";

/// Redis Streams Event Bus
/// 
/// Publica eventos a Redis Streams y procesa eventos usando Consumer Groups
/// para garantizar procesamiento distribuido y persistente.
pub struct RedisStreamsEventBus {
    redis_client: Arc<RedisClient>,
    handlers: Arc<RwLock<HashMap<String, Vec<Arc<dyn EventHandler>>>>>,
    stream_name: String,
    consumer_group: String,
    consumer_name: String,
}

impl RedisStreamsEventBus {
    /// Crear una nueva instancia del Redis Streams Event Bus
    /// 
    /// # Arguments
    /// * `redis_url` - URL de conexión a Redis (ej: "redis://localhost:6379")
    /// 
    /// # Returns
    /// * `Result<Self, AppError>` - Event bus inicializado o error
    pub async fn new(redis_url: &str) -> Result<Self, AppError> {
        let client = RedisClient::open(redis_url)
            .map_err(|e| AppError::InternalError(format!("Failed to connect to Redis: {}", e)))?;

        // Test connection
        let mut conn = client.get_async_connection().await
            .map_err(|e| AppError::InternalError(format!("Failed to get Redis connection: {}", e)))?;
        
        let _: String = redis::cmd("PING")
            .query_async(&mut conn)
            .await
            .map_err(|e| AppError::InternalError(format!("Redis PING failed: {}", e)))?;

        let event_bus = Self {
            redis_client: Arc::new(client),
            handlers: Arc::new(RwLock::new(HashMap::new())),
            stream_name: DOMAIN_EVENTS_STREAM.to_string(),
            consumer_group: CONSUMER_GROUP_NAME.to_string(),
            consumer_name: CONSUMER_NAME.to_string(),
        };

        // Crear consumer group si no existe
        event_bus.ensure_consumer_group().await?;

        info!("✅ Redis Streams Event Bus initialized - Stream: {}", event_bus.stream_name);

        Ok(event_bus)
    }

    /// Asegurar que el consumer group existe
    async fn ensure_consumer_group(&self) -> Result<(), AppError> {
        let mut conn = self.redis_client.get_async_connection().await
            .map_err(|e| AppError::InternalError(format!("Failed to get Redis connection: {}", e)))?;

        // Intentar crear el consumer group desde el inicio (0)
        // Si ya existe, ignoramos el error
        let result: Result<String, redis::RedisError> = redis::cmd("XGROUP")
            .arg("CREATE")
            .arg(&self.stream_name)
            .arg(&self.consumer_group)
            .arg("0")
            .arg("MKSTREAM")
            .query_async(&mut conn)
            .await;

        match result {
            Ok(_) => {
                info!("✅ Created consumer group: {}", self.consumer_group);
            }
            Err(e) => {
                // Si el error es que el grupo ya existe, está bien
                if e.to_string().contains("BUSYGROUP") {
                    info!("ℹ️  Consumer group already exists: {}", self.consumer_group);
                } else {
                    warn!("⚠️  Failed to create consumer group (may already exist): {}", e);
                }
            }
        }

        Ok(())
    }

    /// Serializar evento a JSON para almacenar en Redis
    fn serialize_event(event: &DomainEvent) -> Result<String, AppError> {
        serde_json::to_string(event)
            .map_err(|e| AppError::InternalError(format!("Failed to serialize event: {}", e)))
    }

    /// Deserializar evento desde JSON
    fn deserialize_event(data: &str) -> Result<DomainEvent, AppError> {
        serde_json::from_str(data)
            .map_err(|e| AppError::InternalError(format!("Failed to deserialize event: {}", e)))
    }

    /// Procesar eventos pendientes del consumer group
    /// 
    /// Este método debe ser llamado periódicamente por un worker
    pub async fn process_pending_events(&self) -> Result<usize, AppError> {
        let mut conn = self.redis_client.get_async_connection().await
            .map_err(|e| AppError::InternalError(format!("Failed to get Redis connection: {}", e)))?;

        // Leer eventos pendientes para este consumer
        // XPENDING retorna una lista de tuplas: (id, consumer, idle_time, delivery_count)
        let pending: Vec<(String, String, u64, u64)> = redis::cmd("XPENDING")
            .arg(&self.stream_name)
            .arg(&self.consumer_group)
            .arg("-")
            .arg("+")
            .arg("100") // Max count
            .arg(&self.consumer_name)
            .query_async(&mut conn)
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to read pending events: {}", e)))?;

        let mut processed = 0;

        for (stream_id, _consumer, _idle_time, _delivery_count) in pending {
            // Claim y procesar el evento
            let claimed: Vec<redis::streams::StreamReadReply> = redis::cmd("XCLAIM")
                .arg(&self.stream_name)
                .arg(&self.consumer_group)
                .arg(&self.consumer_name)
                .arg("60000") // Min idle time (ms)
                .arg(&stream_id)
                .query_async(&mut conn)
                .await
                .map_err(|e| AppError::InternalError(format!("Failed to claim event: {}", e)))?;

            for reply in claimed {
                for stream_key in reply.keys {
                    for stream_id_entry in stream_key.ids {
                        if let Err(e) = self.process_event_entry(&stream_id_entry).await {
                            error!("Error processing pending event {}: {:?}", stream_id_entry.id, e);
                        } else {
                            // ACK el evento después de procesarlo
                            let _: i64 = redis::cmd("XACK")
                                .arg(&self.stream_name)
                                .arg(&self.consumer_group)
                                .arg(&stream_id_entry.id)
                                .query_async(&mut conn)
                                .await
                                .map_err(|e| AppError::InternalError(format!("Failed to ACK event: {}", e)))?;
                            
                            processed += 1;
                        }
                    }
                }
            }
        }

        Ok(processed)
    }

    /// Leer y procesar nuevos eventos del stream
    pub async fn read_and_process_events(&self) -> Result<usize, AppError> {
        let mut conn = self.redis_client.get_async_connection().await
            .map_err(|e| AppError::InternalError(format!("Failed to get Redis connection: {}", e)))?;

        // Leer nuevos eventos del stream
        let replies: Vec<redis::streams::StreamReadReply> = redis::cmd("XREADGROUP")
            .arg("GROUP")
            .arg(&self.consumer_group)
            .arg(&self.consumer_name)
            .arg("COUNT")
            .arg("10") // Leer hasta 10 eventos a la vez
            .arg("BLOCK")
            .arg("1000") // Block por 1 segundo si no hay eventos
            .arg("STREAMS")
            .arg(&self.stream_name)
            .arg(">") // Leer solo nuevos eventos
            .query_async(&mut conn)
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to read events from stream: {}", e)))?;

        let mut processed = 0;

        for reply in replies {
            for stream_key in reply.keys {
                for stream_id_entry in stream_key.ids {
                    if let Err(e) = self.process_event_entry(&stream_id_entry).await {
                        error!("Error processing event {}: {:?}", stream_id_entry.id, e);
                    } else {
                        // ACK el evento después de procesarlo
                        let _: i64 = redis::cmd("XACK")
                            .arg(&self.stream_name)
                            .arg(&self.consumer_group)
                            .arg(&stream_id_entry.id)
                            .query_async(&mut conn)
                            .await
                            .map_err(|e| AppError::InternalError(format!("Failed to ACK event: {}", e)))?;
                        
                        processed += 1;
                    }
                }
            }
        }

        Ok(processed)
    }

    /// Procesar una entrada del stream
    async fn process_event_entry(&self, entry: &redis::streams::StreamId) -> Result<(), AppError> {
        // Extraer el evento desde los campos del stream entry
        // StreamId tiene un campo `map` que es un HashMap<String, redis::Value>
        let event_data_value = entry
            .map
            .get("data")
            .ok_or_else(|| AppError::InternalError("Event data not found in stream entry".to_string()))?;
        
        // Convertir Value a String
        let event_data = match event_data_value {
            redis::Value::Data(bytes) => {
                String::from_utf8(bytes.clone())
                    .map_err(|e| AppError::InternalError(format!("Failed to decode event data: {}", e)))?
            }
            redis::Value::Status(s) => s.clone(),
            redis::Value::Int(i) => i.to_string(),
            redis::Value::Bulk(bulk) => {
                // Si es un bulk, intentar convertir el primer elemento
                if let Some(redis::Value::Data(bytes)) = bulk.first() {
                    String::from_utf8(bytes.clone())
                        .map_err(|e| AppError::InternalError(format!("Failed to decode event data: {}", e)))?
                } else {
                    return Err(AppError::InternalError("Unexpected event data format".to_string()));
                }
            }
            _ => {
                return Err(AppError::InternalError("Unexpected event data type".to_string()));
            }
        };

        let event: DomainEvent = Self::deserialize_event(&event_data)?;
        let event_type = event.event_type();

        // Obtener handlers para este tipo de evento
        let handlers = {
            let handlers_guard = self.handlers.read().await;
            handlers_guard.get(event_type).cloned().unwrap_or_default()
        };

        // Procesar con todos los handlers registrados
        for handler in handlers {
            if let Err(e) = handler.handle(&event).await {
                error!("Error in handler for event {}: {:?}", event_type, e);
                // Continuamos con otros handlers aunque uno falle
            }
        }

        Ok(())
    }
}

#[async_trait]
impl EventBus for RedisStreamsEventBus {
    async fn publish(&self, event: DomainEvent) -> Result<(), AppError> {
        let event_type = event.event_type();
        let event_json = Self::serialize_event(&event)?;

        let mut conn = self.redis_client.get_async_connection().await
            .map_err(|e| AppError::InternalError(format!("Failed to get Redis connection: {}", e)))?;

        // Publicar evento al stream usando XADD
        // Usamos el event_type como parte del ID para facilitar particionado
        let stream_id: String = redis::cmd("XADD")
            .arg(&self.stream_name)
            .arg("*") // Auto-generar ID
            .arg("type")
            .arg(event_type)
            .arg("data")
            .arg(&event_json)
            .arg("occurred_at")
            .arg(event.occurred_at().to_rfc3339())
            .query_async(&mut conn)
            .await
            .map_err(|e| AppError::InternalError(format!("Failed to publish event to Redis Stream: {}", e)))?;

        info!("📤 Published event {} to stream: {}", event_type, stream_id);

        Ok(())
    }

    async fn subscribe(&self, event_type: &str, handler: Arc<dyn EventHandler>) -> Result<(), AppError> {
        let mut handlers_guard = self.handlers.write().await;
        
        handlers_guard
            .entry(event_type.to_string())
            .or_insert_with(Vec::new)
            .push(handler);

        info!("✅ Registered handler for event type: {}", event_type);
        
        Ok(())
    }
}

/// Worker para procesar eventos de Redis Streams en background
/// 
/// Este worker debe ejecutarse en un task separado para consumir
/// eventos continuamente del stream.
pub struct RedisStreamsEventWorker {
    event_bus: Arc<RedisStreamsEventBus>,
    running: Arc<RwLock<bool>>,
}

impl RedisStreamsEventWorker {
    /// Crear un nuevo worker
    pub fn new(event_bus: Arc<RedisStreamsEventBus>) -> Self {
        Self {
            event_bus,
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Iniciar el worker en un task separado
    pub fn start(&self) -> tokio::task::JoinHandle<()> {
        let event_bus = Arc::clone(&self.event_bus);
        let running = Arc::clone(&self.running);

        tokio::spawn(async move {
            {
                let mut running_guard = running.write().await;
                *running_guard = true;
            }

            info!("🚀 Redis Streams Event Worker started");

            loop {
                // Verificar si debemos continuar
                {
                    let running_guard = running.read().await;
                    if !*running_guard {
                        info!("🛑 Redis Streams Event Worker stopped");
                        break;
                    }
                }

                // Procesar eventos pendientes primero
                match event_bus.process_pending_events().await {
                    Ok(count) if count > 0 => {
                        info!("✅ Processed {} pending events", count);
                    }
                    Ok(_) => {}
                    Err(e) => {
                        error!("Error processing pending events: {:?}", e);
                    }
                }

                // Leer y procesar nuevos eventos
                match event_bus.read_and_process_events().await {
                    Ok(count) if count > 0 => {
                        info!("✅ Processed {} new events", count);
                    }
                    Ok(_) => {}
                    Err(e) => {
                        error!("Error reading events: {:?}", e);
                        // Esperar un poco antes de reintentar
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    }
                }
            }
        })
    }

    /// Detener el worker
    pub async fn stop(&self) {
        let mut running_guard = self.running.write().await;
        *running_guard = false;
        info!("🛑 Stopping Redis Streams Event Worker...");
    }
}

