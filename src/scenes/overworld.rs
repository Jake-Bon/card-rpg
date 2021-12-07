use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
//use std::num::*;
use std::io::prelude::*;
use std::time::{Instant,Duration};

use serde::Deserialize;
use rand::{thread_rng,Rng};

//use image::io::Reader as ImageReader;
//use image::{Rgba,GenericImage};

use sdl2::pixels::Color;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::keyboard::Keycode;
use sdl2::mixer::Music;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};


use crate::scenes::Scene;
use crate::scenes::GameEvent;
use crate::events::event_subsystem::EventSystem;
use crate::game_manager::TextureManager;
use crate::video::text::FontManager;
use crate::video::gfx::CAM_W;
use crate::video::gfx::CAM_H;
use crate::video::gfx::TILE_SIZE;

const FullW: u32 = 2400;
const FullH: u32 = 1800;

const TileW: u32 = FullW/TILE_SIZE;
const TileH: u32 = FullH/TILE_SIZE;
const SpriteTILE_SIZE: u32 = 40;
const enemyNum: f32 = 20.0;
const perMap:i32 = 1;

const SPEED_LIMIT: f32 = 6.0;
const ACCEL_RATE: f32 = 0.3;

//mod crate::video;

fn map_reader() -> Result<Vec<Vec<u8>>,String>{
	//"src/scenes/world-1.bmp"
	let fileList = vec!["src/scenes/world-1.bmp","src/scenes/world-2.bmp"];
	let mut mapList = Vec::new();
	let mut rng = thread_rng();
	for f in fileList{
		let file = File::open(f.to_string()).expect("File not opened.");
		let mut buf_reader = BufReader::new(file);

		let mut bytes: Vec<u8> = Vec::new();
		buf_reader.read_to_end(&mut bytes);

		bytes.drain(0..70);//remove bmp header + Color header
		bytes.drain((TileH*TileW) as usize..);//remove bmp header + Color header
		let mut map: Vec<u8> = Vec::new();
		for i in 0..TileH{ //mirror map vertically to how it is drawn
			let j = TileH as i32-i as i32;
			map.extend_from_slice(&(bytes[(j-1) as usize*TileW as usize..j as usize*TileW as usize]));
		}

		let mut random_num: f32 = 0.0;

		//map.drain(0..TileW as usize);

		for i in 0..map.len(){
			random_num = rng.gen_range(0.0..2.0);
			if random_num>=1.7{
				if map[i]==1{
					map[i]+=10;
				}else if map[i]==2{
					map[i]+=10;
				}
			}
		}
		mapList.push(map);
	}


	Ok(mapList)
}

pub struct Overworld<'a> {
	wincan: Rc<RefCell<WindowCanvas>>,
	texture_manager: Rc<RefCell<TextureManager<'a>>>,
	font_manager: Rc<RefCell<FontManager<'a>>>,
	event_system: Rc<RefCell<EventSystem>>,
	tile_set: Rc<Texture<'a>>,
	player: Player<'a>,
	jsonEnemyLs: Vec<npcData>,
	enemy: Vec<Enemy<'a>>,
	anim_water: u32,
	frames: u32,
	maps: Vec<Vec<u8>>,
	map_rep: Vec<u8>,
	music: Music<'a>,
	elapsed: Instant,
	last_time: f64,
	is_stopped: bool,
	is_paused: bool,
	gameMode: Modes<'a>,
	currMap: u32,
	win_or_loss: u32,
}

