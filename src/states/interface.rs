use crate::interfaces::frame::Interface;
use crate::interfaces::get_interface;

pub struct InterfaceState {
    pub interface: Box<dyn Interface>,
}

impl InterfaceState {
    pub fn new(input: &str) -> Result<Self, String> {
        let (interface_name, model) = input.split_once(':')
            .ok_or("Invalid interface:model format".to_string())?;
        let interface = get_interface(interface_name, model.to_string())?;
        Ok(Self { interface })
    }
}
