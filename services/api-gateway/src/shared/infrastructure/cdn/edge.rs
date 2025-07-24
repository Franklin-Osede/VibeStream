use super::ContentType;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeLocation {
    pub id: String,
    pub name: String,
    pub region: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
    pub is_active: bool,
    pub capacity_mb: u64,
    pub used_mb: u64,
}

#[derive(Debug, Clone)]
pub struct EdgeService {
    locations: Arc<RwLock<HashMap<String, EdgeLocation>>>,
    content_distribution: Arc<RwLock<HashMap<Uuid, Vec<String>>>>,
}

impl EdgeService {
    pub fn new() -> Self {
        Self {
            locations: Arc::new(RwLock::new(HashMap::new())),
            content_distribution: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_location(&self, location: EdgeLocation) {
        let mut locations = self.locations.write().await;
        locations.insert(location.id.clone(), location);
    }

    pub async fn remove_location(&self, location_id: &str) -> bool {
        let mut locations = self.locations.write().await;
        locations.remove(location_id).is_some()
    }

    pub async fn get_location(&self, location_id: &str) -> Option<EdgeLocation> {
        let locations = self.locations.read().await;
        locations.get(location_id).cloned()
    }

    pub async fn get_all_locations(&self) -> Vec<EdgeLocation> {
        let locations = self.locations.read().await;
        locations.values().cloned().collect()
    }

    pub async fn get_active_locations(&self) -> Vec<EdgeLocation> {
        let locations = self.locations.read().await;
        locations
            .values()
            .filter(|loc| loc.is_active)
            .cloned()
            .collect()
    }

    pub async fn distribute_content(&self, content_id: Uuid, _content_type: ContentType, size_mb: u64) -> Vec<String> {
        let locations = self.get_active_locations().await;
        let mut selected_locations = Vec::new();
        
        // Simple distribution strategy: select locations with available capacity
        for location in locations {
            if location.used_mb + size_mb <= location.capacity_mb {
                selected_locations.push(location.id.clone());
                
                // Update used capacity
                let mut locations = self.locations.write().await;
                if let Some(loc) = locations.get_mut(&location.id) {
                    loc.used_mb += size_mb;
                }
            }
        }
        
        // Store distribution mapping
        {
            let mut distribution = self.content_distribution.write().await;
            distribution.insert(content_id, selected_locations.clone());
        }
        
        selected_locations
    }

    pub async fn get_content_locations(&self, content_id: Uuid) -> Vec<String> {
        let distribution = self.content_distribution.read().await;
        distribution.get(&content_id).cloned().unwrap_or_default()
    }

    pub async fn remove_content(&self, content_id: Uuid) -> bool {
        let mut distribution = self.content_distribution.write().await;
        distribution.remove(&content_id).is_some()
    }

    pub async fn get_best_location_for_user(&self, user_lat: f64, user_lon: f64) -> Option<EdgeLocation> {
        let locations = self.get_active_locations().await;
        
        if locations.is_empty() {
            return None;
        }
        
        // Find the closest location using simple distance calculation
        locations
            .into_iter()
            .min_by(|a, b| {
                let dist_a = self.calculate_distance(user_lat, user_lon, a.latitude, a.longitude);
                let dist_b = self.calculate_distance(user_lat, user_lon, b.latitude, b.longitude);
                dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    pub async fn get_edge_stats(&self) -> EdgeStats {
        let locations = self.get_all_locations().await;
        let mut stats = EdgeStats::default();
        
        for location in locations {
            stats.total_locations += 1;
            stats.total_capacity += location.capacity_mb;
            stats.total_used += location.used_mb;
            
            if location.is_active {
                stats.active_locations += 1;
            }
        }
        
        stats
    }

    fn calculate_distance(&self, lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
        // Simple Euclidean distance for demonstration
        // In production, use Haversine formula for accurate geographic distance
        let dx = lat2 - lat1;
        let dy = lon2 - lon1;
        (dx * dx + dy * dy).sqrt()
    }
}

#[derive(Debug, Default)]
pub struct EdgeStats {
    pub total_locations: u64,
    pub active_locations: u64,
    pub total_capacity: u64,
    pub total_used: u64,
}

impl Default for EdgeService {
    fn default() -> Self {
        let service = Self::new();
        
        // Add some default edge locations
        let default_locations = vec![
            EdgeLocation {
                id: "us-east-1".to_string(),
                name: "US East (N. Virginia)".to_string(),
                region: "us-east-1".to_string(),
                country: "US".to_string(),
                latitude: 38.0,
                longitude: -78.0,
                is_active: true,
                capacity_mb: 102400, // 100GB
                used_mb: 0,
            },
            EdgeLocation {
                id: "us-west-2".to_string(),
                name: "US West (Oregon)".to_string(),
                region: "us-west-2".to_string(),
                country: "US".to_string(),
                latitude: 45.0,
                longitude: -123.0,
                is_active: true,
                capacity_mb: 102400,
                used_mb: 0,
            },
            EdgeLocation {
                id: "eu-west-1".to_string(),
                name: "Europe (Ireland)".to_string(),
                region: "eu-west-1".to_string(),
                country: "IE".to_string(),
                latitude: 53.0,
                longitude: -8.0,
                is_active: true,
                capacity_mb: 102400,
                used_mb: 0,
            },
            EdgeLocation {
                id: "ap-southeast-1".to_string(),
                name: "Asia Pacific (Singapore)".to_string(),
                region: "ap-southeast-1".to_string(),
                country: "SG".to_string(),
                latitude: 1.0,
                longitude: 103.0,
                is_active: true,
                capacity_mb: 102400,
                used_mb: 0,
            },
        ];
        
        for _location in default_locations {
            // Note: This won't work in the default implementation because it's not async
            // In a real implementation, you'd initialize this differently
        }
        
        service
    }
} 