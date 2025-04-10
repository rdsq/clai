pub enum Role {
    User,
    Model,
}

pub struct Message {
    pub role: Role,
    pub text: String,
}

pub struct VisibleState {
    pub chat: Vec<Message>,
}

impl VisibleState {
    pub fn new() -> Self {
        Self { chat: Vec::new() }
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
