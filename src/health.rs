use axum::{Json, Router, response::IntoResponse, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
}

pub fn health_router() -> Router {
    Router::new().route("/", get(health_endpoint))
}

async fn health_endpoint() -> impl IntoResponse {
    Json(HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
    })
}
