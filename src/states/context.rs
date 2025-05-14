use serde::{Serialize, Deserialize};
use std::fs;
use serde_json;
use std::io::Write;
use super::messages::Message;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ContextState {
    #[serde(default)]
    pub chat: Vec<Message>,
    #[serde(default)]
    pub system: Option<String>,
    pub parameters: HashMap<String, serde_json::Value>,
}

impl ContextState {
    pub fn new() -> Self {
        Self {
            chat: Vec::new(),
            system: None,
            parameters: HashMap::new(),
        }
    }
    pub fn from_optional_file(file: &Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(path) = file {
            Self::from_file(&path, true)
        } else {
            Ok(Self::new())
        }
    }
    pub fn from_file(path: &str, allow_not_existing: bool) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path);
        if let Err(err) = contents {
            if allow_not_existing && err.kind() == std::io::ErrorKind::NotFound {
                return Ok(Self::new());
            } else {
                return Err(Box::new(err));
            }
        }
        let contents = contents.unwrap();
        return Ok(serde_json::from_str(&contents)
            .map_err(|e| -> Box<dyn std::error::Error> { format!("JSON parsing error: {}", e).into() })?);
    }
    pub fn write_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = serde_json::to_string_pretty(&self)
            .map_err(|e| -> Box<dyn std::error::Error> { format!("JSON stringifying error: {}", e).into() })?;
        let mut file = fs::File::create(path)
            .map_err(|e| -> Box<dyn std::error::Error> { format!("Error while creating a file: {}", e).into() })?;
        file.write_all(contents.as_bytes())
            .map_err(|e| -> Box<dyn std::error::Error> { format!("Error while writing the file: {}", e).into() })?;
        Ok(())
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
