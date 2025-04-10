use super::{VisibleState, InterfaceState};

pub struct AppState {
    pub visible: VisibleState,
    pub interface: InterfaceState,
}

impl AppState {
    pub fn new(interface: &str) -> Self {
        AppState {
            visible: VisibleState::new(),
            interface: InterfaceState::new(interface),
        }
    }
}
