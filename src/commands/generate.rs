use crate::states::AppState;
use crate::states::messages::Media;

#[derive(clap::Parser, Debug)]
/// Generate a response
pub struct Generate {
    /// The interface and model
    model: String,
    /// The user message
    prompt: String,
    /// Read and save the chat to a file
    #[arg(short, long, default_value = None)]
    file: Option<String>,
    /// System prompt
    #[arg(short, long, default_value = None)]
    system: Option<String>,
    /// Parameters for the model as JSON
    #[arg(short, long, default_value = None)]
    parameters: Option<String>,
    /// Add an image for multimodal models
    #[arg(short, long, default_value = None)]
    image: Option<String>,
}

pub async fn generate(args: Generate) {
    let mut state = AppState::new(args.file, &args.model);
    state.context.system = args.system;
    if let Some(parameters) = args.parameters {
        state.context.parameters = serde_json::from_str(&parameters)
            .unwrap_or_else(|err| {
                eprintln!("Error while parsing JSON: {}", err);
                std::process::exit(1);
            });
    }
    let mut media = Vec::new();
    if let Some(image_path) = args.image {
        let path = std::path::Path::new(&image_path);
        let extension = path.extension().unwrap_or_else(|| {
            eprintln!("Could not find the mime of {}", image_path);
            std::process::exit(1);
        }).to_str().unwrap();
        let data: Vec<u8> = std::fs::read(&image_path)
            .unwrap_or_else(|err| {
                eprintln!("Failed to read image file: {}", err);
                std::process::exit(1);
            });
        media.push(Media::Image {
            data,
            mime: format!("image/{}", extension),
        });
    }
    state.generate_to_output(args.prompt, media).await;
}
