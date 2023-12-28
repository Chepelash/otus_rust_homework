use std::fmt::Display;

use crate::device::Device;
use anyhow::{anyhow, Ok, Result};

#[derive(Debug)]
pub struct Room<'a> {
    name: String,
    devices: Vec<&'a mut dyn Device>,
}

impl<'a> Room<'a> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            devices: vec![],
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn add_device(&mut self, device: &'a mut dyn Device) -> Result<()> {
        if self.devices.iter().any(|d| d.name() == device.name()) {
            return Err(anyhow!("Device with name {} exists!", device.name()));
        }
        self.devices.push(device);
        Ok(())
    }
    pub fn remove_device(&mut self, device_name: &str) -> Result<()> {
        if self.devices.iter().any(|d| d.name() == device_name) {
            self.devices.retain(|d| d.name() != device_name);
            return Ok(());
        }
        Err(anyhow!("Device with name {} does not exists!", device_name))
    }
    pub fn get_report(&self) -> String {
        self.to_string()
    }
    pub fn get_device_report(&self, device_name: &str) -> Result<String> {
        let dev = self.devices.iter().find(|d| d.name() == device_name);
        if dev.is_none() {
            return Err(anyhow!("Device with name {} does not exists!", device_name));
        }
        Ok(dev.unwrap().get_report())
    }
    pub fn get_devices(&self) -> Vec<&str> {
        self.devices.iter().map(|d| d.name()).collect()
    }
    fn get_devices_report(&self) -> String {
        self.devices.iter().map(|d| d.get_report()).collect()
    }
    pub fn turn_on(&mut self, device_name: &str) -> Result<()> {
        let dev = self.devices.iter_mut().find(|d| d.name() == device_name);
        if dev.is_none() {
            return Err(anyhow!("Device with name {} does not exists!", device_name));
        }
        dev.unwrap().turn_on();
        Ok(())
    }
    pub fn turn_off(&mut self, device_name: &str) -> Result<()> {
        let dev = self.devices.iter_mut().find(|d| d.name() == device_name);
        if dev.is_none() {
            return Err(anyhow!("Device with name {} does not exists!", device_name));
        }
        dev.unwrap().turn_off();
        Ok(())
    }
}

impl<'a> Display for Room<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Room name: {}\n\tdevices: [\n{}]\n",
            self.name,
            self.get_devices_report()
        )
    }
}
