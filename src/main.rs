mod states;
mod interfaces;
mod parse_model;
mod get_interface;
mod commands;
mod prompt;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "clai")]
#[command(version = "1.0")]
#[command(about = "Custom AI CLI client", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Gen(commands::generate::Generate),
    Chat(commands::chat::Chat),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Gen(args) => commands::generate::generate(args).await,
        Commands::Chat(args) => commands::chat::chat(args).await,
    }
}
