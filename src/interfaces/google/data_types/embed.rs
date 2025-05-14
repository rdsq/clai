use serde::{Serialize, Deserialize};
use super::message_parts::MessagePartsBullshit;

#[derive(Deserialize)]
pub struct GoogleGenAIEmbedPrediction {
    pub values: Vec<f32>,
}

#[derive(Serialize)]
pub struct GoogleGenAIEmbedItem<'a> {
    pub model: &'a str,
    pub content: MessagePartsBullshit<'a>,
}

impl<'a> GoogleGenAIEmbedItem<'a> {
    pub fn new(model: &'a str, text: &'a str) -> Self {
        Self {
            model,
            content: MessagePartsBullshit::new(text),
        }
    }
}

#[derive(Serialize)]
pub struct GoogleGenAIEmbedRequest<'a> {
    pub model: &'a str,
    pub requests: Vec<GoogleGenAIEmbedItem<'a>>,
}

#[derive(Deserialize)]
pub struct GoogleGenAIEmbedResponse {
    pub embeddings: Vec<GoogleGenAIEmbedPrediction>,
}
