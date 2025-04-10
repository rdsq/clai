use serde::Serialize;
use crate::interfaces::frame;
use futures_util::StreamExt;
use crate::states::context;

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

fn prepare_chat(chat: &Vec<context::Message>) -> Vec<OllamaMessage> {
    return chat.into_iter().map(|msg| OllamaMessage {
        role: match msg.role {
            context::Role::User => "user".to_string(),
            context::Role::Model => "assistant".to_string(),
        },
        content: msg.text.clone(),
    }).collect();
}

#[derive(Serialize)]
struct OllamaRequest {
    messages: Vec<OllamaMessage>,
    model: String,
    stream: bool,
}

#[async_trait::async_trait]
impl frame::Interface for OllamaInterface {
    async fn generate(&self, state: &context::ContextState, callback: Box<dyn Fn(String) -> () + Send>) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let res = client
            .post("http://localhost:11434/api/chat")
            .json(&OllamaRequest {
                messages: prepare_chat(&state.chat),
                model: self.model.clone(),
                stream: true,
            })
            .send()
            .await?
            .error_for_status()?;
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
}
