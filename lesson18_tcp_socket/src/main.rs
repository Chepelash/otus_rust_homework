use std::io;

mod command;
mod device;
mod devices;
mod request;
mod server;
use devices::thermo::Thermometer;

fn main() -> io::Result<()> {
    let mut server = server::Server::new("127.0.0.1", 9871)?;
    println!("Connected to 127.0.0.1:9871");

    let mut thermo1 = Thermometer::new("thermo1");
    server.add_device(&mut thermo1).expect("should be unique");
    let mut thermo2 = Thermometer::new("thermo2");
    server.add_device(&mut thermo2).expect("should be unique");
    server.run()?;
    Ok(())
}
