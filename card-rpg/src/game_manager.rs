use crate::video::sdl_core::SDLCore;
use crate::events::event_subsystem::EventSubsystem;
use crate::video::video_subsystem::VideoSubsystem;

pub enum GameState {
	Running,
	Quit,
}

pub struct GameManager {
	game_state: GameState,
	event_subsystem: EventSubsystem,
	video_subsystem: VideoSubsystem,
}

impl GameManager {

	pub fn init() -> Result<Self, String> {
		let sdl_core = SDLCore::init()?;
		let event_subsystem = EventSubsystem::init(&sdl_core.sdl_context)?;
		let video_subsystem = VideoSubsystem::init(sdl_core.wincan);

		Ok(GameManager {
			game_state: GameState::Running,
			event_subsystem,
			video_subsystem,
		})
	}

	pub fn start_state_machine(&mut self) {

		'running: loop {
			match self.game_state {
				GameState::Quit => break 'running,
				GameState::Running => { self.event_subsystem.update(); self.video_subsystem.update(); },
			}
		}
	}
}
