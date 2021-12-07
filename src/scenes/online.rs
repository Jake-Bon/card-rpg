
use std::net::IpAddr;
use std::net::Ipv4Addr;

use std::task::{Waker, RawWaker, RawWakerVTable};
use std::io;
use std::io::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
//use std::net::SocketAddr;
use std::net::{TcpStream, Shutdown, SocketAddr};

use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};

use sdl2::pixels::Color;
use sdl2::render::{Texture, WindowCanvas};
use crate::game_manager::TextureManager;
use crate::video::text::FontManager;

use crate::EventSystem;
use crate::scenes::{Scene, GameEvent};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TurnData {
	pub turn_id: u16,
	pub card_ids: u16,
}

pub struct Online<'a> {
	buffer: [u8; 4096],
	waker: Waker,
	connected: bool,
	//connection: Option<TcpConnection>,
	tcp_connection: Option<TcpStream>,
	poll_instant: Instant,
	wincan: Rc<RefCell<WindowCanvas>>,
	event_system: Rc<RefCell<EventSystem>>,
	font_manager: Rc<RefCell<FontManager<'a>>>,
	return_button: Rc<Texture<'a>>,
	started: bool,
}

impl Scene for Online<'_> {

		fn handle_input(&mut self, event: GameEvent) {
            match event {
                GameEvent::MouseClick(x_pos, y_pos) => {
                    if self.connected {
                        if (x_pos > 10 && x_pos < 410) && (y_pos > 580 && y_pos < 700) {
                            //send_str = "Quit".to_string();
                            self.tcp_connection.as_ref().unwrap().shutdown(Shutdown::Both);
                            self.tcp_connection = None;
                            self.event_system.borrow().change_scene(0).unwrap();
                            return;
                        }
                    }
                },
				GameEvent::PushCard(chosen) => {
                    if self.connected {
                        let mut send_str = TurnData{turn_id: 0, card_ids: chosen as u16};

                        let mut tcp_con = self.tcp_connection.as_ref().unwrap();
						println!("Attempting to send out {} to remote player", chosen);
                        tcp_con.write_all(serde_json::to_string(&send_str).unwrap().as_bytes());
                        // tcp_con.flush();
                        println!("switching back to battle now");
						self.event_system.borrow().change_scene(2).unwrap();
                    }
                },
				GameEvent::PollFromBattle() => {
					let mut buffer = [0; 4096];
				    match &self.tcp_connection {
				        Some(T) => {
				            //println!("there's a connection");

				            self.connected = true;

				            let mut stream = T;
				            match stream.read(&mut buffer) {
				                Ok(T) => {
				                    if T > 0 { // use this to ignore duplicate data-> && String::from_utf8_lossy(&self.buffer) != String::from_utf8_lossy(&buffer) {
				                        self.buffer = buffer;
				                        match serde_json::from_str::<TurnData>(&String::from_utf8_lossy(&buffer).trim_matches(char::from(0))) {
				                        	Ok(data) => { println!("Success!: {:?}", data); self.event_system.borrow().receive_online(data);},
				                        	Err(e) => { println!("{}", e.to_string()); println!("Received data: {}", String::from_utf8_lossy(&buffer).trim_matches(char::from(0))); }
				                        }
				                    }
				                    // if T (the number of bytes read) is equal to 0, this means that the stream has reached the end of file marker, and the stream was closed. Need to reconnect
				                    else {
				                        println!("Connection lost or closed!");
				                        self.connected = false;
				                        self.tcp_connection = None;
				                    }
				                },
				                Err(ref e) => { /*println!("No data to receive! Would have blocked!");*/ },
				                Err(e) => { println!("Something else went wrong!: {}", e); },
				            }

				        },
				        None => { println!("no connection yet, trying again..."); self.tcp_connection = attempt_connection(); },
				    }
				    self.poll_instant = Instant::now();
				//let mut server_data = poll_server(&mut buffer);
				//let mut cx = Context::from_waker(&self.waker);
				//let pin = unsafe { Pin::new_unchecked(&mut server_data) };
				//pin.poll(&mut cx);
	            
				}
                GameEvent::OnlineTurn(turn_id, card_id) => {
					println!("In online.rs: GameEvent::OnlineTurn() event handlers: Received card_id: {}", card_id);
                	// let data: &mut TurnData = unsafe { &mut *(turn_data as *mut TurnData)};
					if self.started{
						// switch to the battle scene
						println!("switching to the battle scene to start the battle...");
						self.event_system.borrow().change_scene(2).unwrap();
						self.event_system.borrow().set_card_to_play(card_id as u32).unwrap();
					}else{
						self.started = true;
						if(card_id==0){
							// set as player 1
							println!("setting this player as player 1");
							self.event_system.borrow().change_scene(2).unwrap();
							self.event_system.borrow().set_online(1).unwrap();
						}else{
						    println!("setting this player as player 2");
							self.event_system.borrow().change_scene(2).unwrap();
							self.event_system.borrow().set_online(2).unwrap();
						}

					}
                	println!("Turn Data after {}, {}", turn_id, card_id);
                },
                _ => {},
            }

		}

		fn render(&mut self) -> Result<(), String> {
			// because the connection is set to nonblocking once it's established, it would call read every frame.
			// The interval at which it checks for new data can be changed via the Duration. Currently set to check every half second, which may still be too often honestly
			if self.poll_instant.elapsed() >= Duration::from_millis(500) {
			    let mut buffer = [0; 4096];
			    match &self.tcp_connection {
			        Some(T) => {
			            //println!("there's a connection");

			            self.connected = true;

			            let mut stream = T;
			            match stream.read(&mut buffer) {
			                Ok(T) => {
			                    if T > 0 { // use this to ignore duplicate data-> && String::from_utf8_lossy(&self.buffer) != String::from_utf8_lossy(&buffer) {
			                        self.buffer = buffer;
			                        match serde_json::from_str::<TurnData>(&String::from_utf8_lossy(&buffer).trim_matches(char::from(0))) {
			                        	Ok(data) => { println!("Success!: {:?}", data); self.event_system.borrow().receive_online(data);},
			                        	Err(e) => { println!("{}", e.to_string()); println!("Received data: {}", String::from_utf8_lossy(&buffer).trim_matches(char::from(0))); }
			                        }
			                    }
			                    // if T (the number of bytes read) is equal to 0, this means that the stream has reached the end of file marker, and the stream was closed. Need to reconnect
			                    else {
			                        println!("Connection lost or closed!");
			                        self.connected = false;
			                        self.tcp_connection = None;
			                    }
			                },
			                Err(ref e) => { /*println!("No data to receive! Would have blocked!");*/ },
			                Err(e) => { println!("Something else went wrong!: {}", e); },
			            }

			        },
			        None => { println!("no connection yet, trying again..."); self.tcp_connection = attempt_connection(); },
			    }
			    self.poll_instant = Instant::now();
			//let mut server_data = poll_server(&mut buffer);
			//let mut cx = Context::from_waker(&self.waker);
			//let pin = unsafe { Pin::new_unchecked(&mut server_data) };
			//pin.poll(&mut cx);
            }

            // online screen
            let mut wincan = self.wincan.borrow_mut();

            crate::video::gfx::fill_screen(&mut wincan, Color::RGB(0, 120, 150))?;

            crate::video::gfx::draw_sprite(&mut wincan, &self.return_button, (10, 580));

            let mut fontm = self.font_manager.borrow_mut();
            fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 48, Color::RGB(0, 0, 0),
					"Client->Server->Client Demo", (550, 10));
            let bufStr = std::str::from_utf8(&self.buffer);
            //println!("");
            if self.connected  {
                fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 48, Color::RGB(0, 0, 0),
					    "Connected to server!", (10, 100));
			    fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 48, Color::RGB(0, 0, 0),
					    "Waiting for second player...", (10, 160));
				//fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 48, Color::RGB(0, 0, 0),
				//	    "Then, see terminals for more!", (10, 220));
		    }



            wincan.present();

			Ok(())
		}

}

