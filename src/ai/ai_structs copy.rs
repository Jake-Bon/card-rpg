use crate::cards::game_structs::Battler;
use std::fs;

pub struct Node {
    utility: i32,
    ai: Battler,
    player: Battler,
    last_played: Option<i32>,
    //ai_health: i32,
    //ai_mana: i32,
    //ai_armor: i32,
    //ai_status: Vec<i32>, // [0][1] = posion | [2][3] = mana | [4][5] = health regen
    ai_cards: Vec<i32>,
    //player_health: i32,
    //player_mana: i32,
    //player_armor: i32,
    //player_status: Vec<i32>, // [0][1] = posion | [2][3] = mana | [4][5] = health regen
    player_cards: Vec<i32>,
    children: Vec<Node>,
}

impl Node {
    /*
    pub fn new(last_played: Option<i32>, ai_health: i32, ai_mana: i32, ai_armor: i32, ai_status: Vec<i32>, ai_cards: Vec<i32>,
            player_health: i32, player_mana: i32, player_armor: i32, player_status: Vec<i32>, player_cards: Vec<i32>) -> Node {
        let utility = 0;
        let children: Vec<Node> = Vec::new();
        Node {utility, last_played, ai_health, ai_mana, ai_armor, ai_status, ai_cards,
            player_health, player_mana, player_armor, player_status, player_cards,
            children}
    }
    */

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
            let mut curr_cards = self.ai_cards.clone();
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
                self.children.push(Node::new(Some(card_to_play), self.ai_health-5, self.ai_mana, self.ai_armor, self.ai_status.clone(), cards_to_pass_on.clone(), 
                                    self.player_health, self.player_mana, self.player_armor, self.player_status.clone(), self.player_cards.clone()));
            }
        }
        // Actions to take when it is Player's turn to play
        else {
            let mut curr_cards = self.player_cards.clone();
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
                let next_ai_health = self.ai_health;
                let next_ai_mana = self.ai_mana;
                let next_ai_status = self.ai_status.clone();
                let next_player_health = self.player_health;
                let next_player_mana = self.player_mana;
                let next_player_status = self.player_status.clone();

                // Alter new game state based on card played
                match card_to_play {
                    0 => {},//Shroud
                    1 => {},//Lisrae
                    2 => {},//TPS
                    3 => {},//Whirlpool: damage for 1 for 2 turns
                    4 => {},//GTP: regen 3hp for 3 turns
                    5 => {},//HHG
                    6 => {},//Photosynthesis
                    7 => {},//Last Shot
                    8 => {},//Demon's Posion
                    9 => {},//Barrel Protection
                    10 => {},//Ghastly cannonball
                    11 => {},//Vcard
                    12 => {},//Kitty does drugs
                    13 => {},//Straw Hat
                    14 => {},//Sparkle! Sparkle!
                    15 => {},//Time to rock
                    16 => {},//High Noon
                    17 => {},//Swab the deck
                    18 => {},//Cutlass
                    19 => {},//Fishing Minigame
                    20 => {},//Pest Problem
                    21 => {},//Rat
                    22 => {},//Mermaid's Tempation
                    23 => {},//Deadly Blade
                    24 => {},//Melonball
                    _ => panic!("Card id {} is not recognized by GameTree", card_to_play),
                }
                self.children.push(Node::new(Some(card_to_play), next_ai_health-5, next_ai_mana, self.ai_armor, next_ai_status, self.ai_cards.clone(),
                                            next_player_health, next_player_mana, self.player_armor, next_player_status, cards_to_pass_on.clone()));
            }
        }
        // Recursively populate the newly generated children of the current node
        for child in &mut self.children {
            child.populate(!ai_turn);
        }
    }

    // Checks if the current node is in a game terminating state
    pub fn stateIsTerminating(&mut self) -> bool {
        if self.ai_health <= 0 || self.player_health <= 0 {
            return true;
        }
        return false;
    }

    pub fn print(&mut self, tab: u32) {
        for i in 0..tab {
            print!("\t");
        }
        if self.last_played.is_none() {
            print!("root|{}|{}=>{} \n", self.ai_health, self.ai_cards.len(), self.children.len());
        }
        else {
            print!("{}|{}|{}=>{} \n", self.last_played.unwrap(),self.ai_cards.len(), self.ai_health, self.children.len());
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
        let ai_cards = read_deck(enemy_id);
        //let player_cards = read_deck(0);
        let player_cards = read_deck(enemy_id);
        let ai_status = Vec::new();
        let player_status = Vec::new();
        let root = Node::new(None, ai_battler.get_full_health(), ai_battler.get_full_energy(), 0, ai_status, ai_cards,
                            pl_battler.get_full_health(), pl_battler.get_full_energy(), 0, player_status, player_cards);
        GameTree { root }
    }

    pub fn populate(&mut self) {
        self.root.populate(false);
    }

    pub fn print(&mut self) {
        println!("Last Card Played | Cards Left | Ai Health => Children");
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