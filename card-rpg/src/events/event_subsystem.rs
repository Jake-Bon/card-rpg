use sdl2::{Sdl, EventPump};

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
}