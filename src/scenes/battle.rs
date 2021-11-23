use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::time::Instant;
//use std::thread::sleep; // sleep should only be used for testing, sleep will lock the entire program until sleeping is done

use sdl2::pixels::Color;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::keyboard::Keycode;

use crate::scenes::Scene;
use crate::scenes::GameEvent;
use crate::events::event_subsystem::EventSystem;
use crate::game_manager::TextureManager;
use crate::video::text::FontManager;

use crate::ai::ai_structs::*;
use crate::ai::minimax::*;

use crate::cards::game_structs::*;

use crate::cards::battle_enums::*;

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
	armor: Rc<Texture<'a>>,
	healing: Rc<Texture<'a>>,
	posion: Rc<Texture<'a>>,
	mana_boost: Rc<Texture<'a>>,
	mana_drain: Rc<Texture<'a>>,
	multi: Rc<Texture<'a>>,
	accepting_input: bool,
	not_enough_mana: bool,

	tmp_enemy_played_card: usize,

	dummy_drawn_card: DrawnCard,
	frames_elapsed: u32,


	// BATTLE DATA
	battler_map: HashMap<u32, Battler>,
	active_player: i8,
	turn: TurnPhase,
	outcome: BattleOutcome,
	battle_handler: Rc<RefCell<BattleStatus>>,
	enemy_delay_inst: Instant,
	battler_npc_deck_id: u32,

	//enlarge
	enlarged_card: card_size,
	enemy_card: e_card_size,
	playCard: Rc<Texture<'a>>,
	retCard: Rc<Texture<'a>>,
	backDrop: Rc<Texture<'a>>,
}

impl<'a> Battle<'a> {
	pub fn init(texture_manager: Rc<RefCell<TextureManager<'a>>>, wincan: Rc<RefCell<WindowCanvas>>, event_system: Rc<RefCell<EventSystem>>, font_manager: Rc<RefCell<FontManager<'a>>>)  -> Result<Self, String> {
		let play1_i = texture_manager.borrow_mut().load("assets/temp_player_icons/icondummy.png")?;
		let play2_i = texture_manager.borrow_mut().load("assets/temp_player_icons/icondummyenemy.png")?;
		let health = texture_manager.borrow_mut().load("assets/temp_health.png")?;
		let behind_health = texture_manager.borrow_mut().load("assets/behind_health.png")?;
		let mana = texture_manager.borrow_mut().load("assets/temp_energy.png")?;
		let behind_mana = texture_manager.borrow_mut().load("assets/behind_health.png")?;   // can be removed?
		let deck = texture_manager.borrow_mut().load("assets/cards/Card Back.png")?;
		let drop = texture_manager.borrow_mut().load("assets/wood_texture.png")?;
		let e_pip_unfilled = texture_manager.borrow_mut().load("assets/energyPipEmpty.png")?;
		let e_pip_filled = texture_manager.borrow_mut().load("assets/energyPipFilled.png")?;
		let armor = texture_manager.borrow_mut().load("assets/effects/shield.png")?;
		let healing = texture_manager.borrow_mut().load("assets/effects/healing.png")?;
		let posion = texture_manager.borrow_mut().load("assets/effects/posion.png")?;
		let mana_boost = texture_manager.borrow_mut().load("assets/effects/mana_boost.png")?;
		let mana_drain = texture_manager.borrow_mut().load("assets/effects/mana_drain.png")?;
		let multi = texture_manager.borrow_mut().load("assets/effects/mult.png")?;
		let accepting_input = true;
		let tmp_enemy_played_card = 100;
		let dummy_drawn_card = DrawnCard::new(0.0, 800.0).unwrap();
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

		let enlarged_card = card_size{
		    	card_pos: 0,
			x_size: 400,
			y_size: 592,
			x_pos: 450,
			y_pos: 50,
			larger: false,
		    };

		let enemy_card = e_card_size{
			ex_size: 400,
			ey_size: 592,
			ex_pos: 450,
			ey_pos: 50,
			elarger: false,
		    };

		let playCard = texture_manager.borrow_mut().load("assets/play_card.png")?;
		let retCard = texture_manager.borrow_mut().load("assets/return.png")?;
		let backDrop = texture_manager.borrow_mut().load("assets/backdrop.png")?;

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
			dummy_drawn_card,
			frames_elapsed: 0,
			e_pip_unfilled,
			e_pip_filled,
			armor,
			healing,
			posion,
			mana_boost,
			mana_drain,
			multi,
			accepting_input,
			not_enough_mana: false,
			battler_map,
			active_player: 1,
			turn: TurnPhase::NotInitialized,
			outcome: BattleOutcome::Undetermined,
			battle_handler,
			enemy_delay_inst: Instant::now(),
			battler_npc_deck_id: 1,
			enlarged_card,
			enemy_card,
			playCard,
			retCard,
			backDrop,
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
            // change the number in self.battler_map.get(&X) to change battler ID
            //      Now done through the set_battler_npc_deck event
            let _p2 = Rc::new(RefCell::new(self.battler_map.get(&self.battler_npc_deck_id).unwrap().clone()));
            println!("set opponent's deck to the deck with deck_id: {}", self.battler_npc_deck_id);

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
                player1.draw_card(false);  // p1 is player
                player2.draw_card(false);  // p2 is opponent
            }

