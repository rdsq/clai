use serde::Serialize;

struct OllamaInterface {
    model: String,
}

impl OllamaInterface {
    new(model: String) -> Self {
        OllamaInterface { model }
    }
}

struct OllamaMessage {
    role: String,
    content: String,
}

fn prepare_chat(chat: &Vec<crate::app_state::Message>) -> Vec<OllamaMessage> {
    let mut new_chat: Vec<OllamaMessage> = Vec::new();
    for msg in chat {
        match msg.role {
            crate::app_state::Role::User => {
                new_chat.push(OllamaMessage {
                    role: String::from("user"),
                    content: msg.text.clone(),
                });
            }
            crate::app_state::Role::Model => {
                new_chat.push(OllamaMessage {
                    role: String::from("assistant"),
                    content: msg.text.clone(),
                });
            }
        }
    }
    new_chat
}

#[derive(Serialize)]
struct OllamaRequest {
    messages: Vec<OllamaMessage>,
    model: String,
    stream: Boolean,
}

#[async_trait::async_trait]
impl self::trait::Interface for OllamaInterface {
    async fn generate(&self, &state, callback) {
        let client = reqwest::Client::new();
        let res = client
            .post("http://localhost:11434/api/chat")
            .json(&OllamaRequest {
                messages: prepare_chat(&state.chat),
                model: self.model.clone(),
                stream: true,
            })
            .send
            .await?;
        if !res.status().is_success() {
            Err(res.status())
        }
        let mut stream = res.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            callback(chunk);
        }
        Ok(())
    }
}
