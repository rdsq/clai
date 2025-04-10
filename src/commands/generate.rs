use crate::states::{AppState, ContextState};

#[derive(clap::Parser, Debug)]
/// Generate a response
pub struct Generate {
    /// The interface and model
    model: String,
    /// The user message
    prompt: String,
}

pub async fn generate(args: Generate) {
    let context = ContextState::new();
    let mut state = AppState::new(context, &args.model);
    state.generate_to_output(args.prompt).await;
}
