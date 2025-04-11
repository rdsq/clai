use serde::Serialize;
use crate::interfaces::frame;
use crate::states::{messages, ContextState};
use crate::markdown::markdown_to_ansi;

pub struct GoogleGenAIInterface {
    pub model: String,
    api_key: String,
}

impl GoogleGenAIInterface {
    pub fn new(model: String) -> Result<Self, String> {
        let api_key = std::env::var("GOOGLE_GENAI_API_KEY");
        if let Err(err) = api_key {
            return Err(err.to_string());
        }
        Ok(Self {
            model,
            api_key: api_key.unwrap(),
        })
    }
    fn get_endpoint(&self) -> String {
        format!("https://generativelanguage.googleapis.com/v1/models/{}:generateContent?key={}", self.model, self.api_key)
    }
}

#[derive(Serialize)]
struct MessageParts {
    text: String,
}

#[derive(Serialize)]
struct GoogleGenAIMessage {
    role: String,
    parts: Vec<MessageParts>,
}

fn prepare_chat(chat: &Vec<messages::Message>) -> Vec<GoogleGenAIMessage> {
    return chat.into_iter().map(|msg| GoogleGenAIMessage {
        role: match msg.role {
            messages::Role::User => "user".to_string(),
            messages::Role::Model => "model".to_string(),
        },
        parts: vec![MessageParts { text: msg.text.clone() }],
    }).collect();
}

#[derive(Serialize)]
struct GoogleGenAIRequest {
    contents: Vec<GoogleGenAIMessage>,
}

#[async_trait::async_trait]
impl frame::Interface for GoogleGenAIInterface {
    async fn generate(&self, state: &ContextState, callback: Box<dyn Fn(String) -> () + Send>) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let text = client
            .post(&self.get_endpoint())
            .json(&GoogleGenAIRequest {
                contents: prepare_chat(&state.chat),
            })
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        let obj: serde_json::Value = serde_json::from_str(&text)?;
        if let Some(message) = obj
            .get("candidates")
            .and_then(|v| v.get(0))
            .and_then(|v| v.get("content"))
            .and_then(|v| v.get("parts"))
            .and_then(|v| v.get(0))
            .and_then(|v| v.get("text"))
            .and_then(|v| v.as_str()) {
            callback(markdown_to_ansi(message));
            return Ok(message.to_string());
        } else {
            return Err("unknown response format".into());
        }
    }
    fn model_id(&self) -> String {
        format!("google:{}", self.model)
    }
}
