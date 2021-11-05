use std::io::{Write, Result};
use std::thread;
use std::time::Duration;
use std::net::{TcpListener, TcpStream};

fn main() -> Result<()>{
    accept("127.0.0.1:7878")?;
    Ok(())
}

fn handle_connection(stream: &mut TcpStream) -> Result<()>{
    //thread::sleep(Duration::from_secs(4));
    stream.write(b"Hi")?;
    Ok(())
}

fn accept(address: &str) -> Result<()> {
    let listener = TcpListener::bind(address)?;
    for stream in listener.incoming() {
        let mut stream = stream?;
        handle_connection(&mut stream)?;
        println!("Connection from: {}", stream.peer_addr()?);
    }
    Ok(())
}
