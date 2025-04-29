use serde::Serialize;
use super::message_parts::MessageParts;
use crate::states::messages;

#[derive(Serialize)]
pub struct GoogleGenAIMessage<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    pub parts: Vec<MessageParts<'a>>,
}

impl<'a> GoogleGenAIMessage<'a> {
    pub fn new(role: Option<String>, text: &'a str) -> Self {
        Self { role, parts: vec![MessageParts { text }] }
    }
    pub fn from(message: &'a messages::Message) -> Self {
        Self::new(
            Some(match message.role {
                messages::Role::User => "user".to_string(),
                messages::Role::Model => "model".to_string(),
            }),
            &message.text,
        )
    }
}

#[derive(Serialize)]
pub struct GoogleGenAIRequest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<GoogleGenAIMessage<'a>>,
    pub contents: Vec<GoogleGenAIMessage<'a>>,
}

