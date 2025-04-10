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

fn print_status(state: &AppState, file: &Option<String>) {
    println!("Model: {}", state.interface.interface.model_id());
    println!("Messages number: {}", state.context.chat.len());
    println!("Autosave file: {}", match file {
        Some(path) => &path,
        None => "[not set]",
    });
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
            UserActions::Help => print!(include_str!("../help-interactive.txt")),
            UserActions::PromptFromFile(path) => state.generate_to_output(
                std::fs::read_to_string(path)
                .unwrap_or_else(|e| {
                    eprintln!("{}", e);
                    std::process::exit(1);
                })
            ).await,
            UserActions::Status => print_status(&state, &file),
        }
    }
    if let Some(path) = file {
        state.context.write_to_file(&path);
    }
}
