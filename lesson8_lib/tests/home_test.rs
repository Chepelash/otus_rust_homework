use lesson8_lib::*;

const HOME_NAME: &str = "home";
const ROOM_NAME: &str = "room";
const DEVICE_NAME: &str = "dev";
#[test]
fn add_room() {
    let mut home = Home::new(HOME_NAME);
    assert!(home.add_room(ROOM_NAME).is_ok());
    assert_eq!(home.get_room_names().len(), 1);
}
#[test]
fn add_non_unique_room() {
    let mut home = Home::new(HOME_NAME);
    assert!(home.add_room(ROOM_NAME).is_ok());
    assert!(home.add_room(ROOM_NAME).is_err());
    assert_eq!(home.get_room_names().len(), 1);
}
#[test]
fn remove_room() {
    let mut home = Home::new(HOME_NAME);
    assert!(home.add_room(ROOM_NAME).is_ok());
    assert_eq!(home.get_room_names().len(), 1);
    assert!(home.remove_room(ROOM_NAME).is_ok());
    assert_eq!(home.get_room_names().len(), 0);
}
#[test]
fn remove_non_existing_room() {
    let mut home = Home::new(HOME_NAME);
    assert_eq!(home.get_room_names().len(), 0);
    assert!(home.remove_room(ROOM_NAME).is_err());
}

#[test]
fn add_device() {
    let mut home = Home::new(HOME_NAME);
    assert!(home.add_room(ROOM_NAME).is_ok());
    let mut device = Socket::new(DEVICE_NAME);
    assert!(home.add_device(ROOM_NAME, &mut device).is_ok());
    let devices_in_room = home.get_devices_in_room(ROOM_NAME);
    assert!(devices_in_room.is_ok());
    let dev_vec = devices_in_room.unwrap();
    assert_eq!(dev_vec.len(), 1);
    assert_eq!(dev_vec[0], DEVICE_NAME.to_string());
}
#[test]
fn add_device_with_existing_name() {
    let mut home = Home::new(HOME_NAME);
    assert!(home.add_room(ROOM_NAME).is_ok());
    let mut device = Socket::new(DEVICE_NAME);
    assert!(home.add_device(ROOM_NAME, &mut device).is_ok());
    let mut device_duplicate = Socket::new(DEVICE_NAME);
    assert!(home.add_device(ROOM_NAME, &mut device_duplicate).is_err());
}

#[test]
fn remove_device() {
    let mut home = Home::new(HOME_NAME);
    assert!(home.add_room(ROOM_NAME).is_ok());
    let mut device = Socket::new(DEVICE_NAME);
    assert!(home.add_device(ROOM_NAME, &mut device).is_ok());
    let devices_in_room = home.get_devices_in_room(ROOM_NAME);
    assert!(devices_in_room.is_ok());
    let dev_vec = devices_in_room.unwrap();
    assert_eq!(dev_vec.len(), 1);
    assert_eq!(dev_vec[0], format!("{}", DEVICE_NAME));

    let device_info = DeviceInfo::new(DEVICE_NAME, ROOM_NAME);
    assert!(home.remove_device(&device_info).is_ok());
    let devices_in_room = home.get_devices_in_room(ROOM_NAME);
    assert!(devices_in_room.is_ok());
    let dev_vec = devices_in_room.unwrap();
    assert_eq!(dev_vec.len(), 0);
}
#[test]
fn remove_non_existing_device() {
    let mut home = Home::new(HOME_NAME);
    assert!(home.add_room(ROOM_NAME).is_ok());
    let devices_in_room = home.get_devices_in_room(ROOM_NAME);
    assert!(devices_in_room.is_ok());
    let dev_vec = devices_in_room.unwrap();
    assert_eq!(dev_vec.len(), 0);

    let device_info = DeviceInfo::new(DEVICE_NAME, ROOM_NAME);
    assert!(home.remove_device(&device_info).is_err());
}

#[test]
fn turn_on_off_device() {
    let mut home = Home::new(HOME_NAME);
    assert!(home.add_room(ROOM_NAME).is_ok());
    let mut device = Socket::new(DEVICE_NAME);
    home.add_device(ROOM_NAME, &mut device).unwrap();

    let device_info = DeviceInfo::new(DEVICE_NAME, ROOM_NAME);
    assert!(home.turn_on(&device_info).is_ok());
    assert!(home.turn_off(&device_info).is_ok());
}
#[test]
fn turn_on_off_non_existing_device() {
    let mut home = Home::new(HOME_NAME);
    assert!(home.add_room(ROOM_NAME).is_ok());
    let device_info = DeviceInfo::new(DEVICE_NAME, ROOM_NAME);
    assert!(home.turn_on(&device_info).is_err());
    assert!(home.turn_off(&device_info).is_err());
}
