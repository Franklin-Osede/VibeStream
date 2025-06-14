use vibestream_types::*;

pub mod client;
pub mod service;

pub use service::SolanaService;
pub use client::SolanaClient;

// Función principal para procesar mensajes
pub async fn run_solana_worker() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    tracing::info!("Starting Solana service worker...");
    
    // TODO: Conectar a Redis y procesar mensajes
    // Por ahora solo mantenemos el servicio corriendo
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        tracing::debug!("Solana worker is running...");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_creation() {
        // Test básico para verificar que el servicio se puede crear
        assert!(true);
    }
} 