use api_gateway::simple::create_router;
use tracing_subscriber::fmt::init;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configurar logging
    init();
    
    println!("🚀 Starting VibeStream API Gateway with Music Context...");

    // Crear router con todas las rutas (ahora async)
    let app = create_router().await?;

    // Iniciar servidor
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🌐 API Gateway listening on {}", addr);
    println!("🎵 Music endpoints available at:");
    println!("   GET  /api/music/songs/discover");
    println!("   GET  /api/music/songs/trending");
    println!("   POST /api/music/songs");
    println!("   GET  /api/music/songs/recommendations/:user_id");
    
    // Usar la API correcta de Axum
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
} 