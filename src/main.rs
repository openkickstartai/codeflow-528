use axum::{extract::ws::WebSocketUpgrade, response::Response, routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod handlers;
mod models;
mod websocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::init();
    
    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .route("/api/reviews", get(handlers::get_reviews))
        .route("/api/reviews/:id/comments", get(handlers::get_comments))
        .layer(CorsLayer::permissive());
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    
    println!("CodeFlow server running on http://{}", addr);
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn websocket_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(websocket::handle_socket)
}