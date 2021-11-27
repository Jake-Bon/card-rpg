use std::fs;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

use crate::cards::game_structs::*;
use crate::cards::battle_system::*;

// README:
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
//  - Utility function for arbitrary game state

pub struct Node {
    utility: Option<i32>,
    status: BattleStatus,
    last_played_card: Card,
    children: Vec<Node>,
}

impl Node {
    pub fn new(mut incoming_status: BattleStatus, last: Card) -> Node {
        let utility = None;
        let last_played_card = last;
        let children: Vec<Node> = Vec::new();
        let status = reset_ref(incoming_status);
        Node {utility, status, last_played_card, children}
    }

    // Recursive function to populate each game tree node with children
    pub fn populate(&mut self, ai_turn: bool, height: i32) {
        if height == 0 || self.stateIsTerminating() {
            // State is a terminating state. Set utility to a predetermined number to indicate win/loss
            if self.stateIsTerminating() {
                if self.status.get_p2().borrow().get_curr_health() <= 0 {
                    self.set_utility(i32::MIN);
                }
                else {
                    self.set_utility(i32::MAX);
                }
            }
            // Height is 0
            else {
                // TODO: Call utility function here to calculate utility base on curr game state
            }
            return;
        }
        let ai_cards = self.status.get_p2().borrow().get_hand();
        let player_hand = self.status.get_p1().borrow().get_hand();
        let player_deck = self.status.get_p1().borrow().get_deck();
        let mut already_played_cards: HashSet<u32> = HashSet::new();
        // Ai's turn (p2)
        if ai_turn {
            // Pick a card from the ai's hand to play, attempting to play one card from the
            // hand at a time and perseving the others.
            
            for i in 0..ai_cards.len() {
                // The BattleStatus that we will modify to pass on to the next node
                let mut next_status = reset_ref(self.status.clone());
                // Get card ID and related card struct
                let card_id = ai_cards[i];
                let curr_card = next_status.get_card((card_id as u32));
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
                self.children.push(Node::new(next_status, curr_card));
            }
        }
        // Player's turn (p1)
        else {
            for i in 0..player_hand.len() { //+player_deck.len() Removed for performance concerns
                // The BattleStatus that we will modify to pass on to the next node
                let mut next_status = reset_ref(self.status.clone());
                // Removed for performance concerns
                /*
                let card_id = {
                    if i < player_hand.len() {
                        player_hand[i]
                    }
                    else {
                        player_deck[i-player_hand.len()]
                    }
                };
                */
                // Get card ID and card struct
                let card_id = player_hand[i];
                let curr_card = next_status.get_card((card_id as u32));
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
                /*
                if i < player_hand.len() {
                    next_status.get_p1().borrow_mut().hand_del_card(i);
                }
                else {
                    next_status.get_p1().borrow_mut().deck_del_card_specific(i-player_hand.len());
                }
                */
                // Play card
                crate::cards::battle_system::play_card(Rc::new(RefCell::new(next_status.clone())), curr_card.clone());
                // Update effects and turner
                next_status.get_p1().borrow_mut().update_effects();
                next_status.turner();
                // Add new child node representing the game state after
                // card_to_play is played
                self.children.push(Node::new(next_status, curr_card));
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

    pub fn set_utility(&mut self, new_utility: i32) {
        self.utility = Some(new_utility);
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
            println!("ai|c:{}|last:{}|h:{}|p:{}|u:{}", self.status.get_p2().borrow().get_hand().len(), self.last_played_card.get_name(), self.status.get_p2().borrow().get_curr_health(),self.status.get_p2().borrow().get_poison(), util);
        }
        else {
            println!("pl|c:{}|last:{}|h:{}|p:{}|u:{}", self.status.get_p1().borrow().get_hand().len() + self.status.get_p1().borrow().get_deck().len(), self.last_played_card.get_name(), self.status.get_p1().borrow().get_curr_health(),self.status.get_p1().borrow().get_poison(), util);
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
        let last_played_card = Card::new(String::from("Unknown"), String::from("Unknown"), 0, vec![0], vec![0], String::from("None"));
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
    // *hand*, so performance is greater than stated above.
    pub fn populate(&mut self, rounds: i32) {
        self.root.populate(true, rounds*2);
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
