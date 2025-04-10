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
