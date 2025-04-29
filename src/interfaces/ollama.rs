use serde::{Serialize, Deserialize};
use crate::interfaces::frame;
use futures_util::StreamExt;
use crate::states::{messages, ContextState};

pub struct OllamaInterface {
    pub model: String,
}

impl OllamaInterface {
    pub fn new(model: String) -> Self {
        Self { model }
    }
}

#[derive(Serialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

fn prepare_chat(state: &ContextState) -> Vec<OllamaMessage> {
    let mut messages: Vec<OllamaMessage> = Vec::new();
    let system_prompt = &state.system;
    if let Some(system) = system_prompt {
        messages.insert(0, OllamaMessage {
            role: "system".to_string(),
            content: system.to_string(),
        });
    }
    for msg in &state.chat {
        messages.push(OllamaMessage {
            role: match msg.role {
                messages::Role::User => "user".to_string(),
                messages::Role::Model => "assistant".to_string(),
            },
            content: msg.text.clone(),
        });
    }
    messages
}

#[derive(Serialize)]
struct OllamaRequest {
    messages: Vec<OllamaMessage>,
    model: String,
    stream: bool,
}

#[derive(Serialize)]
struct OllamaEmbeddingRequest<'a> {
    model: &'a str,
    input: &'a Vec<&'a str>,
}

#[derive(Deserialize)]
struct OllamaEmbeddingResponse {
    embeddings: Vec<Vec<f32>>,
}

#[async_trait::async_trait]
impl frame::Interface for OllamaInterface {
    async fn generate(&self, state: &ContextState, callback: Box<dyn Fn(String) -> () + Send>) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let res = client
            .post("http://localhost:11434/api/chat")
            .json(&OllamaRequest {
                messages: prepare_chat(&state),
                model: self.model.clone(),
                stream: true,
            })
            .send()
            .await?;
        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await?;
            return Err(format!("Error {}: {}", status, text).into());
        }
        let mut stream = res.bytes_stream();
        let mut full = String::new();
        while let Some(chunk_bytes_unknown) = stream.next().await {
            let chunk_bytes = chunk_bytes_unknown?;
            let obj: serde_json::Value = serde_json::from_slice(&chunk_bytes)?;
            if let Some(message_chunk) = obj.get("message").and_then(|d| d.get("content")).and_then(|v| v.as_str()) {
                full.push_str(&message_chunk);
                callback(message_chunk.to_string());
            }
        }
        Ok(full)
    }
    fn model_id(&self) -> String {
        format!("ollama:{}", self.model)
    }
    async fn embeddings(&self, input: &Vec<String>) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let res = client
            .post("http://localhost:11434/api/embed")
            .json(&OllamaEmbeddingRequest {
                model: &self.model,
                input: &input.into_iter().map(|v| if v.is_empty() {
                    " " // non empty placeholder to not mess up the order
                } else { v }).collect(),
            })
            .send()
            .await?;
        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await?;
            return Err(format!("Error {}: {}", status, text).into());
        }
        let obj: OllamaEmbeddingResponse = serde_json::from_str(&res.text().await?)?;
        Ok(obj.embeddings)
    }
}
