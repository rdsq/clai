use crate::states::AppState;

#[derive(clap::Parser, Debug)]
/// Generate a response
pub struct Generate {
    /// The interface and model
    model: String,
    /// The user message
    prompt: String,
}

pub async fn generate(args: Generate) {
    let mut state = AppState::new(&args.model);
    state.generate_to_output(args.prompt).await;
}
