use serde::Serialize;

#[derive(Serialize)]
pub struct MessageParts<'a> {
    pub text: &'a str,
}

#[derive(Serialize)]
pub struct MessagePartsBullshit<'a> {
    parts: Vec<MessageParts<'a>>,
}

impl<'a> MessagePartsBullshit<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { parts: vec![MessageParts { text }] }
    }
}
