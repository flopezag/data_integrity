use axum::{
    routing::post,
    Json, Router
};
use serde_json::Value;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    // build the application with a route
    let app = Router::new().route("/notification", post(handle_post));

    // specify the port (e.g., 3500)
    let addr = SocketAddr::from(([0, 0, 0, 0], 3500));
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on http://{}\n\n", addr);

    // run the server
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

// handler for POST requests
async fn handle_post(Json(payload): Json<Value>) {
    // Convert JSON Value to a pretty-printed string
    match serde_json::to_string_pretty(&payload) {
        Ok(pretty_json) => {
            println!("Received JSON payload:\n{}\n\n", pretty_json);
        }
        Err(e) => {
            eprintln!("Failed to pretty print JSON payload: {}", e);
        }
    }
}
