use serde::{Serialize, Deserialize};
use std::fs;
use serde_json;
use std::io::Write;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Model,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub text: String,
}

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
}

impl Message {
    pub fn ends_with_nl(&self) -> bool {
        self.text.ends_with("\n")
    }
    pub fn compensate_nl(&self) {
        if !self.ends_with_nl() {
            println!(); // add a new line
        }
    }
}
