enum Role {
    User,
    Model,
}

struct Message {
    role: Role,
    text: String,
}

struct AppState {
    chat: Vec<Message>,
}

impl AppState {
    new() -> Self {
        AppState { chat: Vec::<Message>new() }
    }
}
