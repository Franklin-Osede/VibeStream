use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::bounded_contexts::campaign::application::use_cases::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCampaignRequest {
    pub song_id: String,
    pub artist_id: String,
    pub name: String,
    pub description: String,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: chrono::DateTime<chrono::Utc>,
    pub boost_multiplier: f64,
    pub nft_price: f64,
    pub max_nfts: u32,
    pub target_revenue: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivateCampaignRequest {
    pub nft_contract_address: String,
    pub blockchain_network: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseNFTRequest {
    pub user_id: String,
    pub payment_method: String,
    pub payment_token: String,
    pub wallet_address: String,
    pub quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndCampaignRequest {
    pub reason: String,
    pub force_end: Option<bool>,
}

pub async fn create_campaign(
    request: web::Json<CreateCampaignRequest>,
) -> Result<HttpResponse> {
    let use_case = CreateCampaignUseCase::new();
    
    let command = CreateCampaignCommand {
        song_id: request.song_id.clone(),
        artist_id: request.artist_id.clone(),
        name: request.name.clone(),
        description: request.description.clone(),
        start_date: request.start_date,
        end_date: request.end_date,
        boost_multiplier: request.boost_multiplier,
        nft_price: request.nft_price,
        max_nfts: request.max_nfts,
        target_revenue: request.target_revenue,
    };

    match use_case.execute(command) {
        Ok(response) => Ok(HttpResponse::Created().json(response)),
        Err(error) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": error,
            "success": false
        })))
    }
}

pub async fn activate_campaign(
    path: web::Path<String>,
    request: web::Json<ActivateCampaignRequest>,
) -> Result<HttpResponse> {
    let campaign_id = path.into_inner();
    let use_case = ActivateCampaignUseCase::new();
    
    let command = ActivateCampaignCommand {
        campaign_id,
        nft_contract_address: request.nft_contract_address.clone(),
        blockchain_network: request.blockchain_network.clone(),
    };

    match use_case.execute(command) {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(error) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": error,
            "success": false
        })))
    }
}

pub async fn purchase_nft(
    path: web::Path<String>,
    request: web::Json<PurchaseNFTRequest>,
) -> Result<HttpResponse> {
    let campaign_id = path.into_inner();
    let use_case = PurchaseNFTUseCase::new();
    
    let command = PurchaseNFTCommand {
        campaign_id,
        user_id: request.user_id.clone(),
        payment_method: request.payment_method.clone(),
        payment_token: request.payment_token.clone(),
        wallet_address: request.wallet_address.clone(),
        quantity: request.quantity,
    };

    match use_case.execute(command) {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(error) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": error,
            "success": false
        })))
    }
}

pub async fn get_campaign_analytics(
    path: web::Path<String>,
    query: web::Query<serde_json::Value>,
) -> Result<HttpResponse> {
    let campaign_id = path.into_inner();
    let use_case = GetCampaignAnalyticsUseCase::new();
    
    let include_predictions = query.get("include_predictions")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    
    let include_optimization_suggestions = query.get("include_optimization_suggestions")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    
    let query_obj = GetCampaignAnalyticsQuery {
        campaign_id,
        include_predictions,
        include_optimization_suggestions,
    };

    match use_case.execute(query_obj) {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(error) => Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": error,
            "success": false
        })))
    }
}

pub async fn end_campaign(
    path: web::Path<String>,
    request: web::Json<EndCampaignRequest>,
) -> Result<HttpResponse> {
    let _campaign_id = path.into_inner();
    
    // For now, return a simple success response
    let response = serde_json::json!({
        "success": true,
        "message": format!("Campaign ended with reason: {}", request.reason)
    });

    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_campaign_by_id(
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let campaign_id = path.into_inner();
    
    // For now, return mock data
    let response = serde_json::json!({
        "success": true,
        "campaign_id": campaign_id,
        "name": "Mock Campaign",
        "status": "Active",
        "message": "Campaign retrieved successfully"
    });

    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_campaigns_by_artist(
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let artist_id = path.into_inner();
    
    // For now, return mock data
    let response = serde_json::json!({
        "success": true,
        "artist_id": artist_id,
        "campaigns": [],
        "total": 0,
        "message": "Campaigns retrieved successfully"
    });

    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_active_campaigns() -> Result<HttpResponse> {
    // For now, return mock data
    let response = serde_json::json!({
        "success": true,
        "campaigns": [],
        "total": 0,
        "message": "Active campaigns retrieved successfully"
    });

    Ok(HttpResponse::Ok().json(response))
}

pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "campaign-context",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(
            App::new()
                .route("/health", web::get().to(health_check))
        ).await;

        let req = test::TestRequest::get()
            .uri("/health")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_create_campaign_validation() {
        let app = test::init_service(
            App::new()
                .route("/campaigns", web::post().to(create_campaign))
        ).await;

        let invalid_request = CreateCampaignRequest {
            song_id: "".to_string(), // Invalid empty ID
            artist_id: Uuid::new_v4().to_string(),
            name: "Test Campaign".to_string(),
            description: "Test Description".to_string(),
            start_date: chrono::Utc::now() + chrono::Duration::days(1),
            end_date: chrono::Utc::now() + chrono::Duration::days(30),
            boost_multiplier: 2.0,
            nft_price: 10.0,
            max_nfts: 1000,
            target_revenue: Some(5000.0),
        };

        let req = test::TestRequest::post()
            .uri("/campaigns")
            .set_json(&invalid_request)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(!resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_campaign_analytics() {
        let app = test::init_service(
            App::new()
                .route("/campaigns/{id}/analytics", web::get().to(get_campaign_analytics))
        ).await;

        let campaign_id = Uuid::new_v4().to_string();
        let req = test::TestRequest::get()
            .uri(&format!("/campaigns/{}/analytics?include_predictions=true", campaign_id))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_purchase_nft_validation() {
        let app = test::init_service(
            App::new()
                .route("/campaigns/{id}/purchase", web::post().to(purchase_nft))
        ).await;

        let campaign_id = Uuid::new_v4().to_string();
        let invalid_request = PurchaseNFTRequest {
            user_id: Uuid::new_v4().to_string(),
            payment_method: "invalid_method".to_string(), // Invalid payment method
            payment_token: "ETH".to_string(),
            wallet_address: "0x1234567890123456789012345678901234567890".to_string(),
            quantity: 1,
        };

        let req = test::TestRequest::post()
            .uri(&format!("/campaigns/{}/purchase", campaign_id))
            .set_json(&invalid_request)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(!resp.status().is_success());
    }
} 