use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::shared::domain::errors::AppError;
use crate::bounded_contexts::federation::application::FederationApplicationService;
use crate::bounded_contexts::federation::domain::*;

/// Federation Controller for P2P Integration
pub struct FederationController {
    federation_service: Arc<FederationApplicationService>,
}

impl FederationController {
    pub fn new(federation_service: Arc<FederationApplicationService>) -> Self {
        Self { federation_service }
    }

    pub fn routes(&self) -> Router {
        Router::new()
            // ActivityPub endpoints
            .route("/.well-known/webfinger", get(Self::webfinger))
            .route("/.well-known/nodeinfo", get(Self::nodeinfo))
            .route("/nodeinfo/2.0", get(Self::nodeinfo_20))
            .route("/users/:username", get(Self::user_profile))
            .route("/users/:username/inbox", post(Self::user_inbox))
            .route("/users/:username/outbox", get(Self::user_outbox))
            .route("/users/:username/followers", get(Self::user_followers))
            .route("/users/:username/following", get(Self::user_following))
            
            // Federation management endpoints
            .route("/federation/instances", get(Self::list_instances))
            .route("/federation/instances/:domain", get(Self::get_instance))
            .route("/federation/instances/:domain/trust", post(Self::update_instance_trust))
            .route("/federation/activities", get(Self::list_activities))
            .route("/federation/activities/:id", get(Self::get_activity))
            .route("/federation/users", get(Self::list_users))
            .route("/federation/content", get(Self::list_content))
            .route("/federation/follows", get(Self::list_follows))
            .route("/federation/follows/pending", get(Self::list_pending_follows))
            .route("/federation/follows/:follower/:followee", post(Self::handle_follow_request))
            
            // Content sharing endpoints
            .route("/federation/share/music", post(Self::share_music))
            .route("/federation/share/video", post(Self::share_video))
            .route("/federation/content/:id/reactions", post(Self::add_reaction))
            .route("/federation/content/:id/comments", post(Self::add_comment))
    }
}

// ActivityPub Protocol Endpoints

async fn webfinger(
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<WebFingerResource>, AppError> {
    let resource = params.get("resource")
        .ok_or_else(|| AppError::ValidationError("Missing resource parameter".to_string()))?;
    
    // Parse resource (e.g., "acct:user@domain.com")
    if resource.starts_with("acct:") {
        let acct = resource.strip_prefix("acct:").unwrap();
        let parts: Vec<&str> = acct.split('@').collect();
        if parts.len() == 2 {
            let username = parts[0];
            let domain = parts[1];
            
            // Create WebFinger response
            let resource = WebFingerResource {
                subject: resource.clone(),
                links: vec![
                    WebFingerLink {
                        rel: "self".to_string(),
                        href: Some(format!("https://{}/users/{}", domain, username)),
                        template: None,
                        title: None,
                        media_type: Some("application/activity+json".to_string()),
                        properties: HashMap::new(),
                    },
                    WebFingerLink {
                        rel: "http://webfinger.net/rel/profile-page".to_string(),
                        href: Some(format!("https://{}/users/{}", domain, username)),
                        template: None,
                        title: None,
                        media_type: Some("text/html".to_string()),
                        properties: HashMap::new(),
                    },
                ],
                aliases: vec![
                    format!("https://{}/users/{}", domain, username),
                ],
                properties: HashMap::new(),
            };
            
            return Ok(Json(resource));
        }
    }
    
    Err(AppError::NotFound("Resource not found".to_string()))
}

async fn nodeinfo() -> Result<Json<serde_json::Value>, AppError> {
    let nodeinfo = serde_json::json!({
        "links": [
            {
                "rel": "http://nodeinfo.diaspora.software/ns/schema/2.0",
                "href": "/nodeinfo/2.0"
            }
        ]
    });
    
    Ok(Json(nodeinfo))
}

async fn nodeinfo_20() -> Result<Json<NodeInfo>, AppError> {
    let nodeinfo = NodeInfo {
        version: "2.0".to_string(),
        software: NodeInfoSoftware {
            name: "VibeStream".to_string(),
            version: "1.0.0".to_string(),
            repository: Some("https://github.com/vibestream/vibestream".to_string()),
            homepage: Some("https://vibestream.network".to_string()),
        },
        protocols: vec![
            "activitypub".to_string(),
            "webrtc".to_string(),
            "ipfs".to_string(),
        ],
        services: NodeInfoServices {
            inbound: vec![],
            outbound: vec!["activitypub".to_string()],
        },
        usage: NodeInfoUsage {
            users: NodeInfoUsers {
                total: 1000,
                active_month: 500,
                active_half_year: 800,
            },
            local_posts: 5000,
            local_comments: 15000,
        },
        open_registrations: true,
        metadata: HashMap::new(),
    };
    
    Ok(Json(nodeinfo))
}

async fn user_profile(
    Path(username): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Create ActivityPub Person object
    let person = serde_json::json!({
        "@context": [
            "https://www.w3.org/ns/activitystreams",
            "https://w3id.org/security/v1"
        ],
        "id": format!("https://vibestream.network/users/{}", username),
        "type": "Person",
        "preferredUsername": username,
        "name": format!("@{}", username),
        "summary": "VibeStream user",
        "inbox": format!("https://vibestream.network/users/{}/inbox", username),
        "outbox": format!("https://vibestream.network/users/{}/outbox", username),
        "followers": format!("https://vibestream.network/users/{}/followers", username),
        "following": format!("https://vibestream.network/users/{}/following", username),
        "publicKey": {
            "id": format!("https://vibestream.network/users/{}#main-key", username),
            "owner": format!("https://vibestream.network/users/{}", username),
            "publicKeyPem": "-----BEGIN PUBLIC KEY-----\n...\n-----END PUBLIC KEY-----\n"
        }
    });
    
    Ok(Json(person))
}

async fn user_inbox(
    Path(username): Path<String>,
    headers: HeaderMap,
    body: String,
) -> Result<StatusCode, AppError> {
    // Handle incoming ActivityPub activities
    println!("📥 Received ActivityPub activity for user {}: {}", username, body);
    
    // TODO: Parse and process the activity
    // This would validate the signature and process the activity
    
    Ok(StatusCode::OK)
}

async fn user_outbox(
    Path(username): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Return user's outbox (activities they've sent)
    let outbox = serde_json::json!({
        "@context": "https://www.w3.org/ns/activitystreams",
        "id": format!("https://vibestream.network/users/{}/outbox", username),
        "type": "OrderedCollection",
        "totalItems": 0,
        "orderedItems": []
    });
    
    Ok(Json(outbox))
}

