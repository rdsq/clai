use crate::states::{AppState, messages};
use std::io::{self, Write};

#[derive(clap::Parser, Debug)]
/// Make a model chat with another model
pub struct ModelWithModel {
    /// The start message
    prompt: String,
    /// The first model
    model1: String,
    /// The second model (using the same by default)
    model2: Option<String>,
}

pub async fn model_with_model(args: ModelWithModel) {
    let mut state1 = AppState::new(None, &args.model1);
    let mut state2 = AppState::new(None, &args.model2.unwrap_or(args.model1));
    // the start message
    state1.context.chat.push(messages::Message {
        role: messages::Role::Model,
        text: args.prompt,
    });
    let names_are_same = state1.interface.interface.model_id() == state2.interface.interface.model_id();
    let mut is_first = false;
    loop {
        let (main_state, secondary_state, mut name) = if is_first { (&mut state1, &state2, "Model 1".to_string()) } else { (&mut state2, &state1, "Model 2".to_string()) };
        is_first = !is_first;
        if !names_are_same {
            name.push_str(&format!(" ({})", main_state.interface.interface.model_id()));
        }
        print!("\x1b[36;1m{}:\x1b[0m ", name);
        io::stdout().flush().unwrap();
        let prompt = secondary_state.context.chat.last().unwrap().text.clone();
        main_state.generate_to_output(prompt).await;
    }
}
