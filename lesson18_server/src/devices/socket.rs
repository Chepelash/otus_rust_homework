use std::fmt::Display;

use rand::Rng;

use crate::device::{Device, DeviceState};

/// Example socket
#[derive(Debug)]
pub struct Socket {
    /// Device name
    name: String,
    /// Device state
    state: DeviceState,
}

impl Device for Socket {
    fn name(&self) -> &str {
        self.name.as_str()
    }
    fn state(&self) -> &DeviceState {
        &self.state
    }
    fn turn_off(&mut self) {
        self.state = DeviceState::Off;
    }
    fn turn_on(&mut self) {
        self.state = DeviceState::On;
    }
    fn get_report(&self) -> String {
        self.to_string()
    }
}

impl Socket {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: DeviceState::default(),
        }
    }
    /// Dummy function for measuring power
    fn measure_power(&self) -> Option<u32> {
        match self.state {
            DeviceState::On => Some(rand::thread_rng().gen_range(1..100)),
            DeviceState::Off => None,
        }
    }
}

impl Display for Socket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Socket name: {}\nstate: {}\ncurrent power: {}\n",
            self.name,
            self.state,
            self.measure_power().unwrap_or_default()
        )
    }
}
