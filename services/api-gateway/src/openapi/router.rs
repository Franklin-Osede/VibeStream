//! OpenAPI Router Configuration
//! 
//! Centralized router for Swagger UI, Redoc, and OpenAPI documentation

use axum::{
    routing::get,
    Router,
    response::Json,
    http::StatusCode,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use utoipa_redoc::Redoc;
use crate::openapi::{ApiDoc, generate_openapi_spec};

/// Create router for OpenAPI documentation
pub fn create_openapi_router() -> Router {
    Router::new()
        // Swagger UI - Interactive API documentation
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-docs/openapi.json", ApiDoc::openapi())
        )
        // Redoc - Alternative documentation interface  
        .merge(
            Redoc::new("/redoc", ApiDoc::openapi())
        )
        // Endpoint to get OpenAPI JSON
        .route("/api-docs/openapi.json", get(openapi_spec_handler))
        // Endpoint for API information
        .route("/api-docs/info", get(api_info_handler))
        // Endpoint to validate API coverage
        .route("/api-docs/validate", get(validate_coverage_handler))
}

/// Handler to serve OpenAPI specification in JSON
async fn openapi_spec_handler() -> Result<Json<serde_json::Value>, StatusCode> {
    let spec = ApiDoc::openapi();
    Ok(Json(serde_json::to_value(spec).unwrap_or_else(|_| serde_json::Value::Null)))
}

/// Handler for API information
async fn api_info_handler() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "title": "VibeStream API",
        "version": "1.0.0",
        "description": "Complete VibeStream ecosystem API with microservices architecture",
        "openapi_version": "3.1.0",
        "documentation": {
            "swagger_ui": "/swagger-ui",
            "redoc": "/redoc",
            "openapi_json": "/api-docs/openapi.json"
        },
        "gateways": {
            "user": "http://localhost:3001",
            "music": "http://localhost:3002",
            "payment": "http://localhost:3003",
            "campaign": "http://localhost:3004",
            "listen_reward": "http://localhost:3005",
            "fan_ventures": "http://localhost:3006",
            "notification": "http://localhost:3007",
            "fan_loyalty": "http://localhost:3008"
        },
        "features": {
            "versioning": "Consistent /api/v1 prefix across all gateways",
            "openapi": "Complete OpenAPI 3.1.0 documentation",
            "swagger_ui": "Interactive API documentation",
            "redoc": "Alternative documentation interface",
            "schema_validation": "Request/response validation",
            "microservices": "Independent gateway architecture"
        }
    })))
}

/// Handler to validate API coverage
async fn validate_coverage_handler() -> Result<Json<serde_json::Value>, StatusCode> {
    use crate::openapi::validate_api_coverage;
    
    match validate_api_coverage() {
        Ok(_) => Ok(Json(serde_json::json!({
            "status": "success",
            "message": "All endpoints are properly documented",
            "coverage": "100%",
            "missing_endpoints": []
        }))),
        Err(missing) => Ok(Json(serde_json::json!({
            "status": "warning",
            "message": "Some endpoints are missing documentation",
            "coverage": "partial",
            "missing_endpoints": missing
        })))
    }
}

/// Create documentation router for a specific gateway
pub fn create_gateway_docs_router(gateway_name: String, port: u16) -> Router {
    let gateway_name_clone1 = gateway_name.clone();
    let gateway_name_clone2 = gateway_name.clone();
    Router::new()
        .route("/", get(move || {
            let gateway_name = gateway_name_clone1.clone();
            async move {
                Json(serde_json::json!({
                    "gateway": gateway_name,
                    "port": port,
                    "documentation": {
                        "swagger_ui": format!("http://localhost:{}/swagger-ui", port),
                        "redoc": format!("http://localhost:{}/redoc", port),
                        "openapi_json": format!("http://localhost:{}/api-docs/openapi.json", port)
                    },
                    "endpoints": {
                        "health": format!("http://localhost:{}/health", port),
                        "info": format!("http://localhost:{}/info", port),
                        "api": format!("http://localhost:{}/api/v1", port)
                    }
                }))
            }
        }))
        .route("/health", get(|| async { "Gateway healthy" }))
        .route("/info", get(move || {
            let gateway_name = gateway_name_clone2.clone();
            async move {
                Json(serde_json::json!({
                    "gateway": gateway_name,
                    "port": port,
                    "status": "operational",
                    "version": "1.0.0"
                }))
            }
        }))
}
