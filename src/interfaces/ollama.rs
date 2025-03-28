use serde::Serialize;
use crate::interfaces::frame;
use futures_util::StreamExt;

pub struct OllamaInterface {
    pub model: String,
}

#[derive(Serialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

fn prepare_chat(chat: &Vec<crate::app_state::Message>) -> Vec<OllamaMessage> {
    let mut new_chat: Vec<OllamaMessage> = Vec::new();
    for msg in chat {
        match msg.role {
            crate::app_state::Role::User => {
                new_chat.push(OllamaMessage {
                    role: String::from("user"),
                    content: msg.text.clone(),
                });
            }
            crate::app_state::Role::Model => {
                new_chat.push(OllamaMessage {
                    role: String::from("assistant"),
                    content: msg.text.clone(),
                });
            }
        }
    }
    new_chat
}

#[derive(Serialize)]
struct OllamaRequest {
    messages: Vec<OllamaMessage>,
    model: String,
    stream: bool,
}

#[async_trait::async_trait]
impl frame::Interface for OllamaInterface {
    async fn generate(&self, state: &crate::app_state::AppState, callback: Box<dyn Fn(String) -> () + Send>) -> Result<String, Box<dyn std::error::Error>> {
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
            let chunk = std::str::from_utf8(&chunk_bytes)?.to_string();
            full.push_str(&chunk);
            callback(chunk);
        }
        Ok(full)
    }
}
