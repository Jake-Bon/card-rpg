use std::io::{Write, Result};
use std::thread;
use std::time::Duration;
use std::net::{TcpListener, TcpStream};

use futures::stream::Stream;
use futures::executor::block_on;

fn main() {
    block_on(accept("127.0.0.1:7878"));
}

fn handle_connection(mut stream: TcpStream) {
    thread::sleep(Duration::from_secs(4));
    stream.write(b"Hello");
}

async fn accept(address: &str) -> Result<()> {
    let listener = TcpListener::bind(address)?;
    for stream in listener.incoming() {
        let stream = stream?;
        println!("Connection from: {}", stream.peer_addr()?);
    }
    Ok(())
}
