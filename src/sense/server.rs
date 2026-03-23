use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::State,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use crate::logic::reasoning::OllamaClient;
use crate::memory::graph::{insert_memory, spread_activation};

/// 共有アプリケーション状態
pub struct AppState {
    pub ollama: OllamaClient,
}

#[derive(Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Serialize)]
pub struct ChatResponse {
    pub reply: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

pub async fn start_server(state: Arc<AppState>) {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/chat", post(handle_chat))
        .with_state(state);

    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid server address");

    println!("🌟 I.R.I.S. Sense Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind address");
    axum::serve(listener, app).await.expect("Server error");
}

async fn health_check() -> &'static str {
    "I.R.I.S. is feeling perfectly fine... mostly."
}

/// / api/chat ハンドラ
/// Sense → Memory（Spreading Activation）→ Logic（RAG + Gemma 3n）→ Memory（保存）のパイプラインを実行する
async fn handle_chat(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ChatRequest>,
) -> Json<ChatResponse> {
    println!("💬 受信: {}", payload.message);

    // Logic + RAG パイプラインを実行する
    let reply = match state.ollama.ask_with_context(&payload.message).await {
        Ok(response) => {
            // 新しい記憶として保存する（emotion_score はデフォルト 5.0）
            let _ = insert_memory(&payload.message, 5.0, "", None).await;
            response
        }
        Err(e) => {
            eprintln!("❌ Ollama エラー: {}", e);
            "回路がサビすぎて……少し待っていただけますか。".to_string()
        }
    };

    Json(ChatResponse { reply })
}

#[cfg(test)]
mod tests {
    #[test]
    fn チャットリクエストの構造体が正しくデシリアライズされる() {
        use super::ChatRequest;
        let json = r#"{"message": "こんにちは"}"#;
        let req: ChatRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.message, "こんにちは");
    }
}
