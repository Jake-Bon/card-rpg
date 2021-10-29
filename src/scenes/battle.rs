use std::rc::Rc;
use std::cell::RefCell;

use sdl2::pixels::Color;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::keyboard::Keycode;

use crate::scenes::Scene;
use crate::scenes::GameEvent;
use crate::events::event_subsystem::EventSystem;
use crate::game_manager::TextureManager;

use crate::cards::game_structs::Card;
use crate::cards::game_structs::Battler;
use std::fs;
use std::collections::HashMap;

use crate::cards::battle_enums::TurnPhase;

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
	test_1: Rc<Texture<'a>>,
	test_2: Rc<Texture<'a>>,
	test_3: Rc<Texture<'a>>,
	play_i: Rc<Texture<'a>>,
	health: Rc<Texture<'a>>,
	deck: Rc<Texture<'a>>,
	drop: Rc<Texture<'a>>,
	tmp_button: Rc<Texture<'a>>,
	accepting_input: bool,

	// BATTLE DATA
	file_data: String,
	card_map: HashMap<u32, Card<'a>>,
	active_player: i8,
	turn: TurnPhase,
	
}

impl<'a> Battle<'a> {
	pub fn init(texture_manager: Rc<RefCell<TextureManager<'a>>>, wincan: Rc<RefCell<WindowCanvas>>, event_system: Rc<RefCell<EventSystem>>)  -> Result<Self, String> {
		let test_1 = texture_manager.borrow_mut().load("assets/templates/Attack_Card.png")?;
		let test_2 = texture_manager.borrow_mut().load("assets/templates/Defend_Card.png")?;
		let test_3 = texture_manager.borrow_mut().load("assets/templates/Heal_Card.png")?;
		let play_i = texture_manager.borrow_mut().load("assets/temp_player_icons/icondummy.png")?;
		let health = texture_manager.borrow_mut().load("assets/temp_health.png")?;
		let deck = texture_manager.borrow_mut().load("assets/cards/Card Back.png")?;
		let drop = texture_manager.borrow_mut().load("assets/wood_texture.png")?;
		let tmp_button = texture_manager.borrow_mut().load("assets/tmp.png")?;
		let accepting_input = true;

		let new_file = fs::read_to_string("src/cards/card-library.txt").expect("An error occurred whilst attempting to open the library.").to_owned();
		let card_map = HashMap::new();
		
		Ok(Battle {
			wincan,
			event_system,
			test_1,
			test_2,
			test_3,
			play_i,
			health,
			deck,
			drop,
			tmp_button,
			accepting_input,
			file_data: new_file.to_owned(),
			card_map,
			active_player: 1,
			turn: TurnPhase::NotInitialized,
		})
	}
	
	pub fn step(&'a mut self) -> Result<u8, String> {
	    
	    // initialize things at the start of battle
	    if self.turn == TurnPhase::NotInitialized {
	        
	        // Populate the card map
	        self.card_map = crate::cards::battle_system::populate_card_map(&self.file_data);
	        
	        // player structs and decks will be initialized here
	        
	        self.turn = TurnPhase::PreTurnP1;
	        
	    }
	    
	    if self.active_player == 1 {
	        
	        if self.turn == TurnPhase::TurnP1 {
	            
	            // Essentially just waits until the end turn button is pressed
	            // All of the card playing logic should be in the handle input function
	            
	            // Could also check in here if the player loses all of their health or runs out of cards, to enable designing cards around that
	            
	            // self.turn should be changed to TurnPhase::PostTurnP1 when clicking the end turn button
	            
	        }
	        else if self.turn == TurnPhase::PreTurnP1 {
	            // Resolve things that need to be resolved prior to the Player's turn in here
	            // Intended to check for Statuses that need to be removed at the beginning of the turn
	            
	            // Can add drawing a card in here and checking handsize/remaining cards
	            
	            // Move to the next phase of the turn
	            self.turn == TurnPhase::TurnP1;
	        }
	        else if self.turn == TurnPhase::PostTurnP1 {
	            // Resolve things that need to be resolved after the Player's turn in here
	            // Intended to check for Statuses that need to be removed at the end of the turn
	            
	            self.active_player = -1;
	        }
	        
	        
	    }
	    
	    // Enemy logic in the else
	    else{
	        if self.turn == TurnPhase::TurnP2 {
	            
	            // Enemy AI should be called from here
	            
	            //
	            
	        }
	        else if self.turn == TurnPhase::PreTurnP2 {
	            // Resolve things that need to be resolved prior to the Opponent's turn in here
	            // Intended to check for Statuses that need to be removed at the beginning of the turn
	            
	            // Can add drawing a card in here and checking handsize/remaining cards
	            
	            // Move to the next phase of the turn
	            self.turn == TurnPhase::TurnP2;
	        }
	        else if self.turn == TurnPhase::PostTurnP2 {
	            // Resolve things that need to be resolved after the Opponent's turn in here
	            // Intended to check for Statuses that need to be removed at the end of the turn
	            
	            self.active_player = 1;
	        }
	    }
	    
	    return Ok(0);   
	    
	}
	

	
}

impl Scene for Battle<'_> {
	
	fn handle_input(&mut self, event: GameEvent) {
		
		// Some input should be restricted if it isn't the player's turn
		
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

		//backdrop for cards
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
