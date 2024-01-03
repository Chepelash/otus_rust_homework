use std::fmt::Display;

use crate::{
    device::{Device, DeviceInfo},
    room::Room,
};
use anyhow::{anyhow, Ok, Result};

/// Home struct
/// 
/// Stores vector of rooms.
/// Room names should be unique
#[derive(Debug)]
pub struct Home<'a> {
    /// Home name
    name: String,
    /// vector of rooms
    rooms: Vec<Room<'a>>,
}

impl<'a> Home<'a> {
    /// Returns new home
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rooms: vec![],
        }
    }
    /// Adds new room
    /// 
    /// Returns `Ok(())` if `room_name` is unique, `Err` otherwise
    pub fn add_room(&mut self, room_name: &str) -> Result<()> {
        if self.rooms.iter().any(|r| r.name() == room_name) {
            return Err(anyhow!("Room with name {} exists!", room_name));
        }
        let room = Room::new(room_name);
        self.rooms.push(room);
        Ok(())
    }
    /// Removes a room
    /// 
    /// Returns `Ok(())` if `room_name` is found, `Err` otherwise
    pub fn remove_room(&mut self, room_name: &str) -> Result<()> {
        if self.rooms.iter().any(|r| r.name() == room_name) {
            self.rooms.retain(|el| el.name() != room_name);
            return Ok(());
        }
        Err(anyhow!("Room with name {} does not exists!", room_name))
    }
    /// Adds device
    /// 
    /// Returns `Ok(())` if `room_name` is exists and `device.name` is unique, 
    /// `Err` otherwise
    pub fn add_device(&mut self, room_name: &str, device: &'a mut dyn Device) -> Result<()> {
        let room = self.rooms.iter_mut().find(|r| r.name() == room_name);
        if room.is_none() {
            return Err(anyhow!("Room with name {} does not exists!", room_name));
        }
        let room = room.unwrap();
        room.add_device(device)
    }
    /// Removes device
    /// 
    /// Returns `Ok(())` if `device_info.room_name` and `device_info.device_name` exist,
    /// `Err` otherwise
    pub fn remove_device(&mut self, device_info: &DeviceInfo) -> Result<()> {
        let room = self
            .rooms
            .iter_mut()
            .find(|r| r.name() == device_info.room_name);
        if room.is_none() {
            return Err(anyhow!(
                "Room with name {} does not exists!",
                &device_info.room_name
            ));
        }
        let room = room.unwrap();
        room.remove_device(&device_info.device_name)
    }
    /// Returns vector with room names
    pub fn get_room_names(&self) -> Vec<&str> {
        self.rooms.iter().map(|r| r.name()).collect()
    }
    /// Returns home report with all rooms and devices
    pub fn get_home_report(&self) -> String {
        self.to_string()
    }
    /// Get device report
    /// 
    /// Returns `Ok(String)` if `device_info.room_name` and `device_info.device_name` exist,
    /// `Err` otherwise
    pub fn get_device_report(&self, device_info: &DeviceInfo) -> Result<String> {
        let room = self
            .rooms
            .iter()
            .find(|r| r.name() == device_info.room_name);
        if room.is_none() {
            return Err(anyhow!(
                "Room with name {} does not exists!",
                &device_info.room_name
            ));
        }
        let room = room.unwrap();
        room.get_device_report(&device_info.device_name)
    }
    /// Get devices reports
    /// 
    /// Returns vec with results.
    /// `Ok(String)` if `device_info.room_name` and `device_info.device_name` exist,
    /// `Err` otherwise
    pub fn get_devices_report(&self, device_infos: Vec<&DeviceInfo>) -> Vec<Result<String>> {
        device_infos
            .iter()
            .map(|di| self.get_device_report(di))
            .collect()
    }
    /// Returns `Ok(Vec<&str>)` if `room_name` exists, `Err` otherwise
    pub fn get_devices_in_room(&self, room_name: &str) -> Result<Vec<&str>> {
        let room = self.rooms.iter().find(|r| r.name() == room_name);
        if room.is_none() {
            return Err(anyhow!("Room with name {} does not exists!", room_name));
        }
        Ok(room.unwrap().get_devices())
    }
    /// Turns on a device
    /// 
    /// Returns `Ok(())` if `device_info.room_name` and `device_info.device_name` exist,
    /// `Err` otherwise
    pub fn turn_on(&mut self, device_info: &DeviceInfo) -> Result<()> {
        let room = self
            .rooms
            .iter_mut()
            .find(|r| r.name() == device_info.room_name);
        if room.is_none() {
            return Err(anyhow!(
                "Room with name {} does not exists!",
                &device_info.room_name
            ));
        }
        let room = room.unwrap();
        room.turn_on(&device_info.device_name)
    }
    /// Turns off a device
    /// 
    /// Returns `Ok(())` if `device_info.room_name` and `device_info.device_name` exist,
    /// `Err` otherwise
    pub fn turn_off(&mut self, device_info: &DeviceInfo) -> Result<()> {
        let room = self
            .rooms
            .iter_mut()
            .find(|r| r.name() == device_info.room_name);
        if room.is_none() {
            return Err(anyhow!(
                "Room with name {} does not exists!",
                &device_info.room_name
            ));
        }
        let room = room.unwrap();
        room.turn_off(&device_info.device_name)
    }
    /// Get reports from all rooms
    fn get_rooms_report(&self) -> String {
        self.rooms.iter().map(|r| r.get_report()).collect()
    }
}

impl<'a> Display for Home<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Home name: {}\nrooms: [\n{}]",
            self.name,
            self.get_rooms_report()
        )
    }
}
