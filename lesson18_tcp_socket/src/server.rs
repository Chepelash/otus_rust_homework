use std::{
    io::{self, BufRead, BufReader, Write},
    net::TcpListener,
};

use build_html::{Html, HtmlContainer, HtmlPage};
use lesson8_lib::Device;

pub struct Server<'a> {
    listener: TcpListener,
    devices: Vec<&'a mut dyn Device>,
}

impl<'a> Server<'a> {
    pub fn new(address: &str, port: u32) -> io::Result<Self> {
        let listener = TcpListener::bind(format!("{}:{}", address, port))?;
        Ok(Self {
            listener,
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
        for stream in self.listener.incoming() {
            let mut int_stream = stream?;
            let buf_reader = BufReader::new(&mut int_stream);
            let request_line = buf_reader
                .lines()
                .next()
                .unwrap_or(Ok("GET / HTTP/1.1".to_string()))?;
            let request = Self::parse_request(request_line.as_str());
            match request {
                Request {
                    req_type: RequestType::Get,
                    command: Command::TurnOn { device_name },
                } => self.turn_on_device(int_stream, &device_name)?,
                Request {
                    req_type: RequestType::Get,
                    command: Command::TurnOff { device_name },
                } => {
                    let dev = self.devices.iter_mut().find(|d| d.name() == device_name);
                    if dev.is_none() {
                        self.write_error(
                            int_stream,
                            &format!("Device with name {} does not exist", device_name),
                        )?;
                        return Ok(());
                    }
                    let dev = dev.unwrap();
                    dev.turn_on();
                    self.turn_off_device(int_stream, &device_name)?
                }
                Request {
                    req_type: RequestType::Get,
                    command: Command::GetStatus,
                } => self.write_state_all(&mut int_stream)?,
                Request {
                    req_type: RequestType::Get,
                    command: Command::ShowMain,
                } => self.write_hello(&mut int_stream)?,
                Request {
                    req_type: RequestType::Get,
                    command: Command::Error { error_msg },
                } => self.write_error(&mut int_stream, &error_msg)?,
                Request {
                    req_type: RequestType::Get,
                    command: Command::GetDeviceStatus { device_name },
                } => self.write_state_device(&mut int_stream, &device_name)?,
                _ => (),
            }
        }
        Ok(())
    }
    fn parse_request(request_line: &str) -> Request {
        todo!()
        // Request {
        //     req_type: RequestType::Get,
        //     command: Command::TurnOn {
        //         device_name: "ttt".to_string(),
        //     },
        // }
    }
    fn write_hello<T>(&self, mut stream: T) -> io::Result<()>
    where
        T: Write,
    {
        let content = HtmlPage::new()
            .with_header(1, "Main Page")
            .with_paragraph("instructions here")
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
            .with_paragraph(
                self.devices
                    .iter()
                    .map(|el| el.to_string())
                    .collect::<Vec<String>>()
                    .join("\n"),
            )
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
            .with_paragraph(dev.to_string())
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
            .with_paragraph(&format!("Device {} is on", device_name))
            .to_html_string();
        let status_line = "HTTP/1.1 200 OK";
        let content_length = content.len();
        let response =
            format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{content}");
        stream.write_all(response.as_bytes())?;
        Ok(())
    }
    fn turn_off_device<T>(&self, mut stream: T, device_name: &str) -> io::Result<()>
    where
        T: Write,
    {
        // let dev = self.devices.iter_mut().find(|d| d.name() == device_name);
        // if dev.is_none() {
        //     self.write_error(
        //         stream,
        //         &format!("Device with name {} does not exist", device_name),
        //     )?;
        //     return Ok(());
        // }
        // let dev = dev.unwrap();
        // dev.turn_on();
        let content = HtmlPage::new()
            .with_header(1, "Turn off device")
            .with_paragraph(&format!("Device {} is off", device_name))
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
            .to_html_string();
        let status_line = "HTTP/1.1 503 OK";
        let content_length = content.len();
        let response =
            format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{content}");
        stream.write_all(response.as_bytes())?;
        Ok(())
    }
}

struct Request {
    req_type: RequestType,
    command: Command,
}

enum Command {
    TurnOn { device_name: String },
    TurnOff { device_name: String },
    GetStatus,
    GetDeviceStatus { device_name: String },
    ShowMain,
    Error { error_msg: String },
}

enum RequestType {
    Get,
    Post,
    Del,
    Put,
}
