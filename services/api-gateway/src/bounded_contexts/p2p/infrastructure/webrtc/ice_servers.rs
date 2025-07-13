use serde::{Deserialize, Serialize};

/// ICE Server Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ICEServerConfig {
    pub url: String,
    pub username: Option<String>,
    pub credential: Option<String>,
    pub server_type: ICEServerType,
    pub priority: u32,
}

impl ICEServerConfig {
    /// Create a new ICE server configuration
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            username: None,
            credential: None,
            server_type: ICEServerType::STUN,
            priority: 1,
        }
    }

    /// Create a STUN server configuration
    pub fn stun(url: &str) -> Self {
        Self {
            url: url.to_string(),
            username: None,
            credential: None,
            server_type: ICEServerType::STUN,
            priority: 1,
        }
    }

    /// Create a TURN server configuration
    pub fn turn(url: &str, username: &str, credential: &str) -> Self {
        Self {
            url: url.to_string(),
            username: Some(username.to_string()),
            credential: Some(credential.to_string()),
            server_type: ICEServerType::TURN,
            priority: 2,
        }
    }

    /// Get server URL
    pub fn get_url(&self) -> &str {
        &self.url
    }

    /// Check if server requires authentication
    pub fn requires_auth(&self) -> bool {
        self.username.is_some() && self.credential.is_some()
    }

    /// Get authentication credentials
    pub fn get_credentials(&self) -> Option<(&str, &str)> {
        if let (Some(username), Some(credential)) = (&self.username, &self.credential) {
            Some((username, credential))
        } else {
            None
        }
    }

    /// Get server type
    pub fn get_server_type(&self) -> &ICEServerType {
        &self.server_type
    }

    /// Get priority
    pub fn get_priority(&self) -> u32 {
        self.priority
    }
}

/// ICE Server Type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ICEServerType {
    STUN,
    TURN,
}

/// ICE Server Manager
pub struct ICEServerManager {
    servers: Vec<ICEServerConfig>,
    default_servers: Vec<ICEServerConfig>,
}

impl ICEServerManager {
    /// Create a new ICE server manager
    pub fn new() -> Self {
        let default_servers = vec![
            ICEServerConfig::stun("stun:stun.l.google.com:19302"),
            ICEServerConfig::stun("stun:stun1.l.google.com:19302"),
            ICEServerConfig::stun("stun:stun2.l.google.com:19302"),
            ICEServerConfig::stun("stun:stun3.l.google.com:19302"),
            ICEServerConfig::stun("stun:stun4.l.google.com:19302"),
        ];

        Self {
            servers: default_servers.clone(),
            default_servers,
        }
    }

    /// Add a custom ICE server
    pub fn add_server(&mut self, server: ICEServerConfig) {
        println!("📡 Adding ICE server: {} ({:?})", server.url, server.server_type);
        self.servers.push(server);
    }

    /// Add multiple ICE servers
    pub fn add_servers(&mut self, servers: Vec<ICEServerConfig>) {
        for server in servers {
            self.add_server(server);
        }
    }

    /// Get all ICE servers
    pub fn get_servers(&self) -> &[ICEServerConfig] {
        &self.servers
    }

    /// Get STUN servers only
    pub fn get_stun_servers(&self) -> Vec<&ICEServerConfig> {
        self.servers.iter()
            .filter(|server| server.server_type == ICEServerType::STUN)
            .collect()
    }

    /// Get TURN servers only
    pub fn get_turn_servers(&self) -> Vec<&ICEServerConfig> {
        self.servers.iter()
            .filter(|server| server.server_type == ICEServerType::TURN)
            .collect()
    }

    /// Get servers by priority
    pub fn get_servers_by_priority(&self) -> Vec<&ICEServerConfig> {
        let mut servers: Vec<&ICEServerConfig> = self.servers.iter().collect();
        servers.sort_by(|a, b| a.priority.cmp(&b.priority));
        servers
    }

    /// Reset to default servers
    pub fn reset_to_defaults(&mut self) {
        println!("🔄 Resetting ICE servers to defaults");
        self.servers = self.default_servers.clone();
    }

    /// Get server count
    pub fn get_server_count(&self) -> usize {
        self.servers.len()
    }

    /// Check if any TURN servers are configured
    pub fn has_turn_servers(&self) -> bool {
        self.servers.iter().any(|server| server.server_type == ICEServerType::TURN)
    }

    /// Get recommended servers for a region
    pub fn get_servers_for_region(&self, region: &str) -> Vec<&ICEServerConfig> {
        // Mock implementation - in real scenario, this would return region-specific servers
        match region.to_lowercase().as_str() {
            "us" | "north-america" => {
                vec![
                    &self.servers[0], // stun.l.google.com
                    &self.servers[1], // stun1.l.google.com
                ]
            }
            "eu" | "europe" => {
                vec![
                    &self.servers[2], // stun2.l.google.com
                    &self.servers[3], // stun3.l.google.com
                ]
            }
            "asia" | "asia-pacific" => {
                vec![
                    &self.servers[4], // stun4.l.google.com
                ]
            }
            _ => self.get_servers_by_priority(),
        }
    }

    /// Validate server configuration
    pub fn validate_server(&self, server: &ICEServerConfig) -> Result<(), ICEServerError> {
        // Validate URL format
        if !server.url.starts_with("stun:") && !server.url.starts_with("turn:") {
            return Err(ICEServerError::InvalidURL);
        }

        // Validate TURN server has credentials
        if server.server_type == ICEServerType::TURN && !server.requires_auth() {
            return Err(ICEServerError::MissingCredentials);
        }

        // Validate URL is reachable (mock implementation)
        if !self.is_url_reachable(&server.url) {
            return Err(ICEServerError::Unreachable);
        }

        Ok(())
    }

    /// Check if URL is reachable (mock implementation)
    fn is_url_reachable(&self, url: &str) -> bool {
        // Mock implementation - in real scenario, this would ping the server
        !url.contains("invalid") && !url.contains("unreachable")
    }

    /// Get server statistics
    pub fn get_stats(&self) -> ICEServerStats {
        let stun_count = self.get_stun_servers().len();
        let turn_count = self.get_turn_servers().len();
        let total_count = self.get_server_count();

        ICEServerStats {
            total_servers: total_count,
            stun_servers: stun_count,
            turn_servers: turn_count,
            has_turn_servers: self.has_turn_servers(),
        }
    }
}

/// ICE Server Statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ICEServerStats {
    pub total_servers: usize,
    pub stun_servers: usize,
    pub turn_servers: usize,
    pub has_turn_servers: bool,
}

/// ICE Server Error
#[derive(Debug, thiserror::Error)]
pub enum ICEServerError {
    #[error("Invalid URL format")]
    InvalidURL,
    #[error("Missing credentials for TURN server")]
    MissingCredentials,
    #[error("Server is unreachable")]
    Unreachable,
    #[error("Server configuration error: {0}")]
    ConfigurationError(String),
}

impl Default for ICEServerManager {
    fn default() -> Self {
        Self::new()
    }
} 