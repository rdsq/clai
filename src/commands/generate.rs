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
    /// Parameters for the model as JSON
    #[arg(short, long, default_value = None)]
    parameters: Option<String>,
}

pub async fn generate(args: Generate) {
    let mut state = AppState::new(args.file, &args.model);
    state.context.system = args.system;
    if let Some(parameters) = args.parameters {
        state.context.parameters = serde_json::from_str(&parameters)
            .unwrap_or_else(|err| {
                eprintln!("Error while parsing JSON: {}", err);
                std::process::exit(1);
            });
    }
    state.generate_to_output(args.prompt).await;
}
