pub mod data_types;
use crate::interfaces::frame;
use crate::states::ContextState;
use data_types::generate;
mod embeds;
use futures_util::StreamExt;

pub struct GoogleGenAIInterface {
    pub model: String,
    api_key: String,
}

impl GoogleGenAIInterface {
    pub fn new(model: String) -> Result<Self, String> {
        let api_key = std::env::var("GOOGLE_GENAI_API_KEY");
        if let Err(err) = api_key {
            return Err(format!("GOOGLE_GENAI_API_KEY: {}", err));
        }
        Ok(Self {
            model,
            api_key: api_key.unwrap(),
        })
    }
    fn get_endpoint(&self) -> String {
        format!("https://generativelanguage.googleapis.com/v1beta/models/{}:streamGenerateContent?alt=sse&key={}", self.model, self.api_key)
    }
    fn get_embed_endpoint(&self) -> String {
        format!("https://generativelanguage.googleapis.com/v1beta/models/{}:batchEmbedContents?key={}", self.model, self.api_key)
    }
}

#[async_trait::async_trait]
impl frame::Interface for GoogleGenAIInterface {
    async fn generate(&self, state: &ContextState, callback: Box<dyn Fn(String) -> () + Send>) -> Result<String, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let system_prompt = &state.system;
        let generation_config = &state.parameters;
        let request = generate::GoogleGenAIRequest {
            system_instruction: match system_prompt {
                Some(system) => Some(generate::GoogleGenAIMessage::new(None, &system)),
                None => None,
            },
            contents: state.chat.iter().map(generate::GoogleGenAIMessage::from).collect(),
            generation_config,
        };
        let res = client
            .post(&self.get_endpoint())
            .json(&request)
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
            let chunk_bytes_weird = chunk_bytes_unknown?;
            let chunk_bytes = chunk_bytes_weird.strip_prefix(b"data: ").unwrap_or(&chunk_bytes_weird);
            let obj: data_types::gen_response::GoogleGenAIResponse = serde_json::from_slice(&chunk_bytes)?;
            let text = obj.get_text();
            full.push_str(&text);
            callback(text);
        }
        Ok(full)
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
