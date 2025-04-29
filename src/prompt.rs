pub enum UserActions {
    Prompt(String),
    Exit,
    SetModel(String),
    Save(String),
    SetFile(String),
    Help,
    PromptFromFile(String),
    Status,
    DeleteLast,
    Rewind(Option<usize>),
    SetSystemPrompt(Option<String>),
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
        "/fromfile" => UserActions::PromptFromFile(content),
        "/status" => UserActions::Status,
        "/nvm" => UserActions::DeleteLast,
        "/rewind" => {
            if content == cmd { // according to the split logic
                UserActions::Rewind(None)
            } else {
                UserActions::Rewind(Some(content.parse().unwrap_or_else(|err| {
                    eprintln!("Error while parsing the number: {}", err);
                    std::process::exit(1);
                })))
            }
        },
        "/system" => UserActions::SetSystemPrompt(if content.is_empty() { None } else { Some(content.to_string()) }),
        _ => UserActions::Prompt(inp),
    }
}
