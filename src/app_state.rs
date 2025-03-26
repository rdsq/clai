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
