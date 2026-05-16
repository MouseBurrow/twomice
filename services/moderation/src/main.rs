use axum::routing::{get, post};
use axum::{Json, Router};
use config::app_data::AppData;
use config::config::Config;
use config::logger;
use serde_json::json;

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok", "service": "moderation" }))
}

async fn create_report() -> Json<serde_json::Value> {
    Json(json!({ "status": "stub", "message": "report creation not implemented" }))
}

async fn list_reports() -> Json<serde_json::Value> {
    Json(json!({ "status": "stub", "reports": [] }))
}

async fn take_action() -> Json<serde_json::Value> {
    Json(json!({ "status": "stub", "message": "moderation action not implemented" }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();

    let config = Config::load("moderation");
    let app_data = AppData::new(config.clone()).await?;
    let addr = format!("0.0.0.0:{}", config.port);

    let app = Router::new()
        .route("/health", get(health))
        .route("/reports", post(create_report).get(list_reports))
        .route("/action", post(take_action))
        .with_state(app_data);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
