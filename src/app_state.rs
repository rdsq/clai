pub enum Role {
    User,
    Model,
}

pub struct Message {
    pub role: Role,
    pub text: String,
}

pub struct AppState {
    pub chat: Vec<Message>,
}

impl AppState {
    pub fn new() -> Self {
        AppState { chat: Vec::new() }
    }
}
