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
use crate::cards::battle_enums::BattleOutcome;

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
	card_textures: Vec<Rc<Texture<'a>>>,
	play1_i: Rc<Texture<'a>>,
	play2_i: Rc<Texture<'a>>,
	health: Rc<Texture<'a>>,
	behind_health:Rc<Texture<'a>>,
	mana: Rc<Texture<'a>>,
	behind_mana:Rc<Texture<'a>>,
	deck: Rc<Texture<'a>>,
	drop: Rc<Texture<'a>>,
	e_pip_unfilled: Rc<Texture<'a>>,
	e_pip_filled: Rc<Texture<'a>>,
	accepting_input: bool,

	tmp_enemy_played_card: usize,

	// BATTLE DATA
	battler_map: HashMap<u32, Battler>,
	active_player: i8,
	turn: TurnPhase,
	outcome: BattleOutcome,
	battle_handler: Rc<RefCell<BattleStatus>>,

}

impl<'a> Battle<'a> {
	pub fn init(texture_manager: Rc<RefCell<TextureManager<'a>>>, wincan: Rc<RefCell<WindowCanvas>>, event_system: Rc<RefCell<EventSystem>>, font_manager: Rc<RefCell<FontManager<'a>>>)  -> Result<Self, String> {
		let play1_i = texture_manager.borrow_mut().load("assets/temp_player_icons/icondummy.png")?;
		let play2_i = texture_manager.borrow_mut().load("assets/temp_player_icons/icondummyenemy.png")?;
		let health = texture_manager.borrow_mut().load("assets/temp_health.png")?;
		let behind_health = texture_manager.borrow_mut().load("assets/behind_health.png")?;
		let mana = texture_manager.borrow_mut().load("assets/temp_energy.png")?;
		let behind_mana = texture_manager.borrow_mut().load("assets/behind_health.png")?;
		let deck = texture_manager.borrow_mut().load("assets/cards/Card Back.png")?;
		let drop = texture_manager.borrow_mut().load("assets/wood_texture.png")?;
		let e_pip_unfilled = texture_manager.borrow_mut().load("assets/energyPipEmpty.png")?;
		let e_pip_filled = texture_manager.borrow_mut().load("assets/energyPipFilled.png")?;
		let accepting_input = true;
		let tmp_enemy_played_card = 100;
		let dummy = Rc::new(RefCell::new(Battler::new(("").to_string(),0,0,0,0)));  //REQUIRED TO AVOID USE
																		//of Option<T>. DO NOT REMOVE
		let battler_map = crate::cards::battle_system::populate_battler_map();


		let _p1 = Rc::new(RefCell::new(battler_map.get(&0).unwrap().clone())); //Must UNWRAP AND CLONE players from map for battle use
        let _p2 = Rc::new(RefCell::new(battler_map.get(&1).unwrap().clone()));





		let mut battle_handler = Rc::new(RefCell::new(BattleStatus::new(Rc::clone(&_p1),Rc::clone(&_p2))));

		let num_cards = battle_handler.borrow_mut().get_card_map_size();
		let mut card_textures: Vec<Rc<Texture>> = Vec::new();
		for i in 0..num_cards{
			let tmp_card = battle_handler.borrow_mut().get_card(i as u32);
			let path = tmp_card.get_sprite_name();
			let texture = texture_manager.borrow_mut().load(path)?;
			card_textures.push(texture);
		}

		Ok(Battle {
			wincan,
			event_system,
			font_manager,
			card_textures,
			play1_i,
			play2_i,
			health,
			behind_health,
			mana,
			behind_mana,
			deck,
			drop,
			tmp_enemy_played_card,
			e_pip_unfilled,
			e_pip_filled,
			accepting_input,
			battler_map,
			active_player: 1,
			turn: TurnPhase::NotInitialized,
			outcome: BattleOutcome::Undetermined,
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

			_p1.borrow_mut().shuffle_deck();
			_p2.borrow_mut().shuffle_deck();

		    self.battle_handler = Rc::new(RefCell::new(BattleStatus::new(Rc::clone(&_p1),Rc::clone(&_p2))));

            // free up the borrow_mut slot by using a local variable
            let mut battle_stat = self.battle_handler.borrow_mut();

            let mut _player1 = battle_stat.get_p1();
            let mut player1 = _player1.borrow_mut();
            player1.shuffle_deck();

            let mut _player2 = battle_stat.get_p2();
            let mut player2 = _player2.borrow_mut();
            player2.shuffle_deck();

            println!("The player has {} cards in the deck", player1.get_deck_size());
            println!("The opponent has {} cards in the deck\n", player2.get_deck_size());

            // draw 3 cards for both players to start the battle (they will draw a 4th on their turn)
            for i in 0..4{
                player1.draw_card();  // p1 is player
                player2.draw_card();  // p2 is opponent
            }

            println!("The player has {} cards in the deck", player1.get_deck_size());
            println!("The opponent has {} cards in the deck\n", player2.get_deck_size());

            println!("The player has {} cards in their hand", player1.get_curr_hand_size());
            println!("The opponent has {} cards in the hand\n", player2.get_curr_hand_size());

            println!("{}", player1.to_string());
            println!("{}", player2.to_string());

	        self.turn = TurnPhase::PreTurnP1;
	        self.outcome = BattleOutcome::Undetermined;

	    }

        if self.outcome == BattleOutcome::Undetermined {

	        if self.active_player == 1 {

                let mut battle_stat = self.battle_handler.borrow_mut();
                self.outcome = battle_stat.check_victory();

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

				    let mut _p =battle_stat.get_active_player();
				    let mut player = _p.borrow_mut();
				    print!("{}\n",player.to_string());

				    if(player.get_deck_size()==0&&player.get_curr_hand_size()==0){
					    player.restore_deck();
					    println!("Skipping p1 turn!");
					    self.turn = TurnPhase::PostTurnP1;
				    }else{
					    player.draw_card();  // p1 is player

		                // Move to the next phase of the turn
		                println!("End of PreTurnP1");
		                self.turn = TurnPhase::TurnP1;
				    }

	            }
	            else if self.turn == TurnPhase::PostTurnP1 {
	                // Resolve things that need to be resolved after the Player's turn in here
	                // Intended to check for Statuses that need to be removed at the end of the turn

                    //println!("End of PostTurnP1");

				    let mut _p =battle_stat.get_active_player();
				    let mut player = _p.borrow_mut();
				    player.update_effects();

				    battle_stat.turner();
	                self.active_player = -1;
	                self.turn = TurnPhase::PreTurnP2;

	                println!("End of PostTurnP1");

	            }

	        }

	        // Enemy logic in the else
	        else{

	            self.outcome = self.battle_handler.borrow_mut().check_victory();

	            if self.turn == TurnPhase::TurnP2 {

	                // Enemy AI should be called from here
					let card_rslt = self.battle_handler.borrow_mut().get_p2().borrow().select_hand(0);
					//let card_cost = card_rslt.unwrap().get_cost();
					if (!card_rslt.is_none()){
						let card_ID = card_rslt.unwrap();//self.battle_handler.borrow_mut().get_p1().borrow().select_hand(i).unwrap();
						let curr_card = self.battle_handler.borrow_mut().get_card(card_ID);
						let curr_card_cost = curr_card.get_cost() as i32;
						println!("card cost is {}", curr_card_cost);
						let curr_energy = self.battle_handler.borrow_mut().get_p2().borrow().get_curr_energy();
						println!("current energy is {}", curr_energy);
						// only play if player has enough energy
						if (curr_energy >= curr_card_cost){

							//println!("Trying to play card with ID {}\n{}", card_ID, curr_card.to_string());

							// if the player has enough energy to cover the cost of playing the card:
							crate::cards::battle_system::play_card(Rc::clone(&self.battle_handler), curr_card);
							// add card to discard pile after playing
							self.tmp_enemy_played_card = card_ID as usize;
							self.battle_handler.borrow_mut().get_p2().borrow_mut().hand_discard_card(0);
							self.battle_handler.borrow_mut().get_p2().borrow_mut().adjust_curr_energy(-(curr_card_cost as i32));

						}
						// otherwise, don't
						else {
							println!("Not enough energy!");
						}


						//println!("{}", self.battle_handler.borrow_mut().get_p1().borrow_mut().to_string());
						//println!("{}", self.battle_handler.borrow_mut().get_p2().borrow_mut().to_string());
					}

	                self.turn = TurnPhase::PostTurnP2;

	            }
	            else if self.turn == TurnPhase::PreTurnP2 {
	                // Resolve things that need to be resolved prior to the Opponent's turn in here
	                // Intended to check for Statuses that need to be removed at the beginning of the turn

	                // Can add drawing a card in here and checking handsize/remaining cards

                    // draw a card at the start of the turn

				    let mut _p =self.battle_handler.borrow_mut().get_active_player();
				    let mut player = _p.borrow_mut();
				    if(player.get_deck_size()==0&&player.get_curr_hand_size()==0){
					    player.restore_deck();
					    println!("Skipping p2 turn!");
					    self.turn = TurnPhase::PostTurnP2;
				    }else{
					    player.draw_card();  // p2 is player



	                    // give the opponent 3 energy per turn
                        player.adjust_curr_energy(3);  // p2 is opponent

		                // Move to the next phase of the turn
		                println!("End of PreTurnP2");
		                self.turn = TurnPhase::TurnP2;
				    }

	            }
	            else if self.turn == TurnPhase::PostTurnP2 {
	                // Resolve things that need to be resolved after the Opponent's turn in here
	                // Intended to check for Statuses that need to be removed at the end of the turn

					let mut _p =self.battle_handler.borrow_mut().get_active_player();
					let mut player = _p.borrow_mut();
					player.update_effects();

					println!("End of PostTurnP2");
					self.turn = TurnPhase::RoundOver;
					self.battle_handler.borrow_mut().turner();

	            }
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
				        //let mut battle_stat = self.battle_handler.borrow_mut();

				        let mut p1_hand_size = self.battle_handler.borrow_mut().get_p1().borrow().get_curr_hand_size();//battle_stat.get_p1().borrow().get_curr_hand_size();
				        //let curr_turn = self.battle_handler.borrow_mut().get_turn();

						if (self.battle_handler.borrow_mut().get_turn()==0&&(x_pos > (260 as i32) && x_pos < (360 + (p1_hand_size * 120) as i32)) && (y_pos > 560 && y_pos < 708)){
							let i = ((x_pos-260)/120) as usize;
							//println!("{}", self.battle_handler.borrow_mut().get_p1().borrow_mut().to_string());
							//println!("{}", self.battle_handler.borrow_mut().get_p2().borrow_mut().to_string());

							//println!("game thinks that the player is clicking on card {}", i);

							// play the card
							let card_rslt = self.battle_handler.borrow_mut().get_p1().borrow().select_hand(i);
							//let card_cost = card_rslt.unwrap().get_cost();
							if (!card_rslt.is_none()){
								let card_ID = card_rslt.unwrap();//battle_stat.get_p1().borrow().select_hand(i).unwrap();
								let curr_card = self.battle_handler.borrow_mut().get_card(card_ID);
								let curr_card_cost = curr_card.get_cost() as i32;
								println!("card cost is {}", curr_card_cost);
								let curr_energy = self.battle_handler.borrow_mut().get_p1().borrow().get_curr_energy();
								println!("current energy is {}", curr_energy);
								// only play if player has enough energy
								if (curr_energy >= curr_card_cost){

								    //println!("Trying to play card with ID {}\n{}", card_ID, curr_card.to_string());

								    // if the player has enough energy to cover the cost of playing the card:
								    crate::cards::battle_system::play_card(Rc::clone(&self.battle_handler), curr_card);
								    // add card to discard pile after playing
								    self.battle_handler.borrow_mut().get_p1().borrow_mut().hand_discard_card(i);
								    self.battle_handler.borrow_mut().get_p1().borrow_mut().adjust_curr_energy(-(curr_card_cost as i32));

								}
								// otherwise, don't
			                    else {
			                        println!("Not enough energy!");
			                    }


								//println!("{}", self.battle_handler.borrow_mut().get_p1().borrow_mut().to_string());
								//println!("{}", self.battle_handler.borrow_mut().get_p2().borrow_mut().to_string());
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


		let mut battle_stat = self.battle_handler.borrow_mut();
		let mut _p1 = battle_stat.get_p1();
		let mut player1 = _p1.borrow_mut();
		let mut p1_hand_size = player1.get_curr_hand_size();
		for i in 0..p1_hand_size {
			let curr_hand = player1.select_hand(i as usize).unwrap();
			crate::video::gfx::draw_sprite_to_dims(&mut wincan, &(self.card_textures.get(curr_hand as usize).unwrap()),(100,148), ((260 + (i * 120)) as i32,560))?;
		}

		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (1140,560))?;

		// draw the player's energy pips
		let p1_curr_energy = player1.get_curr_energy();
		for i in 0..10 {
		    if i < p1_curr_energy {
		        crate::video::gfx::draw_sprite(&mut wincan, &self.e_pip_filled, (20 + (i * 20), 530));
		    }
		    else {
		        crate::video::gfx::draw_sprite(&mut wincan, &self.e_pip_unfilled, (20 + (i * 20), 530));
		    }
		}

		//enemy side

		let mut _p2 = battle_stat.get_p2();
		let mut player2 = _p2.borrow_mut();
		let mut p2_hand_size = player2.get_curr_hand_size();
		if player1.get_deck_size()!=0{
			crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (1140,560))?;
		}

		for i in 0..p2_hand_size {
		    crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), ((920 - (i * 120)) as i32,20))?;
		}

		if player2.get_deck_size()!=0{
			crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (40,20))?;
		}