            println!("The player has {} cards in the deck", player1.get_deck_size());
            println!("The opponent has {} cards in the deck\n", player2.get_deck_size());

            println!("The player has {} cards in their hand", player1.get_curr_hand_size());
            println!("The opponent has {} cards in the hand\n", player2.get_curr_hand_size());

            println!("{}", player1.to_string());
            println!("{}", player2.to_string());

	        self.turn = TurnPhase::PreTurnP1;
	        self.outcome = BattleOutcome::Undetermined;

	        self.tmp_enemy_played_card = 100;   // Any number greater than 99 displays the deck card


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

		            if player.get_deck_size()==0&&player.get_curr_hand_size()==0{
			            player.restore_deck();
			            println!("Skipping p1 turn!");
			            self.turn = TurnPhase::PostTurnP1;
		            }else{

			            //player.draw_card(false);  // p1 is player

			            // delaying card draw until after animation finishes
                        if player.get_deck_size() > 0  && player.get_curr_hand_size() < 7 {
                            player.add_draw_num(1);
                            self.dummy_drawn_card.x_pos = 1140.0;
                            self.dummy_drawn_card.y_pos = 560.0;
                            self.frames_elapsed = 0;
                            println!("set self.frames_elapsed to 0");
                        }

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

                // self.enemy_delay_inst is updated in the PreTurnP2 phase. After 1 second, the code below runs
	            if self.turn == TurnPhase::TurnP2 && self.enemy_delay_inst.elapsed().as_secs() >= 1 {

	                // Enemy AI should be called from here
					let mut gametree = GameTree::new(self.battle_handler.borrow().clone());
					gametree.populate(3);
					gametree.print();
					let card_rslt = self.battle_handler.borrow_mut().get_p2().borrow().select_hand(0);
					//let card_cost = card_rslt.unwrap().get_cost();
					if !card_rslt.is_none(){
						let card_ID = card_rslt.unwrap();//self.battle_handler.borrow_mut().get_p1().borrow().select_hand(i).unwrap();
						let curr_card = self.battle_handler.borrow_mut().get_card(card_ID);
						print!("{}\n",curr_card.to_string());
						let curr_card_cost = curr_card.get_cost() as i32;
						println!("card cost is {}", curr_card_cost);
						let curr_energy = self.battle_handler.borrow_mut().get_p2().borrow().get_curr_energy();
						println!("current energy is {}", curr_energy);
						// only play if player has enough energy
						if curr_energy >= curr_card_cost{

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

                    // delay the turn phase by 1 second
                    println!("waiting another second...");
	                //self.enemy_delay_inst = Instant::now();

	                // eventually, when the enemy learns how to play multiple cards per turn, this will have to wait until all cards are played
	                self.turn = TurnPhase::PostTurnP2;


	            }
	            else if self.turn == TurnPhase::PreTurnP2 {
	                // Resolve things that need to be resolved prior to the Opponent's turn in here
	                // Intended to check for Statuses that need to be removed at the beginning of the turn

	                // Can add drawing a card in here and checking handsize/remaining cards

                    // draw a card at the start of the turn

					let mut _p =self.battle_handler.borrow_mut().get_active_player();
				    let mut player = _p.borrow_mut();
				    if player.get_deck_size()==0&&player.get_curr_hand_size()==0{
					    player.restore_deck();
					    println!("Skipping p2 turn!");
					    self.turn = TurnPhase::PostTurnP2;
				    }else{
					    //player.draw_card(false);  // p2 is player

                        if player.get_deck_size() > 0  && player.get_curr_hand_size() < 7 {
                            player.add_draw_num(1);
                            self.dummy_drawn_card.x_pos = 40.0;
                            self.dummy_drawn_card.y_pos = 20.0;
                            self.frames_elapsed = 0;
                            println!("set self.frames_elapsed to 0");
                        }

	                    // give the opponent 3 energy per turn
                        //player.adjust_curr_energy(3);  // p2 is opponent

		                // Move to the next phase of the turn
		                println!("End of PreTurnP2");
		                self.turn = TurnPhase::TurnP2;

		                // delay the turn phase by 1 second
		                println!("waiting a second...");

                        // Using an instant allows us to still let the player do things like hover over cards while it isn't their turn
                        // locking up the program completely via sleep() wouldn't let us do this

                        // Update the delay instant so we can reuse one over and over again
		                self.enemy_delay_inst = Instant::now();

		                // replace the last played card so it's clearer when the enemy plays duplicates
		                self.tmp_enemy_played_card = 100;

				    }

	            }
	            // self.enemy_delay_inst is updated again in the TurnP2 phase. After 1 second, the code below runs
	            else if self.turn == TurnPhase::PostTurnP2  && self.enemy_delay_inst.elapsed().as_secs() >= 1 {
	                // Resolve things that need to be resolved after the Opponent's turn in here
	                // Intended to check for Statuses that need to be removed at the end of the turn

					let mut _p =self.battle_handler.borrow_mut().get_active_player();
					let mut player = _p.borrow_mut();
					player.update_effects();

                    // delay the turn phase by 1 second
		            println!("waiting another second...");
		            //sleep(Duration::new(1, 0));

					println!("End of PostTurnP2");
					self.turn = TurnPhase::RoundOver;
					self.battle_handler.borrow_mut().turner();

	            }
	        }

            // if the outcome of the battle has changed, show prepare to show the result on screen
            if self.outcome != BattleOutcome::Undetermined {
                println!("Updating the enemy_delay_inst to show battle result on screen");

                // reusing enemy_delay to show battle result for a few seconds before moving back to overworld
                self.enemy_delay_inst = Instant::now();
            }

	    }
	    // Else if battle is over (self.outcome != BattleOutcome::Undetermined)
	    // Show the result for 5 seconds, then go back to the overworld
	    else {
	        if self.enemy_delay_inst.elapsed().as_secs() >= 5 {
	            println!("Moving away from the battle scene");
                self.turn = TurnPhase::NotInitialized;
                self.event_system.borrow().change_scene(1).unwrap();
                return Ok(());
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

        // if the outcome of the battle has changed, show prepare to show the result on screen
        /*if self.outcome != BattleOutcome::Undetermined {
            println!("Updating the enemy_delay_inst to show battle result on screen");

            // reusing enemy_delay to show battle result for a few seconds before moving back to overworld
            self.enemy_delay_inst = Instant::now();
        }*/

	    return Ok(());

	}

	fn dup_screen(&mut self,mut curr_card: Card,curr_card_cost:i32,hand:usize){
		let sz = self.battle_handler.borrow_mut().get_p1().borrow_mut().get_curr_hand_size();
		if sz<2||hand>sz-2{
			return;
		}
		self.battle_handler.borrow_mut().get_p1().borrow_mut().hand_discard_card(self.enlarged_card.get_cardpos() );
		self.battle_handler.borrow_mut().get_p1().borrow_mut().adjust_curr_energy(-(curr_card_cost as i32));

		let to_dupe = self.battle_handler.borrow_mut().get_p1().borrow().select_hand(hand);

		curr_card.set_values(19,to_dupe.unwrap() as i32);
		crate::cards::battle_system::play_card(Rc::clone(&self.battle_handler), curr_card);
		self.enlarged_card.set_larger(false);
		self.battle_handler.borrow_mut().get_p1().borrow_mut().shuffle_deck();
	}

}

impl Scene for Battle<'_> {

	fn handle_input(&mut self, event: GameEvent) {

		// Some input should be restricted if it isn't the player's turn

		    match event {
			    GameEvent::SetBattlerNPCDeck(deck_id) => {
			        self.battler_npc_deck_id = deck_id;
			        //println!("IN BATTLE: self.battler_npc_deck_id is {}, should be {}", self.battler_npc_deck_id, deck_id);
			    },
			    GameEvent::KeyPress(k) => {
				    //println!("{}", k);
				    if k.eq(&Keycode::Escape) {
				        self.turn = TurnPhase::BattleOver;  // Changing to BattleOver instead of NotInitialized
				        self.event_system.borrow().change_scene(1).unwrap();
				        self.enlarged_card.set_larger(false);
				        }
			        },
			    GameEvent::MouseClick(x_pos,y_pos) => {
			    	println!("{},{}", x_pos,y_pos);
					self.not_enough_mana = false;
			        if (self.enlarged_card.get_larger() == false && self.enemy_card.get_elarger() == false && x_pos > 1110 && x_pos < 1270) && (y_pos > 470 && y_pos < 530 && self.turn == TurnPhase::TurnP1) {
					    println!("End Turn button was pressed");
					    self.turn = TurnPhase::PostTurnP1;
					}
					else if self.enlarged_card.get_larger() == true{

						// play the card
						let sel_card = self.enlarged_card.get_cardpos();
						let card_rslt = self.battle_handler.borrow_mut().get_p1().borrow().select_hand(sel_card);
						//let card_cost = card_rslt.unwrap().get_cost();
						let card_ID = card_rslt.unwrap();//battle_stat.get_p1().borrow().select_hand(i).unwrap();
						let mut curr_card = self.battle_handler.borrow_mut().get_card(card_ID);
						let curr_card_cost = curr_card.get_cost() as i32;
						//println!("card cost is {}", curr_card_cost);
						let curr_energy = self.battle_handler.borrow_mut().get_p1().borrow().get_curr_energy();
						if (!card_rslt.is_none()){

						if !curr_card.get_actions().contains(&19){
							if(((x_pos > 900 && x_pos < 1100) && (y_pos > 400 && y_pos < 460) && self.turn == TurnPhase::TurnP1)){
								self.enlarged_card.set_larger(false);
							}

							if((x_pos > 900 && x_pos < 1100) && (y_pos > 250 && y_pos < 310) && self.turn == TurnPhase::TurnP1){
								//println!("current energy is {}", curr_energy);
								// only play if player has enough energy
								if (curr_energy >= curr_card_cost){

					    		//println!("Trying to play card with ID {}\n{}", card_ID, curr_card.to_string());

								// add card to discard pile
					    		self.battle_handler.borrow_mut().get_p1().borrow_mut().hand_discard_card(self.enlarged_card.get_cardpos() );
					    		self.battle_handler.borrow_mut().get_p1().borrow_mut().adjust_curr_energy(-(curr_card_cost as i32));
					    		// if the player has enough energy to cover the cost of playing the card:
					    		crate::cards::battle_system::play_card(Rc::clone(&self.battle_handler), curr_card);

								self.enlarged_card.set_larger(false);
								}
								// otherwise, don'
                    			else {
									self.not_enough_mana = true;
                        			println!("Not enough energy!");
                    			}

							}
							}else if curr_card.get_actions().contains(&19){
								if curr_energy >= curr_card_cost{
									if(x_pos>50&&x_pos<250)&&(y_pos>50&&y_pos<346){
										self.dup_screen(curr_card,curr_card_cost,0);
									}else if(x_pos>250&&x_pos<450)&&(y_pos>200&&y_pos<496){
										self.dup_screen(curr_card,curr_card_cost,2);
									}else if(x_pos>50&&x_pos<250)&&(y_pos>350&&y_pos<646){
										self.dup_screen(curr_card,curr_card_cost,1);
									}else if(x_pos>850&&x_pos<1050)&&(y_pos>200&&y_pos<496){
										self.dup_screen(curr_card,curr_card_cost,3);
									}else if(x_pos>1050&&x_pos<1250)&&(y_pos>50&&y_pos<346){
										self.dup_screen(curr_card,curr_card_cost,4);
									}else if(x_pos>1050&&x_pos<1250)&&(y_pos>350&&y_pos<646){
										self.dup_screen(curr_card,curr_card_cost,5);
									}
								}else{
									self.not_enough_mana = true;
								}
								if(x_pos>550&&x_pos<750&&y_pos>640&&y_pos<700){
									let sz = self.battle_handler.borrow_mut().get_p1().borrow_mut().get_curr_hand_size();
									if sz<2{
										self.battle_handler.borrow_mut().get_p1().borrow_mut().hand_discard_card(self.enlarged_card.get_cardpos() );
										self.battle_handler.borrow_mut().get_p1().borrow_mut().adjust_curr_energy(-(curr_card_cost as i32));
									}
									self.enlarged_card.set_larger(false);
								}
						}
					}
				}
				//see the enemy's last played card
					else if (self.enlarged_card.get_larger() == false && self.enemy_card.get_elarger() == false && x_pos > 550 && x_pos < 750) && (y_pos > 230 && y_pos < 526){
						//if there are cards in the discard pile
						if(self.battle_handler.borrow_mut().get_p2().borrow().get_discard_size() > 0){
							self.enemy_card.set_elarger(true);
						}
						//do nothing
						else{
						}

					}
					else if (self.enemy_card.get_elarger() == true){
						if(((x_pos > 900 && x_pos < 1100) && (y_pos > 400 && y_pos < 460) && self.turn == TurnPhase::TurnP1)){
								self.enemy_card.set_elarger(false);
							}
					}
				else{
				        // check if the player is clicking on any of the cards in their hand
				        //let mut battle_stat = self.battle_handler.borrow_mut();

				        let mut p1_hand_size = self.battle_handler.borrow_mut().get_p1().borrow().get_curr_hand_size();//battle_stat.get_p1().borrow().get_curr_hand_size();
				        //let curr_turn = self.battle_handler.borrow_mut().get_turn();

						if (self.battle_handler.borrow_mut().get_turn()==0&&self.enlarged_card.get_larger()==false&&(x_pos > (260 as i32) && x_pos < (360 + (p1_hand_size * 120) as i32)) && (y_pos > 560 && y_pos < 708)){
							let i = ((x_pos-260)/120) as usize;
							//println!("{}", self.battle_handler.borrow_mut().get_p1().borrow_mut().to_string());
							//println!("{}", self.battle_handler.borrow_mut().get_p2().borrow_mut().to_string());

							//println!("game thinks that the player is clicking on card {}", i);

                            if self.turn == TurnPhase::TurnP1 && self.outcome == BattleOutcome::Undetermined {

							    // play the card
							    let card_rslt = self.battle_handler.borrow_mut().get_p1().borrow().select_hand(i);
							    //let card_cost = card_rslt.unwrap().get_cost();
							    if !card_rslt.is_none(){
								    //enlarge the picked card
							    	    self.enlarged_card.set_cardpos(i as usize);
							    	    self.enlarged_card.set_larger(true);

							    }
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

        // draw the player's cards
		let mut battle_stat = self.battle_handler.borrow_mut();
		let mut _p1 = battle_stat.get_p1();
		let mut player1 = _p1.borrow_mut();
		let mut p1_hand_size = player1.get_curr_hand_size();

		let mut _p2 = battle_stat.get_p2();
		let mut player2 = _p2.borrow_mut();
		let mut p2_hand_size = player2.get_curr_hand_size();

		for i in 0..p1_hand_size {
			let curr_hand = player1.select_hand(i as usize).unwrap();
			crate::video::gfx::draw_sprite_to_dims(&mut wincan, &(self.card_textures.get(curr_hand as usize).unwrap()),(100,148), ((260 + (i * 120)) as i32,560))?;
		}

        // Card Draw Animation P1
        if player1.get_draw_num() > 0 {

            if player1.get_deck_size() == 0 {
                //println!("but there's no more cards in the deck! setting draw_num to 0");
                player1.set_draw_num(0);
            }
            else{
                let target_pos = (260 + (p1_hand_size) * 120) as f32;
                //println!("  Trying to draw!");

                // if the dummy card isn't in the
                if self.dummy_drawn_card.x_pos != target_pos {

                    // increment the position over time
                    self.dummy_drawn_card.x_pos = lerp(self.dummy_drawn_card.x_pos, target_pos, self.frames_elapsed as f32 / 60.0);
                    // increase the frames elapsed in the animation
                    self.frames_elapsed = self.frames_elapsed + 1;

                    //println!("self.dummy_drawn_card.x_pos: {} | self.frames_elapsed: {}", self.dummy_drawn_card.x_pos, self.frames_elapsed);
                    // get the correct sprite for the card being drawn

                    let top_card = player1.get_deck_card().unwrap();
                    crate::video::gfx::draw_sprite_to_dims(&mut wincan, &(self.card_textures.get(top_card as usize).unwrap()),(100,148), ((self.dummy_drawn_card.x_pos) as i32, 560))?;

                    // check if card has reached the destination
                    if self.dummy_drawn_card.x_pos == target_pos {

                        // actually draw the card
                        player1.draw_card(false);

                        self.frames_elapsed = 0;
                        self.dummy_drawn_card.x_pos = 1140.0;

                    }
                }
            }

        }

        if player1.get_deck_size()>0 {

			// make it seem like the last card moves over by removing the deck card once the animation starts
			if !(player1.get_deck_size() == 1 && player1.get_draw_num() > 0) {
			    crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (1140,560))?;
			}
		}

		//crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (1140,560))?;

		// draw the player's energy pips
		let p1_curr_energy = player1.get_curr_energy();
		for i in 0..player1.get_full_energy() {
		    if i < p1_curr_energy {
		        crate::video::gfx::draw_sprite(&mut wincan, &self.e_pip_filled, (20 + (i * 20), 530));
		    }
		    else {
		        crate::video::gfx::draw_sprite(&mut wincan, &self.e_pip_unfilled, (20 + (i * 20), 530));
		    }
		}

		let mut fontm = self.font_manager.borrow_mut();

		// draw the player's status effects
		let mut p1_status_effects = Vec::new();
		let mut p1_status_duration = Vec::new();
		let mut p1_status_amount = Vec::new();
		if player2.get_mult() != 1.0 {p1_status_effects.push(&self.multi);p1_status_duration.push(0);p1_status_amount.push(player2.get_mult());}
		if player1.get_defense() > 0 {p1_status_effects.push(&self.armor);p1_status_duration.push(0);p1_status_amount.push(player1.get_defense() as f32);}
		if player1.get_health_regen() > 0 {p1_status_effects.push(&self.healing);p1_status_duration.push(player1.get_health_regen_duration());p1_status_amount.push(player1.get_health_regen() as f32);}
		if player1.get_poison() > 0 {p1_status_effects.push(&self.posion);p1_status_duration.push(player1.get_poison() as i32);p1_status_amount.push(player1.get_poison() as f32);};
		if player1.get_energy_regen() > 0 {p1_status_effects.push(&self.mana_boost);p1_status_duration.push(player1.get_energy_regen_duration());p1_status_amount.push(player1.get_energy_regen() as f32);}
		if player1.get_energy_regen() < 0 {p1_status_effects.push(&self.mana_drain);p1_status_duration.push(player1.get_energy_regen_duration());p1_status_amount.push(player1.get_energy_regen() as f32);}

		for i in 0..p1_status_effects.len() {
			fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 18, Color::RGB(0, 0, 0),
				&p1_status_amount[i].to_string(), (790+285-(i*30) as i32, 440));
			if p1_status_duration[i]>0{
				fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 18, Color::RGB(0, 0, 0),
					&p1_status_duration[i].to_string(), (790+285-(i*30) as i32, 460));
			}
			crate::video::gfx::draw_sprite(&mut wincan, p1_status_effects[i], (790+280-(i*30) as i32, 480));
		}


		//enemy side

		for i in 0..p2_hand_size {
		    crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), ((920 - (i * 120)) as i32,20))?;
		}

        // Card Draw Animation P2
        if player2.get_draw_num() > 0 {

            if player2.get_deck_size() == 0 {
                //println!("but there's no more cards in the deck! setting draw_num to 0");
                player2.set_draw_num(0);
            }
            else{
                let target_pos = (920 - (p2_hand_size) * 120) as f32;
                //println!("  Trying to draw!");

                // if the dummy card isn't in the
                if self.dummy_drawn_card.x_pos != target_pos {

                    // increment the position over time
                    self.dummy_drawn_card.x_pos = lerp(self.dummy_drawn_card.x_pos, target_pos, self.frames_elapsed as f32 / 50.0);
                    // increase the frames elapsed in the animation
                    self.frames_elapsed = self.frames_elapsed + 1;

                    //println!("self.dummy_drawn_card.x_pos: {} | self.frames_elapsed: {}", self.dummy_drawn_card.x_pos, self.frames_elapsed);

                    // don't need to get the correct sprite for the card, as enemy cards are hidden

                    // although if you copy this line and the one from the P1 function, it does show the card the enemy is drawing, might be a neat card effect
                    //let top_card = player2.get_deck_card().unwrap();
                    crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck, (100,148), ((self.dummy_drawn_card.x_pos) as i32, 20))?;

                    // check if card has reached the destination
                    if self.dummy_drawn_card.x_pos == target_pos {

                        // actually draw the card
                        player2.draw_card(false);

                        self.frames_elapsed = 0;
                        self.dummy_drawn_card.x_pos = 1140.0;

                    }
                }
            }

        }

        if player2.get_deck_size()>0 {

			// make it seem like the last card moves over by removing the deck card once the animation starts
			if !(player2.get_deck_size() == 1 && player2.get_draw_num() > 0) {
			    crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(100,148), (40,20))?;
			}
		}


		fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 18, Color::RGB(150, 0, 0),
			"Enemy Played:", (550,250-40));
		if self.tmp_enemy_played_card<100{
			crate::video::gfx::draw_sprite_to_dims(&mut wincan, &(self.card_textures.get(self.tmp_enemy_played_card)).unwrap(),(200,296),(550,230))?;
		}else{
			crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.deck,(200,296),(550,230))?;
		}

        // draw the enemy's energy pips
		let p2_curr_energy = player2.get_curr_energy();

		for i in 0..player2.get_full_energy() {
		    if i < p2_curr_energy {
		        crate::video::gfx::draw_sprite(&mut wincan, &self.e_pip_filled, (1240 - (i * 20), 184));
		    }
		    else {
		        crate::video::gfx::draw_sprite(&mut wincan, &self.e_pip_unfilled, (1240 - (i * 20), 184));
		    }
		}

		let mut p2_status_effects = Vec::new();
		let mut p2_status_duration = Vec::new();
		let mut p2_status_amount = Vec::new();
		if player1.get_mult() != 1.0 {p2_status_effects.push(&self.multi);p2_status_duration.push(0);p2_status_amount.push(player1.get_mult());}
		if player2.get_defense() > 0 {p2_status_effects.push(&self.armor);p2_status_duration.push(0);p2_status_amount.push(player2.get_defense() as f32);}
		if player2.get_health_regen() > 0 {p2_status_effects.push(&self.healing);p2_status_duration.push(player2.get_health_regen_duration());p2_status_amount.push(player2.get_health_regen() as f32);}
		if player2.get_poison() > 0 {p2_status_effects.push(&self.posion);p2_status_duration.push(player2.get_poison() as i32);p2_status_amount.push(player2.get_poison() as f32);};
		if player2.get_energy_regen() > 0 {p2_status_effects.push(&self.mana_boost);p2_status_duration.push(player2.get_energy_regen_duration());p2_status_amount.push(player2.get_energy_regen() as f32);}
		if player2.get_energy_regen() < 0 {p2_status_effects.push(&self.mana_drain);p2_status_duration.push(player2.get_energy_regen_duration());p2_status_amount.push(player2.get_energy_regen() as f32);}

		for i in 0..p2_status_effects.len() {
			fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 18, Color::RGB(0, 0, 0),
				&p2_status_amount[i].to_string(), (200+285-(i*30) as i32, 270));
			if p2_status_duration[i]>0{
				fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 18, Color::RGB(0, 0, 0),
					&p2_status_duration[i].to_string(), (200+285-(i*30) as i32, 250));
			}
			crate::video::gfx::draw_sprite(&mut wincan, p2_status_effects[i], (200+280-(i*30) as i32, 230));
		}

		//mostly static objects (health bars change tho)

		// Can now update the health bars to dynamically update based on the Battler's health
		let p1perc = 300 as f32 * player1.get_health_percent();
		let p2perc = 300 as f32 * player2.get_health_percent();

		let mut draw_color = Color::RGB(81, 71, 71);
		crate::video::gfx::draw_rect(&mut wincan, draw_color,(300,20), (790,520));
		crate::video::gfx::draw_rect(&mut wincan, draw_color,(300,20), (200,190));
		//crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.behind_health,(300,20), (790,520))?;
		//crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.behind_health,(300,20), (200,190))?;

		if p1perc<=75.0{
			draw_color = Color::RGB(210, 27, 27);
		}else if p1perc<=150.0{
			draw_color = Color::RGB(225, 235, 60);
		}else{
			draw_color = Color::RGB(60, 220, 30);
		}

		crate::video::gfx::draw_rect(&mut wincan, draw_color,(p1perc as u32,20), (790+(300-p1perc as i32),520))?; //player health bar
		fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 18, draw_color,
			&player1.get_curr_health().to_string(), (790,520-25));//text

		if p2perc<=75.0{
			draw_color = Color::RGB(210, 27, 27);
		}else if p2perc<=150.0{
			draw_color = Color::RGB(225, 235, 60);
		}else{
			draw_color = Color::RGB(60, 220, 30);
		}

		crate::video::gfx::draw_rect(&mut wincan, draw_color,(p2perc as u32,20), (200,190))?; //enemy health bar
		fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 18, draw_color,
			&player2.get_curr_health().to_string(), (200,190+25)); //text



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

		if(self.enlarged_card.get_larger() == true){
			let curr_selection = player1.select_hand(self.enlarged_card.get_cardpos() as usize);
			if !curr_selection.is_none(){
			let curr_sel = curr_selection.unwrap();
			let curr_card = battle_stat.get_card(curr_sel);
			if curr_card.get_actions().contains(&19){
				crate::video::gfx::draw_sprite_to_fit(&mut wincan, &self.backDrop)?;
				crate::video::gfx::draw_sprite_to_dims(&mut wincan, &(self.card_textures.get(curr_sel as usize).unwrap()),(400,592), (450,50))?;
				let mut num = 0;
				for i in 0..p1_hand_size {
					let curr_hand = player1.select_hand(i as usize).unwrap();
					if curr_hand!=curr_sel{
						match num{
							0=>crate::video::gfx::draw_sprite_to_dims(&mut wincan, &(self.card_textures.get(curr_hand as usize).unwrap()),(200,296), (50,50))?,
							2=>crate::video::gfx::draw_sprite_to_dims(&mut wincan, &(self.card_textures.get(curr_hand as usize).unwrap()),(200,296), (250,200))?,
							1=>crate::video::gfx::draw_sprite_to_dims(&mut wincan, &(self.card_textures.get(curr_hand as usize).unwrap()),(200,296), (50,350))?,
							3=>crate::video::gfx::draw_sprite_to_dims(&mut wincan, &(self.card_textures.get(curr_hand as usize).unwrap()),(200,296), (850,200))?,
							4=>crate::video::gfx::draw_sprite_to_dims(&mut wincan, &(self.card_textures.get(curr_hand as usize).unwrap()),(200,296), (1050,50))?,
							5=>crate::video::gfx::draw_sprite_to_dims(&mut wincan, &(self.card_textures.get(curr_hand as usize).unwrap()),(200,296), (1050,350))?,
							_=>()//unreachable
						};
						num+=1;
					}
				}
				if player1.get_curr_hand_size()>=2{
					crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.retCard, (200,60),(550,640))?;
				}else{
					crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.playCard, (200,60),(550,640))?;
				}
			}else{
				crate::video::gfx::draw_sprite_to_fit(&mut wincan, &self.backDrop)?;
				crate::video::gfx::draw_sprite_to_dims(&mut wincan, &(self.card_textures.get(curr_sel as usize).unwrap()),(400,592), (450,50))?;
				crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.playCard, (200,60),(900,250))?;
				crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.retCard, (200,60),(900,400))?;
			}
		}
		}

		if self.not_enough_mana==true{
			fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 40, Color::RGB(200, 200, 0),
				"Not enough mana!", (500, 10));
		}

		//see the enemy's card
		if(self.enemy_card.get_elarger() == true){
			let curr_card = player2.get_discard_card().unwrap();

			crate::video::gfx::draw_sprite_to_fit(&mut wincan, &self.backDrop)?;
			crate::video::gfx::draw_sprite_to_dims(&mut wincan, &(self.card_textures.get(curr_card as usize).unwrap()),(400,592), (450,50))?;
			crate::video::gfx::draw_sprite_to_dims(&mut wincan, &self.retCard, (200,60),(900,400))?;
		}

		match self.outcome {
		    BattleOutcome::VictoryP1 => fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 64, Color::RGB(0, 0, 0), "VICTORY!", (50, 330)),
		    BattleOutcome::VictoryP2 => fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 64, Color::RGB(0, 0, 0), "DEFEAT", (50, 330)),
		    BattleOutcome::Tie => fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 64, Color::RGB(0, 0, 0), "DRAW...", (50, 330)),
		    _ => {

		        // if the battle is ongoing and it's the enemy's turn, say so
		        if self.turn == TurnPhase::PreTurnP2 || self.turn == TurnPhase::TurnP2 || self.turn == TurnPhase::PostTurnP2 {
                    fontm.draw_text_ext(&mut wincan, "assets/fonts/Roboto-Regular.ttf", 64, Color::RGB(0, 0, 0), "Opponent's Turn...", (50, 330));
                }
                Ok(())

		    },
		};

		wincan.present();
		Ok(())
	}
}

