use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use sdl2::video::WindowContext;
use sdl2::render::{Texture, TextureCreator};
use sdl2::image::LoadTexture;
use std::collections::HashMap;
use std::rc::Rc;

use crate::scenes::Scene;
use crate::scenes::battle::Battle;
use crate::scenes::overworld::Overworld;

use crate::events::event_subsystem::{EventSystem, GameEvent};
use crate::video::video_subsystem::VideoSystem;

pub enum GameState {
	Running,
	Quit,
}

pub struct GameManager<'a> {
	overworld: Box<dyn Scene + 'a>,
	battle: Box<dyn Scene + 'a>,
	game_state: GameState,
	event_subsystem: EventSystem,
	video_subsystem: VideoSystem,
}

impl<'a> GameManager<'a> {

	fn update(&mut self) {
		let game_event = self.event_subsystem.update();

		match game_event {
			Some(GameEvent::WindowClose) => self.game_state = GameState::Quit,
			Some(GameEvent::MouseClick(x, y)) => println!("X: {}, Y: {}", x, y),
			Some(GameEvent::KeyPress(k)) => println!("Key {}", k),
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

	pub fn init(sdl_context: &Sdl, wincan: WindowCanvas, texture_manager: &'a mut TextureManager) -> Result<Self, String> {

		let overworld = Box::new(Overworld::init(texture_manager)?);
		let battle = Box::new(Battle::init(texture_manager)?);

		let event_subsystem = EventSystem::init(sdl_context)?;
		let video_subsystem = VideoSystem::init(wincan);

		Ok(GameManager {
			overworld,
			battle,
			game_state: GameState::Running,
			event_subsystem,
			video_subsystem,
		})
	}
}


pub struct TextureManager<'l> {
	cache: HashMap<String, Rc<Texture<'l>>>,
	loader: &'l TextureCreator<WindowContext>,
}

impl<'l> TextureManager<'l> {
	pub fn new(loader: &'l TextureCreator<WindowContext>) -> Self {
		TextureManager {
			cache: HashMap::new(),
			loader,
		}
	}

	pub fn load(&mut self, path: &str) -> Result<Rc<Texture<'l>>, String> {
		self.cache.get(path).cloned().map_or_else(
			|| {
				let texture = Rc::new(self.loader.load_texture(path)?);
				self.cache.insert(path.to_string(), texture.clone());
				Ok(texture)
			},
			Ok,
		)
	}
}
