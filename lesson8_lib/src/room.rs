use std::fmt::Display;

use crate::device::Device;
use anyhow::{anyhow, Ok, Result};

/// Room struct
///
/// Stores pointers to device.
/// Device name should be unique
#[derive(Debug)]
pub struct Room<'a> {
    /// Room name
    name: String,
    /// Vec to store pointers to devices
    devices: Vec<&'a mut dyn Device>,
}

impl<'a> Room<'a> {
    /// Creates new room with name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            devices: vec![],
        }
    }
    /// Returns room name
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Adds device to room
    ///
    /// Returns `Ok(())` if `device_name` is unique, `Err` with description otherwise
    ///
    pub fn add_device(&mut self, device: &'a mut dyn Device) -> Result<()> {
        if self.devices.iter().any(|d| d.name() == device.name()) {
            return Err(anyhow!("Device with name {} exists!", device.name()));
        }
        self.devices.push(device);
        Ok(())
    }
    /// Removes device from room
    ///
    /// Returns `Ok(())` if `device_name` is found, `Err` with description otherwise
    ///
    pub fn remove_device(&mut self, device_name: &str) -> Result<()> {
        if self.devices.iter().any(|d| d.name() == device_name) {
            self.devices.retain(|d| d.name() != device_name);
            return Ok(());
        }
        Err(anyhow!("Device with name {} does not exists!", device_name))
    }
    /// Returns room report with all internal devices
    pub fn get_report(&self) -> String {
        self.to_string()
    }
    /// Returns device report
    ///
    /// Returns `Ok(String)` if `device_name` is found, `Err` with description otherwise
    ///
    pub fn get_device_report(&self, device_name: &str) -> Result<String> {
        let dev = self.devices.iter().find(|d| d.name() == device_name);
        if dev.is_none() {
            return Err(anyhow!("Device with name {} does not exists!", device_name));
        }
        Ok(dev.unwrap().get_report())
    }
    /// Returns vec with devices' names    
    pub fn get_devices(&self) -> Vec<&str> {
        self.devices.iter().map(|d| d.name()).collect()
    }
    /// Returns devices' reports
    fn get_devices_report(&self) -> String {
        self.devices.iter().map(|d| d.get_report()).collect()
    }
    /// Turns on a device
    ///
    /// Returns `Ok(())` if `device_name` is found, `Err` with description otherwise
    ///
    pub fn turn_on(&mut self, device_name: &str) -> Result<()> {
        let dev = self.devices.iter_mut().find(|d| d.name() == device_name);
        if dev.is_none() {
            return Err(anyhow!("Device with name {} does not exists!", device_name));
        }
        dev.unwrap().turn_on();
        Ok(())
    }
    /// Turns off a device
    ///
    /// Returns `Ok(())` if `device_name` is found, `Err` with description otherwise
    ///
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

#[cfg(test)]
mod tests {
    use crate::Socket;

    use super::*;
    const ROOM_NAME: &str = "room";
    const DEVICE_NAME: &str = "dev";
    #[test]
    fn add_device() {
        let mut room = Room::new(ROOM_NAME);
        let mut device = Socket::new(DEVICE_NAME);
        assert!(room.add_device(&mut device).is_ok());
        assert_eq!(room.get_devices().len(), 1);
    }
    #[test]
    fn add_device_with_non_unique_name() {
        let mut room = Room::new(ROOM_NAME);
        let mut device = Socket::new(DEVICE_NAME);
        assert!(room.add_device(&mut device).is_ok());
        let mut device_duplicate = Socket::new(DEVICE_NAME);
        assert!(room.add_device(&mut device_duplicate).is_err());
    }
    #[test]
    fn remove_device() {
        let mut room = Room::new(ROOM_NAME);
        let mut device = Socket::new(DEVICE_NAME);
        assert!(room.add_device(&mut device).is_ok());
        assert!(room.remove_device(DEVICE_NAME).is_ok());
        assert!(room.get_devices().is_empty());
    }
    #[test]
    fn remove_non_existing_device() {
        let mut room = Room::new(ROOM_NAME);
        assert!(room.remove_device(DEVICE_NAME).is_err());
    }
    #[test]
    fn turn_on_off_device() {
        let mut room = Room::new(ROOM_NAME);
        let mut device = Socket::new(DEVICE_NAME);
        room.add_device(&mut device).unwrap();
        assert!(room.turn_on(DEVICE_NAME).is_ok());
        assert!(room.turn_off(DEVICE_NAME).is_ok());
    }
    #[test]
    fn turn_on_off_non_existing_device() {
        let mut room = Room::new(ROOM_NAME);
        assert!(room.turn_on(DEVICE_NAME).is_err());
        assert!(room.turn_off(DEVICE_NAME).is_err());
    }
}
