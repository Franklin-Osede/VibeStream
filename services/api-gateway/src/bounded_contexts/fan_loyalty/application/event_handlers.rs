use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::bounded_contexts::fan_loyalty::{
    domain::{
        repositories::{
            FanVerificationRepository, WristbandRepository, QrCodeRepository,
            ZkProofRepository, NftRepository
        },
        services::{
            BiometricVerificationService, WristbandService, QrCodeService,
            NftService, ZkProofService, EventPublisher, QrCodeScannedEvent
        },
        entities::{FanId, WristbandId, WristbandType, NftWristband, FanVerificationResult},
    },
    application::dependency_injection::{
        FanVerifiedEvent, WristbandCreatedEvent, WristbandActivatedEvent,
    },
};

/// Event handler trait for domain events
#[async_trait]
pub trait EventHandler<T>: Send + Sync {
    /// Handle domain event
    async fn handle(&self, event: &T) -> Result<(), String>;
}

/// Event handler for fan verified events
pub struct FanVerifiedEventHandler {
    fan_verification_repository: Arc<dyn FanVerificationRepository>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl FanVerifiedEventHandler {
    pub fn new(
        fan_verification_repository: Arc<dyn FanVerificationRepository>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            fan_verification_repository,
            event_publisher,
        }
    }
}

#[async_trait]
impl EventHandler<FanVerifiedEvent> for FanVerifiedEventHandler {
    async fn handle(&self, event: &FanVerifiedEvent) -> Result<(), String> {
        // Log fan verification event
        tracing::info!(
            "Fan verified: fan_id={}, verification_id={}, confidence_score={}, wristband_eligible={}",
            event.fan_id.0,
            event.verification_id,
            event.confidence_score,
            event.wristband_eligible
        );

        // Update fan verification status
        let verification_result = FanVerificationResult {
            is_verified: true,
            confidence_score: event.confidence_score,
            verification_id: event.verification_id.clone(),
            wristband_eligible: event.wristband_eligible,
            benefits_unlocked: event.benefits_unlocked.clone(),
        };

        self.fan_verification_repository.save_verification_result(
            &event.fan_id,
            &verification_result,
        ).await?;

        // Publish downstream events if needed
        if event.wristband_eligible {
            // Could trigger wristband creation workflow
            tracing::info!("Fan is eligible for wristband: {}", event.fan_id.0);
        }

        Ok(())
    }
}

/// Event handler for wristband created events
pub struct WristbandCreatedEventHandler {
    wristband_repository: Arc<dyn WristbandRepository>,
    nft_service: Arc<dyn NftService>,
    qr_code_service: Arc<dyn QrCodeService>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl WristbandCreatedEventHandler {
    pub fn new(
        wristband_repository: Arc<dyn WristbandRepository>,
        nft_service: Arc<dyn NftService>,
        qr_code_service: Arc<dyn QrCodeService>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            wristband_repository,
            nft_service,
            qr_code_service,
            event_publisher,
        }
    }
}

#[async_trait]
impl EventHandler<WristbandCreatedEvent> for WristbandCreatedEventHandler {
    async fn handle(&self, event: &WristbandCreatedEvent) -> Result<(), String> {
        // Log wristband creation event
        tracing::info!(
            "Wristband created: wristband_id={}, fan_id={}, concert_id={}, artist_id={}, type={:?}",
            event.wristband_id.0,
            event.fan_id.0,
            event.concert_id,
            event.artist_id,
            event.wristband_type
        );

        // Create wristband entity
        let wristband = NftWristband::new(
            event.fan_id.clone(),
            event.concert_id.to_string(),
            event.artist_id.to_string(),
            event.wristband_type.clone(),
        );

        // Save wristband
        self.wristband_repository.save_wristband(&wristband).await?;

        // Generate QR code for wristband
        let qr_code = self.qr_code_service.generate_qr_code(&event.wristband_id).await?;
        
        // Log QR code generation
        tracing::info!(
            "QR code generated for wristband: wristband_id={}, qr_code={}",
            event.wristband_id.0,
            qr_code.code
        );

        // Create NFT for wristband (if fan has wallet address)
        // This would typically be triggered by a separate command
        tracing::info!("Wristband created successfully: {}", event.wristband_id.0);

        Ok(())
    }
}

