
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;
use std::pin::Pin;
use std::io;
use std::io::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::net::TcpStream;
use std::future::Future;

use sdl2::render::WindowCanvas;

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
			let stream = unsafe {Pin::new_unchecked(&mut self.stream)};
			let cx = Context::from_waker(&waker);
			match stream.poll(&mut &mut cx) {
				Poll::Ready(value) => println!("Data: {}", String::from_utf8_lossy(&value)),
				Poll::Pending => {},
			}
			Ok(())
		}

}

impl Online {

	pub fn init(wincan: Rc<RefCell<WindowCanvas>>, event_system: Rc<RefCell<EventSystem>>) -> Self {

		let stream = TcpConnection::connect("127.0.0.1:7878").unwrap();

		Online {
			stream,
			wincan,
			event_system,
		}
	}
}

struct TcpConnection {
	stream: TcpStream,
}

impl Future for TcpConnection {
	type Output = [u8; 1024];
	fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
		let mut buffer = [0; 1024];
		if self.stream.read(&mut buffer).unwrap() > 0 {
			return Poll::Ready(buffer)
		}
		Poll::Pending
	}
}

impl TcpConnection {
	fn connect(addr: &str) -> io::Result<Self> {
		let stream = TcpStream::connect(addr)?;

		Ok(TcpConnection {
			stream,
		})
	}
}
