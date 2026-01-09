mod health;

use axum::Router;
use std::env;

fn main() {
    // Define the IP address and port to listen on
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let bind_address = format!("0.0.0.0:{port}");

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime")
        .block_on(async {
            let listener = tokio::net::TcpListener::bind(bind_address)
                .await
                .expect("Failed to bind to address");
            // Call axum serve
            axum::serve(listener, get_routes())
                .await
                .expect("Failed to start server");
        });
}

fn get_routes() -> Router {
    Router::new().nest("/up", health::health_router())
}
