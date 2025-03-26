mod trait;
mod ollama;
use std::collections::HashMap;

pub fn get_interfaces() {
    let mut interfaces: HashMap<String, Box<dyn trait::Interface>> = HashMap::new();
    interfaces.insert("ollama".to_string(), Box::new(ollama::OllamaInterface));
    interfaces
}
