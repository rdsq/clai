use async_trait::async_trait;

#[async_trait]
trait Interface {
    async fn generate(&self, &state: crate::app_state::AppState, callback: F) -> ()
    where
        F: Fn(chunk: &str) -> Result<(), Box<dyn std::error::Error>>,
}
