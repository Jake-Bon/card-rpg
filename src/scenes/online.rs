
use std::pin::Pin;
use std::task::{Context, Waker, RawWaker, RawWakerVTable};
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
	buffer: [u8; 1024],
	waker: Waker,
	connection: Option<TcpConnection>,
	wincan: Rc<RefCell<WindowCanvas>>,
	event_system: Rc<RefCell<EventSystem>>,
}

impl Scene for Online {

		fn handle_input(&mut self, _: GameEvent) {

		}

		fn render(&mut self) -> Result<(), String> {
			let mut buffer = [0; 1024];
			let mut server_data = poll_server(&mut buffer);
			let mut cx = Context::from_waker(&self.waker);
			let pin = unsafe { Pin::new_unchecked(&mut server_data) };
			pin.poll(&mut cx);

			Ok(())
		}

}

impl Online {

	pub fn init(wincan: Rc<RefCell<WindowCanvas>>, event_system: Rc<RefCell<EventSystem>>) -> Self {

		let buffer = [0; 1024];
		let raw_waker = RawWaker::new(&(), &VTABLE);
		let waker = unsafe {Waker::from_raw(raw_waker)}; 

		Online {
			waker,
			buffer,
			connection: None,
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

impl Unpin for TcpConnection {}

unsafe fn vt_clone(data: *const()) -> RawWaker {
	RawWaker::new(data, &VTABLE)
}

unsafe fn vt_wake(data: *const()) {}
unsafe fn vt_wake_by_ref(data: *const()) {}
unsafe fn vt_drop(data: *const()) {}

static VTABLE: RawWakerVTable = RawWakerVTable::new(
	vt_clone,
	vt_wake,
	vt_wake_by_ref,
	vt_drop,
);


