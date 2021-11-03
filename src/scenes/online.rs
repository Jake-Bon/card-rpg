use std::pin::Pin;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::prelude::*;
use std::net::TcpStream;

use sdl2::render::WindowCanvas;

use futures::Future;
use futures::task::*;
use futures::executor::block_on;

use crate::EventSystem;
use crate::scenes::{Scene, GameEvent};

pub struct Online {
	stream: TcpConnection,
	wincan: Rc<RefCell<WindowCanvas>>,
	event_system: Rc<RefCell<EventSystem>>,
}

impl Scene for Online {

		fn handle_input(&mut self, _: GameEvent) {

		}

		fn render(&mut self) -> Result<(), String> {

			Ok(())
		}

}

impl Online {
	pub fn init(wincan: Rc<RefCell<WindowCanvas>>, event_system: Rc<RefCell<EventSystem>>) -> Self {
		Online {
			stream: TcpConnection::init("127.0.0.1:7878"),
			wincan,
			event_system,
		}
	}
}


struct TcpConnection {
	stream: TcpStream,
	buffer: [u8; 1024],
}

impl TcpConnection {
	fn init(address: &str) -> Self {
		let mut stream = TcpStream::connect(address).unwrap();
		let mut buffer = [0; 1024];

		TcpConnection {
			stream,
			buffer,
		}

	}
}

impl Future for TcpConnection {	
	type Output = [u8; 1024];
	fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
		let mut stream = &self.stream;
		let mut buffer = self.buffer;
		if stream.read(&mut buffer).unwrap() > 0 {
			return Poll::Ready(self.buffer)
		}

		Poll::Pending
	}
}

impl Read for TcpConnection {
	fn read(&mut self, buffer: &mut [u8]) -> Result<usize, std::io::Error> {
		self.stream.read(buffer)
	}
}