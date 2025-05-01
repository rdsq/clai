pub enum UserActions {
    Prompt(String),
    Exit,
    SetModel(String),
    Save(String),
    SetFile(String),
    Help,
    Status,
    DeleteLast,
    Rewind(Option<usize>),
    SetSystemPrompt(Option<String>),
    None,
    SetParameter(String, serde_json::Value),
    UnsetParameter(String),
}

pub fn prompt(rl: &mut rustyline::DefaultEditor) -> UserActions {
    let inp = rl.readline("\x1b[36;1m>\x1b[0m ");
    if inp.is_err() {
        return UserActions::Exit;
    }
    let inp = inp.unwrap();
    let (cmd, content) = inp.split_once(' ').unwrap_or((&inp, &inp));
    let content = content.to_string();
    return match cmd {
        "/help" => UserActions::Help,
        "/?" => UserActions::Help,
        "/setmodel" => UserActions::SetModel(content),
        "/save" => UserActions::Save(content),
        "/setfile" => UserActions::SetFile(content),
        "/exit" => UserActions::Exit,
        "/fromfile" => match std::fs::read_to_string(content) {
            Ok(prompt) => UserActions::Prompt(prompt),
            Err(err) => {
                eprintln!("Error while reading from file: {}", err);
                UserActions::None
            }
        },
        "/status" => UserActions::Status,
        "/nvm" => UserActions::DeleteLast,
        "/rewind" => {
            if content == cmd { // according to the split logic
                UserActions::Rewind(None)
            } else {
                match content.parse::<usize>() {
                    Ok(num) => UserActions::Rewind(Some(num)),
                    Err(err) => {
                        eprintln!("Error while parsing the number: {}", err);
                        UserActions::None
                    },
                }
            }
        },
        "/system" => UserActions::SetSystemPrompt(if content.is_empty() { None } else { Some(content.to_string()) }),
        "/param" => {
            if let Some((key, value)) = content.split_once(' ') {
                match serde_json::from_str(value) {
                    Ok(parsed) => UserActions::SetParameter(key.to_string(), parsed),
                    Err(err) => {
                        eprintln!("Error while parsing JSON: {}", err);
                        UserActions::None
                    }
                }
            } else {
                UserActions::UnsetParameter(content.to_string())
            }
        },
        _ => UserActions::Prompt(inp),
    }
}
