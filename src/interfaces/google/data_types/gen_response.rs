use serde::Deserialize;

#[derive(Deserialize)]
pub struct BullshitLayer3 {
    pub text: String,
}

#[derive(Deserialize)]
pub struct BullshitLayer2 {
    parts: Vec<BullshitLayer3>,
}

#[derive(Deserialize)]
pub struct BullshitLayer1 {
    content: BullshitLayer2,
}

#[derive(Deserialize)]
pub struct GoogleGenAIResponse {
    candidates: Vec<BullshitLayer1>,
}

impl GoogleGenAIResponse {
    pub fn get_text(&self) -> String {
        self.candidates[0].content.parts[0].text.clone()
    }
}
