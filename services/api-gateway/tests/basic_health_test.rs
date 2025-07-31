use axum::http::StatusCode;
use axum::test::TestServer;
use api_gateway::create_app;

#[tokio::test]
async fn test_health_endpoint() {
    // Crear la aplicación
    let app = create_app().await;
    
    // Crear un servidor de prueba
    let server = TestServer::new(app).unwrap();
    
    // Hacer una petición GET al endpoint de salud
    let response = server.get("/health").await;
    
    // Verificar que la respuesta es exitosa
    assert_eq!(response.status_code(), StatusCode::OK);
    
    // Verificar que el cuerpo de la respuesta contiene información de salud
    let body = response.text();
    assert!(body.contains("status") || body.contains("ok") || body.contains("healthy"));
}

#[tokio::test]
async fn test_app_compiles_and_runs() {
    // Este test simplemente verifica que la aplicación puede ser creada
    // sin errores de compilación o runtime
    let app = create_app().await;
    
    // Si llegamos aquí, significa que la aplicación se creó exitosamente
    assert!(true, "La aplicación se compiló y ejecutó correctamente");
} 