use super::data_types::embed::{
    GoogleGenAIEmbedRequest,
    GoogleGenAIEmbedResponse,
    GoogleGenAIEmbedItem,
};

impl super::GoogleGenAIInterface {
    pub async fn embeddings_at_most_100(&self, input: &Vec<String>) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {
        let model_bullshit = format!("models/{}", self.model);
        let client = reqwest::Client::new();
        let res = client
            .post(&self.get_embed_endpoint())
            .json(&GoogleGenAIEmbedRequest {
                model: &model_bullshit,
                requests: input.iter().map(|v| GoogleGenAIEmbedItem::new(&model_bullshit, v)).collect(),
            })
            .send()
            .await?;
        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await?;
            return Err(format!("Error {}: {}", status, text).into());
        }
        let text = res.text().await?;
        let obj: GoogleGenAIEmbedResponse = serde_json::from_str(&text)?;
        Ok(obj.embeddings.iter()
            .map(|v| v.values.to_owned())
            .collect())
    }
}
