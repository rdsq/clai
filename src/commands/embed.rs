use crate::states::InterfaceState;

#[derive(clap::Parser, Debug)]
/// Embed strings and output their embedding values. Can be used for caching
pub struct Embed {
    /// The embedding model that will evaluate the results
    model: String,
    /// The strings to embed
    items: Vec<String>,
    /// Output the original strings too
    #[arg(short, long)]
    pairs: bool,
}

fn create_label<'a>(original: &str) -> String {
    if let Some((first, _)) = original.split_once('\n') {
        format!("{}...", first)
    } else {
        original.to_string()
    }
}

pub async fn embed(args: Embed) {
    let state = InterfaceState::new(&args.model);
    let embeddings = state.interface.embeddings(&args.items).await
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });
    for (i, emb) in embeddings.into_iter().enumerate() {
        print!("{}", emb.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(","));
        if args.pairs {
            println!(" {}", create_label(&args.items[i]));
        } else {
            println!();
        }
    }
}
