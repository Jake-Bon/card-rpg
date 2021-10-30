use std::rc::Rc;
use std::cell::RefCell;

use sdl2::pixels::Color;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::ttf::Sdl2TtfContext;

use crate::scenes::Scene;
use crate::scenes::GameEvent;
use crate::game_manager::TextureManager;
use crate::game_manager::GameManager;
use crate::game_manager::GameState;

use crate::video::text::FontManager;

use crate::events::event_subsystem::EventSystem;

//mod crate::video;

pub struct Menu<'a> {
	texture_manager: Rc<RefCell<TextureManager<'a>>>,
	wincan: Rc<RefCell<WindowCanvas>>,
	event_system: Rc<RefCell<EventSystem>>,
	font_manager: Rc<RefCell<FontManager<'a>>>,
	play_button: Rc<Texture<'a>>,
	quit_button: Rc<Texture<'a>>,
	logo: Rc<Texture<'a>>,
	tile_set: Rc<Texture<'a>>,
}

impl<'a> Menu<'a> {
	pub fn init(texture_manager: Rc<RefCell<TextureManager<'a>>>, wincan: Rc<RefCell<WindowCanvas>>, event_system: Rc<RefCell<EventSystem>>, font_manager: Rc<RefCell<FontManager<'a>>>)  -> Result<Self, String> {
		let play_button = texture_manager.borrow_mut().load("assets/play.png")?;
		let quit_button = texture_manager.borrow_mut().load("assets/quit.png")?;
		let logo = texture_manager.borrow_mut().load("assets/logo.png")?;
		let tile_set = texture_manager.borrow_mut().load("assets/tile_sheet4x.png")?;

		Ok(Menu{
			texture_manager,
			wincan,
			event_system,
			font_manager,
			play_button,
			quit_button,
			logo,
			tile_set,
		})
	}
}

impl Scene for Menu<'_> {

	/*let play_x = 50;
	let play_y = 450;
	let quit_x = 50;
	let quit_y = 550;*/

	fn handle_input(&mut self, event: GameEvent) {

		match event {
			GameEvent::MouseClick(x_pos,y_pos) => {

				if (x_pos > 50 && x_pos < 300) && (y_pos > 450 && y_pos < 550) {
					println!("PLAY");
					println!("X {}, Y: {}", x_pos, y_pos);
					self.event_system.borrow().change_scene(1).unwrap();
				}

				if (x_pos > 50 && x_pos < 300) && (y_pos > 550 && y_pos < 650) {
					self.event_system.borrow().change_scene(3).unwrap();
					println!("QUIT");

					//GameEvent::WindowClose;
					//GameManager::game_state = GameState::Quit;
					//event = GameEvent::WindowClose;
				
				}
				
				//if (x_pos > 50 && x_pos < 300) && (y_pos > 0 && y_pos < 200){
				//println!("Battle Time");
				//self.event_system.borrow().change_scene(2).unwrap();
				
				//}

				println!("mouse: {}, {}", x_pos, y_pos);
			},
			_ => {},
		}

		//println!("Hi");
	}

	fn render(&mut self) -> Result<(), String>{

		let mut wincan = self.wincan.borrow_mut();
		crate::video::gfx::fill_screen(&mut wincan, Color::RGB(0, 120, 150));


		// Draw sea tiles
		//crate::video::gfx::tile_sprite_from_sheet(&mut self.wincan, &self.tile_set, (0, 0), (40*5, 40), (0, 0), (4, 18))?;

        //crate::video::text::draw_text(&mut wincan, self.texture)?;
        

		crate::video::gfx::draw_sprite(&mut wincan, &self.play_button, (50, 450))?;
		crate::video::gfx::draw_sprite(&mut wincan, &self.quit_button, (50, 550))?;

		crate::video::gfx::draw_sprite(&mut wincan, &self.logo, (340, 100))?;
		
		/*
		
		// text examples
		
		let mut font_m = self.font_manager.borrow_mut();
		//println!("calling font_m.draw_text()");
		
		font_m.draw_text(&mut wincan, "some text", (30, 30));
		
		font_m.draw_text(&mut wincan, "some more text somewhere else", (20, 140));
		
		font_m.draw_text(&mut wincan, "some text", (400, 400));
		
		font_m.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 48, Color::RGB(0, 0, 255), "text but this time it has a\n line break", (200, 200));
		
		font_m.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 24, Color::RGBA(0, 0, 0, 0), "transparent doesn't work", (300, 350));  
		
        */
		

		wincan.present();

		Ok(())
	}
}
