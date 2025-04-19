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
    /// Show the similarity numbers
    #[arg(short, long)]
    verbose: bool,
    /// Format of the input intems
    #[arg(short = 'f', long, default_value = "text")]
    input_format: InputFormats,
}

#[derive(serde::Deserialize)]
struct InputObject {
    embedding: Vec<f32>,
    text: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum, Debug)]
enum InputFormats {
    Text,
    Pre,
    JSON,
    File,
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum::<f32>();
    let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    dot / (norm_a * norm_b)
}

fn parse_pre(item: &str) -> Result<(String, Vec<f32>), Box<dyn std::error::Error>> {
    let (seq, name) = item.split_once(' ')
        .ok_or("failed to split by whitespace")?;
    let mut parsed = Vec::new();
    for i in seq.split(',') {
        parsed.push(i.parse::<f32>()?);
    }
    Ok((name.to_string(), parsed))
}

async fn get_embeddings(state: &InterfaceState, items: &Vec<String>, input_format: InputFormats) -> (Vec<String>, Vec<Vec<f32>>) {
    let mut names_vec = Vec::new();
    let mut embeds_vec = Vec::new();
    match input_format {
        InputFormats::Pre => {
            for (i, item) in items.iter().enumerate() {
                let (name, parsed) = parse_pre(&item)
                    .unwrap_or_else(|err| {
                        eprintln!("Error while parsing item {}: {}", i+1, err);
                        std::process::exit(1);
                    });
                names_vec.push(name);
                embeds_vec.push(parsed);
            }
        },
        InputFormats::File => {
            let mut values = Vec::new();
            for path in items {
                values.push(std::fs::read_to_string(&path).unwrap_or_else(|err| {
                    eprintln!("Error while reading \"{}\": {}", path, err);
                    std::process::exit(1);
                }));
            }
            names_vec.extend(items.clone());
            let embeddings = state.interface.embeddings(&values).await
                .unwrap_or_else(|err| {
                    eprintln!("{}", err);
                    std::process::exit(1);
                });
            embeds_vec.extend(embeddings);
        },
        InputFormats::Text => {
            let embeddings = state.interface.embeddings(&items).await
                .unwrap_or_else(|err| {
                    eprintln!("{}", err);
                    std::process::exit(1);
                });
            embeds_vec.extend(embeddings);
            names_vec.extend(items.clone());
        },
        InputFormats::JSON => {
            let arg = &items.iter().nth(0)
                .unwrap_or_else(|| {
                    eprintln!("Provide an argument with a JSON array");
                    std::process::exit(1);
                });
            let arr: Vec<InputObject> = serde_json::from_str(arg)
                .unwrap_or_else(|err| {
                    eprintln!("Error while parsing JSON: {}", err);
                    std::process::exit(1);
                });
            for i in arr {
                names_vec.push(i.text);
                embeds_vec.push(i.embedding);
            }
        },
    }
    (names_vec, embeds_vec)
}

pub async fn semantic_search(args: SemanticSearch) {
    let state = InterfaceState::new(&args.model);
    let (names, embeds) = get_embeddings(&state, &args.items, args.input_format).await;
    let prompt_score = &state.interface.embeddings(&vec![args.prompt]).await
        .unwrap_or_else(|err| {
            eprintln!("Error while embedding prompt: {}", err);
            std::process::exit(1);
        })[0];
    let mut results: Vec<(&String, f32)> = names.iter()
        .zip(embeds.iter().map(|e| cosine_similarity(&prompt_score, &e)))
        .collect();
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for i in 0..std::cmp::min(results.len(), args.show) {
        if args.verbose {
            println!("{} ({}). {}", i + 1, results[i].1, results[i].0);
        } else {
            println!("{}. {}", i + 1, results[i].0);
        }
    }
}