async fn user_followers(
    Path(username): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let followers = serde_json::json!({
        "@context": "https://www.w3.org/ns/activitystreams",
        "id": format!("https://vibestream.network/users/{}/followers", username),
        "type": "OrderedCollection",
        "totalItems": 0,
        "orderedItems": []
    });
    
    Ok(Json(followers))
}

async fn user_following(
    Path(username): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let following = serde_json::json!({
        "@context": "https://www.w3.org/ns/activitystreams",
        "id": format!("https://vibestream.network/users/{}/following", username),
        "type": "OrderedCollection",
        "totalItems": 0,
        "orderedItems": []
    });
    
    Ok(Json(following))
}

// Federation Management Endpoints

async fn list_instances(
    State(controller): State<Arc<FederationController>>,
) -> Result<Json<Vec<FederatedInstance>>, AppError> {
    let instances = controller.federation_service.list_trusted_instances().await?;
    Ok(Json(instances))
}

async fn get_instance(
    State(controller): State<Arc<FederationController>>,
    Path(domain): Path<String>,
) -> Result<Json<Option<FederatedInstance>>, AppError> {
    let instance = controller.federation_service.get_instance(&domain).await?;
    Ok(Json(instance))
}

#[derive(Deserialize)]
struct UpdateTrustRequest {
    trust_level: TrustLevel,
}

async fn update_instance_trust(
    State(controller): State<Arc<FederationController>>,
    Path(domain): Path<String>,
    Json(request): Json<UpdateTrustRequest>,
) -> Result<StatusCode, AppError> {
    controller.federation_service.update_instance_trust(&domain, request.trust_level).await?;
    Ok(StatusCode::OK)
}

async fn list_activities(
    State(controller): State<Arc<FederationController>>,
) -> Result<Json<Vec<ActivityPubActivity>>, AppError> {
    let activities = controller.federation_service.get_pending_activities().await?;
    Ok(Json(activities))
}

