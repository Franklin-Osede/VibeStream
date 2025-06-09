use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type DateTimeWithTimeZone = DateTime<Utc>;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration: i64,
    pub server_host: String,
    pub server_port: u16,
} 