use crate::states::InterfaceState;
use std::io::{self, Read};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum, Debug)]
enum IOFormats {
    Plain,
    JSON,
}

#[derive(serde::Serialize)]
struct OutputObject<'a> {
    embedding: &'a Vec<f32>,
    text: &'a str,
}

#[derive(clap::Parser, Debug)]
/// Embed strings and output their embedding values. Can be used for caching
pub struct Embed {
    /// The embedding model that will evaluate the results
    model: String,
    /// The strings to embed
    items: Vec<String>,
    /// Output the original strings too (in plain output format)
    #[arg(short, long)]
    pairs: bool,
    /// Format for the input items
    #[arg(short, long, default_value = "plain")]
    input_format: IOFormats,
    /// Format for the result items
    #[arg(short, long, default_value = "plain")]
    output_format: IOFormats,
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
    let items: Vec<String> = match args.input_format {
        IOFormats::Plain => args.items,
        IOFormats::JSON => {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input).unwrap();
            serde_json::from_str(&input)
                .unwrap_or_else(|err| {
                    eprintln!("Error while parsing JSON: {}", err);
                    std::process::exit(1);
                })
        }
    };
    let embeddings = state.interface.embeddings(&items).await
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });
    let pairs = embeddings.iter()
        .zip(items.iter());
    match args.output_format {
        IOFormats::Plain => for (emb, label) in pairs {
            print!("{}", emb.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(","));
            if args.pairs {
                println!(" {}", create_label(&label));
            } else {
                println!();
            }
        },
        IOFormats::JSON => {
            let stringified = serde_json::to_string(
                &pairs
                .map(|(embedding, text)| OutputObject {
                    embedding,
                    text,
                })
                .collect::<Vec<OutputObject>>()
            ).unwrap();
            println!("{}", stringified);
        },
    };
}
