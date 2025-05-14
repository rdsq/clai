use crate::states::{AppState, messages};
use crate::prompt::{prompt, UserActions};
use crate::recovery_prompt::{recovery_prompt, RecoveryAction};

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
    /// Initial prompt to start the chat from
    #[arg(long, default_value = None)]
    prompt: Option<String>,
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
    if let Some(initial_prompt) = args.prompt {
        state.context.chat.push(messages::Message {
            text: initial_prompt,
            role: messages::Role::User,
            media: vec![],
        });
    }
    loop {
        if state.context.chat.last().and_then(|msg| Some(&msg.role)) == Some(&messages::Role::User) {
            let res = state.generate_to_output().await;
            if let Err(err) = res {
                eprintln!("{}", err);
                match recovery_prompt(&mut rl) {
                    RecoveryAction::Retry => continue,
                    RecoveryAction::Discard => {
                        state.context.chat.pop();
                        state.try_autosave();
                    },
                    RecoveryAction::Exit => break,
                    RecoveryAction::ChangeModel(model) => state.set_interface(&model)
                        .unwrap_or_else(|err| {
                            eprintln!("{}", err);
                        }),
                };
            }
        } else {
            match prompt(&mut rl) {
                UserActions::Prompt(prompt) => state.context.chat.push(messages::Message {
                    text: prompt,
                    role: messages::Role::User,
                    media: vec![],
                }),
                UserActions::Exit => break,
                UserActions::SetModel(model) => state.set_interface(&model)
                    .unwrap_or_else(|err| {
                        eprintln!("{}", err);
                    }),
                UserActions::Save(path) => state.context.write_to_file(&path)
                    .unwrap_or_else(|err| {
                        eprintln!("{}", err);
                    }),
                UserActions::SetFile(path) => state.autosave = if path.is_empty() { None } else { Some(path) },
                UserActions::Help => print!(include_str!("../help-interactive.txt")),
                UserActions::Status => print_status(&state),
                UserActions::DeleteLast => {
                    state.context.chat.pop(); // model response
                    state.context.chat.pop(); // user prompt
                    state.try_autosave();
                },
                UserActions::Rewind(num) => state.context.rewind(&num),
                UserActions::SetSystemPrompt(system) => {
                    state.context.system = system;
                    state.try_autosave();
                },
                UserActions::None => {},
                UserActions::SetParameter(key, value) => {
                    state.context.parameters.insert(key, value);
                    state.try_autosave();
                },
                UserActions::UnsetParameter(key) => {
                    state.context.parameters.remove(&key);
                    state.try_autosave();
                },
            }
        }
    }
}
