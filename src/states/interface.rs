use crate::interfaces::frame::Interface;
use crate::interfaces::get_interface;

pub struct InterfaceState {
    pub interface: Box<dyn Interface>,
}

impl InterfaceState {
    pub fn new(input: &str) -> Result<Self, String> {
        match input.split_once(':') {
            Some((interface_name, model)) => {
                match get_interface(interface_name, model.to_string()) {
                    Ok(interface) => Ok(Self { interface }),
                    Err(err) => Err(err),
                }
            },
            None => Err("Invalid interface:model format".to_string()),
        }
    }
}
