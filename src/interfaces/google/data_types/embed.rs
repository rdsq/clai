use serde::{Serialize, Deserialize};
use super::MessageParts;

#[derive(Deserialize)]
pub struct GoogleGenAIEmbedPrediction {
    pub values: Vec<f32>,
}

#[derive(Serialize)]
pub struct GoogleGenAIEmbedBullshit<'a> {
    pub parts: Vec<MessageParts<'a>>,
}

#[derive(Serialize)]
pub struct GoogleGenAIEmbedItem<'a> {
    pub model: String,
    pub content: GoogleGenAIEmbedBullshit<'a>,
}

impl<'a> GoogleGenAIEmbedItem<'a> {
    pub fn new(model: String, text: &'a str) -> Self {
        Self {
            model,
            content: GoogleGenAIEmbedBullshit {
                parts: vec![MessageParts { text }],
            }
        }
    }
}

#[derive(Serialize)]
pub struct GoogleGenAIEmbedRequest<'a> {
    pub model: String,
    pub requests: Vec<GoogleGenAIEmbedItem<'a>>,
}

#[derive(Deserialize)]
pub struct GoogleGenAIEmbedResponse {
    pub embeddings: Vec<GoogleGenAIEmbedPrediction>,
}