impl<'a> Overworld<'a> {
	pub fn init( texture_manager: Rc<RefCell<TextureManager<'a>>>, wincan: Rc<RefCell<WindowCanvas>>, event_system: Rc<RefCell<EventSystem>>,font_manager: Rc<RefCell<FontManager<'a>>>)  -> Result<Self, String> {
		//let tile_map = [0; 144];
		//let tile_set = texture_manager.borrow_mut().load("assets/download.png")?;
		let tile_set = texture_manager.borrow_mut().load("assets/tile_sheet4x.png")?;
		let maps = map_reader()?;
		let map_rep = maps[0].clone();

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

		let mut i=0;
		let mut enemy: Vec<Enemy> =  Vec::new();
		let mut rng = thread_rng();


		//-----------------------------------------------------------------------------------
		let path = Path::new("src/scenes/npc_loader.json");
		let mut s = String::new();
		let file = File::open(path).unwrap().read_to_string(&mut s).unwrap();
    	//let reader = BufReader::new(file);
		let jsonEnemyLs: Vec<npcData> = serde_json::from_str(&s).unwrap();
		//-----------------------------------------------------------------------------

		while (i as f32) < enemyNum
		{

			let mut random_x: f32 = rng.gen_range(0.0..(FullW - TILE_SIZE) as f32);
			let mut random_y: f32 =rng.gen_range(0.0..(FullH-TILE_SIZE) as f32);

			'inner: loop
			{
				let mut random_x: f32 = rng.gen_range(0.0..(FullW-TILE_SIZE) as f32);
				random_x -= random_x%TILE_SIZE as f32;
			 	let mut random_y: f32 = rng.gen_range(0.0..(FullH-TILE_SIZE) as f32);
				random_y -= random_y%TILE_SIZE as f32;

				let mut person: i32 = rng.gen_range(0..6 as i32);

				//ensure enemy is generated in a safe area
				if !(random_x<(player.Box_x_pos+CAM_W as f32))||!(random_y<(player.Box_y_pos+CAM_H as f32))||!(random_x>(player.Box_x_pos as f32))||!(random_y>(player.Box_y_pos as f32))
				{
					let chosen_map = rng.gen_range(0..2);
					let map_x = (random_x/TILE_SIZE as f32) as usize;
					let map_y = (random_y/TILE_SIZE as f32) as usize;
					let map_x_right = if map_x>=(TileW-1) as usize{map_x}else{map_x+1};
					let map_y_down = if map_y>=(TileH-1) as usize{map_y}else{map_y+1};

					let orig = maps[chosen_map][map_x + TileW as usize*map_y]==0; //2x2 area needed for safe gen
					let right = maps[chosen_map][map_x_right + TileW as usize*map_y]==0;
					let down = maps[chosen_map][map_x + TileW as usize*map_y_down]==0;
					let diag = maps[chosen_map][map_x_right + TileW as usize*map_y_down]==0;

					if !(random_x<=4.0||random_y<=4.0||random_x>=FullW as f32-TILE_SIZE as f32||random_y>=FullH as f32-TILE_SIZE as f32){
						if orig&&right&&down&&diag{ //Create enemy if: OUTSIDE player camera,
							enemy.push( Enemy {		//NOT TOO CLOSE TO border, NO land collisions
								ABSx_pos: random_x,
								ABSy_pos: random_y,
								x_vel: rng.gen_range(0.0..2.0 as f32),
								y_vel: rng.gen_range(0.0..2.0 as f32),
								timer: Instant::now(),
								sprite: texture_manager.borrow_mut().load("assets/simple_enemy_sprite.png")?,
								map_copy: maps[chosen_map].clone(),
								last_safe_x: random_x,
								last_safe_y: random_y,
								is_flipped: false,
								on_map: (chosen_map+1) as u32,
								npc_id: i as u32,//eventually get range from NPC
								npcDia:person// this will be random later but for now it just 0 and each map will get 1 character
							});
							break 'inner;
						}
					}

				}
			 }
			i=i+1;
		}

		let frames = 0;
		let anim_water = 0;

		let frequency = 44100;
    	let format = AUDIO_S16LSB;
    	let channels = DEFAULT_CHANNELS;
    	let chunk_size = 1024;
    	sdl2::mixer::open_audio(frequency, format, channels, chunk_size)?;
    	let _mixer_context = sdl2::mixer::init(InitFlag::OGG)?;

		let music = Music::from_file("assets/music/MAP.ogg")?;
		let is_paused = false;
		let is_stopped = true;
		let elapsed = Instant::now();
		let last_time=0.0;

		let options = texture_manager.borrow_mut().load("assets/rustIsAwful/options1.png")?;
		let quit = texture_manager.borrow_mut().load("assets/rustIsAwful/quit1.png")?;
		let haze = texture_manager.borrow_mut().load("assets/rustIsAwful/haze1.png")?;
		let waste1 = texture_manager.borrow_mut().load("assets/rustIsAwful/haze1.png")?;
		let waste2 = texture_manager.borrow_mut().load("assets/rustIsAwful/haze1.png")?;
		let waste3 = texture_manager.borrow_mut().load("assets/rustIsAwful/haze1.png")?;
		let boxen = texture_manager.borrow_mut().load("assets/rustIsAwful/Dialogue_box_.png")?;
		let por = texture_manager.borrow_mut().load("assets/rustIsAwful/icon_backdrop.png")?;

		let tempdia = DialogRunner {
			line: 0, // this tells us the current line
			inText: 0, //this helps with the sub string
			numVectorVal: 0, //this tells what npc to use
			player: waste1, //store player pic
			background: waste2, //store player pic
			npc: waste3, //store
			diabox: boxen,
			portraitbox: por,
		};
		let gameMode = Modes {
			State: 1,
		    midSentence: true,
			key: false,
			options_button: options,
			quit_button: quit,
			haze_button: haze,
			dialog: tempdia,
			prepost: true,
		};


		Ok(Overworld{
			wincan,
			event_system,
			texture_manager,
			font_manager,
			tile_set,
			player,
			jsonEnemyLs,
			enemy,
			frames,
			anim_water,
			maps,
			map_rep,
			music,
			is_paused,
			is_stopped,
			elapsed,
			last_time,
			gameMode,
			currMap:1,
			win_or_loss:1,
		})
	}

}

