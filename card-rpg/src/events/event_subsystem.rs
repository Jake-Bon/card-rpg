use sdl2::{Sdl, EventPump};
use sdl2::event::Event as SDL_Event;

pub struct EventSubsystem {
	event_pump: EventPump,
}

impl EventSubsystem {

	pub fn update(&mut self) -> Option<GameEvent> {
		for event in self.event_pump.poll_iter() {
			return match event {
				SDL_Event::Quit{..} => Some(GameEvent::WindowClose),
				SDL_Event::MouseButtonDown{x: x_pos, y: y_pos, ..} => Some(GameEvent::MouseClick(x_pos, y_pos)),
				_ => None,
			}
		}

		None
	}

	pub fn init(sdl_context: &Sdl) -> Result<Self, String> {
		let event_pump = sdl_context.event_pump()?;

		Ok(EventSubsystem {
			event_pump,
		})
	}
}

pub trait Listener {
	fn invoke(&self);
}


pub enum GameEvent {
	WindowClose,
	MouseClick(i32, i32),
}