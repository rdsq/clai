use crate::interfaces::frame::Interface;
use crate::interfaces::get_interface;

pub struct InterfaceState {
    pub interface: Box<dyn Interface>,
}

impl InterfaceState {
    pub fn new(input: &str) -> Self {
        let (interface_name, model) = input.split_once(':')
            .unwrap_or_else(|| {
                println!("Invalid interface:model format");
                std::process::exit(1);
            });
        let interface = get_interface(interface_name, model.to_string())
            .unwrap_or_else(|err| {
                eprintln!("{}", err.to_string());
                std::process::exit(1);
            });
        Self {
            interface,
        }
    }
}
