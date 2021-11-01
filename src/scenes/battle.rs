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

use crate::cards::game_structs::*;
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
	font_manager: Rc<RefCell<FontManager<'a>>>,
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
	battler_map: HashMap<u32, Battler>,
	active_player: i8,
	turn: TurnPhase,
	battle_handler: Rc<RefCell<BattleStatus>>,

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
		let accepting_input = true;
		let dummy = Rc::new(RefCell::new(Battler::new(("").to_string(),0,0,0,0)));  //REQUIRED TO AVOID USE
																		//of Option<T>. DO NOT REMOVE
		let battler_map = crate::cards::battle_system::populate_battler_map();

		let _p1 = Rc::new(RefCell::new(battler_map.get(&0).unwrap().clone())); //Must UNWRAP AND CLONE players from map for battle use
        let _p2 = Rc::new(RefCell::new(battler_map.get(&1).unwrap().clone()));

		let mut battle_handler = Rc::new(RefCell::new(BattleStatus::new(Rc::clone(&_p1),Rc::clone(&_p2))));

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
			accepting_input,
			battler_map,
			active_player: 1,
			turn: TurnPhase::NotInitialized,
			battle_handler,
		})
	}

	pub fn start_battle(&'a mut self, p1: Rc<RefCell<Battler>>, p2: Rc<RefCell<Battler>>){
		self.battle_handler = Rc::new(RefCell::new(BattleStatus::new(Rc::clone(&p1),Rc::clone(&p2))));

	}


	// Step should be called via the GameManager

	// Because the program is single threaded, we can't use extra loops to wait on conditions
	//      Instead, we should use the main game loop and check specific conditions at specific times. I've broken a turn/round into phases to do this
	pub fn step(&'_ mut self) -> Result<(), String> {

        //let mut battle_stat = self.battle_handler.borrow_mut();

	    // initialize things at the start of battle
	    if self.turn == TurnPhase::NotInitialized {


	        // player structs and decks will be initialized here

            println!("Start of Battle...");
            self.active_player = 1;

            // initialize (or reinitialize) the player and opponent Battler structs within battle_handler
            let _p1 = Rc::new(RefCell::new(self.battler_map.get(&0).unwrap().clone())); //Must UNWRAP AND CLONE players from map for battle use
            let _p2 = Rc::new(RefCell::new(self.battler_map.get(&1).unwrap().clone()));

		    self.battle_handler = Rc::new(RefCell::new(BattleStatus::new(Rc::clone(&_p1),Rc::clone(&_p2))));

            // free up the borrow_mut slot by using a local variable
            let mut battle_stat = self.battle_handler.borrow_mut();

            println!("The player has {} cards in the deck", battle_stat.get_p1().borrow_mut().get_deck_size());
            println!("The opponent has {} cards in the deck\n", battle_stat.get_p2().borrow_mut().get_deck_size());
            
            // draw 3 cards for both players to start the battle (they will draw a 4th on their turn)
            for i in 0..3{
                battle_stat.get_p1().borrow_mut().draw_card();  // p1 is player
                battle_stat.get_p2().borrow_mut().draw_card();  // p2 is opponent
            }

            println!("The player has {} cards in the deck", battle_stat.get_p1().borrow_mut().get_deck_size());
            println!("The opponent has {} cards in the deck\n", battle_stat.get_p2().borrow_mut().get_deck_size());

            println!("The player has {} cards in their hand", battle_stat.get_p1().borrow_mut().get_curr_hand_size());
            println!("The opponent has {} cards in the hand\n", battle_stat.get_p2().borrow_mut().get_curr_hand_size());

            println!("{}", battle_stat.get_p1().borrow_mut().to_string());
            println!("{}", battle_stat.get_p2().borrow_mut().to_string());

	        self.turn = TurnPhase::PreTurnP1;

	    }

	    if self.active_player == 1 {

            let mut battle_stat = self.battle_handler.borrow_mut();

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
	            
	            // draw a card at the start of the turn
                battle_stat.get_p1().borrow_mut().draw_card();  // p1 is player
                
                //battle_stat = self.battle_handler.borrow_mut();

	            // Move to the next phase of the turn
	            println!("End of PreTurnP1");
	            self.turn = TurnPhase::TurnP1;
	        }
	        else if self.turn == TurnPhase::PostTurnP1 {
	            // Resolve things that need to be resolved after the Player's turn in here
	            // Intended to check for Statuses that need to be removed at the end of the turn

                println!("End of PostTurnP1");
	            self.active_player = -1;
	            self.turn = TurnPhase::PreTurnP2;
	        }


	    }

	    // Enemy logic in the else
	    else{
	    
	        let mut battle_stat = self.battle_handler.borrow_mut();
	    
	        if self.turn == TurnPhase::TurnP2 {

	            // Enemy AI should be called from here


	            self.turn = TurnPhase::PostTurnP2;

	        }
	        else if self.turn == TurnPhase::PreTurnP2 {
	            // Resolve things that need to be resolved prior to the Opponent's turn in here
	            // Intended to check for Statuses that need to be removed at the beginning of the turn

	            // Can add drawing a card in here and checking handsize/remaining cards

                // draw a card at the start of the turn
                battle_stat.get_p2().borrow_mut().draw_card();  // p2 is opponent

	            // Move to the next phase of the turn
	            println!("End of PreTurnP2");
	            self.turn = TurnPhase::TurnP2;
	        }
	        else if self.turn == TurnPhase::PostTurnP2 {
	            // Resolve things that need to be resolved after the Opponent's turn in here
	            // Intended to check for Statuses that need to be removed at the end of the turn

                println!("End of PostTurnP2");
                self.turn = TurnPhase::RoundOver;

	        }
	    }

        if self.turn == TurnPhase::RoundOver {

            println!("Round is now fully over (both players had a turn)\n-");
            self.turn = TurnPhase:: PreTurnP1;
            self.active_player = 1;
        }

        //  Because I'm calling step() through the render method, step() fires AFTER input is handled
        //  This means that if self.turn was reset to NotInitialized, via handle_input() the final call to step() would
        //  cause it to run the setup as if it was being called for the first time for the battle.
        //  Now, handle_input() sets self.turn to BattleOver, and the final call to step() will run the code below.

        //  However, overloading the event pump with other inputs (like moving the mouse) will cause the game to lag,
        //  and render() will run a few more times before the scene change in handle_input() can resolve itself, making
        //  this useless.

        //  This isn't urgent to fix since we're only doing one battle for the midterm, but this may become
        //  more of an issue in the future.
        else if self.turn == TurnPhase::BattleOver {
            println!("Moving away from the battle scene");
            self.turn = TurnPhase::NotInitialized;
        }

	    return Ok(());

	}



}

