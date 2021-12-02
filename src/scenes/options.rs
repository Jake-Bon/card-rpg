use std::rc::Rc;
use std::cell::RefCell;

use sdl2::pixels::Color;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::mixer::Music;

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
	volume_flag: i32,
}

impl<'a> Options<'a> {
	pub fn init(texture_manager: Rc<RefCell<TextureManager<'a>>>, wincan: Rc<RefCell<WindowCanvas>>, event_system: Rc<RefCell<EventSystem>>, font_manager: Rc<RefCell<FontManager<'a>>>)  -> Result<Self, String> {
		let background = texture_manager.borrow_mut().load("assets/download.png")?;
		let return_button = texture_manager.borrow_mut().load("assets/return.png")?;
		let volume_flag = 50;

		Ok(Options{
			texture_manager,
			wincan,
			event_system,
			font_manager,
			background,
			return_button,
			volume_flag,
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

				//decrease volume button
				if (x_pos > 405 && x_pos < 405+50) && (y_pos > 330 && y_pos < 330+50) {
					if (self.volume_flag > 0) {
						self.volume_flag = self.volume_flag - 5;
					}
                }
				//increase volume button
				if (x_pos > 825 && x_pos < 825+50) && (y_pos > 330 && y_pos < 330+50) {
					if (self.volume_flag < 50) {
						self.volume_flag = self.volume_flag + 5;
					}
                }
				println!("vol: {}", self.volume_flag);
				sdl2::mixer::Music::set_volume(self.volume_flag);
				
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

		crate::video::gfx::draw_rect(&mut wincan, Color::RGB(100, 60, 30), (50,50), (405, 330))?;
		crate::video::gfx::draw_rect(&mut wincan, Color::RGB(100, 60, 30), (350,50), (465, 330))?;
		crate::video::gfx::draw_rect(&mut wincan, Color::RGB(100, 60, 30), (50,50), (825, 330))?;

		
		crate::video::gfx::draw_rect(&mut wincan, Color::RGB(245, 230, 175), ((self.volume_flag as f32 / 50 as f32 * 350 as f32) as u32, 50), (465, 330))?;


    	let mut fontm = self.font_manager.borrow_mut();
        fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 50, Color::RGB(245, 230, 175), "<", (405, 330));
		fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 50, Color::RGB(245, 230, 175), ">", (825, 330));
		
		wincan.present();

		Ok(())
	}
}
