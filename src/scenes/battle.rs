use std::rc::Rc;
use std::cell::RefCell;

use sdl2::pixels::Color;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::keyboard::Keycode;

use crate::scenes::Scene;
use crate::scenes::GameEvent;
use crate::events::event_subsystem::EventSystem;
use crate::game_manager::TextureManager;
use crate::video::text::FontManager;

use crate::cards::game_structs::Card;
use crate::cards::game_structs::Battler;
use std::fs;
use std::collections::HashMap;

//orig_posx = u32;
//orig_posy = u32;
//const orig_sizew: u32 = 120;
//const orig_sizeh: u32 = 178;
//const new_posx: u32;
//const new_posy: u32;
//const new_sizew: u32;
//const new_sizeh: u32;


pub struct Battle<'a> {
	wincan: Rc<RefCell<WindowCanvas>>,
	event_system: Rc<RefCell<EventSystem>>,
	font_manager: Rc<RefCell<FontManager<'a>>>,
	test_1: Rc<Texture<'a>>,
	test_2: Rc<Texture<'a>>,
	test_3: Rc<Texture<'a>>,
	play_i: Rc<Texture<'a>>,
	health: Rc<Texture<'a>>,
	deck: Rc<Texture<'a>>,
	drop: Rc<Texture<'a>>,
	tmp_button: Rc<Texture<'a>>,
}

impl<'a> Battle<'a> {
	pub fn init(texture_manager: Rc<RefCell<TextureManager<'a>>>, wincan: Rc<RefCell<WindowCanvas>>, event_system: Rc<RefCell<EventSystem>>, font_manager: Rc<RefCell<FontManager<'a>>>)  -> Result<Self, String> {
		let test_1 = texture_manager.borrow_mut().load("assets/templates/Attack_Card.png")?;
		let test_2 = texture_manager.borrow_mut().load("assets/templates/Defend_Card.png")?;
		let test_3 = texture_manager.borrow_mut().load("assets/templates/Heal_Card.png")?;
		let play_i = texture_manager.borrow_mut().load("assets/temp_player_icons/icondummy.png")?;
		let health = texture_manager.borrow_mut().load("assets/temp_health.png")?;
		let deck = texture_manager.borrow_mut().load("assets/cards/Card Back.png")?;
		let drop = texture_manager.borrow_mut().load("assets/wood_texture.png")?;
		let tmp_button = texture_manager.borrow_mut().load("assets/tmp.png")?;
		Ok(Battle {
			wincan,
			event_system,
			font_manager,
			test_1,
			test_2,
			test_3,
			play_i,
			health,
			deck,
			drop,
			tmp_button,
		})
	}
}

impl Scene for Battle<'_> {
	fn handle_input(&mut self, event: GameEvent) {
		match event {
			GameEvent::KeyPress(k) => {
				//println!("{}", k);
				if k.eq(&Keycode::Escape) {self.event_system.borrow().change_scene(1).unwrap();}
			},
			_ => {println!("No event")},
		}

	}

	fn render(&mut self) -> Result<(), String> {
		let mut wincan = self.wincan.borrow_mut();
		crate::video::gfx::fill_screen(&mut wincan, Color::RGB(154, 195, 225));

		//hardcoded for now too test to make sure the cards and other items appear in the correct places

		//backroop for cards
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.drop,(1280,300), (0,550))?; //wood for the back
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.drop,(1280,180), (0,0))?; //wood for the back

		//player
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.test_1,(100,148), (980,560))?;
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.test_2,(100,148), (860,560))?;
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.test_3,(100,148), (740,560))?;
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.test_2,(100,148), (620,560))?;
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.test_1,(100,148), (500,560))?;
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.test_3,(100,148), (380,560))?;
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.test_1,(100,148), (260,560))?;
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (1140,560))?;
		//enemy side
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (920,20))?;
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (800,20))?;
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (680,20))?;
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (560,20))?;
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (440,20))?;
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (320,20))?;
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (200,20))?;
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (40,20))?;
		//mostly static objects (health bars change tho)
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.health,(300,20), (790,520))?; //player health bar
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.health,(300,20), (200,190))?; //enemy health bar
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.play_i,(150,150), (60,560))?; //player icon
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.play_i,(150,150), (1070,20))?; //enemy icon
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.play_i,(150,150), (1070,20))?; //enemy icon
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.tmp_button,(300,100), (0,300))?;
		wincan.present();
		Ok(())
	}
}

//card size and position
struct card<'a>{
	x_size: u32,
	y_size: u32,
	x_pos: u32,
	y_pos: u32,
	sprite: Rc<Texture<'a>>,
}

impl<'a>card<'a>{
	fn update_size(&mut self){

	}
}
