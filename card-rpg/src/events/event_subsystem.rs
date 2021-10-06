use crate::game_manager::GameState;

use sdl2::{Sdl, EventPump};
use sdl2::event::Event as SDL_Event;

pub struct EventSubsystem {
	event_pump: EventPump,
}

impl EventSubsystem {

	pub fn init(sdl_context: &Sdl) -> Result<Self, String> {
		let event_pump = sdl_context.event_pump()?;

		Ok(EventSubsystem {
			event_pump,
		})
	}

	pub fn pump_events(&mut self, game_state: &mut GameState) {
		for event in self.event_pump.poll_iter() {
			match event {
				SDL_Event::Quit{..} => *game_state = GameState::Quit,
				_ => {},
			}
		}
	}
}

pub trait Listener {
	fn invoke(&self);
}