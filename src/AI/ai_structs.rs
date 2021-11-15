use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

use crate::cards::game_structs::Battler;
use crate::cards::battle_system::*;

// Important info:
//  The heavy lifting of creating the game tree is done in Node.populate()
//  Currently, the tree only assumes the player and ai will play a single
//      card per turn. TODO: simulate multiple cards played per turn for
//      ai and player
//  Energy is not taken into account. TODO: only create nodes for actions
//      that are possible when energy is taken into account. No need to 
//      create impossible children

pub struct Node {
    utility: i32,
    last_played: Option<i32>,
    ai: Rc<RefCell<Battler>>,
    player: Rc<RefCell<Battler>>,
    ai_cards: Vec<i32>,
    player_cards: Vec<i32>,
    children: Vec<Node>,
}

impl Node {
    pub fn new(last_played: Option<i32>, ai: Rc<RefCell<Battler>>, player: Rc<RefCell<Battler>>, ai_cards: Vec<i32>, player_cards: Vec<i32>) -> Node {
        let utility = 0;
        let children: Vec<Node> = Vec::new();
        Node {utility, last_played, ai, player, ai_cards, player_cards, children}
    }

    pub fn set_last_played(&mut self, card_id: i32) {
        self.last_played = Some(card_id);
    }

    // Recursive function to populate each game tree node with children
    pub fn populate(&mut self, ai_turn: bool) {
        // If the node represents a game over state, return and don't generate more children
        if self.stateIsTerminating() {
            return;
        }
        // Actions to take when it is AI's turn to play
        if ai_turn {
            let mut curr_cards = Vec::new();
            /*
            if self.ai_cards.len() <= 0 {
                // TODO: read in variable deck for replenishment
                curr_cards = read_deck(1)
            }
            else {
                curr_cards = self.ai_cards.clone();
            }
            */
            curr_cards = self.ai_cards.clone();
            // Pick a card to play and simulate turn
            for i in 0..curr_cards.len() {
                let card_to_play = curr_cards[i];
                let mut cards_to_pass_on = Vec::new();
                for j in 0..curr_cards.len() {
                    if curr_cards[j] == card_to_play {
                        continue;
                    }
                    cards_to_pass_on.push(curr_cards[j]);
                }
                let mut ai_next = self.ai.clone();
                let mut player_next = self.player.clone();
                // Alter new game state based on card played
                let mut mult = 1.0;
                let mut damage_boost = 0;
                match card_to_play {
                    0 => {
                        defend(2, Rc::clone(&ai_next));
                    },//Shroud: defend for 2
                    1 => {
                        heal(5, Rc::clone(&ai_next));
                    },//Lisrae: heal for 5
                    2 => {
                        damage_boost = 2;
                    },//The Perfect Storm: until next turn all attacks do 2 more damage
                    3 => {

                    },//Whirlpool: damage for 1 for 2 turns
                    4 => {
                        health_regen(3, Rc::clone(&ai_next));
                    },//GTP: regen 3hp for 3 turns
                    5 => {
                        attack(3+damage_boost, mult, Rc::clone(&player_next));
                        change_mana_regen(-2, Rc::clone(&player_next));
                    },//HHG: deal 3 damage and slow enemy mana regen -2
                    6 => {
                        change_mana_regen(3, Rc::clone(&ai_next));
                    },//Photosynthesis: regen 3 energy for 3 turns
                    7 => {
                        attack(10+damage_boost, mult, Rc::clone(&player_next));
                        attack(8+damage_boost, mult, Rc::clone(&ai_next));
                    },//Last Shot: deal 10 dmg take 8 dmg
                    8 => {
                        poison(3, Rc::clone(&player_next));
                    },//Demon's Posion: 3 posion damage for 3 turns
                    9 => {
                        defend(3, Rc::clone(&ai_next));
                    },//Barrel Protection: defend for 3
                    10 => {
                        defend(1, Rc::clone(&ai_next));
                        attack(2+damage_boost, mult, Rc::clone(&player_next));
                    },//Ghastly cannonball: deal 2 dmg defend for 1
                    11 => {
                        defend(5, Rc::clone(&ai_next));
                    },//Vcard: defend for 5
                    12 => {
                        heal(4, Rc::clone(&ai_next));
                        attack(1+damage_boost, mult, Rc::clone(&player_next));
                    },//Kitty does drugs: heal for 4 dmg for 1
                    13 => {
                        defend(4, Rc::clone(&ai_next));
                    },//Straw Hat: defend for 4
                    14 => {
                        cure(Rc::clone(&ai_next));
                    },//Sparkle! Sparkle!: cure posion
                    15 => {
                        change_mana_regen(2, Rc::clone(&ai_next));
                    },//Time to rock: 2 extra mana for 3 turns
                    16 => {
                        //TODO
                    },//High Noon: draw 1 card
                    17 => {
                        //TODO
                    },//Swab the deck: draw 2 cards
                    18 => {
                        attack(4+damage_boost, mult, Rc::clone(&player_next));
                    },//Cutlass: dmg for 4
                    19 => {
                        heal(3, Rc::clone(&ai_next));
                    },//Fishing Minigame: heal 3
                    20 => {
                        //TODO
                    },//Pest Problem: shuffle 3 rats into enemy deck
                    21 => {
                        attack(1+damage_boost, mult, Rc::clone(&ai_next));
                        attack(1+damage_boost, mult, Rc::clone(&player_next));
                    },//Rat: deal 1 dmg to both players
                    22 => {
                        change_mana_regen(-3, Rc::clone(&player_next));
                    },//Mermaid's Tempation: enemy loses 3 mana for 3 turns
                    23 => {
                        //TODO
                    },//Deadly Blade: dmg for 8 and double next attack
                    24 => {
                        attack(7+damage_boost, mult, Rc::clone(&player_next));
                    },//Melonball: dmg for 7
                    _ => panic!("Card id {} is not recognized by GameTree", card_to_play),
                }
                self.children.push(Node::new(Some(card_to_play), ai_next.clone(), player_next.clone(), cards_to_pass_on, self.player_cards.clone()));
            }
        }
        // Actions to take when it is Player's turn to play
        else {
            let mut curr_cards = Vec::new();
            /*
            if self.player_cards.len() <= 0 {
                // TODO: read in variable deck for replenishment
                curr_cards = read_deck(1)
            }
            else {
                curr_cards = self.player_cards.clone();
            }
            */
            curr_cards = self.player_cards.clone();
            //curr_cards = self.player_cards.clone();
            // Pick a card to play and simulate turn
            for i in 0..curr_cards.len() {
                let card_to_play = curr_cards[i];
                let mut cards_to_pass_on = Vec::new();
                for j in 0..curr_cards.len() {
                    if curr_cards[j] == card_to_play {
                        continue;
                    }
                    cards_to_pass_on.push(curr_cards[j]);
                }
                let mut ai_next = self.ai.clone();
                let mut player_next = self.player.clone();
                // Alter new game state based on card played
                let mut mult = 1.0;
                let mut damage_boost = 0;
                match card_to_play {
                    0 => {
                        defend(2, Rc::clone(&player_next));
                    },//Shroud: defend for 2
                    1 => {
                        heal(5, Rc::clone(&player_next));
                    },//Lisrae: heal for 5
                    2 => {
                        damage_boost = 2;
                    },//The Perfect Storm: until next turn all attacks do 2 more damage
                    3 => {

                    },//Whirlpool: damage for 1 for 2 turns
                    4 => {
                        health_regen(3, Rc::clone(&player_next));
                    },//GTP: regen 3hp for 3 turns
                    5 => {
                        attack(3+damage_boost, mult, Rc::clone(&ai_next));
                        change_mana_regen(-2, Rc::clone(&ai_next));
                    },//HHG: deal 3 damage and slow enemy mana regen -2
                    6 => {
                        change_mana_regen(3, Rc::clone(&player_next));
                    },//Photosynthesis: regen 3 energy for 3 turns
                    7 => {
                        attack(10+damage_boost, mult, Rc::clone(&ai_next));
                        attack(8+damage_boost, mult, Rc::clone(&player_next));
                    },//Last Shot: deal 10 dmg take 8 dmg
                    8 => {
                        poison(3, Rc::clone(&ai_next));
                    },//Demon's Posion: 3 posion damage for 3 turns
                    9 => {
                        defend(3, Rc::clone(&player_next));
                    },//Barrel Protection: defend for 3
                    10 => {
                        defend(1, Rc::clone(&player_next));
                        attack(2+damage_boost, mult, Rc::clone(&ai_next));
                    },//Ghastly cannonball: deal 2 dmg defend for 1
                    11 => {
                        defend(5, Rc::clone(&player_next));
                    },//Vcard: defend for 5
                    12 => {
                        heal(4, Rc::clone(&player_next));
                        attack(1+damage_boost, mult, Rc::clone(&ai_next));
                    },//Kitty does drugs: heal for 4 dmg for 1
                    13 => {
                        defend(4, Rc::clone(&player_next));
                    },//Straw Hat: defend for 4
                    14 => {
                        cure(Rc::clone(&player_next));
                    },//Sparkle! Sparkle!: cure posion
                    15 => {
                        change_mana_regen(2, Rc::clone(&player_next));
                    },//Time to rock: 2 extra mana for 3 turns
                    16 => {
                        //TODO
                    },//High Noon: draw 1 card
                    17 => {
                        //TODO
                    },//Swab the deck: draw 2 cards
                    18 => {
                        attack(4+damage_boost, mult, Rc::clone(&ai_next));
                    },//Cutlass: dmg for 4
                    19 => {
                        heal(3, Rc::clone(&player_next));
                    },//Fishing Minigame: heal 3
                    20 => {
                        //TODO
                    },//Pest Problem: shuffle 3 rats into enemy deck
                    21 => {
                        attack(1+damage_boost, mult, Rc::clone(&player_next));
                        attack(1+damage_boost, mult, Rc::clone(&ai_next));
                    },//Rat: deal 1 dmg to both players
                    22 => {
                        change_mana_regen(-3, Rc::clone(&ai_next));
                    },//Mermaid's Tempation: enemy loses 3 mana for 3 turns
                    23 => {
                        //TODO
                    },//Deadly Blade: dmg for 8 and double next attack
                    24 => {
                        attack(7+damage_boost, mult, Rc::clone(&ai_next));
                    },//Melonball: dmg for 7
                    _ => panic!("Card id {} is not recognized by GameTree", card_to_play),
                }
                self.children.push(Node::new(Some(card_to_play), ai_next.clone(), player_next.clone(), self.ai_cards.clone(), cards_to_pass_on.clone()));
            }
        }
        // Recursively populate the newly generated children of the current node
        for child in &mut self.children {
            child.populate(!ai_turn);
        }
    }

