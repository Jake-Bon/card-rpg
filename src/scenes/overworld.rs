use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

//use image::io::Reader as ImageReader;
//use image::{Rgba,GenericImage};

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

const FullW: u32 = 2400;
const FullH: u32 = 1800;

const TileW: u32 = FullW/TILE_SIZE;
const TileH: u32 = FullH/TILE_SIZE;
const SpriteTILE_SIZE: u32 = 40;

const SPEED_LIMIT: f32 = 6.0;
const ACCEL_RATE: f32 = 0.3;

//mod crate::video;

fn map_reader(map: &str) -> Result<Vec<u8>,String>{
	let file = File::open(map.to_string()).expect("File not opened.");
	let mut buf_reader = BufReader::new(file);

	let mut bytes: Vec<u8> = Vec::new();
	buf_reader.read_to_end(&mut bytes);

	bytes.drain(0..66);//remove bmp header
	let mut map: Vec<u8> = Vec::new();
	for i in 0..TileH{ //mirror map vertically to how it is drawn
		let j = TileH as i32-i as i32;
		map.extend_from_slice(&(bytes[(j-1) as usize*TileW as usize..j as usize*TileW as usize]));
	}

	Ok(map)
}

pub struct Overworld<'a> {
	wincan: Rc<RefCell<WindowCanvas>>,
	event_system: Rc<RefCell<EventSystem>>,
	tile_set: Rc<Texture<'a>>,
	enemy_sprite: Rc<Texture<'a>>,
	player: Player<'a>,
	enemy: Enemy<'a>,
	anim_water: u32,
	frames: u32,
	map_rep: Vec<u8>,
}

