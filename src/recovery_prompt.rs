pub enum RecoveryAction {
    Retry,
    Discard,
    Exit,
    ChangeModel(String),
}

pub fn recovery_prompt(rl: &mut rustyline::DefaultEditor) -> RecoveryAction {
    loop {
        let res = rl.readline("Generation failed. [r]etry / [d]discard / [e]xit / [c]hange model: ");
        if let Ok(input) = res {
            match input.trim().to_lowercase().as_str() {
                "r" | "retry" => return RecoveryAction::Retry,
                "d" | "discard" => return RecoveryAction::Discard,
                "e" | "exit" => return RecoveryAction::Exit,
                "c" | "change" | "change model" => {
                    if let Ok(model) = rl.readline("New interface:model: ") {
                        return RecoveryAction::ChangeModel(model);
                    }
                },
                _ => println!("Invalid input, try again"),
            }
        }
    }
}
