use std::str::FromStr;


pub enum Request {
    GetDeviceNames,
    StatusAll,
    StatusDevice { device_name: String },
    TurnOn { device_name: String },
    TurnOff { device_name: String },
    Error { reason: String },
}

impl FromStr for Request {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut collection = s.split_whitespace();
        let command = collection.next().unwrap_or_default();
        let device_name = collection.next();
        match command {
            "get_device_names" => Ok(Self::GetDeviceNames),
            "status_all" => Ok(Self::StatusAll),
            "status_device" => match device_name {
                Some(device_name) => Ok(Self::StatusDevice {
                    device_name: device_name.to_string(),
                }),
                None => Ok(Self::Error {
                    reason: "No device name specified".to_string(),
                }),
            },
            "turn_on" => match device_name {
                Some(device_name) => Ok(Self::TurnOn {
                    device_name: device_name.to_string(),
                }),
                None => Ok(Self::Error {
                    reason: "No device name specified".to_string(),
                }),
            },
            "turn_off" => match device_name {
                Some(device_name) => Ok(Self::TurnOff {
                    device_name: device_name.to_string(),
                }),
                None => Ok(Self::Error {
                    reason: "No device name specified".to_string(),
                }),
            },
            _ => Ok(Self::Error {
                reason: "Unknown command".to_string(),
            }),
        }
    }
}
