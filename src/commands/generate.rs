use crate::states::{AppState, ContextState};

#[derive(clap::Parser, Debug)]
/// Generate a response
pub struct Generate {
    /// The interface and model
    model: String,
    /// The user message
    prompt: String,
    /// Read and save the chat to a file
    #[arg(short, long, default_value = None)]
    file: Option<String>,
}

pub async fn generate(args: Generate) {
    let context = match &args.file {
        Some(path) => ContextState::from_file(&path, true),
        None => ContextState::new()
    };
    let mut state = AppState::new(context, &args.model);
    state.generate_to_output(args.prompt).await;
    if let Some(path) = args.file {
        state.context.write_to_file(&path);
    }
}
