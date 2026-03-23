use serde::{Deserialize, Serialize};
use reqwest::Client;

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
}

impl OllamaClient {
    pub fn new(base_url: &str, model: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            model: model.to_string(),
        }
    }

    pub async fn ask_gemma(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let request_body = OllamaRequest {
            model: self.model.clone(),
            prompt: format!("(System: You are I.R.I.S., a witty, Rusty AI.)\nUser: {}", prompt),
            stream: false,
        };

        let res = self.client.post(&format!("{}/api/generate", self.base_url))
            .json(&request_body)
            .send()
            .await?;

        let parsed: OllamaResponse = res.json().await?;
        Ok(parsed.response)
    }
}
