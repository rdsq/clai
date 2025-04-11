use serde::{Serialize, Deserialize};

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
