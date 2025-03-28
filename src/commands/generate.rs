use crate::app_state;
use std::io::{self, Write};

#[derive(clap::Parser, Debug)]
/// Generate a response
pub struct Generate {
    /// The interface and model
    model: String,
    /// The user message
    prompt: String,
}

pub async fn generate(args: Generate) {
    let interface = crate::get_interface::get_interface(&args.model);
    let mut state = app_state::AppState::new();
    state.chat.push(app_state::Message { role: app_state::Role::User, text: args.prompt });
    let callback = Box::new(|chunk: String| {
        print!("{}", chunk);
        io::stdout().flush().unwrap();
    });
    let res = interface.generate(&state, Box::new(callback)).await;
    match res {
        Ok(response) => {
            if !response.ends_with("\n") {
                println!();
            }
            state.chat.push(app_state::Message { role: app_state::Role::Model, text: response });
        },
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        },
    }
}
