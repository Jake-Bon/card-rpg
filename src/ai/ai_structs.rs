use std::fs;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;
use std::cmp;

use crate::cards::game_structs::*;
use crate::cards::battle_system::*;

// !! README: !!
// Debug: cargo run > treetest.txt (for the most readable print of the tree)

//  The heavy lifting of creating the game tree is done in Node.populate()

// GameTree.populate(), given the battle's current BattleStatus,
// calls Node.populate(). This simulates every playout up to the depth limit

// The AI will match up every card in its hand to every card in the players hand.

//  Currently, the tree only assumes the player and ai will play a single
//      card per turn.

//  Decks do not yet replenish.

// AI considers only the cards in the player's hand. This is kind of cheaty, but
// because of very poor performance when considering all the cards the player could
// play (deck+hand), I opted to leave it cheaty for now.



// TODOs:
//  - Deck replenishment when cards run out during populate() simulation
//  - Simulate for multiple cards being played per turn
//  - Utility function tuning



// !! Known Bugs: !!
//  - Cards that draw more cards do not seem to have an effect on populates() simulation. ie no new cards are drawn
//      -This also means that the calculated utility of the states after a card draw are inacurrate

pub struct Node {
    utility: Option<f32>,
    status: BattleStatus,
    last_played_card: u32,
    children: Vec<Node>,
}

impl Node {
    pub fn new(mut incoming_status: BattleStatus, last: u32) -> Node {
        let utility = None;
        let last_played_card = last;
        let children: Vec<Node> = Vec::new();
        let status = reset_ref(incoming_status);
        Node {utility, status, last_played_card, children}
    }

    // Recursive function to populate each game tree node with children
    pub fn populate(&mut self, ai_turn: bool, height: i32) {
        if height == 0 || self.stateIsTerminating() || self.last_card_was_special()  {
            return;
        }
        let ai_hand = self.status.get_p2().borrow().get_hand();
        let player_hand = self.status.get_p1().borrow().get_hand();
        let player_deck = self.status.get_p1().borrow().get_deck();
        let mut already_played_cards: HashSet<u32> = HashSet::new();
        // Ai's turn (p2)
        // TODO: Simulation for playing more than one card per turn
        // TODO: Deck replenishment during simulation if cards run out
        if ai_turn {
            // Pick a card from the ai's hand to play, attempting to play one card from the
            // hand at a time and perseving the others.
            for i in 0..ai_hand.len() {
                // The BattleStatus that we will modify to pass on to the next node
                let mut next_status = reset_ref(self.status.clone());
                // Get card ID and related card struct
                let card_id = ai_hand[i];
                let curr_card = next_status.get_card(card_id as u32);
                // If we've already simulated this round with this card, skip
                if already_played_cards.contains(&card_id) {
                    continue;
                }
                already_played_cards.insert(card_id);
                // If current card is too costly, skip
                if curr_card.get_cost() as i32 > next_status.get_p2().borrow().get_curr_energy() {
                    continue;
                }
                // Remove card from hand
                next_status.get_p2().borrow_mut().hand_del_card(i);
                // Play the card
                crate::cards::battle_system::play_card(Rc::new(RefCell::new(next_status.clone())), curr_card.clone());
                // Update effects and turner
                next_status.get_p2().borrow_mut().update_effects();
                next_status.turner();
                // Add new child node representing the game state after
                // card_to_play is played
                self.children.push(Node::new(next_status, card_id));
            }
            // Nothing was played, probably low mana
            if ai_hand.len() > 0 && self.children.len() == 0 {
                let mut next_status = reset_ref(self.status.clone());
                next_status.get_p2().borrow_mut().update_effects();
                next_status.turner();
                self.children.push(Node::new(next_status, u32::MAX));
            }
        }
        // Player's turn (p1)
        // TODO: Simulation for playing more than one card per turn
        // TODO: Deck replenishment during simulation if cards run out
        else {
            for i in 0..player_hand.len() { //+player_deck.len() Removed for performance concerns
                // The BattleStatus that we will modify to pass on to the next node
                let mut next_status = reset_ref(self.status.clone());

                // Get card ID and card struct
                let card_id = player_hand[i];
                let curr_card = next_status.get_card(card_id as u32);
                // If we've already played the card for this turn, skip
                if already_played_cards.contains(&card_id) {
                    continue;
                }
                already_played_cards.insert(card_id);
                // If current card is too costly, skip
                if curr_card.get_cost() as i32 > next_status.get_p1().borrow().get_curr_energy() {
                    continue;
                }
                // Delete card from hand
                next_status.get_p1().borrow_mut().hand_del_card(i);
                // Removed for performance concerns

                // Play card
                crate::cards::battle_system::play_card(Rc::new(RefCell::new(next_status.clone())), curr_card.clone());
                // Update effects and turner
                next_status.get_p1().borrow_mut().update_effects();
                next_status.turner();
                // Add new child node representing the game state after
                // card_to_play is played
                self.children.push(Node::new(next_status, card_id));
            }
            // Nothing was played, probably low mana
            if player_hand.len() > 0 && self.children.len() == 0 {
                let mut next_status = reset_ref(self.status.clone());
                next_status.get_p1().borrow_mut().update_effects();
                next_status.turner();
                self.children.push(Node::new(next_status, u32::MAX));
            }
        }
        // Recursively populate each child
        for child in &mut self.children {
            child.populate(!ai_turn, height-1);
        }
    }