/// Event handler for wristband activated events
pub struct WristbandActivatedEventHandler {
    wristband_repository: Arc<dyn WristbandRepository>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl WristbandActivatedEventHandler {
    pub fn new(
        wristband_repository: Arc<dyn WristbandRepository>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            wristband_repository,
            event_publisher,
        }
    }
}

#[async_trait]
impl EventHandler<WristbandActivatedEvent> for WristbandActivatedEventHandler {
    async fn handle(&self, event: &WristbandActivatedEvent) -> Result<(), String> {
        // Log wristband activation event
        tracing::info!(
            "Wristband activated: wristband_id={}, fan_id={}, reason={}, activated_at={}",
            event.wristband_id.0,
            event.fan_id.0,
            event.activation_reason,
            event.activated_at
        );

        // Update wristband status
        self.wristband_repository.update_wristband_status(
            &event.wristband_id,
            true, // is_active
            Some(event.activated_at),
        ).await?;

        // Log activation success
        tracing::info!("Wristband activated successfully: {}", event.wristband_id.0);

        Ok(())
    }
}

/// Event handler for QR code scanned events
pub struct QrCodeScannedEventHandler {
    qr_code_repository: Arc<dyn QrCodeRepository>,
    wristband_repository: Arc<dyn WristbandRepository>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl QrCodeScannedEventHandler {
    pub fn new(
        qr_code_repository: Arc<dyn QrCodeRepository>,
        wristband_repository: Arc<dyn WristbandRepository>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            qr_code_repository,
            wristband_repository,
            event_publisher,
        }
    }
}

#[async_trait]
impl EventHandler<QrCodeScannedEvent> for QrCodeScannedEventHandler {
    async fn handle(&self, event: &QrCodeScannedEvent) -> Result<(), String> {
        // Log QR code scan event
        tracing::info!(
            "QR code scanned: qr_code={}, scanner_id={}, access_granted={}, scanned_at={}",
            event.qr_code,
            event.scanner_id,
            event.access_granted,
            event.scanned_at
        );

        // Log scan in repository
        self.qr_code_repository.log_qr_scan(
            &event.qr_code,
            &event.scanner_id,
            event.location.as_ref().map(|loc| (loc.latitude, loc.longitude, loc.accuracy)),
        ).await?;

        // Update wristband access if applicable
        if let Some(wristband_id) = &event.wristband_id {
            // Could update wristband access logs
            tracing::info!("Wristband access logged: {}", wristband_id.0);
        }

        // Log access result
        if event.access_granted {
            tracing::info!("Access granted for QR code: {}", event.qr_code);
        } else {
            tracing::warn!("Access denied for QR code: {}", event.qr_code);
        }

        Ok(())
    }
}