		let mut fontm = self.font_manager.borrow_mut();
		fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 18, Color::RGB(150, 0, 0),
			"Enemy Played:", (600,300-25));
		if self.tmp_enemy_played_card<100{
			crate::video::gfx::draw_sprite_to_dims(&mut wincan, &(self.card_textures.get(self.tmp_enemy_played_card)).unwrap(),(100,148),(600,300))?;
		}else{
			crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148),(600,300))?;
		}

        // draw the enemy's energy pips
		let p2_curr_energy = player2.get_curr_energy();

		for i in 0..10 {
		    if i < p2_curr_energy {
		        crate::video::gfx::draw_sprite(&mut wincan, &self.e_pip_filled, (1240 - (i * 20), 184));
		    }
		    else {
		        crate::video::gfx::draw_sprite(&mut wincan, &self.e_pip_unfilled, (1240 - (i * 20), 184));
		    }
		}

		//mostly static objects (health bars change tho)

		// Can now update the health bars to dynamically update based on the Battler's health
		let p1perc = 300 as f32 * player1.get_health_percent();
		let p2perc = 300 as f32 * player2.get_health_percent();
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.behind_health,(300,20), (790,520))?;
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.behind_health,(300,20), (200,190))?;
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.health,(p1perc as u32,20), (790+(300-p1perc as i32),520))?; //player health bar
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.health,(p2perc as u32,20), (200,190))?; //enemy health bar

		//add health text
		fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 18, Color::RGB(0, 150, 0),
			&player2.get_curr_health().to_string(), (200,190+25));
		fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 18, Color::RGB(0, 150, 0),
			&player1.get_curr_health().to_string(), (790,520-25));

		//add mana text
		fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 15, Color::RGB(95, 95, 0),
			&player2.get_curr_energy().to_string(), (1060,210));
		fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 15, Color::RGB(95, 95, 0),
			&player1.get_curr_energy().to_string(), (20,505));


		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.play1_i,(150,150), (60,560))?; //player icon
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.play2_i,(150,150), (1070,20))?; //enemy icon

		// End Turn button "sprite"
		crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.drop, (160, 60), (1110, 470))?;
		// End Turn button text
		//let mut fontm = self.font_manager.borrow_mut();
		fontm.draw_text(&mut wincan, "End Turn", (1120, 480));

		match self.outcome {
		    BattleOutcome::VictoryP1 => fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 64, Color::RGB(0, 0, 0), "VICTORY!", (600, 330)),
		    BattleOutcome::VictoryP2 => fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 64, Color::RGB(0, 0, 0), "DEFEAT", (600, 330)),
		    BattleOutcome::Tie => fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 64, Color::RGB(0, 0, 0), "DRAW...", (600, 330)),
		    _ => Ok(()),
		};

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
