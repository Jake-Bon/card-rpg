extern crate rand;
use rand::thread_rng;
use rand::prelude::SliceRandom;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::iter::Zip;

use crate::cards::battle_enums::BattleOutcome;


#[derive(Clone)]
pub struct Card{
    name: String,
    desc: String,
    cost: u32,
    action_list: Vec<i32>, //Actions represented by ID, ex: 0:Attack, 1:Defend, etc...
    value_list: Vec<i32>, //Value of Actions, ie 1 could be 1 attack, 1 defend, etc...
    img_file: String,
}

impl Card{
    pub fn new(name: String, desc: String, cost: u32,action_list: Vec<i32>,value_list: Vec<i32>, img_file: String)->Card{
        Card{
            name,desc,cost,action_list,value_list,img_file,
        }
    }

    pub fn play_card(&self){
        for (act,val) in self.action_list.iter().zip(self.value_list.iter()){
            //calls to PLAY methods will go here
            print!("{}, {}\n",act,val);
        }
    }

    pub fn get_name(&self)->&str{
        &self.name
    }

    pub fn get_description(&self)->&str{
        &self.desc
    }

    pub fn get_cost(&self)->u32{
        self.cost
    }

    pub fn get_sprite_name(&self)->&str{
        &self.img_file.trim()
    }

    pub fn get_actions(&self)->Vec<i32>{
        self.action_list.clone()
    }

    pub fn set_values(&mut self,act: i32, val:i32){
        for i in 0..self.action_list.len(){
            self.value_list[i] = val;
        }
    }

    pub fn get_lists(&self)->Zip<std::slice::Iter<'_, i32>, std::slice::Iter<'_, i32>>{
        let a = &self.action_list;
        let v = &self.value_list;
        a.iter().zip(v.iter())
    }

    pub fn to_string(&self)->String{
        format!("{}: {} | {} energy. | Card Located @ {}",self.name,self.desc,self.cost,self.img_file)
    }
}

#[derive(Clone)]
pub struct Battler{
    name: String,
    full_health: i32,
    curr_health: i32,
    mult: i32, //damage multiplier (- integers considered fractions, ex -2 = 1/2 mult)
    def: i32,//defense
    mana_delta: i32,
    full_energy: i32,
    curr_energy: i32,
    hand_size: usize, //num of cards in Battler hand, may be removed
    hand: Vec<u32>, //Current held cards
    deck: Vec<u32>, //Deck to draw from - treat as queue
    discard: Vec<u32>, //Discarded deck
    draw_num: u32, // number of cards to draw
    poison: u32,
    energy_regen: Vec<i32>,
    health_regen: Vec<i32>,
}

impl Battler{ //HAND and DECK created as INTRINSIC VALUES
    pub fn new(name: String, full_health: i32, curr_health: i32, full_energy: i32, curr_energy: i32)-> Battler{
        let hand = Vec::new();
        let hand_size = 7 as usize;
        let deck = Vec::new();
        let discard = Vec::new();
        let draw_num = 0;
        let mult=1;
        let def = 0;
        let mana_delta = 3;
        let poison = 0;
        let energy_regen = Vec::new();
        let health_regen = Vec::new();
        Battler{name, full_health,curr_health,mult,def,mana_delta,full_energy,curr_energy,hand_size,hand,deck,discard,draw_num,poison,energy_regen,health_regen}
    }

    pub fn shuffle_deck(&mut self){
        (&mut self.deck).shuffle(&mut thread_rng());
    }

    pub fn get_full_health(&self)->i32{
        self.full_health
    }

    pub fn get_curr_health(&self)->i32{
        self.curr_health
    }

    pub fn get_full_energy(&self)->i32{
        self.full_energy
    }

    pub fn get_curr_energy(&self)->i32{
        self.curr_energy
    }

    pub fn get_energy_percent(&self)->f32{ //&&&&&&&&&&&&&&&&
        self.curr_energy as f32/self.full_energy as f32
    }

    pub fn get_full_hand_size(&self)->usize{
        self.hand_size
    }

    pub fn get_defense(&self)->i32{
        self.def
    }

