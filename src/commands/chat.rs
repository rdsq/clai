use crate::states::AppState;
use crate::prompt::{prompt, UserActions};

#[derive(clap::Parser, Debug)]
/// Chat with a chatbot
pub struct Chat {
    // The model to chat with
    model: String,
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

fn print_status(state: &AppState) {
    println!("Model: {}", state.interface.interface.model_id());
    println!("Messages number: {}", state.context.chat.len());
    println!("Autosave file: {}", match &state.autosave {
        Some(path) => &path,
        None => "[not set]",
    });
    if let Some(system) = &state.context.system {
        println!("System prompt: {}", system);
    }
    if !state.context.parameters.is_empty() {
        println!("Parameters:");
        for (key, value) in &state.context.parameters {
            println!("  {}: {:?}", key, value);
        }
    }
}

pub async fn chat(args: Chat) {
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    let mut state = AppState::new(args.file, &args.model);
    state.context.system = args.system;
    if let Some(parameters) = args.parameters {
        state.context.parameters = serde_json::from_str(&parameters)
            .unwrap_or_else(|err| {
                eprintln!("Error while parsing JSON: {}", err);
                std::process::exit(1);
            });
    }
    loop {
        match prompt(&mut rl) {
            UserActions::Prompt(prompt) => state.generate_to_output(prompt).await,
            UserActions::Exit => break,
            UserActions::SetModel(model) => state.set_interface(&model),
            UserActions::Save(path) => state.context.write_to_file(&path),
            UserActions::SetFile(path) => state.autosave = if path.is_empty() { None } else { Some(path) },
            UserActions::Help => print!(include_str!("../help-interactive.txt")),
            UserActions::PromptFromFile(path) => state.generate_to_output(
                std::fs::read_to_string(path)
                .unwrap_or_else(|e| {
                    eprintln!("{}", e);
                    std::process::exit(1);
                })
            ).await,
            UserActions::Status => print_status(&state),
            UserActions::DeleteLast => {
                state.context.chat.pop(); // model response
                state.context.chat.pop(); // user prompt
                state.try_autosave();
            },
            UserActions::Rewind(num) => state.context.rewind(&num),
            UserActions::SetSystemPrompt(system) => state.context.system = system,
            UserActions::None => {},
            UserActions::SetParameter(key, value) => { state.context.parameters.insert(key, value); },
            UserActions::UnsetParameter(key) => { state.context.parameters.remove(&key); },
        }
    }
}
