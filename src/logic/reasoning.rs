use crate::memory::graph::{fetch_top_memories, spread_activation, RetrievedMemory};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

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
        let memories_path = std::env::var("MEMORIES_PATH")
            .unwrap_or_else(|_| "/var/iris/memories".to_string());
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

        // 5. Ollama API に送信する
        let response = self.call_ollama(&prompt).await?;

        // 6. 履歴の更新
        {
            let mut guard = self.history.lock().unwrap();
            if guard.len() >= 5 {
                guard.pop_front();
            }
            guard.push_back(format!("主人: {}\nI.R.I.S.: {}", user_message, response));
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
            ユーザーのことを「主人」と呼ぶ。返答は日本語で行う。";

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
            format!("{}\n\n", context)
        };

        format!(
            "[System: {}]\n{}{}\n主人: {}\nI.R.I.S.: ",
            system, context_str, history_str, user_message
        )
    }

    /// Ollama API を呼び出して推論結果を取得する
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
            .post(&format!("{}/api/generate", self.base_url))
            .json(&request_body)
            .send()
            .await?;

        let parsed: OllamaResponse = res.json().await?;
        Ok(parsed.response)
    }
}

#[cfg(test)]
mod tests {
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