impl <'a> Online<'a> {

	pub fn init(texture_manager: Rc<RefCell<TextureManager<'a>>>, wincan: Rc<RefCell<WindowCanvas>>, event_system: Rc<RefCell<EventSystem>>, font_manager: Rc<RefCell<FontManager<'a>>>) -> Self {

		let buffer = [0; 4096];
		let raw_waker = RawWaker::new(&(), &VTABLE);
		let waker = unsafe {Waker::from_raw(raw_waker)};
		let return_button = texture_manager.borrow_mut().load("assets/return.png").unwrap();

		Online {
			waker,
			buffer,
			//connection: None,
			connected: false,
			tcp_connection: None,
			poll_instant: Instant::now(),
			wincan,
			event_system,
			font_manager,
			return_button,
			started: false,
		}
	}
}

fn attempt_connection() -> Option<TcpStream> {

    //match TcpStream::connect_timeout(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(18, 212, 232, 174)), 7878), Duration::from_secs(5)) { // localhost
    match TcpStream::connect_timeout(&SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7878), Duration::from_secs(5)) {
    //match TcpStream::connect_timeout(&socketAddr::from(([34, 227, 148, 203], 76567)), Duration::from_secs(5)) {
        Ok(T) => { T.set_nonblocking(true).expect("couldn't set stream T as nonblocking"); println!("there's a connection"); return Some(T); }, // setting the stream as nonblocking means calls to read() won't block, allowing us to check however often we want without multithreading
        Err(E) => { println!("Failed to connect! Error: {}", E); return None; },
    }
    //return Some(tcp_stream);
}

//fn read(buf: &mut [u8]) -> io::Result<usize> {

//}

/*
async fn poll_server(buffer: &mut [u8; 1024]) -> io::Result<()> {
	read(buffer).await?;
	println!("data: {}", String::from_utf8_lossy(buffer));
	Ok(())
}
*/

/*
async fn read(buf: &mut [u8]) -> io::Result<usize> {
	let mut connection = TcpConnection::connect("127.0.0.1:7878").await?;
	connection.stream.read(buf)
}
*/

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