    // Checks if the current node is in a game terminating state
    pub fn stateIsTerminating(&mut self) -> bool {
        let player = self.status.get_p1().borrow().clone();
        let ai = self.status.get_p2().borrow().clone();
        if 0 >= player.get_curr_health() || 0 >= ai.get_curr_health() {
            return true;
        }
        return false;
    }

    pub fn last_card_was_special(&mut self) -> bool {
        let last = self.last_played_card;
        if last == 16 || last == 17 || last == 20 || last == 25 || last == 26 {
            return true;
        }
        return false;
    }

    pub fn get_status(&mut self) -> BattleStatus {
        self.status.clone()
    }

    pub fn set_utility(&mut self, new_utility: f32) {
        self.utility = Some(new_utility);
    }

    // Utility function
    pub fn calculate_utilities(&mut self) {
        if self.children.len() > 0 {
            for child in &mut self.children {
                child.calculate_utilities();
            }
            return;
        }
        let mut status = self.get_status();
        let player = status.get_p1().borrow().clone();
        let ai = status.get_p2().borrow().clone();
        if ai.get_curr_health() <= 0 {
            self.set_utility(f32::MIN);
           // println!("UTILITY: {} @ ai_hp {}, p1_hp {}", i32::MIN, ai.get_curr_health(), player.get_curr_health());
            return;
        }
        else if player.get_curr_health() <= 0 {
            self.set_utility(f32::MAX);
            //println!("UTILITY: {} @ ai_hp {}, p1_hp {}", i32::MAX, ai.get_curr_health(), player.get_curr_health());
            return;
        }
        // !! Tune up needed here !!
        // Utility value calculations
        // AI's
        let ai_health = ai.get_curr_health() as f32;
        let ai_energy = ai.get_curr_energy() as f32;
        let ai_deck = ai.get_deck_size() as f32;
        let ai_health_regen = ai.get_health_regen() as f32;
        let ai_energy_regen = ai.get_energy_regen() as f32;
        let ai_poison = ai.get_poison() as f32;
        let ai_defense = ai.get_defense() as f32;
        //let ai_utility = (ai_health*2 + ai_energy + ai_deck.sqrt() as i32 + ai_health_regen + ai_energy_regen + ai_defense) - ai_poison;
        // Player's
        let player_health = player.get_curr_health() as f32;
        let player_energy = player.get_curr_energy() as f32;
        let player_deck = player.get_deck_size() as f32;
        let player_health_regen = player.get_health_regen() as f32;
        let player_energy_regen = player.get_energy_regen() as f32;
        let player_poison = player.get_poison() as f32;
        let player_defense = player.get_defense() as f32;
        //let player_utility = (player_health + player_energy + player_deck.sqrt() as i32 + player_health_regen + player_energy_regen + ai_defense) - ai_poison;
        // Combine
        //let utility = ai_utility - player_utility;

        //println!("ai_hp {}, p1_hp {}, ai_deck {}, ai_def {}", ai_health, player_health, ai_deck, ai_defense);
        
        let utility = -632.5/(ai_health+1.0) // avoids states w/ ai low health
                + 3.1*ai_deck
                + 4.0*ai_defense
                - 2.8*ai_poison
                + 2.4*ai_energy
                + 3.5*ai_health_regen
                + 206.0/(player_health+1.0)   // gravitates to states w/ player low health
                + 3.6*player_poison
                - 0.2*player_energy
                - 0.8*player_health_regen
                - 0.6*player_defense
                - 1.3*player_deck
                ;
        //println!("UTILITY: {} @ ai_hp {}, p1_hp {}", utility, ai_health, player_health);
        self.set_utility(utility);
        //println!();
    }

    // For use in Minimax
    pub fn maximizer(&mut self, mut alpha: f32, mut beta: f32) -> f32 {
        if self.children.len() <= 0 {
            return self.utility.unwrap();
        }
        let mut best_value = f32::MIN;
        for child in &mut self.children {
            let value = child.minimizer(alpha, beta);
            //best_value = cmp::max(best_value, value);
            best_value = if best_value > value {
                best_value
            } else {
                value
            };
            //alpha = cmp::max(alpha, best_value);
            alpha = if alpha > best_value {
                alpha
            } else {
                best_value
            };
            if beta <= alpha {
                break;
            }
        }
        self.set_utility(best_value);
        return best_value;
    }

