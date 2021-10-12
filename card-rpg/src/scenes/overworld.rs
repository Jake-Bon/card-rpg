use std::rc::Rc;
use std::cell::RefCell;

use sdl2::pixels::Color;
use sdl2::render::{Texture, WindowCanvas};

use crate::scenes::Scene;
use crate::scenes::GameEvent;
use crate::game_manager::TextureManager;

//mod crate::video;

pub struct Overworld<'a> {
	wincan: &'a mut WindowCanvas,
	tile_map: [u8; 144],
	player: Player<'a>,
}

impl<'a> Overworld<'a> {
	pub fn init(texture_manager: Rc<RefCell<TextureManager<'a>>>, wincan: &'a mut WindowCanvas)  -> Result<Self, String> {
		let tile_map = [0; 144];
		let player = Player {
			x_pos: 350,
			y_pos: 350,
			sprite: texture_manager.borrow_mut().load("assets/Attack_Card.png")?,
		};

		Ok(Overworld{
			wincan,
			tile_map,
			player,
		})
	}
}

impl Scene for Overworld<'_> {
	fn handle_input(&mut self, event: GameEvent) {
		println!("Hi");
	}

	fn render(&mut self) -> Result<(), String>{
		//self.wincan.clear();
		//self.wincan.set_draw_color(Color::RGB(0, 128, 128));
		
		// below line of code is equivalent to the two above
		crate::video::gfx::fill_screen(&mut self.wincan, Color::RGB(0, 128, 128));
		
		
		//self.wincan.copy(&self.player.sprite, None, None)?;
		
		// below line of code is equivalent to above.
		crate::video::gfx::draw_sprite_to_fit(&mut self.wincan, &self.player.sprite);
		
		// draws a sprite to fit the given dimenstions (in this case, 100x150) at the given pos (in this case, x=200, y=200
		crate::video::gfx::draw_sprite_to_dims(&mut self.wincan, &self.player.sprite, 100, 150, 200, 200);
		
		// draws a sprite at the give pos (in this case, x=400, y=300). Uses the dimensions of the file/texture, does no resizing.
		crate::video::gfx::draw_sprite(&mut self.wincan, &self.player.sprite, 400, 300);
		
		self.wincan.present();

		Ok(())
	}
}

struct Player<'a> {
	x_pos: i32,
	y_pos: i32,
	sprite: Rc<Texture<'a>>,
}