impl Scene for Overworld<'_> {

	fn handle_input(&mut self, event: GameEvent) {
		// Matching events, most importantly KeyPress(k)'s
		match self.gameMode.State {
			0=> match event {
				GameEvent::WinOrLoss(stat) => {
					self.win_or_loss = stat;
					println!("The state: {}",self.win_or_loss);
				},
				_ => {println!(/*"No event"*/)},
			}
			1=>	match event {
					GameEvent::KeyPress(k) => {
						//println!("p:{}", k);
						if k.eq(&Keycode::W) {self.player.keyPress[0]=true}
						if k.eq(&Keycode::S) {self.player.keyPress[1]=true}
						if k.eq(&Keycode::A) {self.player.keyPress[2]=true}
						if k.eq(&Keycode::D) {self.player.keyPress[3]=true}
						if k.eq(&Keycode::Escape) {self.gameMode.State=2}

					},
					GameEvent::KeyRelease(k) => {
						//println!("r:{}", k);
						if k.eq(&Keycode::W) {self.player.keyPress[0]=false}
						if k.eq(&Keycode::S) {self.player.keyPress[1]=false}
						if k.eq(&Keycode::A) {self.player.keyPress[2]=false}
						if k.eq(&Keycode::D) {self.player.keyPress[3]=false}
					},
					GameEvent::WinOrLoss(stat) => {
						self.win_or_loss = stat;
					},
					_ => {println!(/*"No event"*/)},

			}//Default world

			2=>	match event {
					GameEvent::KeyPress(k) => {
						//println!("p:{}", k);
						if k.eq(&Keycode::Escape) {self.gameMode.State=1}
					},
					GameEvent::MouseClick(x_pos,y_pos) => {
						if (x_pos > 50 && x_pos < 300) && (y_pos > 500 && y_pos < 600) {
							println!("options");
							println!("X {}, Y: {}", x_pos, y_pos);
							self.event_system.borrow().change_scene(5).unwrap();
						}
						if (x_pos > 50 && x_pos < 300) && (y_pos > 600 && y_pos < 700)  {
							println!("quit");
							println!("X {}, Y: {}", x_pos, y_pos);
							self.event_system.borrow().change_scene(4).unwrap();
						}
					},//exit or options
					GameEvent::WinOrLoss(stat) => {
						self.win_or_loss = stat;
					},
					_ => {println!(/*"No event"*/)},//pause menu
			} //pause

			3=>	match event {
					GameEvent::KeyPress(k) => {
						//println!("p:{}", k);
						if k.eq(&Keycode::Space) {self.gameMode.key=true}
					},
					GameEvent::WinOrLoss(stat) => {
						self.win_or_loss = stat;
					},
					_ => {println!(/*"No event"*/)},
			}//talking
			_ => {match event{
				GameEvent::WinOrLoss(stat) => {
					self.win_or_loss = stat;
				},
				_ => println!(/*"No event"*/)
			}
		},//pause menu
		}
	}


	fn render(&mut self) -> Result<(), String> {
		if(self.is_stopped){
			self.is_stopped = false;
			self.music = Music::from_file("assets/music/MAP.ogg")?;
			self.music.play(-1);
			self.elapsed = Instant::now() - Duration::from_secs_f64(self.last_time);
			sdl2::mixer::Music::set_pos(self.last_time);
		}
		if(self.is_paused){
			self.is_paused = false;
			sdl2::mixer::Music::resume();
		}
		let mut wincan = self.wincan.borrow_mut();
		if (self.gameMode.State==1)
		{
			self.player.update_movement();

			self.frames = if (self.frames) > 9 {
				self.anim_water = (self.anim_water+1)%9;
				0
			}
			else {
				self.frames + 1
			};

			// hard coded enemy collision
			let mut i = 0 ;
			while (i as usize) < self.enemy.len()
			{
					self.enemy[i as usize].update_movement();
					if self.enemy[i].on_map==self.currMap&&(f32::powf(self.enemy[i as usize].ABSx_pos - self.player.ABSx_pos,2.0) + f32::powf(self.enemy[i as usize].ABSy_pos - self.player.ABSy_pos,2.0).sqrt()) < 40.0
					{
						self.player.x_vel=0.0;
						self.player.y_vel=0.0;
						self.player.delta_x=0.0;
						self.player.delta_y=0.0;
						let id = self.enemy[i].npcDia as u32;
						self.player.keyPress[0]=false;
						self.player.keyPress[1]=false;
						self.player.keyPress[2]=false;
						self.player.keyPress[3]=false;
						self.is_stopped = true;
						sdl2::mixer::Music::halt();
						println!("========is stopped {}",self.is_stopped);

						// set the enemy's deck here. could randomize/set it here or set it during enemy creation
						// the number passed into the function corresponds to the deck with the same number in battler-library.txt
						self.gameMode.dialog.numVectorVal=self.enemy[i].npcDia as usize;
						self.enemy.remove(i as usize);  // remove the enemy
						self.event_system.borrow().set_battler_npc_deck(self.jsonEnemyLs[self.gameMode.dialog.numVectorVal].deckID as u32).unwrap(); //use NPC DECK ID
						self.event_system.borrow().send_enemy_to_battle(id).unwrap(); //use NPC BATTLER ID
						self.last_time = self.elapsed.elapsed().as_secs_f64()%203.0;//song length 3:23
						self.gameMode.dialog.player = self.texture_manager.borrow_mut().load(&(self.jsonEnemyLs[self.gameMode.dialog.numVectorVal].sprites[0]))?;
						self.gameMode.dialog.npc = self.texture_manager.borrow_mut().load(&(self.jsonEnemyLs[self.gameMode.dialog.numVectorVal].sprites[1]))?;
						self.gameMode.dialog.line =0;
						self.gameMode.dialog.inText=0;
						self.gameMode.prepost=true;
						//println!("about to change scene now...");
						//self.event_system.borrow().change_scene(2).unwrap();
						self.gameMode.State=3;
					}
					//println!("p:{}", self.player.ABSx_pos);
				i=i+1;

			}

		}

		if self.currMap==1&&self.player.ABSy_pos<=5.0{
			self.currMap=2;
			self.map_rep = self.maps[1].clone();
			self.player.map_copy = self.maps[1].clone();
			self.player.ABSy_pos = FullH as f32-60.0;
			self.player.Box_y_pos = (self.player.ABSy_pos).clamp(0.0, (FullH-CAM_H) as f32);
			self.player.last_safe_y = self.player.ABSy_pos;
		}

		if self.currMap==2&&self.player.ABSy_pos>=(FullH-45) as f32{
			self.currMap=1;
			self.map_rep = self.maps[0].clone();
			self.player.map_copy = self.maps[0].clone();
			self.player.ABSy_pos = 10.0;
			self.player.Box_y_pos = (0.0 as f32).clamp(0.0, (FullH-CAM_H) as f32);
			self.player.last_safe_y = self.player.ABSy_pos;
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
			}else if self.map_rep[i as usize]==11{ //Sand + Palm
				sprite_x = SpriteTILE_SIZE as i32;
				sprite_y = SpriteTILE_SIZE as i32;
			}else if self.map_rep[i as usize]==2{ //GRASS
				sprite_x = (SpriteTILE_SIZE as i32)*2 as i32;
				sprite_y = SpriteTILE_SIZE as i32;
			}else if self.map_rep[i as usize]==12{ //Grass + Flower
				sprite_x = (SpriteTILE_SIZE as i32)*3 as i32;
				sprite_y = SpriteTILE_SIZE as i32;
			}else if self.map_rep[i as usize]==3{
				sprite_x = (self.anim_water*SpriteTILE_SIZE) as i32;
				sprite_y = (SpriteTILE_SIZE as i32)*2 as i32;
			}else{//Add more types if needed
				continue;
			}
			let x = (i%TileW as i32)*TILE_SIZE as i32;
			let y = (i/TileW as i32)*TILE_SIZE as i32;

			crate::video::gfx::tile_sprite_from_sheet_resize(&mut wincan, &self.tile_set,(sprite_x,sprite_y),(SpriteTILE_SIZE,SpriteTILE_SIZE),(TILE_SIZE,TILE_SIZE),(-(self.player.Box_x_pos as i32)+x,-(self.player.Box_y_pos as i32)+y),(1,1));
		}

		// draw enemy
		for i in 0 as usize..self.enemy.len(){
			if self.enemy[i].on_map==self.currMap{
				self.enemy[i].is_flipped = if self.enemy[i].x_vel<0.0{false}else if self.enemy[i].x_vel>0.0{true}else{self.enemy[i].is_flipped};
				crate::video::gfx::draw_sprite_mirror(&mut wincan, &self.enemy[i].sprite, (self.enemy[i].ABSx_pos as i32-self.player.Box_x_pos as i32, self.enemy[i].ABSy_pos as i32-self.player.Box_y_pos as i32),self.enemy[i].is_flipped,false)?;
			}
		}

		// Draw player
		self.player.is_flipped = if self.player.x_vel>0.0{true}else if self.player.x_vel<0.0{false}else{self.player.is_flipped};
		crate::video::gfx::draw_sprite_mirror(&mut wincan, &self.player.sprite, (self.player.x_pos as i32, self.player.y_pos as i32),self.player.is_flipped,false)?;




		if (self.gameMode.State==2)//pause
		{
			crate::video::gfx::draw_sprite_to_fit(&mut wincan, &self.gameMode.haze_button)?;
			crate::video::gfx::draw_sprite(&mut wincan, &self.gameMode.options_button, (50, 500))?;
			crate::video::gfx::draw_sprite(&mut wincan, &self.gameMode.quit_button, (50, 600))?;
		}
		else if (self.gameMode.State==3)//talking
		{
			crate::video::gfx::draw_sprite_to_fit(&mut wincan,  &self.gameMode.haze_button)?;
			crate::video::gfx::draw_sprite(&mut wincan, &self.gameMode.dialog.diabox, (50, 400))?;

			crate::video::gfx::draw_sprite(&mut wincan, &self.gameMode.dialog.portraitbox, (CAM_W as i32-350, 50))?;//xy
			crate::video::gfx::draw_sprite(&mut wincan, &self.gameMode.dialog.portraitbox, (50, 50))?;

			if(self.gameMode.key==true)//press space
			{
				if(self.gameMode.prepost==true)//pre
				{
					if (self.gameMode.dialog.line as usize >= self.jsonEnemyLs[self.gameMode.dialog.numVectorVal].predialog.len()-1)//into combat
					{
						//set all values for next loop
						self.gameMode.prepost=false;
						self.gameMode.dialog.line=0;
						self.event_system.borrow().change_scene(2).unwrap();
					}
					else
					{
						self.gameMode.dialog.line=self.gameMode.dialog.line+1;
					}

				}
				else//post
				{
					if self.win_or_loss == 2{//win
						if (self.gameMode.dialog.line == (self.jsonEnemyLs[self.gameMode.dialog.numVectorVal].postdialog.len()-1 )as i32)//into combat
						{
							//reset
							self.gameMode.prepost=true;
							self.gameMode.dialog.line=0;
							self.gameMode.State=1;
							self.win_or_loss = 1;
						}
						else
						{
							self.gameMode.dialog.line=self.gameMode.dialog.line+1;
						}
					}else{
						if (self.gameMode.dialog.line == (self.jsonEnemyLs[self.gameMode.dialog.numVectorVal].postdialoglose.len()-1 )as i32)//into combat
						{
							//reset
							self.gameMode.prepost=true;
							self.gameMode.dialog.line=0;
							self.gameMode.State=1;
							self.win_or_loss = 1;
						}
						else
						{
							self.gameMode.dialog.line=self.gameMode.dialog.line+1;
						}
					}

				}
				self.gameMode.key=false;
			}
			else// no input from player
			{
				if(self.gameMode.key==true)//pre
				{

					if(self.gameMode.midSentence==true)//we are mid sentence
					{
						self.gameMode.dialog.inText = self.gameMode.dialog.inText+1;
					}
				}
				else//post
				{

					if(self.gameMode.midSentence==true)//we are mid sentence
					{
						self.gameMode.dialog.inText = self.gameMode.dialog.inText+1;
					}
				}
			}
			//handle screen here
			if(self.gameMode.prepost==true)//pre
			{
				let A =  self.jsonEnemyLs[self.gameMode.dialog.numVectorVal].portraitPlayerPre[self.gameMode.dialog.line as usize];
				let B =  self.jsonEnemyLs[self.gameMode.dialog.numVectorVal].portraitEnemyPre[self.gameMode.dialog.line as usize];
				crate::video::gfx::tile_sprite_from_sheet_resize(&mut wincan, &self.gameMode.dialog.player,(500*A-500 as i32,0),(500 as u32,500 as u32),(260 as u32,260 as u32),(70,70),(1,1))?;
				crate::video::gfx::tile_sprite_from_sheet_resize(&mut wincan, &self.gameMode.dialog.npc,(500*B-500 as i32,0),(500 as u32,500 as u32),(260 as u32,260 as u32),(950,70),(1,1))?;
				self.font_manager.borrow_mut().draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 22, Color::RGB(0, 0, 0),&(self.jsonEnemyLs[self.gameMode.dialog.numVectorVal].predialog[self.gameMode.dialog.line as usize]),  (80, 450));
			}
			else
			{
				if self.win_or_loss==2{
					let A =  self.jsonEnemyLs[self.gameMode.dialog.numVectorVal].portraitPlayerPost[self.gameMode.dialog.line as usize];
					let B =  self.jsonEnemyLs[self.gameMode.dialog.numVectorVal].portraitEnemyPost[self.gameMode.dialog.line as usize];
					crate::video::gfx::tile_sprite_from_sheet_resize(&mut wincan, &self.gameMode.dialog.player,(500*A-500 as i32,0),(500 as u32,500 as u32),(260 as u32,260 as u32),(70,70),(1,1))?;
					crate::video::gfx::tile_sprite_from_sheet_resize(&mut wincan, &self.gameMode.dialog.npc,(500*B-500 as i32,0),(500 as u32,500 as u32),(260 as u32,260 as u32),(950,70),(1,1))?;
					self.font_manager.borrow_mut().draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 22, Color::RGB(0, 0, 0),&(self.jsonEnemyLs[self.gameMode.dialog.numVectorVal].postdialog[self.gameMode.dialog.line as usize]),  (80, 450));
				}else{
					let A =  self.jsonEnemyLs[self.gameMode.dialog.numVectorVal].portraitPlayerPostlose[self.gameMode.dialog.line as usize];
					let B =  self.jsonEnemyLs[self.gameMode.dialog.numVectorVal].portraitEnemyPostlose[self.gameMode.dialog.line as usize];
					crate::video::gfx::tile_sprite_from_sheet_resize(&mut wincan, &self.gameMode.dialog.player,(500*A-500 as i32,0),(500 as u32,500 as u32),(260 as u32,260 as u32),(70,70),(1,1))?;
					crate::video::gfx::tile_sprite_from_sheet_resize(&mut wincan, &self.gameMode.dialog.npc,(500*B-500 as i32,0),(500 as u32,500 as u32),(260 as u32,260 as u32),(950,70),(1,1))?;
					self.font_manager.borrow_mut().draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 22, Color::RGB(0, 0, 0),&(self.jsonEnemyLs[self.gameMode.dialog.numVectorVal].postdialoglose[self.gameMode.dialog.line as usize]),  (80, 450));
				}
				}
		}


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

		if self.keyPress[0]//w
		{
			self.delta_y -= ACCEL_RATE;
			self.y_vel = (self.y_vel + self.delta_y)
				.clamp(-SPEED_LIMIT, SPEED_LIMIT);
		}
		else if self.keyPress[1]//s
		{
			self.delta_y += ACCEL_RATE;
			self.y_vel = (self.y_vel + self.delta_y)
				.clamp(-SPEED_LIMIT, SPEED_LIMIT);
		}
		else //neither
		{
			if self.y_vel>0.0
			{
				self.delta_y -= ACCEL_RATE;
				self.y_vel = (self.y_vel + self.delta_y)
					.clamp(-SPEED_LIMIT, SPEED_LIMIT);
				if self.y_vel<0.0
				{
					self.delta_y = 0.0;
					self.y_vel = 0.0;
				}
			}
			else if self.y_vel<0.0
			{
				self.delta_y += ACCEL_RATE;
				self.y_vel = (self.y_vel + self.delta_y)
					.clamp(-SPEED_LIMIT, SPEED_LIMIT);
				if self.y_vel>0.0
				{
					self.delta_y = 0.0;
					self.y_vel = 0.0;
				}
			}
		}


		if self.keyPress[2]//a
		{
			self.delta_x -= ACCEL_RATE;
			self.x_vel = (self.x_vel + self.delta_x)
				.clamp(-SPEED_LIMIT, SPEED_LIMIT);
		}
		else if self.keyPress[3]//d
		{
			self.delta_x += ACCEL_RATE;
			self.x_vel = (self.x_vel + self.delta_x)
				.clamp(-SPEED_LIMIT, SPEED_LIMIT);
		}
		else
		{
			if self.x_vel>0.0
			{
				self.delta_x -= ACCEL_RATE;
				self.x_vel = (self.x_vel + self.delta_x)
					.clamp(-SPEED_LIMIT, SPEED_LIMIT);
				if self.x_vel< 0.0
				{
					self.x_vel = 0.0;
					self.delta_x=0.0;
				}
			}
			else if self.x_vel<0.0
			{
				self.delta_x += ACCEL_RATE;
				self.x_vel = (self.x_vel + self.delta_x)
					.clamp(-SPEED_LIMIT, SPEED_LIMIT);
				if self.x_vel>0.0
				{
					self.x_vel = 0.0;
					self.delta_x = 0.0
				}
			}

		}

		//check the ACCEL_RATE
		if self.x_vel==-SPEED_LIMIT||self.x_vel==SPEED_LIMIT
		{
			self.delta_x = 0.0;
		}
		if self.y_vel==-SPEED_LIMIT||self.y_vel==SPEED_LIMIT
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

		let up_checks = (self.map_copy[map_x+TileW as usize*(map_y)]!=0&&self.map_copy[map_x+TileW as usize*(map_y)]!=3)||(self.map_copy[map_x_right+TileW as usize*(map_y)]!=0&&self.map_copy[map_x_right+TileW as usize*(map_y)]!=3);
		let down_checks = (self.map_copy[map_x+TileW as usize*(map_y_down)]!=0&&self.map_copy[map_x+TileW as usize*(map_y_down)]!=3)||(self.map_copy[map_x_right+TileW as usize*(map_y_down)]!=0&&self.map_copy[map_x_right+TileW as usize*(map_y_down)]!=3);
		let left_checks = (self.map_copy[map_x+TileW as usize*(map_y)]!=0&&self.map_copy[map_x+TileW as usize*(map_y)]!=3)||(self.map_copy[map_x+TileW as usize*(map_y_down)]!=0&&self.map_copy[map_x+TileW as usize*(map_y_down)]!=3);
		let right_checks = (self.map_copy[map_x_right+TileW as usize*(map_y)]!=0&&self.map_copy[map_x_right+TileW as usize*(map_y)]!=3)||(self.map_copy[map_x_right+TileW as usize*(map_y_down)]!=0&&self.map_copy[map_x_right+TileW as usize*(map_y_down)]!=3);


		if up_checks||down_checks||left_checks||right_checks{ //If any collision detected
			if self.ABSx_pos > (CAM_W/2) as f32&&self.ABSx_pos < (FullW-CAM_W/2) as f32{ //If position is close to border
				self.Box_x_pos += self.last_safe_x-self.ABSx_pos;						//Ensure player snaps back
			}
			if self.ABSy_pos > (CAM_H/2) as f32&&self.ABSy_pos < (FullH-CAM_H/2) as f32{
				self.Box_y_pos += self.last_safe_y-self.ABSy_pos;
			}

			if (up_checks&&self.y_vel<0.0)||(down_checks&&self.y_vel>0.0){ //Checks directions according to velocity
				self.y_vel/=4.0;//lets player approach tile			   	//Following if has same structure
				self.delta_y=0.0;

				self.ABSx_pos = self.last_safe_x; //Stores last safe player position
				self.ABSy_pos = self.last_safe_y-self.y_vel; //Ensures player never stays in tile

				//UP/DOWN COLLISION
			}
			if (left_checks&&self.x_vel<0.0)||(right_checks&&self.x_vel>0.0){
				self.x_vel/=4.0;
				self.delta_x=0.0;

				self.ABSx_pos = self.last_safe_x -self.x_vel;
				self.ABSy_pos = self.last_safe_y;

				//LEFT/RIGHT COLLISION
			}
		}else{
			self.last_safe_x=self.ABSx_pos; //If unsafe position, revert
			self.last_safe_y=self.ABSy_pos;
		}
		// Add velocity to position, clamp to ensure bounds are never exceeded.
		// TILE_SIZE * 4 because the TILE_SIZE are scaled x4

		if(self.ABSx_pos-self.Box_x_pos <= (CAM_W/2+5) as f32) && (self.ABSx_pos-self.Box_x_pos >= (CAM_W/2-5) as f32)
		{
			self.Box_x_pos = (self.Box_x_pos + self.x_vel).clamp(0.0, (FullW-CAM_W) as f32);
		}
		if(self.ABSy_pos-self.Box_y_pos <= (CAM_H/2+5) as f32) && (self.ABSy_pos-self.Box_y_pos >= (CAM_H/2-5) as f32)
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
	x_vel: f32,
	y_vel: f32,
	sprite: Rc<Texture<'a>>,
	timer: Instant,
	map_copy: Vec<u8>,
	last_safe_x: f32,
	last_safe_y: f32,
	is_flipped: bool,
	on_map: u32,
	npc_id: u32,
	npcDia:i32
}