/// Event bus for managing domain events
pub struct EventBus {
    fan_verified_handlers: Vec<Arc<dyn EventHandler<FanVerifiedEvent>>>,
    wristband_created_handlers: Vec<Arc<dyn EventHandler<WristbandCreatedEvent>>>,
    wristband_activated_handlers: Vec<Arc<dyn EventHandler<WristbandActivatedEvent>>>,
    qr_code_scanned_handlers: Vec<Arc<dyn EventHandler<QrCodeScannedEvent>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            fan_verified_handlers: Vec::new(),
            wristband_created_handlers: Vec::new(),
            wristband_activated_handlers: Vec::new(),
            qr_code_scanned_handlers: Vec::new(),
        }
    }

    /// Register fan verified event handler
    pub fn register_fan_verified_handler(&mut self, handler: Arc<dyn EventHandler<FanVerifiedEvent>>) {
        self.fan_verified_handlers.push(handler);
    }

    /// Register wristband created event handler
    pub fn register_wristband_created_handler(&mut self, handler: Arc<dyn EventHandler<WristbandCreatedEvent>>) {
        self.wristband_created_handlers.push(handler);
    }

    /// Register wristband activated event handler
    pub fn register_wristband_activated_handler(&mut self, handler: Arc<dyn EventHandler<WristbandActivatedEvent>>) {
        self.wristband_activated_handlers.push(handler);
    }

    /// Register QR code scanned event handler
    pub fn register_qr_code_scanned_handler(&mut self, handler: Arc<dyn EventHandler<QrCodeScannedEvent>>) {
        self.qr_code_scanned_handlers.push(handler);
    }

    /// Publish fan verified event
    pub async fn publish_fan_verified(&self, event: &FanVerifiedEvent) -> Result<(), String> {
        for handler in &self.fan_verified_handlers {
            if let Err(e) = handler.handle(event).await {
                tracing::error!("Error handling fan verified event: {}", e);
                return Err(e);
            }
        }
        Ok(())
    }

    /// Publish wristband created event
    pub async fn publish_wristband_created(&self, event: &WristbandCreatedEvent) -> Result<(), String> {
        for handler in &self.wristband_created_handlers {
            if let Err(e) = handler.handle(event).await {
                tracing::error!("Error handling wristband created event: {}", e);
                return Err(e);
            }
        }
        Ok(())
    }

    /// Publish wristband activated event
    pub async fn publish_wristband_activated(&self, event: &WristbandActivatedEvent) -> Result<(), String> {
        for handler in &self.wristband_activated_handlers {
            if let Err(e) = handler.handle(event).await {
                tracing::error!("Error handling wristband activated event: {}", e);
                return Err(e);
            }
        }
        Ok(())
    }

    /// Publish QR code scanned event
    pub async fn publish_qr_code_scanned(&self, event: &QrCodeScannedEvent) -> Result<(), String> {
        for handler in &self.qr_code_scanned_handlers {
            if let Err(e) = handler.handle(event).await {
                tracing::error!("Error handling QR code scanned event: {}", e);
                return Err(e);
            }
        }
        Ok(())
    }
}

/// Event store for persisting domain events
#[derive(Debug)]
pub struct EventStore {
    events: Vec<StoredEvent>,
}

impl EventStore {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }

    /// Store domain event
    pub async fn store_event(&mut self, event: &StoredEvent) -> Result<(), String> {
        self.events.push(event.clone());
        Ok(())
    }

    /// Get events by aggregate ID
    pub async fn get_events_by_aggregate_id(&self, aggregate_id: &Uuid) -> Result<Vec<StoredEvent>, String> {
        let filtered_events: Vec<StoredEvent> = self.events
            .iter()
            .filter(|event| event.aggregate_id == *aggregate_id)
            .cloned()
            .collect();
        Ok(filtered_events)
    }

    /// Get events by event type
    pub async fn get_events_by_type(&self, event_type: &str) -> Result<Vec<StoredEvent>, String> {
        let filtered_events: Vec<StoredEvent> = self.events
            .iter()
            .filter(|event| event.event_type == event_type)
            .cloned()
            .collect();
        Ok(filtered_events)
    }

    /// Get all events
    pub async fn get_all_events(&self) -> Result<Vec<StoredEvent>, String> {
        Ok(self.events.clone())
    }
}

/// Stored event for event store
#[derive(Debug, Clone)]
pub struct StoredEvent {
    pub id: Uuid,
    pub aggregate_id: Uuid,
    pub event_type: String,
    pub event_data: serde_json::Value,
    pub version: u32,
    pub occurred_at: DateTime<Utc>,
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
}