    pub fn get_health_percent(&self)->f32{
        self.curr_health as f32/self.full_health as f32
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_mult(&mut self)->f32{
        let m = self.mult;
        if m<0{
            (1 as f32)/(m.abs() as f32)
        }else{
            m as f32
        }
    }

    pub fn set_mult(&mut self, m: i32){
        self.mult = m;
    }

    pub fn set_deck(&mut self, new_deck: Vec<u32>){
        self.deck = new_deck;
    }

    pub fn restore_deck(&mut self){
        self.deck = self.discard.clone();
        self.discard.clear();
        (&mut self.deck).shuffle(&mut thread_rng());

    }

    pub fn set_defense(&mut self,d:i32){
        self.def = d;
    }

    pub fn add_defense(&mut self,d:i32){
        self.def = self.def + d;
    }

    pub fn set_full_health(&mut self,h: i32){
        self.full_health = h;
    }

    pub fn set_curr_health(&mut self,h:i32){
        self.curr_health = h;
    }

    pub fn adjust_curr_health(&mut self,h:i32){
        self.curr_health = self.curr_health+h;
        if self.curr_health>self.full_health{
            self.curr_health = self.full_health;
        }
        if self.curr_health<0{
            self.curr_health = 0 as i32;
        }
    }

    pub fn adjust_curr_energy(&mut self,h:i32){
        self.curr_energy = self.curr_energy+h;
        if self.curr_energy>self.full_energy{
            self.curr_energy = self.full_energy;
        }
    }

    pub fn set_full_energy(&mut self,h:i32){
        self.full_energy = h;
    }

    pub fn set_curr_energy(&mut self,h:i32){
        self.curr_energy = h;
    }

    pub fn set_hand_size(&mut self, s:usize){
        self.hand_size = s;
    }

    pub fn add_card_to_hand(&mut self,c: u32){ //add card to ACTIVE hand
        self.hand.push(c);
    }

    pub fn add_card_to_deck(&mut self,c: u32){ //add card to deck to DRAW from
        self.deck.push(c);
    }

    pub fn add_card_to_discard(&mut self,c:u32){ //add card to DISCARD PILE
        self.discard.push(c);
    }

    pub fn get_deck_size(&self)->usize{
        self.deck.len()
    }

    pub fn get_deck(&self)->Vec<u32>{//use for ai purposes only
        self.deck.clone()
    }

    pub fn get_hand(&self)->Vec<u32> {// use for ai purposes only
        self.hand.clone()
    }

    pub fn get_curr_hand_size(&self)->usize{
        self.hand.len()
    }

    pub fn get_discard_size(&self)->usize{
        self.discard.len()
    }

    pub fn deck_del_card(&mut self){
        if self.deck.len()>0{
            self.deck.remove(0);
        }
    }

    pub fn hand_del_card(&mut self,index:usize){
        if self.hand.len()>0{
            self.hand.remove(index);
        }
    }

    pub fn deck_del_card_specific(&mut self, index: usize) {
        if self.hand.len() > 0 {
            self.deck.remove(index);
        }
    }

    pub fn hand_discard_card(&mut self,index:usize){ //Hand => Discard
        if self.hand.len()>0{
            self.add_card_to_discard(self.hand[index]);
            self.hand.remove(index);
        }
    }

    // gets the card from the top of the deck
    pub fn get_deck_card(&self)->Option<u32>{
        if self.deck.len()>0{
            Some(self.deck[0])
        }else{
            None
        }
    }
    
    // gets the card from the top of the discard pile
    pub fn get_discard_card(&self)->Option<u32>{
        if self.discard.len()>0{
            Some(self.discard[self.discard.len() - 1])
        }else{
            None
        }
    }

    // add cards to be drawn
    // self.draw_num is decremented in draw card
    pub fn add_draw_num(&mut self, change: u32){
        self.draw_num = self.draw_num + change;
        println!("add_draw_num is now {}", self.draw_num);
    }

    pub fn set_draw_num(&mut self, new_num: u32){
        self.draw_num = new_num;
    }

    pub fn get_draw_num(&self)->u32 {
        self.draw_num
    }

    pub fn draw_card(&mut self,is_safe: bool){ //Deck => Hand
        if self.deck.len()>0 && (is_safe||self.hand.len()<self.hand_size){
            self.add_card_to_hand(self.deck[0]);
            self.deck_del_card();
        }
        // this is to skip the draw animations at the start of battle
        if self.draw_num > 0 {
            self.draw_num = self.draw_num - 1;
        }

    }

    pub fn select_hand(&self,index:usize)->Option<u32>{
        if self.hand.len()>0 && index < self.hand.len(){
            Some(self.hand[index])
        }else{
            None
        }

    }

    pub fn remove_sel_card(&mut self, id: u32){
        for i in 0..self.deck.len(){
            if self.deck[i]==id{
                self.deck.remove(i);
                return;
            }
        }
        for i in 0..self.discard.len(){
            if self.discard[i]==id{
                self.discard.remove(i);
                return;
            }
        }
        for i in 0..self.hand.len(){
            if self.hand[i]==id{
                self.hand.remove(i);
                return;
            }
        }
    }

    pub fn remove_all_sel_card(&mut self, id: u32){
        let mut copy: Vec<u32> = Vec::new();
        for i in 0..self.deck.len(){
            if self.deck[i]!=id{
                copy.push(self.deck[i]);
            }
        }
        self.deck = copy;

        let mut copy: Vec<u32> = Vec::new();
        for i in 0..self.discard.len(){
            if self.discard[i]!=id{
                copy.push(self.discard[i]);
            }
        }
        self.discard = copy;

        let mut copy: Vec<u32> = Vec::new();
        for i in 0..self.hand.len(){
            if self.hand[i]!=id{
                copy.push(self.hand[i]);
            }
        }
        self.hand = copy;
    }

    //EFFECTS

    pub fn add_poison(&mut self,amt: u32){
        self.poison = self.poison+amt;
    }

    pub fn get_poison(&self)->u32{
        self.poison
    }

    pub fn clear_poison(&mut self){
        self.poison = 0;
    }

    pub fn add_energy_regen(&mut self, val:i32){ //CAN BE NEGATIVE!
        self.energy_regen.push(3 as i32);//turns
        self.energy_regen.push(val);
    }

    pub fn get_energy_regen_duration(& self)->i32{
        let mut rslt = 0 as i32;
        if self.energy_regen.len()==0{
            return rslt;
        }
        for i in 0..self.energy_regen.len()/2{ //First get amount of regen
            rslt = if self.energy_regen[i*2]>rslt{
                self.energy_regen[i*2]
            }else{
                rslt
            };
        }
        rslt
    }

    pub fn get_energy_regen(& self)->i32{
        let mut rslt = 0 as i32;
        if self.energy_regen.len()==0{
            return rslt;
        }
        for i in 0..self.energy_regen.len()/2{ //First get amount of regen
            rslt += if self.energy_regen[i*2]>0{
                self.energy_regen[i*2+1]
            }else{
                0
            };
        }
        rslt
    }

    pub fn add_health_regen(&mut self, val:i32){ //CAN BE NEGATIVE!
        self.health_regen.push(3 as i32);//turns
        self.health_regen.push(val);
    }

    pub fn get_health_regen_duration(& self)->i32{
        let mut rslt = 0 as i32;
        if self.health_regen.len()==0{
            return rslt;
        }
        for i in 0..self.health_regen.len()/2{ //First get amount of regen
            rslt = if self.health_regen[i*2]>rslt{
                self.health_regen[i*2]
            }else{
                rslt
            };
        }
        rslt
    }

    pub fn get_health_regen(& self)->i32{ //for display purposes
        let mut rslt = 0 as i32;
        if self.health_regen.len()==0{
            return rslt;
        }
        for i in 0..self.health_regen.len()/2{
            rslt += if self.health_regen[i*2]>0{
                self.health_regen[i*2+1]
            }else{
                0
            };
        }
        rslt
    }

    pub fn dup_card(&mut self, id:u32){
        self.deck.push(id);
    }

    pub fn update_effects(&mut self){//apply and decrement all other effects. If 0, remove.
        if self.poison>0{
            self.curr_health = self.curr_health-self.poison as i32;
            if self.curr_health < 0 { self.curr_health = 0; }
            self.poison = self.poison - 1;
        }

        self.curr_energy = self.curr_energy+3 as i32;//base regen of energy

        // stop overflow via base regen
        if self.curr_energy>self.full_energy{
            self.curr_energy = self.full_energy;
        }

        if self.energy_regen.len()>0{
            for i in 0..(self.energy_regen.len())/2{ //Go through every value/turn pair
                self.curr_energy = self.curr_energy+self.energy_regen[i*2+1];
                if self.curr_energy>self.full_energy{
                    self.curr_energy = self.full_energy;
                }
                self.energy_regen[i*2] = self.energy_regen[i*2] - 1 as i32;
            }

            let mut new_vec: Vec<i32> = Vec::new();
            for i in 0..self.energy_regen.len()/2{ //then remove any exhausted regen effects
                if self.energy_regen[i*2]>0{
                    new_vec.push(self.energy_regen[i*2]);
                    new_vec.push(self.energy_regen[i*2+1]);
                }
            }
            self.energy_regen = new_vec;
        }
        if self.health_regen.len()>0{
            for i in 0..(self.health_regen.len())/2{ //Go through every value/turn pair
                self.curr_health = self.curr_health+self.health_regen[i*2+1];
                if self.curr_health>self.full_health{
                    self.curr_health = self.full_health;
                }
                self.health_regen[i*2] = self.health_regen[i*2] - 1 as i32;
            }

            let mut new_vec: Vec<i32> = Vec::new(); //Then remove any exhausted health effects
            for i in 0..self.health_regen.len()/2{
                if self.health_regen[i*2]>0{
                    new_vec.push(self.health_regen[i*2]);
                    new_vec.push(self.health_regen[i*2+1]);
                }
            }
            self.health_regen = new_vec;
        }
    }

    pub fn to_string(&self)->String{
        format!("Name: {}\nHealth: {}/{}\nEnergy: {}/{}\nHand Size: {}/{}\nDeck Size: {}\nDiscard Size: {}\n",self.name,self.curr_health,self.full_health,self.curr_energy,self.full_energy,self.hand.len(),self.hand_size,self.deck.len(),self.discard.len())
    }
}

pub fn populate_card_map()->HashMap<u32,Card>{
    let mut cards = HashMap::new();
    let file_data = fs::read_to_string("src/cards/card-library.txt").expect("An error occurred whilst attempting to open the library.");
    for line in file_data.trim().split('\n'){ //Remove first character, \u was messing with things
        //println!("Currently trying to parse: {}", line);
        if line.len()==0{ //If empty line, skip
            continue;
        }else if line.starts_with("##"){ //If commented line, skip
            continue;
        }

        let line_data: Vec<&str> = line.split("::").collect();
        //Collect and parse data into new card
        cards.insert(line_data[0].parse::<u32>().unwrap(),Card::new(line_data[1].to_string(),line_data[2].to_string(),line_data[3].parse::<u32>().unwrap(),line_data[4].split(',').map(|v| v.parse::<i32>().unwrap()).collect(),line_data[5].split(',').map(|v| v.parse::<i32>().unwrap()).collect(),line_data[6].to_string()));
    }
    cards
}

#[derive(Clone)]
pub struct BattleStatus{
    p1: Rc<RefCell<Battler>>,
    p2: Rc<RefCell<Battler>>,
    turn: u32,
    card_map: HashMap<u32,Card>,
}

impl BattleStatus{
    pub fn new(p1: Rc<RefCell<Battler>>, p2: Rc<RefCell<Battler>>)->BattleStatus{
        let turn =0;
        let card_map = populate_card_map();
        BattleStatus{p1,p2,turn,card_map}
    }