impl<'a> Overworld<'a> {
	pub fn init(texture_manager: Rc<RefCell<TextureManager<'a>>>, wincan: Rc<RefCell<WindowCanvas>>, event_system: Rc<RefCell<EventSystem>>)  -> Result<Self, String> {
		//let tile_map = [0; 144];
		//let tile_set = texture_manager.borrow_mut().load("assets/download.png")?;
		let tile_set = texture_manager.borrow_mut().load("assets/tile_sheet4x.png")?;
		let enemy_sprite = texture_manager.borrow_mut().load("assets/simple_enemy_sprite.png")?;
		let map_rep = map_reader("src/scenes/world-1.bmp")?;

		let player = Player {
			x_pos: (CAM_W/2) as f32,
			y_pos: (CAM_H/2) as f32,
			ABSx_pos: (FullW/2) as f32,
			ABSy_pos: (FullH/2) as f32,
			Box_x_pos: (FullW/2 - CAM_W/2) as f32,
			Box_y_pos: (FullH/2 - CAM_H/2) as f32,
			x_vel: 0.0,
			y_vel: 0.0,
			delta_x: 0.0,
			delta_y: 0.0,
			sprite: texture_manager.borrow_mut().load("assets/player4x.png")?,
			keyPress: [false; 4],
			map_copy: map_rep.clone(),
			last_safe_x: (CAM_W/2 - TILE_SIZE) as f32,
			last_safe_y: (CAM_H/2 - TILE_SIZE) as f32,
			is_flipped: false,
		};

		let enemy = Enemy {
			ABSx_pos: 300.0, //160.0,
			ABSy_pos: 400.0, //240.0,
			Box_x_pos: (FullW/2 - CAM_W/2  - TILE_SIZE) as f32,
			Box_y_pos: (FullH/2 - CAM_H/2 - TILE_SIZE) as f32,
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
			enemy_sprite,
			player,
			enemy,
			frames,
			anim_water,
			map_rep,
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
		if (self.player.ABSx_pos > self.enemy.ABSx_pos && self.player.ABSx_pos < self.enemy.ABSx_pos + 40.0
			&& self.player.ABSy_pos > self.enemy.ABSy_pos && self.player.ABSy_pos < self.enemy.ABSy_pos+40.0) {
			self.player.x_vel=0.0;
			self.player.y_vel=0.0;
			self.player.delta_x=0.0;
			self.player.delta_y=0.0;

			self.event_system.borrow().change_scene(2).unwrap();
		}


		// Draw background color
		crate::video::gfx::fill_screen(&mut wincan, Color::RGB(0, 128, 128))?;

		// Draw background image
		//crate::video::gfx::draw_sprite_from_sheet(&mut wincan, &self.tile_set,(self.player.Box_x_pos as i32,self.player.Box_y_pos as i32),(CAM_W,CAM_H),(0,0))?;
		crate::video::gfx::tile_sprite_from_sheet_resize(&mut wincan, &self.tile_set,((self.anim_water*SpriteTILE_SIZE) as i32,0),(SpriteTILE_SIZE,SpriteTILE_SIZE),(TILE_SIZE,TILE_SIZE),(-(self.player.Box_x_pos as i32),-(self.player.Box_y_pos as i32)),(TileW,TileH))?;

		let mut sprite_x: i32 = 0;
		let mut sprite_y: i32 = 0;
		for i in 0 as i32..self.map_rep.len() as i32{

			if self.map_rep[i as usize]==1{ //SAND
				sprite_x = 0;
				sprite_y = SpriteTILE_SIZE as i32;
			}else if self.map_rep[i as usize]==2{ //GRASS
				sprite_x = SpriteTILE_SIZE as i32;
				sprite_y = SpriteTILE_SIZE as i32;
			}else{//Add more types if needed
				continue;
			}
			let x = (i%TileW as i32)*TILE_SIZE as i32;
			let y = (i/TileW as i32)*TILE_SIZE as i32;

			crate::video::gfx::tile_sprite_from_sheet_resize(&mut wincan, &self.tile_set,(sprite_x,sprite_y),(SpriteTILE_SIZE,SpriteTILE_SIZE),(TILE_SIZE,TILE_SIZE),(-(self.player.Box_x_pos as i32)+x,-(self.player.Box_y_pos as i32)+y),(1,1));
		}

		// draw enemy
		crate::video::gfx::draw_sprite(&mut wincan, &self.enemy_sprite, (self.enemy.ABSx_pos as i32-self.player.Box_x_pos as i32, self.enemy.ABSy_pos as i32-self.player.Box_y_pos as i32))?;


		// Draw player
		self.player.is_flipped = if self.player.x_vel>0.0{true}else if self.player.x_vel<0.0{false}else{self.player.is_flipped};
		crate::video::gfx::draw_sprite_mirror(&mut wincan, &self.player.sprite, (self.player.x_pos as i32, self.player.y_pos as i32),self.player.is_flipped,false)?;


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
	map_copy: Vec<u8>,
	last_safe_x: f32,
	last_safe_y: f32,
	is_flipped: bool,
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


		if (self.x_pos + self.x_vel > CAM_W as f32 - TILE_SIZE as f32) || self.x_pos + self.x_vel < 0.0 {
			self.x_vel = 0.0;
		}
		if self.y_pos + self.y_vel > CAM_H as f32 - TILE_SIZE as f32 || self.y_pos + self.y_vel < 0.0 {
			self.y_vel = 0.0;
		}

		let map_x = (self.ABSx_pos/TILE_SIZE as f32) as usize;
		let map_y = (self.ABSy_pos/TILE_SIZE as f32) as usize;
		let map_x_left = if map_x==0{map_x}else{map_x-1};
		let map_x_right = if map_x==(TileW) as usize{map_x}else{map_x+1};
		let map_y_up = if map_y==0{map_y}else{map_y-1};
		let map_y_down = if map_y==(TileH) as usize{map_y}else{map_y+1};

		let up_checks = self.map_copy[map_x+TileW as usize*(map_y)]!=0||self.map_copy[map_x_right+TileW as usize*(map_y)]!=0;
		let down_checks = self.map_copy[map_x+TileW as usize*(map_y_down)]!=0||self.map_copy[map_x_right+TileW as usize*(map_y_down)]!=0;
		let left_checks = self.map_copy[map_x+TileW as usize*(map_y)]!=0||self.map_copy[map_x+TileW as usize*(map_y_down)]!=0;
		let right_checks = self.map_copy[map_x_right+TileW as usize*(map_y)]!=0||self.map_copy[map_x_right+TileW as usize*(map_y_down)]!=0;

		if up_checks||down_checks||left_checks||right_checks{
			if (up_checks&&self.y_vel<0.0)||(down_checks&&self.y_vel>0.0){
				self.y_vel/=4.0;
				self.delta_y=0.0;

				self.Box_x_pos += self.last_safe_x-self.ABSx_pos;
				self.Box_y_pos += self.last_safe_y-self.ABSy_pos;
				self.ABSx_pos = self.last_safe_x;
				self.ABSy_pos = self.last_safe_y;

				//UP/DOWN COLLISION
			}
			if (left_checks&&self.x_vel<0.0)||(right_checks&&self.x_vel>0.0){
				self.x_vel/=4.0;
				self.delta_x=0.0;

				self.Box_x_pos += self.last_safe_x-self.ABSx_pos;
				self.Box_y_pos += self.last_safe_y-self.ABSy_pos;
				self.ABSx_pos = self.last_safe_x;
				self.ABSy_pos = self.last_safe_y;

				//LEFT/RIGHT COLLISION
			}
		}else{
			self.last_safe_x=self.ABSx_pos;
			self.last_safe_y=self.ABSy_pos;
		}
		// Add velocity to position, clamp to ensure bounds are never exceeded.
		// TILE_SIZE * 4 because the TILE_SIZE are scaled x4

		if((self.ABSx_pos-self.Box_x_pos <= (CAM_W/2+5) as f32) && (self.ABSx_pos-self.Box_x_pos >= (CAM_W/2-5) as f32))
		{
			self.Box_x_pos = (self.Box_x_pos + self.x_vel).clamp(0.0, (FullW-CAM_W) as f32);
		}
		if((self.ABSy_pos-self.Box_y_pos <= (CAM_H/2+5) as f32) && (self.ABSy_pos-self.Box_y_pos >= (CAM_H/2-5) as f32))
		{
			self.Box_y_pos = (self.Box_y_pos + self.y_vel).clamp(0.0, (FullH-CAM_H) as f32);
		}

		self.ABSx_pos = (self.ABSx_pos + self.x_vel).clamp(0.0, FullW as f32 - (TILE_SIZE as f32));
		self.ABSy_pos = (self.ABSy_pos + self.y_vel).clamp(0.0, FullH as f32 - (TILE_SIZE as f32));

		self.x_pos = (self.ABSx_pos-self.Box_x_pos).clamp(0.0, CAM_W as f32 - (TILE_SIZE as f32));
		self.y_pos = (self.ABSy_pos-self.Box_y_pos).clamp(0.0, CAM_H as f32 - (TILE_SIZE as f32));


	}
}


struct Enemy<'a> {
	//src: Rect,
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
