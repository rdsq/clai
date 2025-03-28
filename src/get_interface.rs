use std::process::exit;
use crate::parse_model::parse_model;

pub fn get_interface(input: &str) -> Box<dyn crate::interfaces::frame::Interface> {
    let parsed = parse_model(&input).unwrap_or_else(|err| {
        eprintln!("{}", err.to_string());
        exit(1);
    });
    let interface = crate::interfaces::get_interface(&parsed.interface, parsed.model.to_string()).unwrap_or_else(|err| {
        eprintln!("{}", err.to_string());
        exit(1);
    });
    interface
}
