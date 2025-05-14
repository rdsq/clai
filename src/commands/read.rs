use crate::states::ContextState;

#[derive(clap::Parser, Debug)]
/// Read a chat save file
pub struct Read {
    /// The file to read from
    file: String,
}

pub fn read(args: Read) {
    let state = ContextState::from_file(&args.file, false)
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });
    for message in state.chat {
        message.print_with_role();
    }
}
