//extern crate card_experiments;
use std::rc::Rc;
use std::cell::RefCell;

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
        &self.img_file
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
    effects: Rc<RefCell<Vec<Vec<i32>>>>, //[[effect1 type,effect1 value,effect1 duration],etc]
}

impl Battler{ //HAND and DECK created as INTRINSIC VALUES
    pub fn new(name: String, full_health: i32, curr_health: i32, full_energy: i32, curr_energy: i32)-> Battler{
        let hand = Vec::new();
        let hand_size = 7 as usize;
        let deck = Vec::new();
        let discard = Vec::new();
        let mult=1;
        let def = 0;
        let mana_delta = 3;
        let effects_type = Vec::new();
        let effects_val = Vec::new();
        let effects_duration = Vec::new();
        let effects = Rc::new(RefCell::new(vec![effects_type,effects_val,effects_duration]));
        Battler{name, full_health,curr_health,mult,def,mana_delta,full_energy,curr_energy,hand_size,hand,deck,discard,effects}
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

    pub fn get_full_hand_size(&self)->usize{
        self.hand_size
    }

    pub fn get_defense(&self)->i32{
        self.def
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_mult(&mut self, m: i32){
        self.mult = m;
    }

    pub fn set_deck(&mut self, new_deck: Vec<u32>){
        self.deck = new_deck;
    }

    pub fn set_defense(&mut self,d:i32){
        self.def = d;
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
        if self.curr_energy>self.curr_energy{
            self.curr_energy = self.curr_energy;
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

    pub fn add_effect(&mut self, etype:i32,eval:i32,edur:i32){
        self.effects.borrow_mut().push(vec![etype,eval,edur]);
    }

    pub fn remove_effect(&mut self, pos:usize){
        self.effects.borrow_mut().remove(pos);
    }

    pub fn get_effects(&mut self) -> Rc<RefCell<Vec<Vec<i32>>>>{
        Rc::clone(&self.effects)
    }

    pub fn get_deck_size(&self)->usize{
        self.deck.len()
    }

    pub fn get_curr_hand_size(&self)->usize{
        self.hand.len()
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

    pub fn hand_discard_card(&mut self,index:usize){ //Hand => Discard
        if self.hand.len()>0{
            self.add_card_to_discard(self.hand[index]);
            self.hand.remove(index);
        }
    }

    pub fn get_deck_card(&self)->Option<u32>{
        if self.deck.len()>0{
            Some(self.deck[0])
        }else{
            None
        }
    }

    pub fn draw_card(&mut self){ //Deck => Hand
        if self.deck.len()>0 && self.hand.len()<self.hand_size{
            self.add_card_to_hand(self.deck[0]);
            self.deck_del_card();
        }
    }

    pub fn select_hand(&self,index:usize)->Option<u32>{
        if self.hand.len()>0{
            Some(self.hand[index])
        }else{
            None
        }

    }

    pub fn to_string(&self)->String{
        format!("Name: {}\nHealth: {}/{}\nEnergy: {}/{}\nHand Size: {}/{}",self.name,self.curr_health,self.full_health,self.curr_energy,self.full_energy,self.hand.len(),self.hand_size)
    }
}

pub struct BattleStatus{
    p1: Rc<RefCell<Battler>>,
    p2: Rc<RefCell<Battler>>,
    turn: u32,
}

impl BattleStatus{
    pub fn new(p1: Rc<RefCell<Battler>>, p2: Rc<RefCell<Battler>>)->BattleStatus{
        let turn =0;
        BattleStatus{p1,p2,turn}
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
}
