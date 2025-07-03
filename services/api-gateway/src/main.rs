use api_gateway::simple::create_router;
use tracing_subscriber::fmt::init;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configurar logging
    init();
    
    println!("🚀 Starting VibeStream API Gateway (Simple Mode)...");

    // Crear router con todas las rutas
    let app = create_router();

    // Iniciar servidor
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🌐 API Gateway listening on {}", addr);
    
    // Usar la API correcta de Axum
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
} 