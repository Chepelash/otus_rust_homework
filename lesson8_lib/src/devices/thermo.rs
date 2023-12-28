use std::fmt::Display;

use rand::Rng;

use crate::device::{Device, DeviceState};

#[derive(Debug)]
pub struct Thermometer {
    name: String,
    state: DeviceState,
}

impl Device for Thermometer {
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

impl Thermometer {
    fn measure_temperature(&self) -> i32 {
        match self.state {
            DeviceState::On => rand::thread_rng().gen_range(-30..40),
            DeviceState::Off => 0,
        }
    }
}

impl Display for Thermometer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Socket name: {}\nstate: {}\ncurrent temperature: {}",
            self.name,
            self.state,
            self.measure_temperature()
        )
    }
}
