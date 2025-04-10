pub struct ParsedModel<'a> {
    pub interface: &'a str,
    pub model: &'a str,
}

pub fn parse_model(input: &str) -> Result<ParsedModel, &'static str> {
    if let Some((interface, model)) = input.split_once(':') {
        Ok(ParsedModel { interface, model })
    } else {
        Err("invalid interface:model format")
    }
}
