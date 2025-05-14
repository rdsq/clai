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
    let msg = messages::Message {
        role: messages::Role::Model,
        text: args.prompt,
        media: vec![],
    };
    let names_are_same = state1.interface.interface.model_id() == state2.interface.interface.model_id();
    fn model_name(name: String, state: &AppState, names_are_same: &bool) {
        messages::print_actor(&if *names_are_same {
            name
        } else {
            format!("{} ({})", name, state.interface.interface.model_id())
        });
    }
    model_name("Model 1".to_string(), &state1, &names_are_same);
    msg.print();
    let mut is_first = false;
    state1.context.chat.push(msg);
    loop {
        let (main_state, secondary_state, name) = if is_first { (&mut state1, &state2, "Model 1".to_string()) } else { (&mut state2, &state1, "Model 2".to_string()) };
        is_first = !is_first;
        model_name(name, &main_state, &names_are_same);
        io::stdout().flush().unwrap();
        main_state.context.chat.push(messages::Message {
            text: secondary_state.context.chat.last().unwrap().text.clone(),
            role: messages::Role::User,
            media: vec![],
        });
        main_state.generate_to_output().await
            .unwrap_or_else(|err| {
                eprintln!("{}", err);
                std::process::exit(1);
            });
    }
}
