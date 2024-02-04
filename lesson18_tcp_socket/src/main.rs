use std::io;

mod server;

fn main() -> io::Result<()> {
    let mut server = server::Server::new("127.0.0.1", 9871)?;
    println!("Connected to 127.0.0.1:9871");
    server.run()?;
    Ok(())
}
