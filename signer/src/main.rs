mod handlers;
mod openapi;

use axum::{Json, Router, routing::{get, post}, http::StatusCode, response::IntoResponse};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use utoipa::OpenApi;
use serde_json::json;

use tracing::{info};

use flexi_logger::{Logger, Criterion, Naming, Cleanup, FileSpec, Duplicate};

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
    use flexi_logger::{DeferredNow, Record};
    use std::io::Write;

    // 💾 Formatter for file logs (includes timestamp, level, and module path)
    fn file_format(
        w: &mut dyn Write,
        now: &mut DeferredNow,
        record: &Record,
    ) -> std::io::Result<()> {
        write!(
            w,
            "{} [{}] [{}] {}",
            now.now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.module_path().unwrap_or("<unnamed>"),
            &record.args()
        )
    }

    // 🖥️ Formatter for stdout logs (clean message only)
    fn stdout_format(
        w: &mut dyn Write,
        _now: &mut DeferredNow,
        record: &Record,
    ) -> std::io::Result<()> {
        write!(w, "{} [{}] {}\n", record.level(), record.module_path().unwrap_or(""), record.args())
    }

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
        .format_for_files(file_format) // 💾 format for file logs
        .format_for_stdout(stdout_format) // simpler: can be your own too
        .duplicate_to_stdout(Duplicate::All)
        .start()
        .unwrap();
}
