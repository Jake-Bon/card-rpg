use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::time::{Instant, Duration};

use sdl2::Sdl;
use sdl2::image::LoadTexture;
use sdl2::video::WindowContext;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use crate::video::text::FontManager;

use crate::scenes::Scene;
use crate::scenes::battle::Battle;
use crate::scenes::online::Online;
use crate::scenes::overworld::Overworld;
use crate::scenes::menu::Menu; // <-- implement with scene change
use crate::scenes::credits::Credits;

use crate::events::event_subsystem::{EventSystem, GameEvent};

pub enum GameState {
	Running,
	Quit,
}

pub struct GameManager<'a> {
	overworld: Box<dyn Scene + 'a>,
	battle: Box<dyn Scene + 'a>,
	menu: Box<dyn Scene + 'a>,
	online: Box<dyn Scene + 'a>,
	credits: Box<dyn Scene + 'a>,
	game_state: GameState,
	wincan: Rc<RefCell<WindowCanvas>>,
	event_system: Rc<RefCell<EventSystem>>,
	font_manager: Rc<RefCell<FontManager<'a>>>,
	curr_scene: u32,
}

impl<'a> GameManager<'a> {

	fn handle_input(&mut self, e: GameEvent) {
		match self.curr_scene {
			0 => self.menu.handle_input(e),
			1 => self.overworld.handle_input(e),
			2 => self.battle.handle_input(e),
			3 => self.online.handle_input(e),
			4 => self.credits.handle_input(e),
			_ => {},
		}
	}

	fn update(&mut self) -> Result<(), String>{
		let game_events = self.event_system.borrow_mut().update();
		for event in game_events {
			match event {
				Some(GameEvent::WindowClose) => self.game_state = GameState::Quit,
				Some(GameEvent::SetBattlerNPCDeck(deck_id)) => { 
				    //println!("sending the SetBattlerNPCDeck event to battle, deck_id was {}", deck_id as u32);
				    self.battle.handle_input(GameEvent::SetBattlerNPCDeck(deck_id)); // only scene that should care about the battler deck id is battle.rs
				},
				Some(GameEvent::SceneChange(scene_id)) => self.curr_scene = scene_id,
				Some(e) => self.handle_input(e),
				None => {},
			}
		}


		match self.curr_scene {
			0 => self.menu.render()?,
			1 => self.overworld.render()?,
			2 => self.battle.render()?,
			3 => self.online.render()?,
			4 => self.credits.render()?,
			_ => {},
		};

		Ok(())
	}

	pub fn start_state_machine(&mut self) {

		let mut current_time = Instant::now();
		let mut accumulator = Duration::new(0, 0);

		'running: loop {

			let new_time = Instant::now();
			let frame_time = new_time - current_time;
			current_time = new_time;

			accumulator += frame_time;
			// Timestep lock 60fps

			while accumulator > Duration::from_millis(16) {
				match self.game_state {
					GameState::Quit => break 'running,
					GameState::Running => self.update().unwrap(),
				};
			}
		}
	}

	pub fn init(sdl_context: &Sdl, wincan: Rc<RefCell<WindowCanvas>>, texture_manager: Rc<RefCell<TextureManager<'a>>>, font_manager: Rc<RefCell<FontManager<'a>>>) -> Result<Self, String> {

		let event_system = Rc::new(RefCell::new(EventSystem::init(&sdl_context)?));

		let menu = Box::new(Menu::init(Rc::clone(&texture_manager), Rc::clone(&wincan), Rc::clone(&event_system), Rc::clone(&font_manager))?);
		let battle = Box::new(Battle::init(Rc::clone(&texture_manager), Rc::clone(&wincan), Rc::clone(&event_system), Rc::clone(&font_manager))?);
		let overworld = Box::new(Overworld::init(Rc::clone(&texture_manager), Rc::clone(&wincan), Rc::clone(&event_system))?);
		let online = Box::new(Online::init(Rc::clone(&texture_manager), Rc::clone(&wincan), Rc::clone(&event_system), Rc::clone(&font_manager)));
		let credits = Box::new(Credits::init(Rc::clone(&texture_manager), Rc::clone(&wincan), Rc::clone(&event_system))?);

		Ok(GameManager {
			overworld,
			battle,
			menu,
			online,
			credits,
			game_state: GameState::Running,
			wincan,
			event_system,
			font_manager,
			curr_scene: 0,
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
