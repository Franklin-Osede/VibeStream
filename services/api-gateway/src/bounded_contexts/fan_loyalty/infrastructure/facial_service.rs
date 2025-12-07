//! Facial Recognition Service Integration
//! 
//! Integración con servicio de reconocimiento facial open source (gratuito)
//! Usa face_recognition (Python) expuesto como API REST

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::bounded_contexts::fan_loyalty::domain::entities::FanId;
use crate::shared::domain::errors::AppError;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

// Base64 encoding helper
fn encode_base64(data: &[u8]) -> String {
    BASE64.encode(data)
}

/// Trait para servicios de reconocimiento facial
#[async_trait]
pub trait FacialRecognitionService: Send + Sync {
    /// Registrar template facial de un usuario
    async fn register_face(
        &self,
        fan_id: &FanId,
        image_bytes: &[u8],
    ) -> Result<(), AppError>;
    
    /// Verificar que una imagen coincide con template almacenado
    /// Retorna confidence score (0.0 - 1.0)
    async fn verify_face(
        &self,
        fan_id: &FanId,
        image_bytes: &[u8],
    ) -> Result<f32, AppError>;
    
    /// Eliminar template facial de un usuario
    async fn delete_face(&self, fan_id: &FanId) -> Result<(), AppError>;
}

/// Implementación usando servicio open source (face_recognition)
pub struct OpenSourceFacialService {
    client: Client,
    service_url: String,
}

#[derive(Serialize)]
struct RegisterFaceRequest {
    fan_id: String,
    image: String, // base64
}

#[derive(Serialize)]
struct VerifyFaceRequest {
    fan_id: String,
    image: String, // base64
}

#[derive(Deserialize)]
struct VerifyFaceResponse {
    success: bool,
    confidence_score: f32,
    is_match: bool,
    distance: Option<f32>,
    #[serde(default)]
    message: Option<String>,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
}

impl OpenSourceFacialService {
    /// Crear nuevo servicio de reconocimiento facial
    pub fn new(service_url: String) -> Self {
        Self {
            client: Client::new(),
            service_url: service_url.trim_end_matches('/').to_string(),
        }
    }
    
    /// Crear desde variable de entorno
    pub fn from_env() -> Result<Self, AppError> {
        let service_url = std::env::var("FACIAL_RECOGNITION_SERVICE_URL")
            .unwrap_or_else(|_| "http://localhost:8004".to_string());
        Ok(Self::new(service_url))
    }
}

#[async_trait]
impl FacialRecognitionService for OpenSourceFacialService {
    async fn register_face(
        &self,
        fan_id: &FanId,
        image_bytes: &[u8],
    ) -> Result<(), AppError> {
        let image_base64 = encode_base64(image_bytes);
        
        let request = RegisterFaceRequest {
            fan_id: fan_id.0.to_string(),
            image: image_base64,
        };
        
        let response = self.client
            .post(&format!("{}/register", self.service_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::ExternalServiceError(
                format!("Failed to call facial recognition service: {}", e)
            ))?;
        
        if response.status().is_success() {
            Ok(())
        } else {
            let error: ErrorResponse = response.json().await
                .unwrap_or_else(|_| ErrorResponse {
                    error: "Unknown error".to_string()
                });
            Err(AppError::ExternalServiceError(error.error))
        }
    }
    
    async fn verify_face(
        &self,
        fan_id: &FanId,
        image_bytes: &[u8],
    ) -> Result<f32, AppError> {
        let image_base64 = encode_base64(image_bytes);
        
        let request = VerifyFaceRequest {
            fan_id: fan_id.0.to_string(),
            image: image_base64,
        };
        
        let response = self.client
            .post(&format!("{}/verify", self.service_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::ExternalServiceError(
                format!("Failed to call facial recognition service: {}", e)
            ))?;
        
        if response.status().is_success() {
            let result: VerifyFaceResponse = response.json().await
                .map_err(|e| AppError::ExternalServiceError(
                    format!("Failed to parse response: {}", e)
                ))?;
            
            if result.is_match {
                Ok(result.confidence_score)
            } else {
                Ok(0.0) // No match, retornar 0.0
            }
        } else if response.status() == 404 {
            // Face not registered
            Ok(0.0)
        } else {
            let error: ErrorResponse = response.json().await
                .unwrap_or_else(|_| ErrorResponse {
                    error: "Unknown error".to_string()
                });
            Err(AppError::ExternalServiceError(error.error))
        }
    }
    
    async fn delete_face(&self, fan_id: &FanId) -> Result<(), AppError> {
        let response = self.client
            .delete(&format!("{}/delete/{}", self.service_url, fan_id.0))
            .send()
            .await
            .map_err(|e| AppError::ExternalServiceError(
                format!("Failed to call facial recognition service: {}", e)
            ))?;
        
        if response.status().is_success() || response.status() == 404 {
            Ok(())
        } else {
            let error: ErrorResponse = response.json().await
                .unwrap_or_else(|_| ErrorResponse {
                    error: "Unknown error".to_string()
                });
            Err(AppError::ExternalServiceError(error.error))
        }
    }
}

/// Mock implementation para testing
pub struct MockFacialService;

#[async_trait]
impl FacialRecognitionService for MockFacialService {
    async fn register_face(
        &self,
        _fan_id: &FanId,
        _image_bytes: &[u8],
    ) -> Result<(), AppError> {
        Ok(())
    }
    
    async fn verify_face(
        &self,
        _fan_id: &FanId,
        _image_bytes: &[u8],
    ) -> Result<f32, AppError> {
        // Retornar score mock alto para testing
        Ok(0.95)
    }
    
    async fn delete_face(&self, _fan_id: &FanId) -> Result<(), AppError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    
    #[tokio::test]
    async fn test_mock_facial_service() {
        let service = MockFacialService;
        let fan_id = FanId(Uuid::new_v4());
        let image_bytes = b"fake_image_data";
        
        // Test register
        let result = service.register_face(&fan_id, image_bytes).await;
        assert!(result.is_ok());
        
        // Test verify
        let score = service.verify_face(&fan_id, image_bytes).await.unwrap();
        assert!(score > 0.9);
        
        // Test delete
        let result = service.delete_face(&fan_id).await;
        assert!(result.is_ok());
    }
}
