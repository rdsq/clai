use serde::Serialize;

#[derive(Serialize)]
pub struct ImageInlineData<'a> {
    pub mime_type: &'a str,
    pub data: String,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum MessageParts<'a> {
    Text { text: &'a str },
    Image { inline_data: ImageInlineData<'a> },
}

#[derive(Serialize)]
pub struct MessagePartsBullshit<'a> {
    parts: Vec<MessageParts<'a>>,
}

impl<'a> MessagePartsBullshit<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { parts: vec![MessageParts::Text { text }] }
    }
}
