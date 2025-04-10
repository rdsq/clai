use crate::interfaces::frame::Interface;
use crate::get_interface::get_interface;

pub struct InterfaceState {
    pub interface: Box<dyn Interface>,
}

impl InterfaceState {
    pub fn new(interface: &str) -> Self {
        Self {
            interface: get_interface(interface),
        }
    }
}
