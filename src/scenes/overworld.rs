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

const FullW: u32 = 2400;
const FullH: u32 = 1800;
const TileS: u32 = 40;
const TileW: u32 = FullW/TileS;
const TileH: u32 = FullW/TileS;
const SpriteTileS: u32 = 40;

const SPEED_LIMIT: f32 = 6.0;
const ACCEL_RATE: f32 = 0.6;

//mod crate::video;

pub struct Overworld<'a> {
	wincan: Rc<RefCell<WindowCanvas>>,
	event_system: Rc<RefCell<EventSystem>>,
	tile_set: Rc<Texture<'a>>,
	enemy_map: Rc<Texture<'a>>,
	player: Player<'a>,
	enemy: Enemy<'a>,
	anim_water: u32,
	frames: u32,
}

impl<'a> Overworld<'a> {
	pub fn init(texture_manager: Rc<RefCell<TextureManager<'a>>>, wincan: Rc<RefCell<WindowCanvas>>, event_system: Rc<RefCell<EventSystem>>)  -> Result<Self, String> {
		//let tile_map = [0; 144];
		//let tile_set = texture_manager.borrow_mut().load("assets/download.png")?;
		let tile_set = texture_manager.borrow_mut().load("assets/tile_sheet4x.png")?;
		let enemy_map = texture_manager.borrow_mut().load("assets/enemy_map.png")?;

		let player = Player {
			x_pos: (CAM_W/2 - TILE_SIZE*modSize as u32 ) as f32,
			y_pos: (CAM_H/2 - TILE_SIZE*modSize as u32) as f32,
			ABSx_pos: (FullW/2 - TILE_SIZE*modSize as u32) as f32,
			ABSy_pos: (FullH/2 - TILE_SIZE*modSize as u32) as f32,
			Box_x_pos: (FullW/2 - CAM_W/2  - TILE_SIZE*modSize as u32) as f32,
			Box_y_pos: (FullH/2 - CAM_H/2 - TILE_SIZE*modSize as u32) as f32,
			x_vel: 0.0,
			y_vel: 0.0,
			delta_x: 0.0,
			delta_y: 0.0,
			sprite: texture_manager.borrow_mut().load("assets/player4x.png")?,
			keyPress: [false; 4],
		};

		let enemy = Enemy {
			x_pos: 300.0 - 160.0, //0,
			y_pos: 300.0 - 240.0, //0,
			ABSx_pos: 300.0, //160.0,
			ABSy_pos: 300.0, //240.0,
			Box_x_pos: (FullW/2 - CAM_W/2  - TILE_SIZE*modSize as u32) as f32,
			Box_y_pos: (FullH/2 - CAM_H/2 - TILE_SIZE*modSize as u32) as f32,
			x_vel: 0.0,
			y_vel: 0.0,
			sprite: texture_manager.borrow_mut().load("assets/player4x.png")?,
		};


		let frames = 0;
		let anim_water = 0;

		Ok(Overworld{
			wincan,
			event_system,
			tile_set,
			enemy_map,
			player,
			enemy,
			frames,
			anim_water,
		})
	}
}

impl Scene for Overworld<'_> {
	fn handle_input(&mut self, event: GameEvent) {
		// Matching events, most importantly KeyPress(k)'s
		match event {
			GameEvent::KeyPress(k) => {
				//println!("p:{}", k);
				if k.eq(&Keycode::W) {self.player.keyPress[0]=true}
				if k.eq(&Keycode::S) {self.player.keyPress[1]=true}
				if k.eq(&Keycode::A) {self.player.keyPress[2]=true}
				if k.eq(&Keycode::D) {self.player.keyPress[3]=true}
				if k.eq(&Keycode::Escape) {
					//removing 'press esc' functionality
					/*self.player.delta_x=0.0;
					self.player.delta_y=0.0;
					self.player.x_vel=0.0;
					self.player.y_vel=0.0;
					self.event_system.borrow().change_scene(2).unwrap();*/
				}

			},
			GameEvent::KeyRelease(k) => {
				//println!("r:{}", k);
				if k.eq(&Keycode::W) {self.player.keyPress[0]=false}
				if k.eq(&Keycode::S) {self.player.keyPress[1]=false}
				if k.eq(&Keycode::A) {self.player.keyPress[2]=false}
				if k.eq(&Keycode::D) {self.player.keyPress[3]=false}
			},
			_ => {println!(/*"No event"*/)},
		}
	}


	fn render(&mut self) -> Result<(), String> {
		let mut wincan = self.wincan.borrow_mut();
		self.player.update_movement();
		self.enemy.update_movement();


		self.frames = if (self.frames) > 5 {
					self.anim_water = ((self.anim_water+1)%9);
					0
				}
				else {
					self.frames + 1
				};




		// hard coded enemy collision
		if (self.player.ABSx_pos > 300.0-40.0 && self.player.ABSx_pos < 340.0
			&& self.player.ABSy_pos > 300.0-40.0 && self.player.ABSy_pos < 340.0) {
			self.player.x_vel=0.0;
			self.player.y_vel=0.0;
			self.player.delta_x=0.0;
			self.player.delta_y=0.0;
			//self.player.x_pos= (CAM_W/2 - TILE_SIZE*modSize as u32 ) as f32;
			//self.player.y_pos= (CAM_H/2 - TILE_SIZE*modSize as u32) as f32;
			//self.player.ABSx_pos= (FullW/2 - TILE_SIZE*modSize as u32) as f32;
			//self.player.ABSy_pos= (FullH/2 - TILE_SIZE*modSize as u32) as f32;
			//self.player.Box_x_pos= (FullW/2 - CAM_W/2  - TILE_SIZE*modSize as u32) as f32;
			//self.player.Box_y_pos= (FullH/2 - CAM_H/2 - TILE_SIZE*modSize as u32) as f32;
			self.event_system.borrow().change_scene(2).unwrap();
		}


		// Draw background color
		crate::video::gfx::fill_screen(&mut wincan, Color::RGB(0, 128, 128))?;

		// Draw background image
		//crate::video::gfx::draw_sprite_from_sheet(&mut wincan, &self.tile_set,(self.player.Box_x_pos as i32,self.player.Box_y_pos as i32),(CAM_W,CAM_H),(0,0))?;
		crate::video::gfx::tile_sprite_from_sheet_resize(&mut wincan, &self.tile_set,((self.anim_water*SpriteTileS) as i32,0),(40,40),(TileS,TileS),(-(self.player.Box_x_pos as i32),-(self.player.Box_y_pos as i32)),(TileW,TileH))?;

		// draw enemy map (temporary hotfix for purpose of having presentable midterm build)
		crate::video::gfx::draw_sprite_from_sheet(&mut wincan, &self.enemy_map,
			(self.player.Box_x_pos as i32,self.player.Box_y_pos as i32),
			(CAM_W,CAM_H),(0,0))?;

		// Draw player
		crate::video::gfx::draw_sprite(&mut wincan, &self.player.sprite, (self.player.x_pos as i32, self.player.y_pos as i32))?;

		// Draw enemy (doesn't work yet)
		//crate::video::gfx::draw_sprite(&mut wincan, &self.enemy.sprite,
		//	(self.enemy.x_pos as i32, self.enemy.y_pos as i32))?;


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
	delta_x: f32,
	delta_y: f32,
	sprite: Rc<Texture<'a>>,
	keyPress: [bool; 4],
}

