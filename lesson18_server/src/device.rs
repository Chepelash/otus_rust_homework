use std::fmt::{Debug, Display};

/// Trait for house devices
pub trait Device: Display + Debug {
    /// Change device status to DeviceState::On
    fn turn_on(&mut self);
    /// Change device status to DeviceState::Off
    fn turn_off(&mut self);
    /// Returns device name
    fn name(&self) -> &str;
    /// Returns device state
    fn state(&self) -> &DeviceState;
    /// Returns device report
    fn get_report(&self) -> String {
        self.to_string()
    }
}

/// Enum for device state
#[derive(Debug, Default)]
pub enum DeviceState {
    On,
    #[default]
    Off,
}

impl Display for DeviceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceState::On => write!(f, "On"),
            DeviceState::Off => write!(f, "Off"),
        }
    }
}
