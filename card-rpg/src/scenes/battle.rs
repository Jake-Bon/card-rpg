use std::rc::Rc;
use sdl2::render::Texture;
use crate::TextureManager;
use sdl2::video::WindowContext;
use sdl2::render::TextureCreator;

use crate::scenes::Scene;
use crate::scenes::GameEvent;

pub struct Battle<'a> {
	test: i32,
	texture: Rc<Texture<'a>>,
}

impl<'a> Battle<'a> {
	pub fn init(texture_manager: &'a mut TextureManager)  -> Result<Self, String> {
		Ok(Battle {
			texture: texture_manager.load("placeholder")?,
			test: 10,
		})
	}
}

impl Scene for Battle<'_> {
	fn handle_input(&mut self, event: GameEvent) {

	}

	fn render(&self) {
		
	}
}