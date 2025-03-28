struct ParsedModel<'a> {
    pub interface: &'a str,
    pub model: &'a str,
}

pub fn parse_model(input: &str) -> Result<ParsedModel, &'static str> {
    let mut sp = input.split(':');
    if let (Some(interface), Some(model)) = (sp.next(), sp.next()) {
        Ok(ParsedModel { interface, model })
    } else {
        Err("invalid interface:model format")
    }
}
