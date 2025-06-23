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
    let app = Router::new().route("/", post(handle_post));

    // specify the port (e.g., 3500)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3500));
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on http://{}", addr);

    // run the server
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

// handler for POST requests
async fn handle_post(Json(payload): Json<Value>) {
    println!("Received JSON payload:\n{}", payload);
}
