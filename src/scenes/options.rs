use std::rc::Rc;
use std::cell::RefCell;

use sdl2::pixels::Color;
use sdl2::render::{Texture, WindowCanvas};

use crate::scenes::Scene;
use crate::scenes::GameEvent;
use crate::game_manager::TextureManager;

use crate::video::text::FontManager;

use crate::events::event_subsystem::EventSystem;

//mod crate::video;

pub struct Options<'a> {
	texture_manager: Rc<RefCell<TextureManager<'a>>>,
	wincan: Rc<RefCell<WindowCanvas>>,
	event_system: Rc<RefCell<EventSystem>>,
	font_manager: Rc<RefCell<FontManager<'a>>>,
	background: Rc<Texture<'a>>,
	return_button: Rc<Texture<'a>>,
}

impl<'a> Options<'a> {
	pub fn init(texture_manager: Rc<RefCell<TextureManager<'a>>>, wincan: Rc<RefCell<WindowCanvas>>, event_system: Rc<RefCell<EventSystem>>, font_manager: Rc<RefCell<FontManager<'a>>>)  -> Result<Self, String> {
		let background = texture_manager.borrow_mut().load("assets/download.png")?;
		let return_button = texture_manager.borrow_mut().load("assets/return.png")?;

		Ok(Options{
			texture_manager,
			wincan,
			event_system,
			font_manager,
			background,
			return_button,
		})
	}
}

impl Scene for Options<'_> {

	fn handle_input(&mut self, event: GameEvent) {

		match event {
			GameEvent::MouseClick(x_pos,y_pos) => {

				if (x_pos > 490 && x_pos < 490+300) && (y_pos > 530 && y_pos < 530+90) {
					self.event_system.borrow().change_scene(0).unwrap();
                }

				println!("mouse: {}, {}", x_pos, y_pos);
			},
			_ => {},
		}
	}

	fn render(&mut self) -> Result<(), String>{

		let mut wincan = self.wincan.borrow_mut();
		crate::video::gfx::fill_screen(&mut wincan, Color::RGB(0, 120, 150))?;
        crate::video::gfx::draw_sprite_to_fit(&mut wincan, &self.background)?;
        
        
        crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.return_button, (300, 90), (490, 530))?;

        //let mut fontm = self.font_manager.borrow_mut();
        //fontm.draw_text(&mut wincan, "work in progress", (0, 0));

		wincan.present();

		Ok(())
	}
}
