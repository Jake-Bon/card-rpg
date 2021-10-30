use std::rc::Rc;
use std::cell::RefCell;

use sdl2::pixels::Color;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::keyboard::Keycode;


use crate::scenes::Scene;
use crate::scenes::GameEvent;
use crate::events::event_subsystem::EventSystem;
use crate::game_manager::TextureManager;
use crate::video::gfx::CAM_W;
use crate::video::gfx::CAM_H;
use crate::video::gfx::TILE_SIZE;

const modSize: f32 = 4.0;

const FullW: u32 = 1600;
const FullH: u32 = 1200;

const SPEED_LIMIT: f32 = 2.0;
const ACCEL_RATE: f32 = 1.0;

//mod crate::video;

pub struct Overworld<'a> {
	wincan: Rc<RefCell<WindowCanvas>>,
	event_system: Rc<RefCell<EventSystem>>,
	tile_set: Rc<Texture<'a>>,
	player: Player<'a>,
	tmp_button: Rc<Texture<'a>>,
}

impl<'a> Overworld<'a> {
	pub fn init(texture_manager: Rc<RefCell<TextureManager<'a>>>, wincan: Rc<RefCell<WindowCanvas>>, event_system: Rc<RefCell<EventSystem>>)  -> Result<Self, String> {
		//let tile_map = [0; 144];
		let tile_set = texture_manager.borrow_mut().load("assets/download.png")?;
		let tmp_button = texture_manager.borrow_mut().load("assets/tmp.png")?;
		let player = Player {
			x_pos: (CAM_W/2 - TILE_SIZE*modSize as u32 ) as f32,
			y_pos: (CAM_H/2 - TILE_SIZE*modSize as u32) as f32,
			ABSx_pos: (FullW/2 - TILE_SIZE*modSize as u32) as f32,
			ABSy_pos: (FullH/2 - TILE_SIZE*modSize as u32) as f32,
			Box_x_pos: (FullW/2 - CAM_W/2  - TILE_SIZE*modSize as u32) as f32,
			Box_y_pos: (FullH/2 - CAM_H/2 - TILE_SIZE*modSize as u32) as f32,
			x_vel: 0.0,
			y_vel: 0.0,
			sprite: texture_manager.borrow_mut().load("assets/player4x.png")?,
		};
		let frame_counter = 0;

		Ok(Overworld{
			wincan,
			event_system,
			tile_set,
			player,
			tmp_button,
		})
	}
}

impl Scene for Overworld<'_> {
	fn handle_input(&mut self, event: GameEvent) {
		let mut delta_x = 0.0;
		let mut delta_y = 0.0;
		// Matching events, most importantly KeyPress(k)'s
		match event {
			GameEvent::KeyPress(k) => {
				//println!("{}", k);
				if k.eq(&Keycode::W) {delta_y -= ACCEL_RATE}
				if k.eq(&Keycode::A) {delta_x -= ACCEL_RATE}
				if k.eq(&Keycode::S) {delta_y += ACCEL_RATE}
				if k.eq(&Keycode::D) {delta_x += ACCEL_RATE}
				if k.eq(&Keycode::Escape) {
					delta_x=0.0;
					delta_y=0.0;
					self.player.x_vel=0.0;
					self.player.y_vel=0.0;
					self.event_system.borrow().change_scene(2).unwrap();}
				self.player.x_vel = (self.player.x_vel + delta_x)
					.clamp(-SPEED_LIMIT, SPEED_LIMIT);
				self.player.y_vel = (self.player.y_vel + delta_y)
					.clamp(-SPEED_LIMIT, SPEED_LIMIT);
			},
			_ => {/*println!("No event")*/},
		}
	}


	fn render(&mut self) -> Result<(), String> {
		let mut wincan = self.wincan.borrow_mut();
		self.player.update_movement();
		// Draw background
		crate::video::gfx::fill_screen(&mut wincan, Color::RGB(0, 128, 128))?;
		// Draw sea tiles
		crate::video::gfx::draw_sprite_from_sheet(&mut wincan, &self.tile_set,(self.player.Box_x_pos as i32,self.player.Box_y_pos as i32),(CAM_W,CAM_H),(0,0))?;
		// Draw player
		crate::video::gfx::draw_sprite(&mut wincan, &self.player.sprite, (self.player.x_pos as i32, self.player.y_pos as i32))?;
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.tmp_button,(300,100), (0,300))?;

		wincan.present();

		Ok(())
	}
}

struct Player<'a> {
	//src: Rect,
	x_pos: f32,
	y_pos: f32,
	ABSx_pos: f32,
	ABSy_pos: f32,
	Box_x_pos: f32,
	Box_y_pos: f32,
	x_vel: f32,
	y_vel: f32,
	sprite: Rc<Texture<'a>>,
}

impl<'a> Player<'a> {
	fn update_movement(&mut self) {
		// Check if player will go beyond the bounds of the camera
		// - If yes, set their velocity to 0
		if self.x_pos + self.x_vel > CAM_W as f32 - TILE_SIZE as f32 *modSize || self.x_pos + self.x_vel < 0.0 {
			self.x_vel = 0.0;
		}
		if self.y_pos + self.y_vel > CAM_H as f32 - TILE_SIZE as f32 *modSize || self.y_pos + self.y_vel < 0.0 {
			self.y_vel = 0.0;
		}
		// Add velocity to position, clamp to ensure bounds are never exceeded.
		// TILE_SIZE * 4 because the tiles are scaled x4
		if((self.ABSx_pos-self.Box_x_pos <= (CAM_W/2+5) as f32) && (self.ABSx_pos-self.Box_x_pos >= (CAM_W/2-5) as f32))
		{
			self.Box_x_pos = (self.Box_x_pos + self.x_vel).clamp(0.0, (FullW-CAM_W) as f32);
		}
		if((self.ABSy_pos-self.Box_y_pos <= (CAM_H/2+5) as f32) && (self.ABSy_pos-self.Box_y_pos >= (CAM_H/2-5) as f32))
		{
			self.Box_y_pos = (self.Box_y_pos + self.y_vel).clamp(0.0, (FullH-CAM_H) as f32);
		}

		self.ABSx_pos = (self.ABSx_pos + self.x_vel).clamp(0.0, FullW as f32 - (TILE_SIZE as f32 *modSize));
		self.ABSy_pos = (self.ABSy_pos + self.y_vel).clamp(0.0, FullH as f32 - (TILE_SIZE as f32 *modSize));

		self.x_pos = (self.ABSx_pos-self.Box_x_pos).clamp(0.0, CAM_W as f32 - (TILE_SIZE as f32 *modSize));
		self.y_pos = (self.ABSy_pos-self.Box_y_pos).clamp(0.0, CAM_H as f32 - (TILE_SIZE as f32 *modSize));


	}
}



//###########################
