use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use sdl2::Sdl;
use sdl2::image::LoadTexture;
use sdl2::video::WindowContext;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};

use crate::scenes::Scene;
use crate::scenes::battle::Battle;
use crate::scenes::overworld::Overworld;
//use crate::scenes::menu::Menu; // <-- implement with scene change
use crate::cards::card_system;

use crate::events::event_subsystem::{EventSystem, GameEvent};

pub enum GameState {
	Running,
	Quit,
}

pub struct GameManager<'a> {
	overworld: Box<dyn Scene + 'a>,
	battle: Box<dyn Scene + 'a>,
	//menu: Box<dyn Scene + 'a>,  // <-- implement with scene change
	game_state: GameState,
	event_subsystem: EventSystem,
	//video_subsystem: VideoSystem<'a>,
}

impl<'a> GameManager<'a> {

	fn update(&mut self) -> Result<(), String>{
		let game_event = self.event_subsystem.update();

		match game_event {
			Some(GameEvent::WindowClose) => self.game_state = GameState::Quit,
			Some(e) => self.overworld.handle_input(e), // <-- implement with scene change... somehow...
			None => {},
		}

		self.overworld.render()?;

		Ok(())
	}

	pub fn start_state_machine(&mut self) {

		'running: loop {
			match self.game_state {
				GameState::Quit => break 'running,
				GameState::Running => self.update().unwrap(),
			};
		}
	}

	pub fn init(sdl_context: &Sdl, wincan: &'a mut WindowCanvas, texture_manager: Rc<RefCell<TextureManager<'a>>>) -> Result<Self, String> {

		let overworld = Box::new(Overworld::init(Rc::clone(&texture_manager), wincan)?);
		//let menu = Box::new(Menu::init(Rc::clone(&texture_manager), wincan)?);
		let battle = Box::new(Battle::init(Rc::clone(&texture_manager))?);

		let event_subsystem = EventSystem::init(sdl_context)?;

		Ok(GameManager {
			overworld,
			battle,
			//menu,  // <-- implement with scene change
			game_state: GameState::Running,
			event_subsystem,
			//video_subsystem,
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
