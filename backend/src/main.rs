use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

#[derive(Debug, Clone)]
struct AppState {
    users: Arc<RwLock<Vec<User>>>,
    posts: Arc<RwLock<Vec<Post>>>,
    ads: Arc<RwLock<Vec<Ad>>>,
    messages: Arc<RwLock<Vec<Message>>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            users: Arc::new(RwLock::new(Vec::new())),
            posts: Arc::new(RwLock::new(Vec::new())),
            ads: Arc::new(RwLock::new(Vec::new())),
            messages: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: Uuid,
    name: String,
    role: String,
}

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    name: String,
    role: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthResponse {
    user_id: Uuid,
    token: String,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    user_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Post {
    id: Uuid,
    content: String,
    media_url: Option<String>,
    status: String,
    scheduled_time: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CreatePostRequest {
    content: String,
    media_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SchedulePostRequest {
    post_id: Uuid,
    scheduled_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Ad {
    id: Uuid,
    campaign_id: String,
    performance_metrics: AdPerformance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AdPerformance {
    ctr: f64,
    cpc: f64,
    conversion_rate: f64,
    engagement_rate: f64,
}

#[derive(Debug, Deserialize)]
struct CreateAdRequest {
    campaign_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    id: Uuid,
    platform: String,
    content: String,
    user_id: Uuid,
    status: String,
}

#[derive(Debug, Deserialize)]
struct RespondMessageRequest {
    message_id: Uuid,
    response: String,
}

#[derive(Debug, Serialize)]
struct AnalyticsDashboard {
    total_ads: usize,
    total_posts: usize,
    total_messages: usize,
    avg_ctr: f64,
    avg_conversion_rate: f64,
}

fn app_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/content/create", post(create_content))
        .route("/content/schedule", post(schedule_content))
        .route("/content/list", get(list_content))
        .route("/ads/create", post(create_ad))
        .route("/ads/performance", get(ads_performance))
        .route("/messages", get(get_messages))
        .route("/messages/respond", post(respond_message))
        .route("/analytics/dashboard", get(get_dashboard))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "backend=info,tower_http=info".to_string()),
        )
        .init();

    let state = AppState::new();
    let app = app_router(state);

    let addr: SocketAddr = "0.0.0.0:3000".parse().expect("valid listen address");
    info!(%addr, "starting backend server");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind listener");

    axum::serve(listener, app)
        .await
        .expect("backend server crashed");
}

async fn health() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "ok",
            "service": "marketing-tools-backend"
        })),
    )
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> (StatusCode, Json<AuthResponse>) {
    let user = User {
        id: Uuid::new_v4(),
        name: payload.name,
        role: payload.role,
    };

    state.users.write().await.push(user.clone());

    (
        StatusCode::CREATED,
        Json(AuthResponse {
            user_id: user.id,
            token: format!("mock-jwt-{}", user.id),
        }),
    )
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), StatusCode> {
    let users = state.users.read().await;

    if users.iter().any(|user| user.id == payload.user_id) {
        Ok((
            StatusCode::OK,
            Json(AuthResponse {
                user_id: payload.user_id,
                token: format!("mock-jwt-{}", payload.user_id),
            }),
        ))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn create_content(
    State(state): State<AppState>,
    Json(payload): Json<CreatePostRequest>,
) -> (StatusCode, Json<Post>) {
    let post = Post {
        id: Uuid::new_v4(),
        content: payload.content,
        media_url: payload.media_url,
        status: "draft".to_string(),
        scheduled_time: None,
    };

    state.posts.write().await.push(post.clone());

    (StatusCode::CREATED, Json(post))
}

async fn schedule_content(
    State(state): State<AppState>,
    Json(payload): Json<SchedulePostRequest>,
) -> Result<(StatusCode, Json<Post>), StatusCode> {
    let mut posts = state.posts.write().await;

    let Some(post) = posts.iter_mut().find(|p| p.id == payload.post_id) else {
        return Err(StatusCode::NOT_FOUND);
    };

    post.status = "scheduled".to_string();
    post.scheduled_time = Some(payload.scheduled_time);

    Ok((StatusCode::OK, Json(post.clone())))
}

async fn list_content(State(state): State<AppState>) -> (StatusCode, Json<Vec<Post>>) {
    let posts = state.posts.read().await;
    (StatusCode::OK, Json(posts.clone()))
}

async fn create_ad(
    State(state): State<AppState>,
    Json(payload): Json<CreateAdRequest>,
) -> (StatusCode, Json<Ad>) {
    let ad = Ad {
        id: Uuid::new_v4(),
        campaign_id: payload.campaign_id,
        performance_metrics: AdPerformance {
            ctr: 0.0,
            cpc: 0.0,
            conversion_rate: 0.0,
            engagement_rate: 0.0,
        },
    };

    state.ads.write().await.push(ad.clone());

    (StatusCode::CREATED, Json(ad))
}

async fn ads_performance(State(state): State<AppState>) -> (StatusCode, Json<Vec<Ad>>) {
    let ads = state.ads.read().await;
    (StatusCode::OK, Json(ads.clone()))
}

async fn get_messages(State(state): State<AppState>) -> (StatusCode, Json<Vec<Message>>) {
    let messages = state.messages.read().await;
    (StatusCode::OK, Json(messages.clone()))
}

async fn respond_message(
    State(state): State<AppState>,
    Json(payload): Json<RespondMessageRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let mut messages = state.messages.write().await;

    let Some(message) = messages.iter_mut().find(|m| m.id == payload.message_id) else {
        return Err(StatusCode::NOT_FOUND);
    };

    message.status = "responded".to_string();

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "message_id": payload.message_id,
            "status": "responded",
            "response": payload.response,
        })),
    ))
}

