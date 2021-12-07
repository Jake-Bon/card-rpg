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

pub struct Menu<'a> {
	texture_manager: Rc<RefCell<TextureManager<'a>>>,
	wincan: Rc<RefCell<WindowCanvas>>,
	event_system: Rc<RefCell<EventSystem>>,
	font_manager: Rc<RefCell<FontManager<'a>>>,
	background: Rc<Texture<'a>>,
	play_button: Rc<Texture<'a>>,
	online_button: Rc<Texture<'a>>,
	options_button: Rc<Texture<'a>>,
	quit_button: Rc<Texture<'a>>,
	logo: Rc<Texture<'a>>,
	tile_set: Rc<Texture<'a>>,
}

impl<'a> Menu<'a> {
	pub fn init(texture_manager: Rc<RefCell<TextureManager<'a>>>, wincan: Rc<RefCell<WindowCanvas>>, event_system: Rc<RefCell<EventSystem>>, font_manager: Rc<RefCell<FontManager<'a>>>)  -> Result<Self, String> {
		let background = texture_manager.borrow_mut().load("assets/download.png")?;
		let play_button = texture_manager.borrow_mut().load("assets/play.png")?;
		let online_button = texture_manager.borrow_mut().load("assets/online.png")?;
		let options_button = texture_manager.borrow_mut().load("assets/options.png")?;
		let quit_button = texture_manager.borrow_mut().load("assets/quit.png")?;
		let logo = texture_manager.borrow_mut().load("assets/logo.png")?;
		let tile_set = texture_manager.borrow_mut().load("assets/tile_sheet4x.png")?;

		Ok(Menu{
			texture_manager,
			wincan,
			event_system,
			font_manager,
			background,
			play_button,
			online_button,
			options_button,
			quit_button,
			logo,
			tile_set,
		})
	}
}

impl Scene for Menu<'_> {

	fn handle_input(&mut self, event: GameEvent) {

		//so players don't blow off their eardrums when playing w/o going to options
		if (sdl2::mixer::Music::get_volume() > 50) {
			sdl2::mixer::Music::set_volume(50);
		}

		match event {
			GameEvent::MouseClick(x_pos,y_pos) => {

				if (x_pos > 140 && x_pos < 390) && (y_pos > 550 && y_pos < 650) {
					println!("PLAY");
					println!("X {}, Y: {}", x_pos, y_pos);
					self.event_system.borrow().change_scene(1).unwrap();
				}

				if (x_pos > 390 && x_pos < 640) && (y_pos > 550 && y_pos < 650) {
					println!("ONLINE");
					self.event_system.borrow().change_scene(3).unwrap();
				}

				if (x_pos > 640 && x_pos < 890) && (y_pos > 550 && y_pos < 650) {
					println!("OPTIONS");
					self.event_system.borrow().change_scene(5).unwrap();
				}

				if (x_pos > 890 && x_pos < 1140) && (y_pos > 550 && y_pos < 650) {
					println!("QUIT");
					self.event_system.borrow().change_scene(4).unwrap();
				}

				println!("mouse: {}, {}", x_pos, y_pos);
			},
			_ => {},
		}

		//println!("Hi");
	}

	fn render(&mut self) -> Result<(), String>{

		let mut wincan = self.wincan.borrow_mut();
		crate::video::gfx::fill_screen(&mut wincan, Color::RGB(0, 120, 150))?;


		// Draw sea tiles
		//crate::video::gfx::tile_sprite_from_sheet(&mut self.wincan, &self.tile_set, (0, 0), (40*5, 40), (0, 0), (4, 18))?;

        //crate::video::text::draw_text(&mut wincan, self.texture)?;
        
		crate::video::gfx::draw_sprite_to_fit(&mut wincan, &self.background)?;

		crate::video::gfx::draw_sprite(&mut wincan, &self.play_button, (140, 550))?;
		crate::video::gfx::draw_sprite(&mut wincan, &self.online_button, (390, 550))?;
		crate::video::gfx::draw_sprite(&mut wincan, &self.options_button, (640, 550))?;
		crate::video::gfx::draw_sprite(&mut wincan, &self.quit_button, (890, 550))?;

		crate::video::gfx::draw_sprite(&mut wincan, &self.logo, (340, 100))?;



		wincan.present();

		Ok(())
	}
}
