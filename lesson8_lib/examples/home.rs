use lesson8_lib::{DeviceInfo, Home, Socket, Thermometer};

fn main() {
    let home_name = "home";
    let room1_name = "room1";

    let socket1_name = "socket1";
    let thermo1_name = "thermo1";

    // create devices with names unique in room
    let mut socket1 = Socket::new(socket1_name);
    let mut thermo1 = Thermometer::new(thermo1_name);
    // create home
    let mut home = Home::new(home_name);
    // add room with unique name
    home.add_room(room1_name).unwrap();
    // add devices to existing room
    home.add_device(room1_name, &mut socket1).unwrap();
    home.add_device(room1_name, &mut thermo1).unwrap();

    println!("Home report: {}", home.get_home_report());
    println!(
        "Vec of devices in room '{}': {:?}",
        room1_name,
        home.get_devices_in_room(room1_name).unwrap()
    );
    println!("Rooms in house: {:?}", home.get_room_names());

    // get report from existing device in existing room
    let dev_info = DeviceInfo::new(socket1_name, room1_name);
    let report = home.get_device_report(&dev_info);

    println!(
        "Report from room '{}', device '{}': {}",
        room1_name,
        socket1_name,
        report.unwrap()
    );

    // turn on existing device and get report
    home.turn_on(&dev_info).unwrap();
    let report = home.get_device_report(&dev_info);
    println!(
        "Report from room '{}', turned on device '{}': {}",
        room1_name,
        socket1_name,
        report.unwrap()
    );
}
