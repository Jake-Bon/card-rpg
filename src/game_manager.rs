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
use crate::scenes::menu::Menu; // <-- implement with scene change
use crate::cards::card_system;

use crate::events::event_subsystem::{EventSystem, GameEvent};

pub enum GameState {
	Running,
	Quit,
}

pub struct GameManager<'a> {
	overworld: Box<dyn Scene + 'a>,
	battle: Box<dyn Scene + 'a>,
	menu: Box<dyn Scene + 'a>,  // <-- implement with scene change
	//current_scene: &'a Box<dyn Scene + 'a>,
	scene_id: u8,
	game_state: GameState,
	event_subsystem: EventSystem,
	//video_subsystem: VideoSystem<'a>,
}

impl<'a> GameManager<'a> {

	fn update(&mut self) -> Result<(), String>{
		let game_event = self.event_subsystem.update();

		match game_event {
			Some(GameEvent::WindowClose) => self.game_state = GameState::Quit,
			//Some(e) => self.overworld.handle_input(e), // <-- implement with scene change... somehow...	
		    //Some(e) => self.current_scene.handle_input(e), 
		    Some(e) => {
		        match self.scene_id {
		            0 => self.menu.handle_input(e),
		            1 => self.overworld.handle_input(e),
		            2 => self.battle.handle_input(e),
		            _ => {},
		        }
		    }
			None => {},
		}

		//self.overworld.render()?;
		//self.current_scene.render()?;
		match self.scene_id {
		            0 => self.menu.render(),
		            1 => self.overworld.render(),
		            2 => self.battle.render(),
		            _ => { println!("scene id {} doesn't correspond to any scenes", self.scene_id); Ok(())},
		};

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

	pub fn init(sdl_context: &Sdl, wincan: Rc<RefCell<WindowCanvas>>, texture_manager: Rc<RefCell<TextureManager<'a>>>) -> Result<Self, String> {

		let overworld = Box::new(Overworld::init(Rc::clone(&texture_manager), Rc::clone(&wincan))?);
		let menu = Box::new(Menu::init(Rc::clone(&texture_manager), Rc::clone(&wincan))?);
		let battle = Box::new(Battle::init(Rc::clone(&texture_manager))?);
		//let mut current_scene = &menu;
		let mut scene_id = 0;

		let event_subsystem = EventSystem::init(sdl_context)?;

		Ok(GameManager {
			overworld,
			battle,
			menu,  // <-- implement with scene change
			scene_id,
			game_state: GameState::Running,
			event_subsystem,
			//video_subsystem,
		})
	}
	
	pub fn switch_scene(&mut self, dest_scene_id: u8) {
	    
	    self.scene_id = dest_scene_id;
	    
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
