use axum::{
    extract::Path,
    http::{HeaderValue, Method},
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod demo_data;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3004".into())
        .parse()
        .expect("PORT must be a number");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/demo/info", get(demo_info))
        .route("/api/v1/dashboard", get(dashboard))
        .route("/api/v1/people", get(list_people))
        .route("/api/v1/people/:id", get(get_person))
        .route("/api/v1/events", get(list_events))
        .route("/api/v1/tasks", get(list_tasks))
        .route("/api/v1/things", get(list_things))
        .route("/api/v1/spaces", get(list_spaces))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("HamR Demo Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> Json<Value> {
    Json(json!({"status": "ok", "service": "hamr-demo-server"}))
}

async fn demo_info() -> Json<Value> {
    Json(json!({
        "notice": "这是 HamR 在线演示环境，数据每日凌晨 3:00 自动重置",
        "demo_account": {
            "email": "demo@hamr.store",
            "note": "演示环境使用预置示例数据，无需登录"
        },
        "features": ["五维管理体验", "家庭数据看板", "任务与事件管理"],
        "limitations": ["数据只读", "每日自动重置", "无数据持久化"]
    }))
}

async fn dashboard() -> Json<Value> {
    Json(demo_data::dashboard())
}

async fn list_people() -> Json<Value> {
    Json(json!(demo_data::people()))
}

async fn get_person(Path(id): Path<String>) -> Json<Value> {
    let people = demo_data::people();
    if let Some(person) = people.iter().find(|p| p["id"] == id) {
        Json(person.clone())
    } else {
        Json(json!({"error": "not found"}))
    }
}

async fn list_events() -> Json<Value> {
    Json(json!(demo_data::events()))
}

async fn list_tasks() -> Json<Value> {
    Json(json!(demo_data::tasks()))
}

async fn list_things() -> Json<Value> {
    Json(json!(demo_data::things()))
}

async fn list_spaces() -> Json<Value> {
    Json(json!(demo_data::spaces()))
}
