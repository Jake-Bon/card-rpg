use std::io::Write;
use std::thread;
use std::time::Duration;
use std::net::{TcpListener, TcpStream};
use futures::stream::Stream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream)
    }

    println!("Connection Established")
}

fn handle_connection(mut stream: TcpStream) {
    thread::sleep(Duration::from_secs(4));
    stream.write(b"Hello");
}
