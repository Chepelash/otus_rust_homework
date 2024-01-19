pub mod home_errors {
    use std::{error::Error, fmt::Display};

    use super::room_errors::{self, RoomErrors};

    #[derive(Debug)]
    pub enum HomeErrors {
        RoomNameExists(String),
        RoomNameDoesNotExist(String),
        InternalError(room_errors::RoomErrors),
    }

    impl From<RoomErrors> for HomeErrors {
        fn from(value: RoomErrors) -> Self {
            HomeErrors::InternalError(value)
        }
    }

    impl Display for HomeErrors {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    HomeErrors::InternalError(value) => value.to_string(),
                    HomeErrors::RoomNameExists(name) =>
                        format!("Room with name {} already exists!", name),
                    HomeErrors::RoomNameDoesNotExist(name) =>
                        format!("Room with name {} does not exist!", name),
                }
            )
        }
    }

    impl Error for HomeErrors {}
}

pub(crate) mod room_errors {
    use std::{error::Error, fmt::Display};

    #[derive(Debug)]
    pub enum RoomErrors {
        DeviceNameExists(String),
        DeviceNameDoesNotExist(String),
    }

    impl Display for RoomErrors {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Device with name {}",
                match self {
                    RoomErrors::DeviceNameExists(name) => format!("{} {}", name, "already exists!"),
                    RoomErrors::DeviceNameDoesNotExist(name) =>
                        format!("{} {}", name, "does not exist"),
                }
            )
        }
    }

    impl Error for RoomErrors {}
}
