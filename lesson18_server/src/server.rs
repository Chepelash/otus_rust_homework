use std::{
    io::{self, BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
    str::FromStr,
};

use crate::{device::Device, request::Request, response::Response};

pub struct Server<'a> {
    address: String,
    devices: Vec<&'a mut dyn Device>,
}

impl<'a> Server<'a> {
    pub fn new(address: &str, port: u32) -> io::Result<Self> {
        Ok(Self {
            address: format!("{}:{}", address, port),
            devices: vec![],
        })
    }

    /// Adds device to device vector
    /// Returns `Err(String)` if device name is not unique
    pub fn add_device(&mut self, device: &'a mut dyn Device) -> Result<(), String> {
        if self.devices.iter().any(|d| d.name() == device.name()) {
            return Err(format!("Device with name {} exists!", device.name()));
        }
        self.devices.push(device);
        Ok(())
    }
    pub fn run(&mut self) -> io::Result<()> {
        let listener = TcpListener::bind(self.address.as_str())?;
        for stream in listener.incoming() {
            let mut stream = stream?;
            let request = Self::get_request(&mut stream)?;
            let request = Request::from_str(request.as_str()).unwrap();
            let response = match request {
                Request::GetDeviceNames => {
                    if self.devices.is_empty() {
                        Response::Ok {
                            result: Some("".to_string()),
                        }
                    } else {
                        let result = self
                            .devices
                            .iter()
                            .map(|d| d.name())
                            .collect::<Vec<&str>>()
                            .join(", ");
                        Response::Ok {
                            result: Some(result),
                        }
                    }
                }
                Request::StatusAll => {
                    if self.devices.is_empty() {
                        Response::Ok {
                            result: Some("".to_string()),
                        }
                    } else {
                        Response::Ok {
                            result: Some(
                                self.devices
                                    .iter()
                                    .map(|d| d.to_string())
                                    .collect::<Vec<String>>()
                                    .join(";;;"),
                            ),
                        }
                    }
                }
                Request::StatusDevice { device_name } => {
                    let dev = self.devices.iter().find(|d| d.name() == device_name);
                    if let Some(dev) = dev {
                        Response::Ok {
                            result: Some(dev.to_string()),
                        }
                    } else {
                        Response::Error {
                            reason: format!("Device with name '{device_name}' does not exist"),
                        }
                    }
                }
                Request::TurnOn { device_name } => {
                    let dev = self.devices.iter_mut().find(|d| d.name() == device_name);
                    if let Some(dev) = dev {
                        dev.turn_on();
                        Response::Ok { result: None }
                    } else {
                        Response::Error {
                            reason: format!("Device with name '{device_name}' does not exist"),
                        }
                    }
                }
                Request::TurnOff { device_name } => {
                    let dev = self.devices.iter_mut().find(|d| d.name() == device_name);
                    if let Some(dev) = dev {
                        dev.turn_off();
                        Response::Ok { result: None }
                    } else {
                        Response::Error {
                            reason: format!("Device with name '{device_name}' does not exist"),
                        }
                    }
                }
                _ => Response::Error {
                    reason: "".to_string(),
                },
            };
            Self::send_response(&mut stream, response.to_string().as_str())?;
        }
        Ok(())
    }
    fn get_request(stream: &mut TcpStream) -> io::Result<String> {
        let mut buf_reader = BufReader::new(stream);
        let mut request_line = String::new();
        let _ = buf_reader.read_line(&mut request_line)?;
        Ok(request_line)
    }
    fn send_response(stream: &mut TcpStream, response: &str) -> io::Result<()> {
        let mut writer = BufWriter::new(stream);
        writer.write_all(response.to_string().as_bytes())?;
        Ok(())
    }
}
