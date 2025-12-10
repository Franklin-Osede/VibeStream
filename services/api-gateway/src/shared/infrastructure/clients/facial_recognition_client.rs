use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::time::Duration;

#[derive(Clone)]
pub struct FacialRecognitionClient {
    client: Client,
    base_url: String,
}

#[derive(Debug, Serialize)]
struct RegisterFaceRequest {
    fan_id: Uuid,
    image: String, // base64
}

#[derive(Debug, Deserialize)]
struct RegisterFaceResponse {
    success: bool,
    fan_id: Uuid,
    message: String,
}

#[derive(Debug, Serialize)]
struct VerifyFaceRequest {
    fan_id: Uuid,
    image: String, // base64
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VerifyFaceResponse {
    pub success: bool,
    pub fan_id: Uuid,
    pub confidence_score: f64,
    pub is_match: bool,
    pub distance: f64,
}

impl FacialRecognitionClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
            base_url,
        }
    }

    pub async fn register_face(&self, fan_id: Uuid, image_base64: String) -> Result<bool> {
        let url = format!("{}/register", self.base_url);
        let request = RegisterFaceRequest {
            fan_id,
            image: image_base64,
        };

        let response = self.client.post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Facial Recognition Service")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Facial Registration failed: {}", error_text);
        }

        let body: RegisterFaceResponse = response.json().await
            .context("Failed to parse response from Facial Recognition Service")?;

        Ok(body.success)
    }

    pub async fn verify_face(&self, fan_id: Uuid, image_base64: String) -> Result<VerifyFaceResponse> {
        let url = format!("{}/verify", self.base_url);
        let request = VerifyFaceRequest {
            fan_id,
            image: image_base64,
        };

        let response = self.client.post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Facial Recognition Service")?;

        if !response.status().is_success() {
             let error_text = response.text().await.unwrap_or_default();
             anyhow::bail!("Facial Verification failed: {}", error_text);
        }

        let body: VerifyFaceResponse = response.json().await
            .context("Failed to parse response from Facial Recognition Service")?;

        Ok(body)
    }

    pub async fn delete_face(&self, fan_id: Uuid) -> Result<bool> {
        let url = format!("{}/delete/{}", self.base_url, fan_id);
       
        let response = self.client.delete(&url)
            .send()
            .await
            .context("Failed to send request to Facial Recognition Service")?;

        if response.status().as_u16() == 404 {
            return Ok(false);
        }

        if !response.status().is_success() {
             let error_text = response.text().await.unwrap_or_default();
             anyhow::bail!("Delete Face failed: {}", error_text);
        }

        Ok(true)
    }
}