impl Scene for Battle<'_> {

	fn handle_input(&mut self, event: GameEvent) {

		// Some input should be restricted if it isn't the player's turn

		    match event {
			    GameEvent::KeyPress(k) => {
				    //println!("{}", k);
				    if k.eq(&Keycode::Escape) {
				        self.turn = TurnPhase::BattleOver;  // Changing to BattleOver instead of NotInitialized
				        self.event_system.borrow().change_scene(1).unwrap();}
			        },
			    GameEvent::MouseClick(x_pos,y_pos) => {
			        if (x_pos > 1110 && x_pos < 1270) && (y_pos > 470 && y_pos < 530 && self.turn == TurnPhase::TurnP1) {
					    println!("End Turn button was pressed");
					    self.turn = TurnPhase::PostTurnP1;

				    }
				    else{
				        // check if the player is clicking on any of the cards in their hand
				        let mut battle_stat = self.battle_handler.borrow_mut();
				        let mut p1_hand_size = battle_stat.get_p1().borrow_mut().get_curr_hand_size();
				        for i in 0..p1_hand_size{
				            if ((x_pos > (260 + (i * 120) as i32) && x_pos < (360 + (i * 120) as i32)) && (y_pos > 560 && y_pos < 708)){
				                println!("game thinks that the player is clicking on card {}", i);

				            }    
				        }
				        
				    }
			    }

			    _ => {},
		    }

	}

	fn render(&mut self) -> Result<(), String> {

        // Calling step() in here since it isn't possible through game_manager.rs without changing the Scene struct
        // and implementing step() for all the other scenes (might end up doing this anyway)
        self.step();

		let mut wincan = self.wincan.borrow_mut();
		crate::video::gfx::fill_screen(&mut wincan, Color::RGB(154, 195, 225));

		//hardcoded for now too test to make sure the cards and other items appear in the correct places

		//backdrop for cards
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.drop,(1280,300), (0,550))?; //wood for the back
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.drop,(1280,180), (0,0))?; //wood for the back

		//player
		//crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.test_1,(100,148), (980,560))?;
		//crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.test_2,(100,148), (860,560))?;
		//crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.test_3,(100,148), (740,560))?;
		//crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.test_2,(100,148), (620,560))?;
		//crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.test_1,(100,148), (500,560))?;
		//crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.test_3,(100,148), (380,560))?;
		//crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.test_1,(100,148), (260,560))?;

		let mut battle_stat = self.battle_handler.borrow_mut();
		let mut p1_hand_size = battle_stat.get_p1().borrow_mut().get_curr_hand_size();
		for i in 0..p1_hand_size {
		    crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.test_1,(100,148), ((260 + (i * 120)) as i32,560))?;
		}

		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (1140,560))?;
		//enemy side
		//crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (920,20))?;
		//crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (800,20))?;
		//crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (680,20))?;
		//crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (560,20))?;
		//crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (440,20))?;
		//crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (320,20))?;
		//crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (200,20))?;

        let mut p2_hand_size = battle_stat.get_p2().borrow_mut().get_curr_hand_size();
		for i in 0..p2_hand_size {
		    crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), ((920 - (i * 120)) as i32,20))?;
		}

		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (40,20))?;

		//mostly static objects (health bars change tho)

		// Can now update the health bars to dynamically update based on the Battler's health
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.health,(300,20), (790,520))?; //player health bar
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.health,(300,20), (200,190))?; //enemy health bar

		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.play_i,(150,150), (60,560))?; //player icon
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.play_i,(150,150), (1070,20))?; //enemy icon
		//crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.play_i,(150,150), (1070,20))?; //enemy icon
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.tmp_button,(300,100), (0,300))?;

		// End Turn button "sprite"
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.drop, (160, 60), (1110, 470))?;
		// End Turn button text
		let mut fontm = self.font_manager.borrow_mut();
		fontm.draw_text(&mut wincan, "End Turn", (1120, 480));

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
