use crate::states::AppState;

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
    /// System prompt
    #[arg(short, long, default_value = None)]
    system: Option<String>,
}

pub async fn generate(args: Generate) {
    let mut state = AppState::new(args.file, &args.model);
    state.context.system = args.system;
    state.generate_to_output(args.prompt).await;
}
