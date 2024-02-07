use std::fmt::Display;

use rand::Rng;

use crate::device::{Device, DeviceState};

/// Exampte thermometer
#[derive(Debug)]
pub struct Thermometer {
    /// Device name
    name: String,
    /// Device state
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
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: DeviceState::default(),
        }
    }
    /// Dummy function for measuring temperature
    fn measure_temperature(&self) -> Option<i32> {
        match self.state {
            DeviceState::On => Some(rand::thread_rng().gen_range(-30..40)),
            DeviceState::Off => None,
        }
    }
}

impl Display for Thermometer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Socket name: {}\nstate: {}\ncurrent temperature: {}\n",
            self.name,
            self.state,
            self.measure_temperature().unwrap_or_default()
        )
    }
}
