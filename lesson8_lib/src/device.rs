use std::fmt::{Debug, Display};

pub trait Device: Display + Debug {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn name(&self) -> &str;
    fn state(&self) -> &DeviceState;
    fn get_report(&self) -> String {
        self.to_string()
    }
}

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

pub struct DeviceInfo {
    pub device_name: String,
    pub room_name: String,
}
impl DeviceInfo {
    pub fn new(device_name: &str, room_name: &str) -> Self {
        Self {
            device_name: device_name.to_string(),
            room_name: room_name.to_string(),
        }
    }
}
