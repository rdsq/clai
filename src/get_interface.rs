use std::process::exit;

pub fn get_interface(input: &str) {
    let parsed = crate::parse_model(&input).unwrap_or_else(|err| {
        eprintln!("{}", err.to_string());
        exit(1);
    });
    let interfaces = crate::interfaces::get_interfaces();
    if let Some(interface) = interfaces.get(parsed.interface) {
        interface::new(parsed.model)
    } else {
        eprintln!("Unknown interface: {}", parsed.interface);
        exit(1);
    }
}
