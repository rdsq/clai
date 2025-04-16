mod states;
mod interfaces;
mod commands;
mod prompt;
mod markdown;

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
    Read(commands::read::Read),
    ModelWithModel(commands::model_with_model::ModelWithModel),
    Semsearch(commands::semantic_search::SemanticSearch),
    Embed(commands::embed::Embed),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Gen(args) => commands::generate::generate(args).await,
        Commands::Chat(args) => commands::chat::chat(args).await,
        Commands::Read(args) => commands::read::read(args),
        Commands::ModelWithModel(args) => commands::model_with_model::model_with_model(args).await,
        Commands::Semsearch(args) => commands::semantic_search::semantic_search(args).await,
        Commands::Embed(args) => commands::embed::embed(args).await,
    }
}
