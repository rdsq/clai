use serde::Serialize;
use super::MessageParts;
use crate::states::messages;

#[derive(Serialize)]
pub struct GoogleGenAIMessage<'a> {
    pub role: String,
    pub parts: Vec<MessageParts<'a>>,
}

impl<'a> GoogleGenAIMessage<'a> {
    pub fn new(role: String, text: &'a str) -> Self {
        Self { role, parts: vec![MessageParts { text }] }
    }
    pub fn from(message: &'a messages::Message) -> Self {
        Self::new(
            match message.role {
                messages::Role::User => "user".to_string(),
                messages::Role::Model => "model".to_string(),
            },
            &message.text,
        )
    }
}

#[derive(Serialize)]
pub struct GoogleGenAIRequest<'a> {
    pub contents: Vec<GoogleGenAIMessage<'a>>,
}

