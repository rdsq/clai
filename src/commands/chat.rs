use crate::states::{AppState, ContextState};
use crate::prompt::{prompt, UserActions};

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
    let mut file: Option<String> = args.file;
    let context = ContextState::from_optional_file(&file);
    let mut state = AppState::new(context, &args.model);
    loop {
        match prompt(&mut rl) {
            UserActions::Prompt(prompt) => state.generate_to_output(prompt).await,
            UserActions::Exit => break,
            UserActions::SetModel(model) => state.set_interface(&model),
            UserActions::Save(path) => state.context.write_to_file(&path),
            UserActions::SetFile(path) => file = if path.is_empty() { None } else { Some(path) },
        }
    }
    if let Some(path) = file {
        state.context.write_to_file(&path);
    }
}
