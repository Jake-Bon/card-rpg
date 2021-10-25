use std::rc::Rc;
use std::cell::RefCell;
use sdl2::render::Texture;
use crate::TextureManager;



use crate::scenes::Scene;
use crate::scenes::GameEvent;

pub struct Battle<'a> {
	test: i32,
	texture: Rc<Texture<'a>>,
}

impl<'a> Battle<'a> {
	pub fn init(texture_manager: Rc<RefCell<TextureManager<'a>>>)  -> Result<Self, String> {
		Ok(Battle {
			texture: texture_manager.borrow_mut().load("assets/Attack_Card.png")?,
			test: 10,
		})
	}
}

impl Scene for Battle<'_> {
	fn handle_input(&mut self, event: GameEvent) {

	}

	fn render(&mut self) -> Result<(), String> {
		// print!("battle");
		Ok(())
	}
}