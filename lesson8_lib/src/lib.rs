mod device;
mod devices;
mod home;
mod room;

pub use home::Home;
#[cfg(test)]
mod tests {
    use crate::{devices::{socket::Socket, thermo::Thermometer}, device::DeviceInfo};

    use super::*;

    #[test]
    fn it_works() {
        let home_name = "home";
        let room1_name = "room1";
        let room2_name = "room2";
        let socket1_name = "socket1";
        let thermo1_name = "thermo1";


        let mut socket1 = Socket::new(socket1_name);
        let mut thermo1 = Thermometer::new(thermo1_name);
        let mut home = Home::new(home_name);
        assert!(home.add_room(room1_name).is_ok());
        assert!(home.add_room(room1_name).is_err());
        assert!(home.add_device(room1_name, &mut socket1).is_ok());
        assert!(home.add_device(room1_name, &mut thermo1).is_ok());
        let mut failed_device = Socket::new(socket1_name);
        assert!(home.add_device(room1_name, &mut failed_device).is_err());

        assert!(home.add_room(room2_name).is_ok());
        let mut socket2 = Socket::new(socket1_name);
        let mut thermo2 = Thermometer::new(thermo1_name);
        assert!(home.add_device(room2_name, &mut socket2).is_ok());
        assert!(home.add_device(room2_name, &mut thermo2).is_ok());

        println!("{}", home.get_home_report());
        println!("{:?}", home.get_devices_in_room(room1_name).unwrap());
        println!("{:?}", home.get_room_names());

        let mut dev_info = DeviceInfo::new(socket1_name, room1_name);
        let report = home.get_device_report(&dev_info);
        assert!(report.is_ok());
        println!("{}", report.unwrap());

        

        assert!(home.turn_on(&dev_info).is_ok());
        let report = home.get_device_report(&dev_info);
        assert!(report.is_ok());
        println!("{}", report.unwrap());


        dev_info.device_name = "failed".to_string();
        assert!(home.get_device_report(&dev_info).is_err());
    }
}