impl<'a> Player<'a> {
	fn update_movement(&mut self) {
		// Check if player will go beyond the bounds of the camera
		// - If yes, set their velocity to 0

		if(self.keyPress[0])//w
		{
			self.delta_y -= ACCEL_RATE;
			self.y_vel = (self.y_vel + self.delta_y)
				.clamp(-SPEED_LIMIT, SPEED_LIMIT);
		}
		else if(self.keyPress[1])//s
		{
			self.delta_y += ACCEL_RATE;
			self.y_vel = (self.y_vel + self.delta_y)
				.clamp(-SPEED_LIMIT, SPEED_LIMIT);
		}
		else //niether
		{
			if(self.y_vel>0.0)
			{
				self.delta_y -= ACCEL_RATE;
				self.y_vel = (self.y_vel + self.delta_y)
					.clamp(-SPEED_LIMIT, SPEED_LIMIT);
				if(self.y_vel<0.0)
				{
					self.delta_y = 0.0;
					self.y_vel = 0.0;
				}
			}
			else if(self.y_vel<0.0)
			{
				self.delta_y += ACCEL_RATE;
				self.y_vel = (self.y_vel + self.delta_y)
					.clamp(-SPEED_LIMIT, SPEED_LIMIT);
				if(self.y_vel>0.0)
				{
					self.delta_y = 0.0;
					self.y_vel = 0.0;
				}
			}
		}


		if(self.keyPress[2])//a
		{
			self.delta_x -= ACCEL_RATE;
			self.x_vel = (self.x_vel + self.delta_x)
				.clamp(-SPEED_LIMIT, SPEED_LIMIT);
		}
		else if(self.keyPress[3])//d
		{
			self.delta_x += ACCEL_RATE;
			self.x_vel = (self.x_vel + self.delta_x)
				.clamp(-SPEED_LIMIT, SPEED_LIMIT);
		}
		else
		{
			if(self.x_vel>0.0)
			{
				self.delta_x -= ACCEL_RATE;
				self.x_vel = (self.x_vel + self.delta_x)
					.clamp(-SPEED_LIMIT, SPEED_LIMIT);
				if(self.x_vel< 0.0)
				{
					self.x_vel = 0.0;
					self.delta_x=0.0;
				}
			}
			else if(self.x_vel<0.0)
			{
				self.delta_x += ACCEL_RATE;
				self.x_vel = (self.x_vel + self.delta_x)
					.clamp(-SPEED_LIMIT, SPEED_LIMIT);
				if(self.x_vel>0.0)
				{
					self.x_vel = 0.0;
					self.delta_x = 0.0
				}
			}

		}

		//check the ACCEL_RATE
		if(self.x_vel==-SPEED_LIMIT||self.x_vel==SPEED_LIMIT)
		{
			self.delta_x = 0.0;
		}
		if(self.y_vel==-SPEED_LIMIT||self.y_vel==SPEED_LIMIT)
		{
			self.delta_y = 0.0;
		}


		// println!("{}", self.x_vel);
		// println!("{}", self.y_vel);
		// println!("{}", self.delta_x);
		// println!("{}", self.delta_y);




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


struct Enemy<'a> {
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

impl<'a> Enemy<'a> {
	fn update_movement(&mut self) {
		/* needs to be implemented... */
	}
}

//###########################
//###########################
