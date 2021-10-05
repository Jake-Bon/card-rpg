use crate::video::sdl_core::SDLCore;
use crate::events::event_subsystem::EventSubsystem;

enum GameState {
	Running,
	Quit,
}

pub struct GameManager {
	game_state: GameState,
	event_subsystem: EventSubsystem,
}

impl GameManager {

	fn change_state(&mut self, game_state: GameState) {
		self.game_state = game_state;
	}

	pub fn init() -> Result<Self, String> {

		let sdl_core = SDLCore::init()?;
		let event_subsystem = EventSubsystem::init(&sdl_core.sdl_context)?;

		Ok(GameManager {
			game_state: GameState::Running,
			event_subsystem,
		})
	}

	pub fn start_state_machine(&self) {

		'running: loop {
			match self.game_state {
				GameState::Quit => break 'running,
				GameState::Running => {},
			}
		}
	}
}