    pub fn turner(&mut self){
        self.turn=(self.turn+1)%2;
    }
    pub fn get_turn(&self)->u32{
        self.turn
    }

    pub fn get_p1(&mut self)->Rc<RefCell<Battler>>{
        Rc::clone(&self.p1)
    }

    pub fn get_p2(&mut self)->Rc<RefCell<Battler>>{
        Rc::clone(&self.p2)
    }

    pub fn get_active_player(&mut self)->Rc<RefCell<Battler>>{
        if self.turn==0{
            Rc::clone(&self.p1)
        }else{
            Rc::clone(&self.p2)
        }
    }

    pub fn get_inactive_player(&mut self)->Rc<RefCell<Battler>>{
        if self.turn==0{
            Rc::clone(&self.p2)
        }else{
            Rc::clone(&self.p1)
        }
    }

    pub fn get_card(&self, id: u32)->Card{
        self.card_map.get(&id).unwrap().clone()
    }

    pub fn get_card_map_size(&self)->usize{
        self.card_map.len()
    }

    pub fn update_player_effects(&self){ //WILL NOT USE FOR GAME. USE battler.update_effects() instead
        self.p1.borrow_mut().update_effects();
        self.p2.borrow_mut().update_effects();
    }

    pub fn check_victory(&self) -> BattleOutcome {

        let p1_health = self.p1.borrow().get_curr_health();
        let p2_health = self.p2.borrow().get_curr_health();

        if p1_health > 0 && p2_health <= 0 {
            return BattleOutcome::VictoryP1;
        }
        else if p1_health <= 0 && p2_health > 0 {
            return BattleOutcome::VictoryP2;
        }
        else if p1_health <= 0 && p2_health <= 0 {
            return BattleOutcome::Tie;
        }
        // else if both players have health above 0, battle isn't over yet
        return BattleOutcome::Undetermined;

    }

}
