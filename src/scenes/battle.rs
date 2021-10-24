use std::rc::Rc;
use std::cell::RefCell;
use sdl2::render::Texture;
use crate::TextureManager;



use crate::scenes::Scene;
use crate::scenes::GameEvent;

use crate::video::gfx::FRAME_DELAY;

pub struct Battle<'a> {
	test: i32,
	texture: Rc<Texture<'a>>,
	frame_counter: u32,
}

impl<'a> Battle<'a> {
	pub fn init(texture_manager: Rc<RefCell<TextureManager<'a>>>)  -> Result<Self, String> {
		Ok(Battle {
			texture: texture_manager.borrow_mut().load("assets/Attack_Card.png")?,
			test: 10,
			frame_counter: 0,
		})
	}
}

impl Scene for Battle<'_> {
	fn handle_input(&mut self, event: GameEvent) {

	}

	fn render(&mut self) -> Result<(), String> {
		
		self.frame_counter = if ((self.frame_counter + 1) / FRAME_DELAY) < 5 { self.frame_counter + 1 } else { 0 }; 
		
		self.frame_counter = self.frame_counter + 1;
		Ok(())
	}
}
