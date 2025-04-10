use crate::states::{AppState, visible};
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
    let mut state = AppState::new(&args.model);
    state.visible.chat.push(visible::Message { role: visible::Role::User, text: args.prompt });
    let callback = Box::new(|chunk: String| {
        print!("{}", chunk);
        io::stdout().flush().unwrap();
    });
    let res = state.interface.interface.generate(&state.visible, Box::new(callback)).await;
    match res {
        Ok(response) => {
            if !response.ends_with("\n") {
                println!();
            }
            state.visible.chat.push(visible::Message { role: visible::Role::Model, text: response });
        },
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        },
    }
}
