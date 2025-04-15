use async_trait::async_trait;

#[async_trait]
pub trait Interface {
    async fn generate(&self, state: &crate::states::ContextState, callback: Box<dyn Fn(String) -> () + Send>) -> Result<String, Box<dyn std::error::Error>>;
    fn model_id(&self) -> String;
    async fn embeddings(&self, input: &Vec<String>) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>>;
}
