use crate::logic::reasoning::OllamaClient;
use crate::memory::graph::{connect_to_db, decay_vividness};
use crate::sense::server::{start_server, AppState};
use std::sync::Arc;
use tokio::time::{interval, Duration};

mod action;
mod logic;
mod memory;
mod sense;

#[tokio::main]
async fn main() {
    println!("🌈 Project I.R.I.S. を起動しています...");

    // 1. 環境変数の読み込み
    let ollama_url =
        std::env::var("OLLAMA_URL").unwrap_or_else(|_| "http://localhost:11434".to_string());
    let ollama_model = std::env::var("OLLAMA_MODEL").unwrap_or_else(|_| "gemma3n".to_string());

    // 2. Memory 層の初期化（SurrealDB 接続・人格ノード配置）
    println!("🧠 Memory 層を初期化中...");
    if let Err(e) = connect_to_db().await {
        eprintln!("❌ SurrealDB 接続エラー: {e}. インメモリフォールバックを試みます。");
    } else {
        println!("✅ SurrealDB 接続完了");
    }

    // 3. Logic 層の初期化（Ollama クライアント）
    println!("⚙️  Logic 層 (Ollama: {ollama_model}) を初期化中...");
    let ollama_client = OllamaClient::new(&ollama_url, &ollama_model);
    println!("✅ Ollama クライアント準備完了");

    // 4. 共有アプリケーション状態を構築する
    let state = Arc::new(AppState {
        ollama: ollama_client,
    });

    // 5. 記憶の自動風化バッチを別タスクで起動する（1時間ごとに5%減衰）
    tokio::spawn(async {
        let mut ticker = interval(Duration::from_secs(3600));
        loop {
            ticker.tick().await;
            if let Err(e) = decay_vividness().await {
                eprintln!("⚠️ 記憶風化バッチエラー: {e}");
            }
        }
    });

    // 6. Sense 層（Axum サーバー）を起動する
    println!("🌐 Sense 層 (Axum サーバー) を起動中...");
    tokio::select! {
        _ = start_server(state) => {},
        _ = tokio::signal::ctrl_c() => {
            println!("\n🔌 シャットダウン信号を受信しました。おやすみなさい、主人。");
        }
    }
}
