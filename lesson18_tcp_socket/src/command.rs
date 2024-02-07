use std::str::FromStr;


#[derive(Debug, Default)]
pub enum Command {
    TurnOn {
        device_name: String,
    },
    TurnOff {
        device_name: String,
    },
    GetStatus,
    GetDeviceStatus {
        device_name: String,
    },
    #[default]
    ShowMain,
    Error {
        error_msg: String,
    },
    Ignore,
}
impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        dbg!("Request: {s}");
        
        let mut collection = s.split('/');
        dbg!(&collection);
        // should be '/' symb
        collection.next();
        let section = collection.next();
        if section.is_none() {
            return Ok(Command::Error { error_msg: "Cannot parse request".to_string() });
        }
        let section = section.unwrap().trim().to_lowercase();
        let dev_name = collection.next();
        match section.as_str() {
            "" => Ok(Command::ShowMain),
            "turn_on" => {
                if dev_name.is_none() {
                    return Err(());
                }
                let device_name = dev_name.unwrap();
                Ok(Command::TurnOn {
                    device_name: device_name.to_string(),
                })
            }
            "turn_off" => {
                if dev_name.is_none() {
                    return Err(());
                }
                let device_name = dev_name.unwrap();
                Ok(Command::TurnOff {
                    device_name: device_name.to_string(),
                })
            }
            "status_all" => Ok(Command::GetStatus),
            "status_device" => {
                if dev_name.is_none() {
                    return Err(());
                }
                let device_name = dev_name.unwrap();
                Ok(Command::GetDeviceStatus {
                    device_name: device_name.to_string(),
                })
            }
            "flavicon.ico" => Ok(Command::Ignore),
            _ => Ok(Command::Error {
                error_msg: "Unknown request".to_string(),
            }),
        }
    }
}
