use crate::video::sdl_core::SDLCore;
use crate::events::event_subsystem::{EventSubsystem, GameEvent};
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

	fn update(&mut self) {
		let game_event = self.event_subsystem.update();

		match game_event {
			Some(GameEvent::WindowClose) => self.game_state = GameState::Quit,
			Some(GameEvent::MouseClick(x, y)) => println!("X: {}, Y: {}", x, y),
			None => {},
		}

		self.video_subsystem.update();
	}

	pub fn start_state_machine(&mut self) {

		'running: loop {
			match self.game_state {
				GameState::Quit => break 'running,
				GameState::Running => self.update(),
			}
		}
	}

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
}
