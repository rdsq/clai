use super::{ContextState, InterfaceState, messages};
use std::io::{self, Write};

pub struct AppState {
    pub context: ContextState,
    pub interface: InterfaceState,
    pub autosave: Option<String>,
}

impl AppState {
    pub fn new(file: Option<String>, interface: &str) -> Self {
        AppState {
            context: ContextState::from_optional_file(&file),
            interface: InterfaceState::new(interface),
            autosave: file,
        }
    }
    pub fn try_autosave(&self) {
        if let Some(file) = &self.autosave {
            self.context.write_to_file(&file);
        }
    }
    pub fn set_interface(&mut self, interface: &str) {
        self.interface = InterfaceState::new(interface);
    }
    pub async fn generate(&mut self, prompt: String, callback: Box<dyn Fn(String) -> () + Send>) {
        self.context.chat.push(messages::Message { role: messages::Role::User, text: prompt });
        let res = self.interface.interface.generate(&self.context, callback).await;
        match res {
            Ok(response) => {
                self.context.chat.push(messages::Message { role: messages::Role::Model, text: response });
                self.try_autosave();
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
