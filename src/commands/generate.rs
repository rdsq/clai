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
    let state = app_state::AppState::new();
    state.chat.push(app_state::Message { role: app_state::Role::User, text: args.prompt });
    let mut response = String::from("");
    let res = interface.generate(&state, |chunk| {
        response.push_str(chunk);
        print!("{}", chunk);
        io::stdout().flush().unwrap();
    }).await;
    match res {
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
    if !response.ends_with("\n") {
        println!();
    }
    state.chat.push(app_state::Message { role: app_state::Role::Model, text: response });
}
