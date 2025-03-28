use async_trait::async_trait;

#[async_trait]
pub trait Interface {
    async fn generate(&self, state: &crate::app_state::AppState, callback: Box<dyn Fn(String) -> () + Send>) -> Result<String, Box<dyn std::error::Error>>;
}
