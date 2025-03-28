mod commands;

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
}

#[tokio::main]
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate(args) => commands::generate::generate(args).await,
    }
}