async fn get_dashboard(State(state): State<AppState>) -> (StatusCode, Json<AnalyticsDashboard>) {
    let ads = state.ads.read().await;
    let posts = state.posts.read().await;
    let messages = state.messages.read().await;

    let total_ads = ads.len();
    let avg_ctr = if total_ads == 0 {
        0.0
    } else {
        ads.iter().map(|ad| ad.performance_metrics.ctr).sum::<f64>() / total_ads as f64
    };

    let avg_conversion_rate = if total_ads == 0 {
        0.0
    } else {
        ads.iter()
            .map(|ad| ad.performance_metrics.conversion_rate)
            .sum::<f64>()
            / total_ads as f64
    };

    (
        StatusCode::OK,
        Json(AnalyticsDashboard {
            total_ads,
            total_posts: posts.len(),
            total_messages: messages.len(),
            avg_ctr,
            avg_conversion_rate,
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_endpoint_returns_ok() {
        let app = app_router(AppState::new());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .expect("request builder should construct request"),
            )
            .await
            .expect("health request should succeed");

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn register_then_login_succeeds() {
        let app = app_router(AppState::new());

        let register_body = serde_json::to_vec(&serde_json::json!({
            "name": "Owner",
            "role": "business_owner"
        }))
        .expect("register payload should serialize");

        let register_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/auth/register")
                    .header("content-type", "application/json")
                    .body(Body::from(register_body))
                    .expect("register request should build"),
            )
            .await
            .expect("register request should succeed");

        assert_eq!(register_response.status(), StatusCode::CREATED);

        let bytes = register_response
            .into_body()
            .collect()
            .await
            .expect("register response body should read")
            .to_bytes();

        let auth: AuthResponse =
            serde_json::from_slice(&bytes).expect("register response should deserialize");

        let login_body = serde_json::to_vec(&serde_json::json!({
            "user_id": auth.user_id
        }))
        .expect("login payload should serialize");

        let login_response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/auth/login")
                    .header("content-type", "application/json")
                    .body(Body::from(login_body))
                    .expect("login request should build"),
            )
            .await
            .expect("login request should succeed");

        assert_eq!(login_response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn create_then_list_content() {
        let app = app_router(AppState::new());

        let create_body = serde_json::to_vec(&serde_json::json!({
            "content": "Launch campaign",
            "media_url": "https://example.com/asset.png"
        }))
        .expect("create payload should serialize");

        let create_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/content/create")
                    .header("content-type", "application/json")
                    .body(Body::from(create_body))
                    .expect("create request should build"),
            )
            .await
            .expect("create request should succeed");

        assert_eq!(create_response.status(), StatusCode::CREATED);

        let list_response = app
            .oneshot(
                Request::builder()
                    .uri("/content/list")
                    .body(Body::empty())
                    .expect("list request should build"),
            )
            .await
            .expect("list request should succeed");

        assert_eq!(list_response.status(), StatusCode::OK);

        let list_bytes = list_response
            .into_body()
            .collect()
            .await
            .expect("list response body should read")
            .to_bytes();

        let posts: Vec<Post> =
            serde_json::from_slice(&list_bytes).expect("list response should deserialize");

        assert_eq!(posts.len(), 1);
        assert_eq!(posts[0].status, "draft");
    }
}
