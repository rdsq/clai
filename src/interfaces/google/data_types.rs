use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct MessageParts<'a> {
    pub text: &'a str,
}

#[derive(Serialize)]
pub struct GoogleGenAIMessage<'a> {
    pub role: String,
    pub parts: Vec<MessageParts<'a>>,
}

#[derive(Serialize)]
pub struct GoogleGenAIRequest<'a> {
    pub contents: Vec<GoogleGenAIMessage<'a>>,
}

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

#[derive(Serialize)]
pub struct GoogleGenAIEmbedRequest<'a> {
    pub model: String,
    pub requests: Vec<GoogleGenAIEmbedItem<'a>>,
}

#[derive(Deserialize)]
pub struct GoogleGenAIEmbedResponse {
    pub embeddings: Vec<GoogleGenAIEmbedPrediction>,
}
