use super::data_types::embed::{GoogleGenAIEmbedRequest, GoogleGenAIEmbedResponse, GoogleGenAIEmbedItem};

impl super::GoogleGenAIInterface {
    fn model_bullshit(&self) -> String {
        format!("models/{}", self.model)
    }
    pub async fn embeddings_at_most_100(&self, input: &Vec<String>) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let text = client
            .post(&self.get_embed_endpoint())
            .json(&GoogleGenAIEmbedRequest {
                model: self.model_bullshit(),
                requests: input.iter().map(|v| GoogleGenAIEmbedItem::new(self.model_bullshit(), v)).collect(),
            })
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        let obj: GoogleGenAIEmbedResponse = serde_json::from_str(&text)?;
        Ok(obj.embeddings.iter()
            .map(|v| v.values.to_owned())
            .collect())
    }
}
