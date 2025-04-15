use crate::states::InterfaceState;

#[derive(clap::Parser, Debug)]
/// Search which strings better match a pattern
pub struct SemanticSearch {
    /// The embedding model that will evaluate the results
    model: String,
    /// The search prompt
    prompt: String,
    /// The items to search from
    items: Vec<String>,
    /// How many top results to show
    #[arg(short, long, default_value = "10")]
    show: usize,
    /// Treat the items as file paths
    #[arg(short, long)]
    files: bool,
    /// Show the similarity numbers
    #[arg(short, long)]
    verbose: bool,
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum::<f32>();
    let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    dot / (norm_a * norm_b)
}

pub async fn semantic_search(args: SemanticSearch) {
    let state = InterfaceState::new(&args.model);
    let mut inputs = vec![args.prompt];
    if args.files {
        for path in &args.items {
            inputs.push(std::fs::read_to_string(&path).unwrap_or_else(|err| {
                eprintln!("Error while reading \"{}\": {}", path, err);
                std::process::exit(1);
            }));
        }
    } else {
        inputs.extend(args.items.clone());
    }
    let resp = state.interface.embeddings(&inputs).await
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });
    let prompt_score = &resp[0];
    let mut results: Vec<(String, f32)> = Vec::new();
    for i in 1..inputs.len() {
        // show filenames if files, not contents
        results.push((args.items[i-1].clone(), cosine_similarity(&prompt_score, &resp[i])));
    }
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for i in 0..std::cmp::min(results.len(), args.show) {
        if args.verbose {
            println!("{} ({}). {}", i + 1, results[i].1, results[i].0);
        } else {
            println!("{}. {}", i + 1, results[i].0);
        }
    }
}
