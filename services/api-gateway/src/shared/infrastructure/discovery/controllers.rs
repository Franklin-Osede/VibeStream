use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use super::service::DiscoveryService;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddRSSFeedRequest {
    pub url: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterWebhookRequest {
    pub url: String,
    pub name: String,
    pub description: Option<String>,
    pub events: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<serde_json::Value>,
    pub total_count: usize,
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveryStats {
    pub rss_feeds: usize,
    pub webhook_endpoints: usize,
    pub recent_discoveries: usize,
}

pub async fn add_rss_feed(
    State(_service): State<Arc<DiscoveryService>>,
    Json(_request): Json<AddRSSFeedRequest>,
) -> Result<Json<Uuid>, StatusCode> {
    // TODO: Implementar con mutabilidad apropiada
    let feed_id = Uuid::new_v4();
    Ok(Json(feed_id))
}

pub async fn remove_rss_feed(
    State(_service): State<Arc<DiscoveryService>>,
    Path(_feed_id): Path<Uuid>,
) -> StatusCode {
    // TODO: Implementar con mutabilidad apropiada
    StatusCode::OK
}

pub async fn register_webhook(
    State(_service): State<Arc<DiscoveryService>>,
    Json(_request): Json<RegisterWebhookRequest>,
) -> Result<Json<Uuid>, StatusCode> {
    // TODO: Implementar con mutabilidad apropiada
    let endpoint_id = Uuid::new_v4();
    Ok(Json(endpoint_id))
}

pub async fn unregister_webhook(
    State(_service): State<Arc<DiscoveryService>>,
    Path(_endpoint_id): Path<Uuid>,
) -> StatusCode {
    // TODO: Implementar con mutabilidad apropiada
    StatusCode::OK
}

pub async fn search_content(
    State(service): State<Arc<DiscoveryService>>,
    Query(request): Query<SearchRequest>,
) -> Result<Json<SearchResponse>, StatusCode> {
    let results = service.search_content(&request.query).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let total_count = results.len();
    
    Ok(Json(SearchResponse {
        results,
        total_count,
        query: request.query,
    }))
}

pub async fn get_discovery_stats(
    State(service): State<Arc<DiscoveryService>>,
) -> Json<DiscoveryStats> {
    let stats = service.get_stats().await;
    
    // Extraer valores del JSON
    let rss_feeds = stats["rss"]["total_feeds"].as_u64().unwrap_or(0) as usize;
    let webhook_endpoints = stats["webhooks"]["total_endpoints"].as_u64().unwrap_or(0) as usize;
    
    Json(DiscoveryStats {
        rss_feeds,
        webhook_endpoints,
        recent_discoveries: 0, // TODO: implementar
    })
}

pub async fn trigger_event(
    State(_service): State<Arc<DiscoveryService>>,
    Path(_event_type): Path<String>,
    Json(_data): Json<serde_json::Value>,
) -> Json<u32> {
    // TODO: Implementar con mutabilidad apropiada
    Json(0)
} 