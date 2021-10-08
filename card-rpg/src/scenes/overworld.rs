use std::rc::Rc;
use crate::game_manager::TextureManager;
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};

use crate::scenes::Scene;
use crate::scenes::GameEvent;

pub struct Overworld<'a> {
	tile_map: [u8; 144],
	player: Player<'a>,
}

impl<'a> Overworld<'a> {
	pub fn init(texture_manager: &mut TextureManager<'a>)  -> Result<Self, String> {
		let tile_map = [0; 144];
		let player = Player {
			x_pos: 350,
			y_pos: 350,
			sprite: texture_manager.load("placeholder")?,
		};

		Ok(Overworld{
			tile_map,
			player,
		})
	}
}

impl Scene for Overworld<'_> {
	fn handle_input(&mut self, event: GameEvent) {

	}

	fn render(&self) {
		
	}
}

struct Player<'a> {
	x_pos: i32,
	y_pos: i32,
	sprite: Rc<Texture<'a>>,
}