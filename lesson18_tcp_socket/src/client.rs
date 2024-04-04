use std::{
    io::{self, BufReader, BufWriter, Read, Write},
    net::TcpStream,
    thread,
    time::Duration,
};

pub struct Client {
    address: String,
}

impl Client {
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
        }
    }
    pub fn get_device_names(&mut self) -> io::Result<Vec<String>> {
        let stream = TcpStream::connect(&self.address)?;
        thread::sleep(Duration::from_secs(1));
        let mut reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        let r = writer.write("GET / HTTP1/1\n".as_bytes());
writer.flush()?;
        dbg!(r);
        let mut buf = String::new();
        let r = reader.read_to_string(&mut buf)?;
        dbg!(r);
        dbg!(buf);
        Ok(vec![])
    }
    pub fn get_status_all(&self) -> io::Result<()> {
        Ok(())
    }
    pub fn get_status_device(&self, device_name: &str) -> io::Result<()> {
        Ok(())
    }
    pub fn turn_on_device(&self, device_name: &str) -> io::Result<()> {
        Ok(())
    }
    pub fn turn_off_device(&self, device_name: &str) -> io::Result<()> {
        Ok(())
    }
}
