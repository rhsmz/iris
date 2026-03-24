use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Tavily API への検索リクエスト
#[derive(Serialize)]
pub struct TavilySearchRequest {
    pub api_key: String,
    pub query: String,
    pub search_depth: String,
}

/// Tavily API からの検索レスポンス
#[derive(Deserialize, Debug)]
pub struct TavilySearchResponse {
    pub results: Vec<TavilyResult>,
}

/// Tavily 検索結果の個別アイテム
#[derive(Deserialize, Debug, Clone)]
pub struct TavilyResult {
    pub title: String,
    pub url: String,
    pub content: String,
}

/// Tavily API クライアント（Trait ベースでモック可能）
#[async_trait::async_trait]
pub trait ResearchEngine: Send + Sync {
    async fn search(
        &self,
        query: &str,
    ) -> Result<Vec<TavilyResult>, Box<dyn std::error::Error + Send + Sync>>;
}

/// 本番用 Tavily クライアント
pub struct TavilyClient {
    client: Client,
    api_key: String,
}

impl TavilyClient {
    /// 環境変数 `TAVILY_API_KEY` から API キーを読み込んで初期化する
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let api_key = std::env::var("TAVILY_API_KEY")
            .map_err(|_| "環境変数 TAVILY_API_KEY が設定されていません")?;
        Ok(Self {
            client: Client::new(),
            api_key,
        })
    }

    /// API キーを直接指定して初期化する
    pub fn new(api_key: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
        }
    }
}

#[async_trait::async_trait]
impl ResearchEngine for TavilyClient {
    async fn search(
        &self,
        query: &str,
    ) -> Result<Vec<TavilyResult>, Box<dyn std::error::Error + Send + Sync>> {
        let req = TavilySearchRequest {
            api_key: self.api_key.clone(),
            query: query.to_string(),
            search_depth: "basic".to_string(),
        };

        let res = self
            .client
            .post("https://api.tavily.com/search")
            .json(&req)
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            return Err(format!("Tavily API エラー (HTTP {status}): {body}").into());
        }

        let parsed: TavilySearchResponse = res.json().await?;
        Ok(parsed.results)
    }
}

/// Tavily 検索結果をRAGコンテキスト文字列に整形する
pub fn format_search_results(results: &[TavilyResult]) -> String {
    if results.is_empty() {
        return "検索結果はありませんでした。".to_string();
    }

    let mut parts = vec!["== Web検索結果 ==".to_string()];
    for (i, r) in results.iter().enumerate().take(5) {
        parts.push(format!(
            "{}. {} ({})\n   {}",
            i + 1,
            r.title,
            r.url,
            // 長すぎるコンテンツは300文字で切る
            if r.content.len() > 300 {
                let mut end = 300;
                while !r.content.is_char_boundary(end) && end > 0 {
                    end -= 1;
                }
                format!("{}...", &r.content[..end])
            } else {
                r.content.clone()
            }
        ));
    }
    parts.join("\n")
}

/// Ollama の出力から `[SEARCH: クエリ]` タグを抽出する
/// 複数のタグがあれば最初の1つだけを返す
pub fn extract_search_query(text: &str) -> Option<String> {
    // [SEARCH: ...] パターンを手動で検索（regex クレート不要）
    let start_tag = "[SEARCH:";
    let start_pos = text.find(start_tag)?;
    let after_tag = &text[start_pos + start_tag.len()..];
    let end_pos = after_tag.find(']')?;
    let query = after_tag[..end_pos].trim();
    if query.is_empty() {
        None
    } else {
        Some(query.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 検索タグが正しく抽出される() {
        let text = "うーん、それについては調べてみましょうか。[SEARCH: Rust 非同期処理 tokio]それでは結果をお伝えします。";
        let result = extract_search_query(text);
        assert_eq!(result, Some("Rust 非同期処理 tokio".to_string()));
    }

    #[test]
    fn 検索タグがない場合はnoneが返る() {
        let text = "普通の回答です。検索の必要はありません。";
        assert_eq!(extract_search_query(text), None);
    }

    #[test]
    fn 空の検索タグはnoneが返る() {
        let text = "空のタグです。[SEARCH: ]何もなし。";
        assert_eq!(extract_search_query(text), None);
    }

    #[test]
    fn 検索結果のフォーマットが正しく生成される() {
        let results = vec![TavilyResult {
            title: "Rust公式".to_string(),
            url: "https://www.rust-lang.org/".to_string(),
            content: "Rustは安全性と速度を両立するプログラミング言語です。".to_string(),
        }];
        let formatted = format_search_results(&results);
        assert!(formatted.contains("Rust公式"));
        assert!(formatted.contains("Web検索結果"));
    }

    #[test]
    fn 空の検索結果でもパニックしない() {
        let formatted = format_search_results(&[]);
        assert!(formatted.contains("検索結果はありませんでした"));
    }
}