impl<'a> Enemy<'a> {
	fn update_movement(&mut self) {

		if self.timer.elapsed().as_secs()==3
		{
			self.timer = Instant::now();
			let mut rngx = thread_rng();
			let mut rngy = thread_rng();

			self.x_vel = rngx.gen_range(-2.0..2.0);
			self.y_vel = rngy.gen_range(-2.0..2.0);
		}

		let map_x = (self.ABSx_pos/TILE_SIZE as f32) as usize;
		let map_y = (self.ABSy_pos/TILE_SIZE as f32) as usize;
		let map_x_left = if map_x==0{map_x}else{map_x-1};
		let map_x_right = if map_x>=(TileW-1) as usize{map_x}else{map_x+1};
		let map_y_up = if map_y==0{map_y}else{map_y-1};
		let map_y_down = if map_y>=(TileH-1) as usize{map_y}else{map_y+1};

		let border_checks = self.ABSx_pos<=4.0||self.ABSy_pos<=4.0||self.ABSx_pos>=FullW as f32-TILE_SIZE as f32||self.ABSy_pos>=FullH as f32-TILE_SIZE as f32;
		let up_checks = self.map_copy[map_x+TileW as usize*(map_y)]!=0||self.map_copy[map_x_right+TileW as usize*(map_y)]!=0;
		let down_checks = self.map_copy[map_x+TileW as usize*(map_y_down)]!=0||self.map_copy[map_x_right+TileW as usize*(map_y_down)]!=0;
		let left_checks = self.map_copy[map_x+TileW as usize*(map_y)]!=0||self.map_copy[map_x+TileW as usize*(map_y_down)]!=0;
		let right_checks = self.map_copy[map_x_right+TileW as usize*(map_y)]!=0||self.map_copy[map_x_right+TileW as usize*(map_y_down)]!=0;

		if up_checks||down_checks||left_checks||right_checks||border_checks{ //If any collision detected
			self.timer = Instant::now();

			let mut rngx = thread_rng();
			let mut rngy = thread_rng();

			self.x_vel = rngx.gen_range(-2.0..2.0);
			self.y_vel = rngy.gen_range(-2.0..2.0);

			self.ABSx_pos = self.last_safe_x;
			self.ABSy_pos = self.last_safe_y;

		}else{
			self.last_safe_x=self.ABSx_pos; //If unsafe position, revert
			self.last_safe_y=self.ABSy_pos;
		}



		self.ABSx_pos = (self.ABSx_pos + self.x_vel).clamp(0.0, FullW as f32 - (TILE_SIZE as f32));
		self.ABSy_pos = (self.ABSy_pos + self.y_vel).clamp(0.0, FullH as f32 - (TILE_SIZE as f32));
	}
}

