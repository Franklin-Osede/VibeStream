// =============================================================================
// OPENAPI INTEGRATION TESTS - TDD APPROACH
// =============================================================================
// 
// RED PHASE: Tests que verifican que OpenAPI genera paths correctamente
// GREEN PHASE: Asegurar que los paths estén registrados

use api_gateway::openapi::{ApiDoc, generate_openapi_spec};
use utoipa::openapi::OpenApi;

// =============================================================================
// TEST 1: OpenAPI debe tener paths registrados (no vacío)
// =============================================================================

#[test]
fn test_openapi_has_paths() {
    // Arrange & Act
    let spec = ApiDoc::openapi();
    
    // Assert: Debe tener paths (no vacío)
    // utoipa usa un HashMap interno para paths
    let paths_count = spec.paths.paths().len();
    assert!(
        paths_count > 0,
        "OpenAPI spec should have paths registered, but got {} paths",
        paths_count
    );
    
    // Verificar que tiene al menos los paths básicos
    let path_keys: Vec<String> = spec.paths.paths().keys().cloned().collect();
    assert!(
        path_keys.iter().any(|p| p.contains("/users/register") || p.contains("register")),
        "Should have /users/register path. Found paths: {:?}",
        path_keys
    );
    
    assert!(
        path_keys.iter().any(|p| p.contains("/users/login") || p.contains("login")),
        "Should have /users/login path. Found paths: {:?}",
        path_keys
    );
}

// =============================================================================
// TEST 2: OpenAPI debe tener schemas registrados
// =============================================================================

#[test]
fn test_openapi_has_schemas() {
    // Arrange & Act
    let spec = ApiDoc::openapi();
    
    // Assert: Debe tener schemas
    if let Some(components) = spec.components.as_ref() {
        let schemas_count = components.schemas.len();
        assert!(
            schemas_count > 0,
            "OpenAPI spec should have schemas registered, got {} schemas",
            schemas_count
        );
        
        // Verificar schemas básicos
        let schema_keys: Vec<String> = components.schemas.keys().cloned().collect();
        assert!(
            schema_keys.iter().any(|s| s.contains("User")),
            "Should have User schema. Found schemas: {:?}",
            schema_keys
        );
    } else {
        panic!("OpenAPI spec should have components");
    }
}

// =============================================================================
// TEST 3: OpenAPI JSON debe ser válido
// =============================================================================

#[test]
fn test_openapi_json_is_valid() {
    // Arrange & Act
    let spec = ApiDoc::openapi();
    let json_value = serde_json::to_value(&spec)
        .expect("OpenAPI spec should be serializable to JSON");
    
    // Assert: Verificar estructura básica
    assert_eq!(
        json_value["info"]["title"],
        "VibeStream API",
        "Should have correct title"
    );
    
    assert_eq!(
        json_value["info"]["version"],
        "2.0.0",
        "Should have correct version"
    );
    
    // Verificar que tiene paths
    assert!(
        json_value["paths"].is_object(),
        "Should have paths object"
    );
    
    let paths = json_value["paths"].as_object().unwrap();
    assert!(
        !paths.is_empty(),
        "Paths should not be empty, got {} paths",
        paths.len()
    );
}

// =============================================================================
// TEST 4: OpenAPI debe tener tags definidos
// =============================================================================

#[test]
fn test_openapi_has_tags() {
    // Arrange & Act
    let spec = ApiDoc::openapi();
    
    // Assert: Debe tener tags
    assert!(
        !spec.tags.is_empty(),
        "OpenAPI spec should have tags, got {} tags",
        spec.tags.len()
    );
    
    // Verificar tags básicos
    let tag_names: Vec<String> = spec.tags.iter()
        .map(|tag| tag.name.clone())
        .collect();
    
    assert!(
        tag_names.contains(&"users".to_string()),
        "Should have 'users' tag. Found tags: {:?}",
        tag_names
    );
    
    assert!(
        tag_names.contains(&"payments".to_string()),
        "Should have 'payments' tag. Found tags: {:?}",
        tag_names
    );
}

// =============================================================================
// TEST 5: OpenAPI debe tener servers configurados
// =============================================================================

#[test]
fn test_openapi_has_servers() {
    // Arrange & Act
    let spec = ApiDoc::openapi();
    
    // Assert: Debe tener servers
    assert!(
        !spec.servers.is_empty(),
        "OpenAPI spec should have servers, got {} servers",
        spec.servers.len()
    );
    
    // Verificar que tiene el server de User Gateway
    let server_urls: Vec<String> = spec.servers.iter()
        .map(|server| server.url.clone())
        .collect();
    
    assert!(
        server_urls.contains(&"http://localhost:3001".to_string()),
        "Should have User Gateway server. Found servers: {:?}",
        server_urls
    );
}

