//extern crate card_experiments;

#[derive(Clone)]
pub struct Card <'a>{
    name: &'a str,
    desc: &'a str,
    cost: u32,
    action_list: Vec<u32>, //Actions represented by ID, ex: 0:Attack, 1:Defend, etc...
    value_list: Vec<u32>, //Value of Actions, ie 1 could be 1 attack, 1 defend, etc...
    img_file: &'a str,
}

impl <'a> Card <'a>{
    pub fn new(name: &'a str, desc: &'a str, cost: u32,action_list: Vec<u32>,value_list: Vec<u32>, img_file: &'a str)->Card<'a>{
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
        self.name
    }

    pub fn get_description(&self)->&str{
        self.desc
    }

    pub fn get_cost(&self)->u32{
        self.cost
    }

    pub fn get_sprite_name(&self)->&str{
        self.img_file
    }

    pub fn to_string(&self)->String{
        format!("{}: {} | {} energy. | Card Located @ {}",self.name,self.desc,self.cost,self.img_file)
    }
}

pub struct Battler <'a>{
    name: &'a str,
    full_health: u32,
    curr_health: u32,
    def: u32,
    full_energy: u32,
    curr_energy: u32,
    hand_size: usize, //num of cards in Battler hand, may be removed
    hand: Vec<u32>, //Current held cards
    deck: Vec<u32>, //Deck to draw from - treat as queue
    discard: Vec<u32>, //Discarded deck
}

impl <'a> Battler <'a>{ //HAND and DECK created as INTRINSIC VALUES
    pub fn new(name: &'a str, full_health: u32, curr_health: u32, full_energy: u32, curr_energy: u32, hand_size: usize)-> Battler<'a>{
        let hand = Vec::new();
        let deck = Vec::new();
        let discard = Vec::new();
        let def = 0;
        Battler{name, full_health,curr_health,def,full_energy,curr_energy,hand_size,hand,deck,discard}
    }

    pub fn get_full_health(&self)->u32{
        self.full_health
    }

    pub fn get_curr_health(&self)->u32{
        self.curr_health
    }

    pub fn get_full_energy(&self)->u32{
        self.full_energy
    }

    pub fn get_curr_energy(&self)->u32{
        self.curr_energy
    }

    pub fn get_full_hand_size(&self)->usize{
        self.hand_size
    }

    pub fn get_defense(&self)->u32{
        self.def
    }

    pub fn get_name(&self) -> &str{
        self.name
    }

    pub fn set_defense(&mut self,d:u32){
        self.def = d;
    }

    pub fn set_full_health(&mut self,h: u32){
        self.full_health = h;
    }

    pub fn set_curr_health(&mut self,h:u32){
        self.curr_health = h;
    }

    pub fn adjust_curr_health(&mut self,h:i32){
        self.curr_health = ((self.curr_health as i32)+(h as i32)) as u32;
        if self.curr_health>self.full_health{
            self.curr_health = self.full_health;
        }
    }

    pub fn adjust_curr_energy(&mut self,h:i32){
        self.curr_energy = ((self.curr_energy as i32)+(h as i32)) as u32;
        if self.curr_energy>self.curr_energy{
            self.curr_energy = self.curr_energy;
        }
    }

    pub fn set_full_energy(&mut self,h:u32){
        self.full_energy = h;
    }

    pub fn set_curr_energy(&mut self,h:u32){
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
