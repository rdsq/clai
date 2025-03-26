struct ParsedModel {
    interface: &str,
    model: &str,
}

pub fn parse_model(input: &str) -> Result<ParsedModel, Box<dyn std::error::Error>> {
    let format = input.split(':');
    if (let Some(interface) = format.next()) && (let Some(model) = format.next()) {
        ParsedModel { interface, model }
    } else {
        Err("invalid interface:model format")
    }
}