    // For use in Minimax
    pub fn minimizer(&mut self, mut alpha: f32, mut beta: f32) -> f32 {
        if self.children.len() <= 0 {
            return self.utility.unwrap();
        }
        let mut best_value = f32::MAX;
        for child in &mut self.children {
            let value = child.maximizer(alpha, beta);
            //best_value = cmp::min(best_value, value);
            best_value = if best_value < value {
                best_value
            } else {
                value
            };
            //beta = cmp::min(beta, best_value);
            beta = if beta < best_value {
                beta
            } else {
                best_value
            };
            if beta <= alpha {
                break;
            }
        }
        self.set_utility(best_value);
        return best_value;
    }

    pub fn print(&mut self, tab: u32) {
        let mut util = ("None").to_string();
        if self.utility.is_some() {
            util = self.utility.unwrap().to_string();
        }

        for i in 0..tab {
            print!("-");
        }

        if tab % 2 == 0 {
            let mut status = self.get_status();
            let ai = status.get_p2().borrow().clone();
            let card_id = self.last_played_card;
            let mut card_name = String::new();
            if card_id == u32::MAX {
                card_name = String::from("Unknown");
            }
            else {
                card_name = status.get_card(card_id).get_name().to_string();
            }
            println!("ai|d:{}|c:{}|last:{}|h:{}|e:{}|u:{}", ai.get_deck().len(), ai.get_deck().len() + ai.get_hand().len(), card_name, ai.get_curr_health(), ai.get_curr_energy(), util);
        }
        else {
            let mut status = self.get_status();
            let player = status.get_p1().borrow().clone();
            let card_id = self.last_played_card;
            let mut card_name = String::new();
            if card_id == u32::MAX {
                card_name = String::from("Unknown");
            }
            else {
                card_name = status.get_card(card_id).get_name().to_string();
            }
            println!("pl|d:{}|c:{}|last:{}|h:{}|e:{}|u:{}", player.get_deck().len(), player.get_deck().len() + player.get_hand().len(), card_name, player.get_curr_health(), player.get_curr_energy(), util);
        }
        for child in &mut self.children {
            child.print(tab+1);
        }
    }
}

pub struct GameTree {
    root: Node,
}

impl GameTree {
    pub fn new(status: BattleStatus) -> GameTree {
        //let status: BattleStatus = rc_status.borrow().clone();
        let last_played_card = u32::MAX;
        let root = Node::new(reset_ref(status), last_played_card);
        GameTree { root }
    }

    // Takes rounds as a parameter. Each round consists of a player turn and an AI
    // turn. In testing, numbers beyond 3 may take prohibitively long or stall the
    // program completely for decks with a great variance of card. For relatively
    // simple/homogenous decks, performance for simulating beyond 3 turns is far
    // greater. Keep this in mind if we want to have AI of varying ability to
    // 'see into the future' and therefore probably be more difficult for the player.
    // In short, longer simulations pair best with a more homogenous AI deck.
    //
    // !! Currently, the ai only simulates the game tree based on the player's
    // !! hand, so performance is greater than stated above.
    pub fn populate(&mut self, rounds: i32) {
        self.root.populate(true, rounds*2);
    }

    pub fn calculate_utilities(&mut self) {
        self.root.calculate_utilities();
    }

    pub fn minimax(&mut self) -> Option<u32> {
        let best_utility = self.root.maximizer(f32::MIN, f32::MAX);
        for child in &mut self.root.children {
            if child.utility.unwrap() == best_utility && child.last_played_card != u32::MAX {
                return Some(child.last_played_card);
            }
        }
        return None;
    }

    pub fn has_ties(&mut self) -> bool {
        let best_utility = self.root.maximizer(f32::MIN, f32::MAX);
        let mut tie_flag = 0;
        for child in &mut self.root.children {
            if child.utility.is_some() && child.utility.unwrap() == best_utility {
                tie_flag = tie_flag + 1;
            }
        }
        if (tie_flag > 1) {
            return true;
        }
        return false;
    }

    pub fn print(&mut self) {
        println!("Name | Children | Last Card Played | This player's health | Posion");
        self.root.print(0);
    }
}

// Resets the refrences within BattleStatus so that when we clone BattleStatus
// we have distinct battlers rather than a reference to the same Battlers as the
// source of the copy
pub fn reset_ref(mut incoming_status: BattleStatus) -> BattleStatus {
    let mut status = BattleStatus::new(
        Rc::new(RefCell::new(incoming_status.get_p1().borrow().clone())),
        Rc::new(RefCell::new(incoming_status.get_p2().borrow().clone()))
    );
    if incoming_status.get_turn() > 0 {
        status.turner();
    }
    status
}

