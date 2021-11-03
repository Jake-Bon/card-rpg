use std::rc::Rc;
use std::cell::RefCell;
use std::io::prelude::*;
use std::net::TcpStream;

use sdl2::render::WindowCanvas;

use futures::executor::block_on;

use crate::EventSystem;
use crate::scenes::{Scene, GameEvent};

pub struct Online {
	stream: Option<TcpStream>,
	wincan: Rc<RefCell<WindowCanvas>>,
	event_system: Rc<RefCell<EventSystem>>,
}

impl Scene for Online {

		fn handle_input(&mut self, _: GameEvent) { todo!() }

		fn render(&mut self) -> Result<(), String> {

			// If no connection open, open one
			if self.stream.is_none() {
				self.stream = Some(TcpStream::connect("127.0.0.1:7878").expect("Connection Failed"));
			}

			block_on(self.update_from_server());

			Ok(())
		}

}

impl Online {

	async fn update_from_server(&mut self) {
		match &mut self.stream {
			None => println!("No open connection"),
			Some(stream) => {
				let mut buffer = [0; 1024];
				stream.read(&mut buffer).unwrap();
				println!("Response: {}", String::from_utf8_lossy(&buffer[..]));
			},
		}
	}

	pub fn init(wincan: Rc<RefCell<WindowCanvas>>, event_system: Rc<RefCell<EventSystem>>) -> Self {
		Online {
			stream: None,
			wincan,
			event_system,
		}
	}
}