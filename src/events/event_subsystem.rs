use std::os::raw::c_void;
use sdl2::{Sdl, EventPump, EventSubsystem};
use sdl2::keyboard::Keycode;
use sdl2::event::Event as SDL_Event;

pub struct EventSystem {
	event_pump: EventPump,
	event_subsystem: EventSubsystem,
	scene_change_event_id: u32,
}

impl EventSystem {

	pub fn update(&mut self) -> Vec<Option<GameEvent>> {

		let mut game_events:Vec<Option<GameEvent>> = Vec::new();
		let mut i = 0;
		for event in self.event_pump.poll_iter() {
			i += 1;
			match event {
				SDL_Event::Quit{..} => game_events.push(Some(GameEvent::WindowClose)),
				SDL_Event::MouseButtonDown{x: x_pos, y: y_pos, ..} => game_events.push(Some(GameEvent::MouseClick(x_pos, y_pos))),
				//SDL_Event::MouseMotion{x: x_pos, y: y_pos, ..} => Some(GameEvent::MouseHover(x_pos, y_pos)),
				SDL_Event::KeyDown{keycode: Some(k), ..} => {
					game_events.push(Some(GameEvent::KeyPress(k)));
					println!("ESS: Key down");
				},
				SDL_Event::KeyUp{keycode: Some(k), ..} => {
					game_events.push(Some(GameEvent::KeyRelease(k)));
					println!("ESS: Key up");
				},
				SDL_Event::User{code: scene_change_event_id, data1: scene_id, ..} => game_events.push(Some(GameEvent::SceneChange(scene_id as u32))),
				_ => game_events.push(None),
			}
		}
		if i > 0 {println!("{}", i);}
		return game_events;
	}

	pub fn init(sdl_context: &Sdl) -> Result<Self, String> {
		let event_pump = sdl_context.event_pump()?;
		let event_subsystem = sdl_context.event()?;

		let scene_change_event_id = unsafe { event_subsystem.register_event()? };

		Ok(EventSystem {
			event_pump,
			event_subsystem,
			scene_change_event_id,
		})
	}

	pub fn change_scene(&self, scene_id: u32) -> Result<(), String>{
		let event = sdl2::event::Event::User {
			timestamp: 0,
			window_id: 0,
			type_: self.scene_change_event_id,
			code: 200,
			data1: scene_id as *mut c_void,
			data2: 0x5678 as *mut c_void,
		};

		self.event_subsystem.push_event(event)?;
		Ok(())
	}
}

pub enum GameEvent {
	WindowClose,
	SceneChange(u32),
	MouseClick(i32, i32),
	MouseHover(i32, i32),
	KeyPress(Keycode),
	KeyRelease(Keycode),
}
