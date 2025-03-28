use async_trait::async_trait;

pub type Callback = Box<dyn FnMut(&str) -> ()>;

#[async_trait]
pub trait Interface {
    async fn generate(&self, state: &crate::app_state::AppState, callback: Callback) -> Result<(), Box<dyn std::error::Error>>;
}
