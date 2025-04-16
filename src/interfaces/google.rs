use serde::{Serialize, Deserialize};
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
    fn get_embed_endpoint(&self) -> String {
        format!("https://generativelanguage.googleapis.com/v1beta/models/{}:batchEmbedContents?key={}", self.model, self.api_key)
    }
    fn model_bullshit(&self) -> String {
        format!("models/{}", self.model)
    }
}

#[derive(Serialize)]
struct MessageParts<'a> {
    text: &'a str,
}

#[derive(Serialize)]
struct GoogleGenAIMessage<'a> {
    role: String,
    parts: Vec<MessageParts<'a>>,
}

fn prepare_chat(chat: &Vec<messages::Message>) -> Vec<GoogleGenAIMessage> {
    return chat.into_iter().map(|msg| GoogleGenAIMessage {
        role: match msg.role {
            messages::Role::User => "user".to_string(),
            messages::Role::Model => "model".to_string(),
        },
        parts: vec![MessageParts { text: &msg.text }],
    }).collect();
}

#[derive(Serialize)]
struct GoogleGenAIRequest<'a> {
    contents: Vec<GoogleGenAIMessage<'a>>,
}

#[derive(Deserialize)]
struct GoogleGenAIEmbedPrediction {
    values: Vec<f32>,
}

#[derive(Serialize)]
struct GoogleGenAIEmbedBullshit<'a> {
    parts: Vec<MessageParts<'a>>,
}

#[derive(Serialize)]
struct GoogleGenAIEmbedItem<'a> {
    model: String,
    content: GoogleGenAIEmbedBullshit<'a>,
}

#[derive(Serialize)]
struct GoogleGenAIEmbedRequest<'a> {
    model: String,
    requests: Vec<GoogleGenAIEmbedItem<'a>>,
}

#[derive(Deserialize)]
struct GoogleGenAIEmbedResponse {
    embeddings: Vec<GoogleGenAIEmbedPrediction>,
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
    async fn embeddings(&self, input: &Vec<String>) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let text = client
            .post(&self.get_embed_endpoint())
            .json(&GoogleGenAIEmbedRequest {
                model: self.model_bullshit(),
                requests: input.iter().map(|v| GoogleGenAIEmbedItem {
                    content: GoogleGenAIEmbedBullshit {
                        parts: vec![MessageParts {
                            text: v,
                        }],
                    },
                    model: self.model_bullshit(),
                }).collect(),
            })
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        let obj: GoogleGenAIEmbedResponse = serde_json::from_str(&text)?;
        Ok(obj.embeddings.iter()
            .map(|v| v.values.to_owned())
            .collect())
    }
}
