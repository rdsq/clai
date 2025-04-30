use serde::Serialize;
use super::message_parts::{MessageParts, ImageInlineData};
use crate::states::messages;
use base64::prelude::*;

#[derive(Serialize)]
pub struct GoogleGenAIMessage<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<&'a str>,
    pub parts: Vec<MessageParts<'a>>,
}

impl<'a> GoogleGenAIMessage<'a> {
    pub fn new(role: Option<&'a str>, text: &'a str, images: Vec<(&Vec<u8>, &'a str)>) -> Self {
        let mut parts = vec![MessageParts::Text { text }];
        parts.extend(images.iter().map(|img| MessageParts::Image{
            inline_data: ImageInlineData { data: BASE64_STANDARD.encode(img.0), mime_type: img.1 },
        }));
        Self { role, parts }
    }
    pub fn from(message: &'a messages::Message) -> Self {
        Self::new(
            Some(match message.role {
                messages::Role::User => "user",
                messages::Role::Model => "model",
            }),
            &message.text,
            message.media.iter().map(|media| match media {
                messages::Media::Image { data, mime } => (data, mime as &str),
            }).collect(),
        )
    }
}

#[derive(Serialize)]
pub struct GoogleGenAIRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<GoogleGenAIMessage<'a>>,
    pub contents: Vec<GoogleGenAIMessage<'a>>,
    #[serde(rename = "generationConfig")]
    pub generation_config: &'a std::collections::HashMap<String, serde_json::Value>,
}