async fn get_activity(
    State(controller): State<Arc<FederationController>>,
    Path(activity_id): Path<String>,
) -> Result<Json<Option<ActivityPubActivity>>, AppError> {
    // This would need to be implemented in the service
    Err(AppError::NotImplemented("Not implemented yet".to_string()))
}

async fn list_users(
    State(controller): State<Arc<FederationController>>,
) -> Result<Json<Vec<FederatedUser>>, AppError> {
    // This would need to be implemented in the service
    Err(AppError::NotImplemented("Not implemented yet".to_string()))
}

async fn list_content(
    State(controller): State<Arc<FederationController>>,
) -> Result<Json<Vec<FederatedContent>>, AppError> {
    // This would need to be implemented in the service
    Err(AppError::NotImplemented("Not implemented yet".to_string()))
}

async fn list_follows(
    State(controller): State<Arc<FederationController>>,
) -> Result<Json<Vec<FederationFollow>>, AppError> {
    // This would need to be implemented in the service
    Err(AppError::NotImplemented("Not implemented yet".to_string()))
}

async fn list_pending_follows(
    State(controller): State<Arc<FederationController>>,
) -> Result<Json<Vec<FederationFollow>>, AppError> {
    let follows = controller.federation_service.get_pending_follows().await?;
    Ok(Json(follows))
}

#[derive(Deserialize)]
struct FollowRequestRequest {
    approve: bool,
}

async fn handle_follow_request(
    State(controller): State<Arc<FederationController>>,
    Path((follower, followee)): Path<(String, String)>,
    Json(request): Json<FollowRequestRequest>,
) -> Result<StatusCode, AppError> {
    controller.federation_service.handle_follow_request(&follower, &followee, request.approve).await?;
    Ok(StatusCode::OK)
}

// Content Sharing Endpoints

#[derive(Deserialize)]
struct ShareMusicRequest {
    title: String,
    artist: String,
    album: Option<String>,
    duration: u32,
    audio_url: String,
    cover_url: Option<String>,
    tags: Vec<String>,
}

async fn share_music(
    State(controller): State<Arc<FederationController>>,
    Json(request): Json<ShareMusicRequest>,
) -> Result<StatusCode, AppError> {
    let content = FederatedContent::new(
        format!("music_{}", uuid::Uuid::new_v4()),
        ContentType::Music,
        request.title,
        format!("New music by {}", request.artist),
        "local_user".to_string(),
        "vibestream.network".to_string(),
    );
    
    controller.federation_service.share_content(content).await?;
    Ok(StatusCode::CREATED)
}

#[derive(Deserialize)]
struct ShareVideoRequest {
    title: String,
    description: String,
    video_url: String,
    thumbnail_url: Option<String>,
    duration: u32,
    tags: Vec<String>,
}

async fn share_video(
    State(controller): State<Arc<FederationController>>,
    Json(request): Json<ShareVideoRequest>,
) -> Result<StatusCode, AppError> {
    let content = FederatedContent::new(
        format!("video_{}", uuid::Uuid::new_v4()),
        ContentType::Video,
        request.title,
        request.description,
        "local_user".to_string(),
        "vibestream.network".to_string(),
    );
    
    controller.federation_service.share_content(content).await?;
    Ok(StatusCode::CREATED)
}

#[derive(Deserialize)]
struct AddReactionRequest {
    emoji: String,
    user_uri: String,
}

async fn add_reaction(
    State(controller): State<Arc<FederationController>>,
    Path(content_id): Path<String>,
    Json(request): Json<AddReactionRequest>,
) -> Result<StatusCode, AppError> {
    let reaction = Reaction {
        id: format!("reaction_{}", uuid::Uuid::new_v4()),
        emoji: request.emoji,
        count: 1,
        users: vec![request.user_uri],
        created_at: Utc::now(),
    };
    
    controller.federation_service.add_reaction_to_content(&content_id, reaction).await?;
    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
struct AddCommentRequest {
    content: String,
    author_uri: String,
}

async fn add_comment(
    State(controller): State<Arc<FederationController>>,
    Path(content_id): Path<String>,
    Json(request): Json<AddCommentRequest>,
) -> Result<StatusCode, AppError> {
    let comment = Comment {
        id: format!("comment_{}", uuid::Uuid::new_v4()),
        content: request.content,
        author: request.author_uri,
        parent_id: None,
        replies_count: 0,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    controller.federation_service.add_comment_to_content(&content_id, comment).await?;
    Ok(StatusCode::OK)
} 