use serde::{Serialize, Deserialize};
use std::fs;
use serde_json;
use std::io::Write;
use super::messages::Message;

#[derive(Serialize, Deserialize)]
pub struct ContextState {
    pub chat: Vec<Message>,
}

impl ContextState {
    pub fn new() -> Self {
        Self { chat: Vec::new() }
    }
    pub fn from_optional_file(file: &Option<String>) -> Self {
        if let Some(path) = file {
            Self::from_file(&path, true)
        } else {
            Self::new()
        }
    }
    pub fn from_file(path: &str, allow_not_existing: bool) -> Self {
        let contents = fs::read_to_string(path);
        if let Err(err) = contents {
            if allow_not_existing && err.kind() == std::io::ErrorKind::NotFound {
                return Self::new();
            } else {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        }
        let contents = contents.unwrap();
        return serde_json::from_str(&contents)
            .unwrap_or_else(|e| {
                eprintln!("JSON parsing error: {}", e);
                std::process::exit(1);
            });
    }
    pub fn write_to_file(&self, path: &str) {
        let contents = serde_json::to_string_pretty(&self)
            .unwrap_or_else(|e| {
                eprintln!("JSON stringifying error: {}", e);
                std::process::exit(1);
            });
        let mut file = fs::File::create(path)
            .unwrap_or_else(|e| {
                eprintln!("Error while creating a file: {}", e);
                // will it cause issues with overwriting? We'll find out
                std::process::exit(1);
            });
        file.write_all(contents.as_bytes())
            .unwrap_or_else(|e| {
                eprintln!("Error while writing the file: {}", e);
                std::process::exit(1);
            });
    }
    pub fn rewind(&self, num: &Option<usize>) {
        let mut start_index = 0;
        let len = self.chat.len();
        if let Some(custom) = num {
            if *custom < len {
                start_index = len - custom;
            }
        }
        for i in start_index..len {
            self.chat[i].print_with_role();
        }
    }
}
