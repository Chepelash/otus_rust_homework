use std::{
    io::{self, BufRead, BufReader, Write},
    net::TcpListener,
    str::FromStr,
};

use build_html::{Html, HtmlContainer, HtmlPage};
use lesson8_lib::Device;

use crate::{
    command::Command,
    request::{Request, RequestType},
};

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
            let buf_reader = BufReader::new(&mut stream);
            let request_line = buf_reader
                .lines()
                .next()
                .unwrap_or(Ok("GET / HTTP/1.1".to_string()))?;
            let request = Self::parse_request(request_line.as_str()).unwrap_or_default();
            dbg!(&request);
            match request {
                Request {
                    req_type: RequestType::Get,
                    command: Command::TurnOn { device_name },
                } => self.turn_on_device(stream, &device_name)?,
                Request {
                    req_type: RequestType::Get,
                    command: Command::TurnOff { device_name },
                } => self.turn_off_device(stream, &device_name)?,
                Request {
                    req_type: RequestType::Get,
                    command: Command::GetStatus,
                } => self.write_state_all(&mut stream)?,
                Request {
                    req_type: RequestType::Get,
                    command: Command::ShowMain,
                } => self.write_hello(&mut stream)?,
                Request {
                    req_type: RequestType::Get,
                    command: Command::Error { error_msg },
                } => self.write_error(&mut stream, &error_msg)?,
                Request {
                    req_type: RequestType::Get,
                    command: Command::GetDeviceStatus { device_name },
                } => self.write_state_device(&mut stream, &device_name)?,
                Request {
                    req_type: _,
                    command: Command::Ignore,
                } => (),
                _ => self.write_error(&mut stream, "Page not found")?,
            }
        }
        Ok(())
    }
    fn parse_request(request_line: &str) -> Result<Request, ()> {
        let mut collection = request_line.split_whitespace();
        let req_type = collection.next().unwrap_or("GET");
        let req_type = RequestType::from_str(req_type)?;
        let path = collection.next().unwrap_or("/");
        let command = Command::from_str(path)?;
        Ok(Request::new(req_type, command))
    }
    fn write_hello<T>(&self, mut stream: T) -> io::Result<()>
    where
        T: Write,
    {
        let content = HtmlPage::new()
            .with_header(1, "Main Page")
            .with_header(2, "Devices")
            .with_paragraph({
                let devs: Vec<String> = self.devices.iter().map(|d| d.name().to_string()).collect();
                match devs.is_empty() {
                    true => "No devices registered".to_string(),
                    false => devs.join(", "),
                }
            })
            .with_header(2, "Urls")
            .with_paragraph("/status_all - get statuses of all devices")
            .with_paragraph("/status_device/{device_name} - get status of device")
            .with_paragraph("/turn_on/{device_name} - turn on device")
            .with_paragraph("/turn_off/{device_name} - turn off device")
            .to_html_string();
        let status_line = "HTTP/1.1 200 OK";
        let content_length = content.len();
        let response =
            format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{content}");
        stream.write_all(response.as_bytes())?;
        Ok(())
    }
    fn write_state_all<T>(&self, mut stream: T) -> io::Result<()>
    where
        T: Write,
    {
        let content = HtmlPage::new()
            .with_header(1, "All devices status")
            .with_preformatted(match self.devices.is_empty() {
                false => self
                    .devices
                    .iter()
                    .map(|el| el.to_string())
                    .collect::<Vec<String>>()
                    .join("\n"),
                true => "No devices registered".to_string(),
            })
            .with_link("/", "Return home")
            .to_html_string();
        let status_line = "HTTP/1.1 200 OK";
        let content_length = content.len();
        let response =
            format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{content}");
        stream.write_all(response.as_bytes())?;
        Ok(())
    }
    fn write_state_device<T>(&self, mut stream: T, device_name: &str) -> io::Result<()>
    where
        T: Write,
    {
        let dev = self.devices.iter().find(|d| d.name() == device_name);
        if dev.is_none() {
            self.write_error(
                stream,
                &format!("Device with name {} does not exist", device_name),
            )?;
            return Ok(());
        }
        let dev = dev.unwrap();
        let content = HtmlPage::new()
            .with_header(1, "Device state")
            .with_preformatted(dev.to_string())
            .with_link("/", "Return home")
            .to_html_string();
        let status_line = "HTTP/1.1 200 OK";
        let content_length = content.len();
        let response =
            format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{content}");
        stream.write_all(response.as_bytes())?;
        Ok(())
    }
    fn turn_on_device<T>(&mut self, mut stream: T, device_name: &str) -> io::Result<()>
    where
        T: Write,
    {
        let dev = self.devices.iter_mut().find(|d| d.name() == device_name);
        if dev.is_none() {
            self.write_error(
                stream,
                &format!("Device with name {} does not exist", device_name),
            )?;
            return Ok(());
        }
        let dev = dev.unwrap();
        dev.turn_on();
        let content = HtmlPage::new()
            .with_header(1, "Turn on device")
            .with_paragraph(format!("Device {} is on", device_name))
            .with_link("/", "Return home")
            .to_html_string();
        let status_line = "HTTP/1.1 200 OK";
        let content_length = content.len();
        let response =
            format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{content}");
        stream.write_all(response.as_bytes())?;
        Ok(())
    }
    fn turn_off_device<T>(&mut self, mut stream: T, device_name: &str) -> io::Result<()>
    where
        T: Write,
    {
        let dev = self.devices.iter_mut().find(|d| d.name() == device_name);
        if dev.is_none() {
            self.write_error(
                stream,
                &format!("Device with name {} does not exist", device_name),
            )?;
            return Ok(());
        }
        let dev = dev.unwrap();
        dev.turn_on();
        let content = HtmlPage::new()
            .with_header(1, "Turn off device")
            .with_paragraph(format!("Device {} is off", device_name))
            .with_link("/", "Return home")
            .to_html_string();
        let status_line = "HTTP/1.1 200 OK";
        let content_length = content.len();
        let response =
            format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{content}");
        stream.write_all(response.as_bytes())?;
        Ok(())
    }
    fn write_error<T>(&self, mut stream: T, error_msg: &str) -> io::Result<()>
    where
        T: Write,
    {
        let content = HtmlPage::new()
            .with_header(1, "Error")
            .with_paragraph(format!("Error message: {}", error_msg))
            .with_link("/", "Return home")
            .to_html_string();
        let status_line = "HTTP/1.1 503 OK";
        let content_length = content.len();
        let response =
            format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{content}");
        stream.write_all(response.as_bytes())?;
        Ok(())
    }
}
