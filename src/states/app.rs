use super::{ContextState, InterfaceState, context};
use std::io::{self, Write};

pub struct AppState {
    pub context: ContextState,
    pub interface: InterfaceState,
}

impl AppState {
    pub fn new(context: ContextState, interface: &str) -> Self {
        AppState {
            context,
            interface: InterfaceState::new(interface),
        }
    }
    pub async fn generate(&mut self, prompt: String, callback: Box<dyn Fn(String) -> () + Send>) {
        self.context.chat.push(context::Message { role: context::Role::User, text: prompt });
        let res = self.interface.interface.generate(&self.context, callback).await;
        match res {
            Ok(response) => {
                self.context.chat.push(context::Message { role: context::Role::Model, text: response });
            },
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            },
        }
    }
    pub async fn generate_to_output(&mut self, prompt: String) {
        let callback = Box::new(|chunk: String| {
            print!("{}", chunk);
            io::stdout().flush().unwrap();
        });
        self.generate(prompt, callback).await;
        self.context.chat.last().unwrap().compensate_nl();
    }
}
