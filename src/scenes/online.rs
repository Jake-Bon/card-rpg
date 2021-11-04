use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

use sdl2::render::WindowCanvas;

use futures::Future;
use futures::executor::ThreadPool;

use crate::EventSystem;
use crate::scenes::{Scene, GameEvent};

pub struct Online {
	//pool: ThreadPool,
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

		//let pool = ThreadPool::new().expect("Failed to create thread pool");
		Online {
			//pool,
			wincan,
			event_system,
		}
	}
}


struct TcpConnection {
	watcher: Arc<TcpStream>,
}

impl TcpConnection {
	pub async fn connect(address: &str) -> io::Result<TcpConnection> {
		let watcher = Arc::new(TcpStream::connect(address)?);
		Ok(TcpConnection {
			watcher,
		})
	}

	async fn poll_server() -> io::Result<()> {
		let stream = TcpConnection::connect("127.0.0.1:7878").await;
		let (reader, mut writer) = (&stream, &stream);
		Ok(())
	}

}