//card size and position
//note: Don't need to delete the original card when enlarging it. Makes life easier
struct card_size{
	card_pos: usize, //where it is in the player's hand
	x_size: u32, //size of the card width-wise (will just multiply it by some number)
	y_size: u32, //size of the card height-wise (will just multiply it by some number)
	x_pos: u32, //when enlarged, x position =
	y_pos: u32, //when enlarged, y position =
	larger: bool,
}

impl card_size{

	fn get_cardpos(&mut self)->usize{
        	self.card_pos
	}


	fn get_larger(&mut self)->bool{
		self.larger
	}

	fn set_cardpos(&mut self, h: usize){
		self.card_pos = h;
	}

	fn set_larger(&mut self, h: bool){
		self.larger = h;
	}
}


//card size and position
//note: Don't need to delete the original card when enlarging it. Makes life easier
struct e_card_size{
	ex_size: u32, //size of the card width-wise (will just multiply it by some number)
	ey_size: u32, //size of the card height-wise (will just multiply it by some number)
	ex_pos: u32, //when enlarged, x position =
	ey_pos: u32, //when enlarged, y position =
	elarger: bool,
}

impl e_card_size{

	fn get_elarger(&mut self)->bool{
		self.elarger
	}

	fn set_elarger(&mut self, h: bool){
		self.elarger = h;
	}
}

struct DrawnCard {
	x_pos: f32,
	y_pos: f32,
}

// essentially just keeps track of where the "drawn" card should be
impl <'a> DrawnCard {
    pub fn new(x_pos: f32, y_pos: f32) -> Result<Self, String> {
        Ok(DrawnCard { x_pos, y_pos })
    }
}

// animation helper functions

// for more info look up "linear interpolation"

// essentially, progress can be thought of as progress through the animation length
// for the card draw, the animation length is 60 frames, meaning no matter where the card has to go, it will
// always finish in 60 frames due to how linear interpolation works, allowing us to have a relatively constant time animation

// smoothly move from start_pos to end_pos (one dimension, so either x or y) based on the current progress
// will return start_pos when progress == 0
// will return end_pos when progress == 1
// will return -1 if progress < 0 or progress > 1
// if start_pos == end_pos, returns start_pos
pub fn lerp(start_pos: f32, end_pos: f32, progress: f32) -> f32 {

    if progress > 1.0 || progress < 0.0{ return -1 as f32 }


    //println!("lerp was given start_pos: {} | end_pos: {} | progress: {}, calculated: {}", start_pos, end_pos, progress, start_pos + progress * (end_pos - start_pos));

    start_pos + progress * (end_pos - start_pos)


}
