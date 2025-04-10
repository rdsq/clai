pub enum UserActions {
    Prompt(String),
    Exit,
    SetModel(String),
    Save(String),
    SetFile(String),
    Help,
    PromptFromFile(String),
    Status,
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
        _ => UserActions::Prompt(inp),
    }
}
