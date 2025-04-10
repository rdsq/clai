mod ollama;
pub mod frame;

pub fn get_interface(interface: &str, model: String) -> Result<Box<dyn frame::Interface>, String> {
    return match interface {
        "ollama" => Ok(Box::new(ollama::OllamaInterface::new(model))),
        _ => Err(format!("unknown interface: {}", interface)),
    }
}
