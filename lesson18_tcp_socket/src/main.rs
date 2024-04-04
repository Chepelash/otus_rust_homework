use std::{io, thread, time::Duration};

mod client;
mod command;
mod device;
mod devices;
mod request;
mod server;
use devices::socket::Socket;

use crate::client::Client;

fn main() -> io::Result<()> {
    let mut server = server::Server::new("127.0.0.1", 9871)?;
    println!("Connected to 127.0.0.1:9871");

    let mut socket1 = Socket::new("socket1");
    server.add_device(&mut socket1).expect("should be unique");
    let mut socket2 = Socket::new("socket2");
    server.add_device(&mut socket2).expect("should be unique");
    thread::spawn(|| {
        thread::sleep(Duration::from_secs(5));
        let mut client = Client::new("127.0.0.1:9871");
        let _ = client.get_device_names();
        
    });
    server.run()?;
    Ok(())
}
