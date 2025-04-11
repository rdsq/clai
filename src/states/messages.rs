use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Model,
}

impl Role {
    pub fn to_user_string(&self) -> String {
        return match self {
            Self::User => "User",
            Self::Model => "Model",
        }.to_string();
    }
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub text: String,
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
        print!("\x1b[36;1m{}:\x1b[0m {}", self.role.to_user_string(), self.text);
        self.compensate_nl();
    }
}
