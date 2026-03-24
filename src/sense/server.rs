use crate::action::research::TavilyClient;
use crate::logic::reasoning::OllamaClient;
use crate::memory::graph::insert_memory;
use axum::{
    extract::State,
    response::{
        sse::{Event, Sse},
        IntoResponse,
    },
    routing::{get, post},
    Json, Router,
};
use futures_util::stream::{self, Stream};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

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

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response()
    }
}

pub async fn start_server(state: Arc<AppState>) {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/chat", post(handle_chat))
        .route("/api/chat/stream", post(handle_chat_stream))
        .with_state(state);

    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);

    let addr: SocketAddr = format!("{host}:{port}")
        .parse()
        .expect("Invalid server address");

    println!("🌟 I.R.I.S. Sense Server listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");
    axum::serve(listener, app).await.expect("Server error");
}

async fn health_check() -> &'static str {
    "I.R.I.S. is feeling perfectly fine... mostly."
}

/// / api/chat ハンドラ
/// Sense → Memory（Spreading Activation）→ Logic（RAG + Gemma 3n）→ Memory（保存）のパイプラインを実行する
/// TAVILY_API_KEY が設定されている場合は自律リサーチ機能を有効化する
async fn handle_chat(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, ErrorResponse> {
    println!("💬 受信: {}", payload.message);

    // Tavily クライアントが利用可能なら自律リサーチ付き推論を使う
    let result = if let Ok(tavily) = TavilyClient::from_env() {
        state
            .ollama
            .ask_with_research(&payload.message, &tavily)
            .await
    } else {
        state.ollama.ask_with_context(&payload.message).await
    };

    match result {
        Ok(response) => {
            // 新しい記憶として保存する（emotion_score はデフォルト 5.0）
            let _ = insert_memory(&payload.message, 5.0, "", &response).await;
            Ok(Json(ChatResponse { reply: response }))
        }
        Err(e) => {
            eprintln!("❌ Ollama エラー: {e}");
            Err(ErrorResponse {
                error: "LogicError".to_string(),
                message: "回路がサビすぎて……少し待っていただけますか。".to_string(),
            })
        }
    }
}

/// /api/chat/stream ハンドラ (SSE版)
/// 推論結果をチャンクごとにServer-Sent Eventsでクライアントへストリーミングで返却する
async fn handle_chat_stream(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ChatRequest>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("🌊 ストリーム受信: {}", payload.message);

    let (tx, rx) = tokio::sync::mpsc::channel::<String>(100);
    let message = payload.message.clone();

    // Logic 側を非同期タスクとしてバックグラウンドで走らせる
    tokio::spawn(async move {
        match state
            .ollama
            .ask_with_context_stream(&message, tx.clone())
            .await
        {
            Ok(full_response) => {
                // 完了後に記憶を保存
                let _ = insert_memory(&message, 5.0, "", &full_response).await;
            }
            Err(e) => {
                eprintln!("❌ ストリーミング推論エラー: {e}");
                let _ = tx
                    .send("回路がサビすぎて……少し待っていただけますか。".to_string())
                    .await;
            }
        }
    });

    // Receiver を Stream に変換して SSE レスポンスとして返す
    let stream = stream::unfold(rx, |mut rx| async move {
        rx.recv()
            .await
            .map(|chunk| (Ok(Event::default().data(chunk)), rx))
    });

    Sse::new(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check_endpoint() {
        let app = Router::new().route("/health", get(health_check));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn チャットリクエストの構造体が正しくデシリアライズされる() {
        use super::ChatRequest;
        let json = r#"{"message": "こんにちは"}"#;
        let req: ChatRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.message, "こんにちは");
    }

    #[tokio::test]
    async fn test_chat_endpoint_error_handling() {
        use crate::logic::reasoning::OllamaClient;
        use std::sync::Arc;

        if let Err(e) = crate::memory::graph::connect_to_db().await {
            println!("Skipping chat endpoint test because SurrealDB is unavailable: {e}");
            return;
        }

        let client = OllamaClient::new("http://127.0.0.1:1", "dummy_model");
        let state = Arc::new(AppState { ollama: client });
        let app = Router::new()
            .route("/api/chat", post(handle_chat))
            .with_state(state);

        let body = r#"{"message": "テスト"}"#;
        let request = Request::builder()
            .method("POST")
            .uri("/api/chat")
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
