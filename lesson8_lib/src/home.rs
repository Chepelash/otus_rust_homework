use std::fmt::Display;

use crate::{
    device::{Device, DeviceInfo},
    room::Room,
};
use anyhow::{anyhow, Ok, Result};

#[derive(Debug)]
pub struct Home<'a> {
    name: String,
    rooms: Vec<Room<'a>>,
}

impl<'a> Home<'a> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rooms: vec![],
        }
    }
    pub fn add_room(&mut self, room_name: &str) -> Result<()> {
        if self.rooms.iter().any(|r| r.name() == room_name) {
            return Err(anyhow!("Room with name {} exists!", room_name));
        }
        let room = Room::new(room_name);
        self.rooms.push(room);
        Ok(())
    }
    pub fn remove_room(&mut self, room_name: &str) -> Result<()> {
        if self.rooms.iter().any(|r| r.name() == room_name) {
            self.rooms.retain(|el| el.name() != room_name);
            return Ok(());
        }
        Err(anyhow!("Room with name {} does not exists!", room_name))
    }
    pub fn add_device(&mut self, room_name: &str, device: &'a mut dyn Device) -> Result<()> {
        let room = self.rooms.iter_mut().find(|r| r.name() == room_name);
        if room.is_none() {
            return Err(anyhow!("Room with name {} does not exists!", room_name));
        }
        let room = room.unwrap();
        room.add_device(device)
    }
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
    pub fn get_room_names(&self) -> Vec<&str> {
        self.rooms.iter().map(|r| r.name()).collect()
    }
    pub fn get_home_report(&self) -> String {
        self.to_string()
    }
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
    pub fn get_devices_report(&self, device_infos: Vec<&DeviceInfo>) -> Vec<Result<String>> {
        device_infos
            .iter()
            .map(|di| self.get_device_report(di))
            .collect()
    }
    pub fn get_devices_in_room(&self, room_name: &str) -> Result<Vec<&str>> {
        let room = self.rooms.iter().find(|r| r.name() == room_name);
        if room.is_none() {
            return Err(anyhow!("Room with name {} does not exists!", room_name));
        }
        Ok(room.unwrap().get_devices())
    }
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
