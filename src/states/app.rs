use super::{VisibleState, InterfaceState, visible};
use std::io::{self, Write};

pub struct AppState {
    pub visible: VisibleState,
    pub interface: InterfaceState,
}

impl AppState {
    pub fn new(interface: &str) -> Self {
        AppState {
            visible: VisibleState::new(),
            interface: InterfaceState::new(interface),
        }
    }
    pub async fn generate(&mut self, prompt: String, callback: Box<dyn Fn(String) -> () + Send>) {
        self.visible.chat.push(visible::Message { role: visible::Role::User, text: prompt });
        let res = self.interface.interface.generate(&self.visible, callback).await;
        match res {
            Ok(response) => {
                self.visible.chat.push(visible::Message { role: visible::Role::Model, text: response });
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
        self.visible.chat.last().unwrap().compensate_nl();
    }
}
