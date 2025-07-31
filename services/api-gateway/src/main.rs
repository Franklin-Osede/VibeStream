use api_gateway::simple::create_router;
use api_gateway::openapi::{generate_openapi_spec, generate_openapi_json};
use axum::{
    routing::get,
    Router,
    response::Json,
    http::StatusCode,
};
use tracing_subscriber::fmt::init;
use std::net::SocketAddr;
use utoipa_swagger_ui::SwaggerUi;
use utoipa_redoc::{Redoc, Servable};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configurar logging
    init();
    
    println!("🚀 Starting VibeStream API Gateway with OpenAPI Documentation...");

    // Crear router con todas las rutas (ahora async)
    let app = create_router().await?;

    // Generar especificación OpenAPI
    let openapi_spec = generate_openapi_spec();
    
    // Añadir rutas de documentación OpenAPI
    let app_with_docs = app
        .merge(
            SwaggerUi::new("/docs")
                .url("/api-docs/openapi.json", openapi_spec.clone())
        )
        .merge(
            Redoc::with_url("/redoc", openapi_spec.clone())
        )
        .route("/api-docs/openapi.yaml", get(serve_openapi_yaml))
        .route("/api-docs/health", get(docs_health_check));

    // Iniciar servidor
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("🌐 API Gateway listening on {}", addr);
    println!("");
    println!("📚 DOCUMENTACIÓN OPENAPI:");
    println!("   📖 Swagger UI:  http://localhost:3001/docs");
    println!("   📘 ReDoc:       http://localhost:3001/redoc");
    println!("   📄 OpenAPI JSON: http://localhost:3001/api-docs/openapi.json");
    println!("   📄 OpenAPI YAML: http://localhost:3001/api-docs/openapi.yaml");
    println!("");
    println!("🎵 ENDPOINTS DISPONIBLES:");
    println!("   GET  /api/music/songs/discover");
    println!("   GET  /api/music/songs/trending");
    println!("   POST /api/music/songs");
    println!("   GET  /api/music/songs/recommendations/:user_id");
    println!("");
    println!("🔗 PRÓXIMAMENTE:");
    println!("   💰 Fractional Ownership API");
    println!("   💎 Campaign NFT API");
    println!("   🎧 Listen Rewards API");
    println!("   👤 User Management API");
    
    // Usar la API correcta de Axum
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app_with_docs).await?;

    Ok(())
}

/// Endpoint para servir la especificación OpenAPI en formato YAML
async fn serve_openapi_yaml() -> Result<String, StatusCode> {
    let spec = generate_openapi_spec();
    match serde_yaml::to_string(&spec) {
        Ok(yaml) => Ok(yaml),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// Health check específico para documentación
async fn docs_health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "openapi-docs",
        "timestamp": chrono::Utc::now(),
        "endpoints": {
            "swagger": "/docs",
            "redoc": "/redoc",
            "openapi_json": "/api-docs/openapi.json",
            "openapi_yaml": "/api-docs/openapi.yaml"
        }
    }))
} 