//###########################
//###########################
//###########################
#[derive(Debug, Deserialize)]
struct npcData
{
	npcName: String,
	deckID: i32,
	portraitPlayerPre: Vec<i32>,
	portraitEnemyPre: Vec<i32>,
	predialog: Vec<String>,
	portraitPlayerPost: Vec<i32>,
	portraitEnemyPost: Vec<i32>,
	postdialog: Vec<String>,
	portraitPlayerPostlose: Vec<i32>,
	portraitEnemyPostlose: Vec<i32>,
	postdialoglose: Vec<String>,
	sprites: Vec<String>,
}

struct Modes<'a>
{
    State: i32,
	key: bool,
	midSentence: bool,
	options_button: Rc<Texture<'a>>,
    quit_button: Rc<Texture<'a>>,
	haze_button: Rc<Texture<'a>>,
	dialog: DialogRunner<'a>, // literally a dialog runner
	prepost: bool, // true, pre combat ---- false, post combat
}

struct DialogRunner<'a>
{
	line: i32, // this tells us the current line
	inText: i32, //this helps with the sub string
	numVectorVal: usize, //this tells what npc to use
	player: Rc<Texture<'a>>, //store player pic
	npc: Rc<Texture<'a>>, //store
	background: Rc<Texture<'a>>, //store player pic
	diabox: Rc<Texture<'a>>,
	portraitbox: Rc<Texture<'a>>,
}
//###########################
