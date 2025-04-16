use serde::Serialize;

#[derive(Serialize)]
pub struct MessageParts<'a> {
    pub text: &'a str,
}

pub mod generate;
pub mod embed;
