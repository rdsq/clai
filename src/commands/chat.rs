use crate::states::{AppState, ContextState};

#[derive(clap::Parser, Debug)]
/// Chat with a chatbot
pub struct Chat {
    // The model to chat with
    model: String,
    /// Read and save the chat to a file
    #[arg(short, long, default_value = None)]
    file: Option<String>,
}

pub async fn chat(args: Chat) {
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    let context = ContextState::from_optional_file(&args.file);
    let mut state = AppState::new(context, &args.model);
    loop {
        let prompt = rl.readline("\x1b[36;1m>\x1b[0m ");
        if prompt.is_err() {
            // probably EOF
            break;
        }
        let prompt = prompt.unwrap();
        state.generate_to_output(prompt).await;
    }
    if let Some(path) = args.file {
        state.context.write_to_file(&path);
    }
}
