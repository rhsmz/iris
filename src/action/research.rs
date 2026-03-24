#![allow(dead_code)]
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct TavilySearchRequest {
    pub api_key: String,
    pub query: String,
    pub search_depth: String,
}

#[derive(Deserialize, Debug)]
pub struct TavilySearchResponse {
    pub results: Vec<TavilyResult>,
}

#[derive(Deserialize, Debug)]
pub struct TavilyResult {
    pub title: String,
    pub url: String,
    pub content: String,
}

pub struct TavilyClient {
    client: Client,
    api_key: String,
}

impl TavilyClient {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
        }
    }

    pub async fn search(&self, query: &str) -> Result<Vec<TavilyResult>, Box<dyn std::error::Error + Send + Sync>> {
        let req = TavilySearchRequest {
            api_key: self.api_key.clone(),
            query: query.to_string(),
            search_depth: "basic".to_string(),
        };

        let res = self.client.post("https://api.tavily.com/search")
            .json(&req)
            .send()
            .await?;

        let parsed: TavilySearchResponse = res.json().await?;
        Ok(parsed.results)
    }
}
