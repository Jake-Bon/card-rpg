
use std::io;
use std::io::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::net::TcpStream;


use sdl2::render::WindowCanvas;

use crate::EventSystem;
use crate::scenes::{Scene, GameEvent};

pub struct Online {
	buffer: [u8; 1024],
	wincan: Rc<RefCell<WindowCanvas>>,
	event_system: Rc<RefCell<EventSystem>>,
}

impl Scene for Online {

		fn handle_input(&mut self, _: GameEvent) {

		}

		fn render(&mut self) -> Result<(), String> {
			let mut buffer = [0; 1024];
			let server_data = poll_server(&mut buffer);

			Ok(())
		}

}

impl Online {

	pub fn init(wincan: Rc<RefCell<WindowCanvas>>, event_system: Rc<RefCell<EventSystem>>) -> Self {

		let buffer = [0; 1024];

		Online {
			buffer,
			wincan,
			event_system,
		}
	}
}

async fn poll_server(buffer: &mut [u8; 1024]) -> io::Result<()> {
	read(buffer).await?;
	println!("data: {}", String::from_utf8_lossy(buffer));
	Ok(())
}

async fn read(buf: &mut [u8]) -> io::Result<usize> {
	let mut connection = TcpConnection::connect("127.0.0.1:7878").await?;
	connection.stream.read(buf)
}

struct TcpConnection {
	stream: TcpStream,
}

impl TcpConnection {

	async fn connect(addr: &str) -> io::Result<TcpConnection> {
		let stream = TcpStream::connect(addr)?;
		Ok(TcpConnection {
			stream,
		})
	}
}


