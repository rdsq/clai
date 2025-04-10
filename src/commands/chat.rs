use crate::states::{AppState, ContextState};

#[derive(clap::Parser, Debug)]
/// Chat with a chatbot
pub struct Chat {
    // The model to chat with
    model: String,
}

pub async fn chat(args: Chat) {
    let mut rl = rustyline::DefaultEditor::new().unwrap();
    let context = ContextState::new();
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
}
