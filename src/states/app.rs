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
    pub async fn generate(&mut self, callback: Box<dyn Fn(String) -> () + Send>) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.interface.interface.generate(&self.context, callback).await;
        match res {
            Ok(response) => {
                self.context.chat.push(messages::Message { role: messages::Role::Model, text: response, media: vec![] });
                self.try_autosave();
                Ok(())
            },
            Err(err) => Err(err),
        }
    }
    pub async fn generate_to_output(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let callback = Box::new(|chunk: String| {
            print!("{}", chunk);
            io::stdout().flush().unwrap();
        });
        match self.generate(callback).await {
            Ok(()) => Ok(self.context.chat.last().unwrap().compensate_nl()),
            Err(err) => Err(err),
        }
    }
}
