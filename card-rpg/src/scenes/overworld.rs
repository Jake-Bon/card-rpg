use std::rc::Rc;
use std::cell::RefCell;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

use crate::scenes::Scene;
use crate::scenes::GameEvent;
use crate::game_manager::TextureManager;

pub struct Overworld<'a> {
	wincan: Rc<RefCell<WindowCanvas>>,
	tile_map: [u8; 144],
	player: Player<'a>,
}

impl<'a> Overworld<'a> {
	pub fn init(texture_manager: Rc<RefCell<TextureManager<'a>>>, wincan: Rc<RefCell<WindowCanvas>>)  -> Result<Self, String> {
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
		match event {
			GameEvent::MouseClick(x, y) => {self.player.x_pos = x; self.player.y_pos = y},
			_ => {},
		}
	}

	fn render(&mut self) -> Result<(), String>{
		self.wincan.borrow_mut().clear();
		self.wincan.borrow_mut().set_draw_color(Color::RGB(0, 128, 128));
		self.wincan.borrow_mut().copy(&self.player.sprite, None, Rect::new(self.player.x_pos, self.player.y_pos, 80, 80))?;
		self.wincan.borrow_mut().present();

		Ok(())
	}
}

struct Player<'a> {
	x_pos: i32,
	y_pos: i32,
	sprite: Rc<Texture<'a>>,
}