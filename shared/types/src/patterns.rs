use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::errors::AppError;

/// Generic Command Handler trait to reduce duplication
#[async_trait]
pub trait CommandHandler<TCommand, TResult>: Send + Sync
where
    TCommand: Command + Send + Sync,
    TResult: Send + Sync,
{
    async fn handle(&self, command: TCommand) -> Result<TResult, AppError>;
}

/// Generic Query Handler trait to reduce duplication
#[async_trait]
pub trait QueryHandler<TQuery, TResult>: Send + Sync
where
    TQuery: Query + Send + Sync,
    TResult: Send + Sync,
{
    async fn handle(&self, query: TQuery) -> Result<TResult, AppError>;
}

/// Generic Event Handler trait to reduce duplication
#[async_trait]
pub trait EventHandler<TEvent>: Send + Sync
where
    TEvent: DomainEvent + Send + Sync,
{
    async fn handle(&self, event: TEvent) -> Result<(), AppError>;
}

/// Base Repository trait with common CRUD operations
#[async_trait]
pub trait Repository<TAggregate, TId>: Send + Sync
where
    TAggregate: AggregateRoot<TId> + Send + Sync,
    TId: Send + Sync + Clone + Debug,
{
    type Error: std::error::Error + Send + Sync;

    async fn find_by_id(&self, id: &TId) -> Result<Option<TAggregate>, Self::Error>;
    async fn save(&self, aggregate: &TAggregate) -> Result<(), Self::Error>;
    async fn update(&self, aggregate: &TAggregate) -> Result<(), Self::Error>;
    async fn delete(&self, id: &TId) -> Result<(), Self::Error>;
    async fn exists(&self, id: &TId) -> Result<bool, Self::Error>;
    
    // Pagination support
    async fn find_all(&self, pagination: &Pagination) -> Result<Vec<TAggregate>, Self::Error>;
    async fn count(&self) -> Result<u64, Self::Error>;
}

/// Aggregate Root marker trait
pub trait AggregateRoot<TId>: Send + Sync + Clone + Debug {
    fn id(&self) -> &TId;
    fn version(&self) -> u64;
    fn get_uncommitted_events(&self) -> Vec<Box<dyn DomainEvent>>;
    fn mark_events_as_committed(&mut self);
}

/// Domain Event marker trait
pub trait DomainEvent: Send + Sync + Debug {
    fn event_type(&self) -> &'static str;
    fn event_version(&self) -> u32;
    fn occurred_at(&self) -> DateTime<Utc>;
    fn aggregate_id(&self) -> Uuid;
}

/// Command marker trait
pub trait Command: Send + Sync + Debug {
    fn validate(&self) -> Result<(), AppError>;
}

/// Query marker trait
pub trait Query: Send + Sync + Debug {
    fn validate(&self) -> Result<(), AppError>;
}

/// Pagination parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u32,
    pub size: u32,
    pub sort_by: Option<String>,
    pub sort_direction: SortDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            size: 20,
            sort_by: None,
            sort_direction: SortDirection::Desc,
        }
    }
}

impl Pagination {
    pub fn new(page: u32, size: u32) -> Self {
        Self {
            page,
            size,
            sort_by: None,
            sort_direction: SortDirection::Desc,
        }
    }
    
    pub fn with_sort(mut self, sort_by: String, direction: SortDirection) -> Self {
        self.sort_by = Some(sort_by);
        self.sort_direction = direction;
        self
    }
    
    pub fn offset(&self) -> u32 {
        (self.page - 1) * self.size
    }
    
    pub fn limit(&self) -> u32 {
        self.size
    }
}

/// Paginated result wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total_count: u64,
    pub page: u32,
    pub size: u32,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_previous: bool,
}

impl<T> PaginatedResult<T> {
    pub fn new(items: Vec<T>, total_count: u64, pagination: &Pagination) -> Self {
        let total_pages = (total_count as f64 / pagination.size as f64).ceil() as u32;
        let has_next = pagination.page < total_pages;
        let has_previous = pagination.page > 1;
        
        Self {
            items,
            total_count,
            page: pagination.page,
            size: pagination.size,
            total_pages,
            has_next,
            has_previous,
        }
    }
}

/// Base Application Service trait
#[async_trait]
pub trait ApplicationService: Send + Sync {
    type Error: std::error::Error + Send + Sync;
    
    async fn health_check(&self) -> Result<(), Self::Error>;
}

/// Integration Event for cross-context communication
#[async_trait]
pub trait IntegrationEvent: Send + Sync + Debug {
    fn event_type(&self) -> &'static str;
    fn event_version(&self) -> u32;
    fn occurred_at(&self) -> DateTime<Utc>;
    fn source_context(&self) -> &'static str;
    fn correlation_id(&self) -> Option<Uuid>;
}

/// Integration Event Handler
#[async_trait]
pub trait IntegrationEventHandler<TEvent>: Send + Sync
where
    TEvent: IntegrationEvent + Send + Sync,
{
    async fn handle(&self, event: TEvent) -> Result<(), AppError>;
}

/// Event Bus for publishing events
#[async_trait]
pub trait EventBus: Send + Sync {
    async fn publish<T: DomainEvent>(&self, event: T) -> Result<(), AppError>;
    async fn publish_integration<T: IntegrationEvent>(&self, event: T) -> Result<(), AppError>;
}

/// Command Bus for processing commands
#[async_trait]
pub trait CommandBus: Send + Sync {
    async fn dispatch<TCommand, TResult>(&self, command: TCommand) -> Result<TResult, AppError>
    where
        TCommand: Command + Send + Sync,
        TResult: Send + Sync;
}

/// Query Bus for processing queries
#[async_trait]
pub trait QueryBus: Send + Sync {
    async fn execute<TQuery, TResult>(&self, query: TQuery) -> Result<TResult, AppError>
    where
        TQuery: Query + Send + Sync,
        TResult: Send + Sync;
}

/// Audit Trail for tracking changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditTrail {
    pub entity_type: String,
    pub entity_id: String,
    pub action: AuditAction,
    pub user_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,
    pub old_values: Option<serde_json::Value>,
    pub new_values: Option<serde_json::Value>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditAction {
    Create,
    Update,
    Delete,
    View,
    Export,
    Import,
    Approve,
    Reject,
}

/// Soft Delete trait for entities that support soft deletion
pub trait SoftDelete {
    fn is_deleted(&self) -> bool;
    fn deleted_at(&self) -> Option<DateTime<Utc>>;
    fn soft_delete(&mut self);
    fn restore(&mut self);
}

/// Timestamped trait for entities with creation/update timestamps
pub trait Timestamped {
    fn created_at(&self) -> DateTime<Utc>;
    fn updated_at(&self) -> DateTime<Utc>;
    fn touch(&mut self);
}

/// Versioned trait for entities with version control
pub trait Versioned {
    fn version(&self) -> u64;
    fn increment_version(&mut self);
} 