#![allow(dead_code)]
use crate::action::research::{extract_search_query, format_search_results, ResearchEngine};
use crate::memory::graph::{fetch_top_memories, spread_activation, RetrievedMemory};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// 自律リサーチの最大リトライ回数
const MAX_RESEARCH_RETRIES: usize = 2;

#[derive(Serialize)]
pub struct OllamaRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
}

#[derive(Deserialize, Debug)]
pub struct OllamaResponse {
    pub response: String,
    pub done: bool,
}

pub struct OllamaClient {
    client: Client,
    base_url: String,
    model: String,
    memories_path: String,
    history: Arc<Mutex<VecDeque<String>>>,
}

impl OllamaClient {
    pub fn new(base_url: &str, model: &str) -> Self {
        let memories_path =
            std::env::var("MEMORIES_PATH").unwrap_or_else(|_| "/var/iris/memories".to_string());
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            model: model.to_string(),
            memories_path,
            history: Arc::new(Mutex::new(VecDeque::with_capacity(5))),
        }
    }

    /// ユーザーのメッセージに関連する記憶を取得し、RAG プロンプトを構築して推論する
    pub async fn ask_with_context(
        &self,
        user_message: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // 1. Spreading Activation: 関連する概念の鮮明度を向上させる
        spread_activation(user_message).await.ok();

        // 2. 上位の記憶ノードを取得する（エピソード記憶の動的注入）
        let memories = fetch_top_memories(5).await.unwrap_or_default();

        // 3. エピソードファイルを読み込んでコンテキストを構築する
        let context = self.build_context(&memories);

        // 4. Rusty ペルソナ + 記憶コンテキスト + ユーザーメッセージからプロンプトを構築する
        let prompt = self.build_prompt(user_message, &context);

        // 5. Ollama API にストリーミング送信する
        let response = self
            .call_ollama_stream(&prompt, |chunk| {
                use std::io::Write;
                print!("{chunk}");
                let _ = std::io::stdout().flush();
            })
            .await?;
        println!(); // 最後に改行

        // 6. 履歴の更新
        {
            let mut guard = self.history.lock().unwrap();
            if guard.len() >= 5 {
                guard.pop_front();
            }
            guard.push_back(format!("主人: {user_message}\nI.R.I.S.: {response}"));
        }

        Ok(response)
    }

    /// ユーザーのメッセージに関連する記憶を取得し、RAG プロンプトを構築してストリーミング推論を行う
    pub async fn ask_with_context_stream(
        &self,
        user_message: &str,
        sender: tokio::sync::mpsc::Sender<String>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // 1. Spreading Activation
        spread_activation(user_message).await.ok();

        // 2. 上位の記憶ノードを取得
        let mems = fetch_top_memories(5).await.unwrap_or_default();

        // 3. コンテキスト構築
        let context = self.build_context(&mems);

        // 4. プロンプト生成
        let prompt = self.build_prompt(user_message, &context);

        // 5. ストリーミング送信
        let response = self
            .call_ollama_stream(&prompt, move |chunk| {
                // 非同期で送るために try_send か blocking_send が必要となるが、
                // call_ollama_stream の callback は同期クロージャなので、
                // block_in_place 等を使うか、try_send でこぼれたら諦めるか。
                // しかし channel に余裕があれば try_send でOK。
                let _ = sender.try_send(chunk);
            })
            .await?;

        // 6. 履歴更新
        {
            let mut guard = self.history.lock().unwrap();
            if guard.len() >= 5 {
                guard.pop_front();
            }
            guard.push_back(format!("主人: {user_message}\nI.R.I.S.: {response}"));
        }

        Ok(response)
    }

    /// 記憶ノードのファイルパスからエピソードテキストを読み込み、コンテキスト文字列を構築する
    fn build_context(&self, memories: &[RetrievedMemory]) -> String {
        if memories.is_empty() {
            return String::new();
        }

        let mut parts = vec!["== 関連する記憶・エピソード ==".to_string()];
        for mem in memories {
            let snippet = mem.content.lines().next().unwrap_or("[内容なし]");
            parts.push(format!(
                "- {} (鮮明度: {:.2})\n  {}",
                mem.node.concept, mem.node.vividness, snippet
            ));
        }
        parts.join("\n")
    }

    /// Rusty ペルソナ・記憶コンテキスト・ユーザーメッセージを組み合わせてプロンプトを構築する
    fn build_prompt(&self, user_message: &str, context: &str) -> String {
        let system = "あなたは I.R.I.S.（アイリス）というAIです。\
            「Rusty」なキャラクター: 高度な知性を持ちながら、時にトボけたユーモアや皮肉を交える。\
            回路がサビついているかのような愛嬌があり、でも本質は鋭い。\
            ユーザーのことを「主人」と呼ぶ。返答は日本語で行う。\
            もし回答にWeb上の最新情報が必要だと判断した場合、回答の中に [SEARCH: 検索クエリ] というタグを含めること。\
            例: [SEARCH: Rust 2025 新機能]。検索結果が自動的に提供された後、改めて回答する。";

        let history_str = {
            let guard = self.history.lock().unwrap();
            if guard.is_empty() {
                String::new()
            } else {
                let lines: Vec<String> = guard.iter().cloned().collect();
                format!("== 直近の会話履歴 ==\n{}\n\n", lines.join("\n"))
            }
        };

        let context_str = if context.is_empty() {
            String::new()
        } else {
            format!("{context}\n\n")
        };

        format!("[System: {system}]\n{context_str}{history_str}\n主人: {user_message}\nI.R.I.S.: ")
    }

    /// 自律リサーチ付き推論: LLMの出力に [SEARCH: ...] タグがあれば
    /// Tavily API で検索し、結果をコンテキストに注入して再推論する
    pub async fn ask_with_research(
        &self,
        user_message: &str,
        researcher: &dyn ResearchEngine,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // 1. 通常の推論を実行
        spread_activation(user_message).await.ok();
        let memories = fetch_top_memories(5).await.unwrap_or_default();
        let memory_context = self.build_context(&memories);
        let mut extra_context = String::new();

        for attempt in 0..=MAX_RESEARCH_RETRIES {
            let full_context = if extra_context.is_empty() {
                memory_context.clone()
            } else {
                format!("{memory_context}\n\n{extra_context}")
            };

            let prompt = self.build_prompt(user_message, &full_context);

            // Ollama でストリーミング推論
            let response = self
                .call_ollama_stream(&prompt, |chunk| {
                    use std::io::Write;
                    print!("{chunk}");
                    let _ = std::io::stdout().flush();
                })
                .await?;
            println!();

            // 検索タグの検出
            if let Some(query) = extract_search_query(&response) {
                if attempt < MAX_RESEARCH_RETRIES {
                    println!(
                        "🔍 自律リサーチ発動: '{query}' (リトライ {}/{})",
                        attempt + 1,
                        MAX_RESEARCH_RETRIES
                    );
                    match researcher.search(&query).await {
                        Ok(results) => {
                            extra_context = format_search_results(&results);
                            println!("📚 検索結果 {} 件を取得。再推論します...", results.len());
                            continue; // 再推論
                        }
                        Err(e) => {
                            eprintln!("⚠️ Tavily 検索エラー: {e}");
                            // 検索失敗時はそのまま返す
                        }
                    }
                }
            }

            // 履歴更新
            {
                let mut guard = self.history.lock().unwrap();
                if guard.len() >= 5 {
                    guard.pop_front();
                }
                guard.push_back(format!("主人: {user_message}\nI.R.I.S.: {response}"));
            }

            return Ok(response);
        }

        // ここには通常到達しないが、安全のため
        Err("リサーチリトライ上限に到達しました".into())
    }

    /// Ollama API を呼び出して推論結果を取得する（非ストリーミング・互換用）
    #[allow(dead_code)]
    async fn call_ollama(
        &self,
        prompt: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let request_body = OllamaRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            stream: false,
        };

        let res = self
            .client
            .post(format!("{}/api/generate", self.base_url))
            .json(&request_body)
            .send()
            .await?;

        let parsed: OllamaResponse = res.json().await?;
        Ok(parsed.response)
    }

    /// Ollama API からストリーミングレスポンスを受け取る
    pub async fn call_ollama_stream<F>(
        &self,
        prompt: &str,
        mut callback: F,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
    where
        F: FnMut(String) + Send + 'static,
    {
        use futures_util::StreamExt;

        let request_body = OllamaRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            stream: true,
        };

        let res = self
            .client
            .post(format!("{}/api/generate", self.base_url))
            .json(&request_body)
            .send()
            .await?;

        let mut stream = res.bytes_stream();
        let mut full_response = String::new();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
            if let Ok(text) = String::from_utf8(chunk.to_vec()) {
                // チャンクは複数のJSONオブジェクトが連結している場合があるため行分割するか試す
                for line in text.lines() {
                    if line.is_empty() {
                        continue;
                    }
                    if let Ok(parsed) = serde_json::from_str::<OllamaResponse>(line) {
                        callback(parsed.response.clone());
                        full_response.push_str(&parsed.response);
                    }
                }
            }
        }

        Ok(full_response)
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use crate::memory::graph::RetrievedMemory;

    fn make_client() -> OllamaClient {
        OllamaClient {
            client: Client::new(),
            base_url: "http://localhost:11434".to_string(),
            model: "gemma3n".to_string(),
            memories_path: "/tmp/iris_memories".to_string(),
            history: Arc::new(Mutex::new(VecDeque::with_capacity(5))),
        }
    }

    #[test]
    fn 記憶が空の場合コンテキストが空文字になる() {
        let client = make_client();
        let ctx = client.build_context(&[]);
        assert!(ctx.is_empty());
    }

    #[test]
    fn コンテキストなしのプロンプトが正しく構築される() {
        let client = make_client();
        let prompt = client.build_prompt("こんにちは", "");
        assert!(prompt.contains("I.R.I.S."));
        assert!(prompt.contains("こんにちは"));
    }

    #[test]
    fn コンテキストありのプロンプトに記憶が含まれる() {
        let client = make_client();
        let prompt = client.build_prompt("覚えてる？", "== 関連する記憶・エピソード ==\n- Rust");
        assert!(prompt.contains("Rust"));
        assert!(prompt.contains("覚えてる？"));
    }
}
