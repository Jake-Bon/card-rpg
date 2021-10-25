//Credits
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;
use std::thread;

use sdl2::pixels::Color;
use sdl2::render::{Texture, WindowCanvas};

use crate::scenes::Scene;
use crate::scenes::GameEvent;
use crate::events::event_subsystem::EventSystem;
use crate::game_manager::TextureManager;

const TIMEOUT: u64 = 1500;

pub struct Credits<'a> {
	wincan: Rc<RefCell<WindowCanvas>>,
	event_system: Rc<RefCell<EventSystem>>,
	slides: Vec<Rc<Texture<'a>>>,
}

impl<'a> Credits<'a> {
	pub fn init(texture_manager: Rc<RefCell<TextureManager<'a>>>, wincan: Rc<RefCell<WindowCanvas>>, event_system: Rc<RefCell<EventSystem>>) -> Result<Self, String> {
		let mut slides: Vec<Rc<Texture<'a>>> = Vec::new();

		let mut images = vec![
							"assets/credits/david.png","assets/credits/derek.png",
							"assets/credits/emilio.png","assets/credits/gabe.png",
							"assets/credits/jacob.png","assets/credits/louisa.png",
							"assets/credits/max.png","assets/credits/merrick.png"];
		for image in images.iter() {
			slides.push(texture_manager.borrow_mut().load(image).unwrap());
		}

		Ok(Credits{
			wincan,
			event_system,
			slides,
		})
	}
}

impl Scene for Credits<'_> {
	fn handle_input(&mut self, event: GameEvent) {
		// Nothing... for now
	}

	fn render(&mut self) -> Result<(), String> {
		let mut wincan = self.wincan.borrow_mut();

		let mut i = 0;
		for slide in &self.slides {
			i += 1;
			println!("{}", i);
			crate::video::gfx::fill_screen(&mut wincan, Color::RGBA(0,0,0,255));
			crate::video::gfx::draw_sprite_to_fit(&mut wincan, &slide);
			wincan.present();
			thread::sleep(Duration::from_millis(TIMEOUT));
			wincan.clear();
		}

		self.event_system.borrow().change_scene(0).unwrap();

		Ok(())

	}
}