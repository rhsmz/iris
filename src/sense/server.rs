use axum::{
    routing::{get, post},
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Serialize)]
pub struct ChatResponse {
    pub reply: String,
}

pub async fn start_server() {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/chat", post(handle_chat));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Sense server listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> &'static str {
    "I.R.I.S. is feeling perfectly fine... mostly."
}

async fn handle_chat(Json(payload): Json<ChatRequest>) -> Json<ChatResponse> {
    println!("Received message: {}", payload.message);
    // In the future, this will hook into the logic and memory layers.
    Json(ChatResponse {
        reply: format!("I hear you: {}", payload.message),
    })
}
