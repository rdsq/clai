pub mod data_types;
use crate::interfaces::frame;
use crate::states::ContextState;
use crate::markdown::markdown_to_ansi;
use data_types::generate;
mod embeds;

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
}

#[async_trait::async_trait]
impl frame::Interface for GoogleGenAIInterface {
    async fn generate(&self, state: &ContextState, callback: Box<dyn Fn(String) -> () + Send>) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let text = client
            .post(&self.get_endpoint())
            .json(&generate::GoogleGenAIRequest {
                contents: state.chat.iter().map(generate::GoogleGenAIMessage::from).collect(),
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
        let mut res = Vec::new();
        for batch in input.chunks(100) {
            res.extend(self.embeddings_at_most_100(&Vec::from(batch)).await?);
        }
        Ok(res)
    }
}
