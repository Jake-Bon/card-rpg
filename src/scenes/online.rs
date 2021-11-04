use std::pin::Pin;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::prelude::*;
use std::net::TcpStream;

use sdl2::render::WindowCanvas;

use futures::Future;
use futures::task::*;
use futures::executor::{block_on, LocalPool};

use crate::EventSystem;
use crate::scenes::{Scene, GameEvent};

pub struct Online {
	// pool: LocalPool,
	wincan: Rc<RefCell<WindowCanvas>>,
	event_system: Rc<RefCell<EventSystem>>,
}

impl Scene for Online {

		fn handle_input(&mut self, _: GameEvent) {

		}

		fn render(&mut self) -> Result<(), String> {
			// self.pool.run();
			Ok(())
		}

}

impl Online {

	pub fn init(wincan: Rc<RefCell<WindowCanvas>>, event_system: Rc<RefCell<EventSystem>>) -> Self {
		
		// let mut pool = LocalPool::new();

		Online {
			// pool,
			wincan,
			event_system,
		}
	}
}

async fn try_run(address: &str) -> Result<(), String> {
	let stream = TcpStream::connect(address);
	Ok(())
}

