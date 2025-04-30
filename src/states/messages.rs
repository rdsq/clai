use serde::{Serialize, Deserialize};
use crate::markdown::markdown_to_ansi;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Model,
}

impl Role {
    pub fn to_user_string(&self) -> &'static str {
        return match self {
            Self::User => "User",
            Self::Model => "Model",
        };
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Media {
    Image {
        mime: String,
        data: Vec<u8>,
    },
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub text: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub media: Vec<Media>,
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
    pub fn print(&self) {
        print!("{}", markdown_to_ansi(&self.text));
        self.compensate_nl();
    }
    pub fn print_with_role(&self) {
        print!("\x1b[36;1m{}:\x1b[0m ", self.role.to_user_string());
        self.print();
    }
}