impl StoredEvent {
    pub fn new(
        aggregate_id: Uuid,
        event_type: String,
        event_data: serde_json::Value,
        version: u32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            aggregate_id,
            event_type,
            event_data,
            version,
            occurred_at: Utc::now(),
            correlation_id: None,
            causation_id: None,
        }
    }
}

/// Event sourcing service for managing domain events
pub struct EventSourcingService {
    event_store: Arc<EventStore>,
    event_bus: Arc<EventBus>,
}

impl EventSourcingService {
    pub fn new(event_store: Arc<EventStore>, event_bus: Arc<EventBus>) -> Self {
        Self {
            event_store,
            event_bus,
        }
    }

    /// Publish and store domain event
    pub async fn publish_and_store_event(&self, event: &StoredEvent) -> Result<(), String> {
        // Store event
        self.event_store.store_event(event).await?;

        // Publish event to handlers
        match event.event_type.as_str() {
            "fan_verified" => {
                let fan_verified_event: FanVerifiedEvent = serde_json::from_value(event.event_data.clone())?;
                self.event_bus.publish_fan_verified(&fan_verified_event).await?;
            }
            "wristband_created" => {
                let wristband_created_event: WristbandCreatedEvent = serde_json::from_value(event.event_data.clone())?;
                self.event_bus.publish_wristband_created(&wristband_created_event).await?;
            }
            "wristband_activated" => {
                let wristband_activated_event: WristbandActivatedEvent = serde_json::from_value(event.event_data.clone())?;
                self.event_bus.publish_wristband_activated(&wristband_activated_event).await?;
            }
            "qr_code_scanned" => {
                let qr_code_scanned_event: QrCodeScannedEvent = serde_json::from_value(event.event_data.clone())?;
                self.event_bus.publish_qr_code_scanned(&qr_code_scanned_event).await?;
            }
            _ => {
                tracing::warn!("Unknown event type: {}", event.event_type);
            }
        }

        Ok(())
    }

    /// Replay events for aggregate
    pub async fn replay_events(&self, aggregate_id: &Uuid) -> Result<Vec<StoredEvent>, String> {
        self.event_store.get_events_by_aggregate_id(aggregate_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_bus_creation() {
        // Given & When
        let event_bus = EventBus::new();

        // Then
        assert!(event_bus.fan_verified_handlers.is_empty());
        assert!(event_bus.wristband_created_handlers.is_empty());
        assert!(event_bus.wristband_activated_handlers.is_empty());
        assert!(event_bus.qr_code_scanned_handlers.is_empty());
    }

    #[test]
    fn test_event_store_creation() {
        // Given & When
        let event_store = EventStore::new();

        // Then
        assert!(event_store.events.is_empty());
    }

    #[test]
    fn test_stored_event_creation() {
        // Given
        let aggregate_id = Uuid::new_v4();
        let event_type = "fan_verified".to_string();
        let event_data = serde_json::json!({
            "fan_id": Uuid::new_v4(),
            "verification_id": "verification_123",
            "confidence_score": 0.95,
            "wristband_eligible": true,
            "benefits_unlocked": ["Verified Fan Status"]
        });
        let version = 1;

        // When
        let stored_event = StoredEvent::new(
            aggregate_id,
            event_type.clone(),
            event_data.clone(),
            version,
        );

        // Then
        assert_eq!(stored_event.aggregate_id, aggregate_id);
        assert_eq!(stored_event.event_type, event_type);
        assert_eq!(stored_event.event_data, event_data);
        assert_eq!(stored_event.version, version);
        assert!(stored_event.correlation_id.is_none());
        assert!(stored_event.causation_id.is_none());
    }

    #[test]
    fn test_event_sourcing_service_creation() {
        // Given
        let event_store = Arc::new(EventStore::new());
        let event_bus = Arc::new(EventBus::new());

        // When
        let service = EventSourcingService::new(event_store, event_bus);

        // Then
        assert!(service.event_store.events.is_empty());
        assert!(service.event_bus.fan_verified_handlers.is_empty());
    }
}

