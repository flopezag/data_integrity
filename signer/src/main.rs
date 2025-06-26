mod handlers;
mod openapi;

use axum::{Json, Router, routing::{get, post}, http::StatusCode, response::IntoResponse};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use utoipa::OpenApi;
use serde_json::json;

use tracing::{info};
use tracing_subscriber::fmt;

use flexi_logger::{Logger, Criterion, Naming, Cleanup, FileSpec, Duplicate};
use tracing_log::LogTracer;

// use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    setup_logging();
    
    info!("✅ Logging initialized");


    if std::env::args().any(|arg| arg == "--dump-openapi") {
        let openapi = openapi::ApiDoc::openapi();
        let yaml = serde_yaml::to_string(&openapi).unwrap();
        std::fs::write("doc/openapi.yaml", yaml).unwrap();
        info!("✅ OpenAPI YAML saved as openapi.yaml");
        return;
    }

    let _api = openapi::ApiDoc::openapi();

    // TODO: Fix SwaggerUi integration
    // let swagger_router = SwaggerUi::new("/docs").url("/api-doc/openapi.json", api);

    let app = Router::new()
        .route("/info", get(handlers::version::service_info))
        .route("/sign", post(handlers::sign::sign_handler))
        .route("/config", post(handlers::config::config_handler))
        .route("/verify", post(handlers::verify::verify_handler))
        .fallback(fallback_handler);
        // .merge(swagger_router);


    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("🚀 Listening on http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn fallback_handler() -> impl IntoResponse {
    let body = json!({ "error": "Endpoint not implemented" });
    (StatusCode::NOT_FOUND, Json(body))
}

fn setup_logging() {
    LogTracer::init().expect("Failed to set logger");

    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory("logs")
                .basename("app")
                .suffix("log"),
        )
        .rotate(
            Criterion::Size(10_000_000), // 10 MB
            Naming::Numbers,
            Cleanup::KeepLogFiles(5),
        )
        .duplicate_to_stdout(Duplicate::All) // Also log to stdout
        .start()
        .unwrap();

    // Optional: set up tracing
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();
}
