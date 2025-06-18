mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/version", get(handlers::version::service_info))
        .route("/sign", post(handlers::sign::sign_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
