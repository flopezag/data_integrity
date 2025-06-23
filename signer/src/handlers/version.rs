use axum::Json;
use serde::Serialize;
use std::time::Instant;
use once_cell::sync::Lazy;
use utoipa::ToSchema;

static START_TIME: Lazy<Instant> = Lazy::new(Instant::now);
static GITHUB_REPO: &str = "https://github.com/flopezag/data_integrity";
static VERSION: &str = "0.1.0";

#[derive(Serialize, ToSchema)]
pub struct ServiceInfo {
    version: String,
    repository: String,
    uptime_seconds: u64,
}

#[utoipa::path(
    get,
    path = "/info",
    responses(
        (status = 200, description = "Service info", body = ServiceInfo)
    )
)]

pub async fn service_info() -> Json<ServiceInfo> {
    Json(ServiceInfo {
        version: VERSION.to_string(),
        repository: GITHUB_REPO.to_string(),
        uptime_seconds: START_TIME.elapsed().as_secs(),
    })
}
