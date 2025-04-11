use crate::states::ContextState;

#[derive(clap::Parser, Debug)]
/// Read a chat save file
pub struct Read {
    /// The file to read from
    file: String,
}

pub fn read(args: Read) {
    let state = ContextState::from_file(&args.file, false);
    for message in state.chat {
        message.print_with_role();
    }
}