    // Checks if the current node is in a game terminating state
    pub fn stateIsTerminating(&mut self) -> bool {
        if self.ai.borrow().get_curr_health() <= 0 || self.player.borrow().get_curr_health() <= 0 {
            return true;
        }
        return false;
    }

    pub fn print(&mut self, tab: u32) {
        for i in 0..tab {
            print!("\t");
        }
        if self.last_played.is_none() {
            print!("root|{}|{}=>{} \n", self.ai.clone().borrow().get_curr_health(), self.ai_cards.len(), self.children.len());
        }
        else {
            print!("{}|{}|{}=>{} \n", self.last_played.unwrap(), self.ai.clone().borrow().get_curr_health(), self.ai_cards.len(), self.children.len());
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
    pub fn new(ai_battler: Battler, pl_battler: Battler) -> GameTree {
        // Match battler name to battler id from battler-library.txt
        let enemy_id = {
            match ai_battler.get_name() {
                "Basic Enemy" => 1,
                "Stall" => 2,
                _=> 1,
            }
        };
        // Read in the decks of the players from battler-library.txt
        // DEBUG: Both players use the generic enemy deck
        let ai_cards = read_deck(enemy_id);
        //let player_cards = read_deck(0); // True player deck
        let player_cards = read_deck(enemy_id);

        let ai = Rc::new(RefCell::new(Battler::new(String::from("AI"), 20, 20, 10, 10)));
        let player = Rc::new(RefCell::new(Battler::new(String::from("Player"), 20, 20, 10, 10)));


        let root = Node::new(None, ai, player, ai_cards, player_cards);
        GameTree { root }
    }

    pub fn populate(&mut self) {
        self.root.populate(false);
    }

    pub fn print(&mut self) {
        println!("Last Card Played | Health | Cards => children");
        self.root.print(0);
    }
}

pub fn read_deck(battler_id: i32) -> Vec<i32> {
    let mut cards = Vec::new();
    let file_data = fs::read_to_string("src/cards/battler-library.txt").expect("An error occurred whilst attempting to open the library.");
    for line in file_data.trim().split('\n') {
        if line.len() == 0 {
            continue;
        }
        else if line.starts_with("##") {
            continue;
        }
        else if line.starts_with(&battler_id.to_string()) {
            let line_data: Vec<&str> = line.split("::").collect();
            let card_data = line_data[4].split(',').map(|v| v.trim().parse::<i32>().unwrap());

            for data in card_data {
                //println!("{}", data);
                cards.push(data);
            }
        }

    }
    cards
}