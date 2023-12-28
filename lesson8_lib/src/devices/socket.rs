use std::fmt::Display;

use rand::Rng;

use crate::device::{Device, DeviceState};

#[derive(Debug)]
pub struct Socket {
    name: String,
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
    fn measure_power(&self) -> u32 {
        match self.state {
            DeviceState::On => rand::thread_rng().gen_range(1..100),
            DeviceState::Off => 0u32,
        }
    }
}

impl Display for Socket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Socket name: {}\nstate: {}\ncurrent power: {}",
            self.name,
            self.state,
            self.measure_power()
        )
    }
}
