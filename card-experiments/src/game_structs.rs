//extern crate card_experiments;

#[derive(Clone)]
pub enum CardType{
    Null, Heal, Defend, Attack,
}

#[derive(Clone)]
pub struct Card <'a>{
    name: &'a str,
    desc: &'a str,
    cost: u32,
    action: CardType,
    value: u32,
}

impl <'a> Card <'a>{
    pub fn new(name: &'a str, desc: &'a str, cost: u32, action: CardType, value: u32)->Card<'a>{
        Card{
            name,desc,cost,action,value,
        }
    }

    pub fn getName(&self)->&str{
        self.name
    }

    pub fn getDescription(&self)->&str{
        self.desc
    }

    pub fn getCost(&self)->u32{
        self.cost
    }

    fn typeToString(&self)->String{
        match self.action{
            CardType::Heal => "Heal".to_string(),
            CardType::Attack => "Attack".to_string(),
            CardType::Defend => "Defend".to_string(),
            CardType::Null => "Null".to_string(),
        }
    }

    pub fn getType(&self)->String{
        self.typeToString()
    }

    pub fn getValue(&self)->u32{
        self.value
    }

    pub fn toString(&self)->String{
        format!("{} {} {} {} {}",self.name,self.desc,self.cost,self.typeToString(),self.value)
    }
}

pub struct Player <'a>{
    full_health: u32, //may be redundant
    curr_health: u32,
    full_energy: u32,
    curr_energy: u32,
    hand_size: u32, //num of cards in player hand, may be removed
    hand: Vec<Card<'a>>,
    deck: Vec<Card<'a>>, //treat as queue
}

impl <'a> Player <'a>{ //HAND and DECK created as INTRINSIC VALUES
    pub fn new(full_health: u32, curr_health: u32, full_energy: u32, curr_energy: u32, hand_size: u32)-> Player<'a>{
        let hand = Vec::new();
        let deck = Vec::new();
        Player{full_health,curr_health,full_energy,curr_energy,hand_size,hand,deck}
    }

    pub fn getFullHealth(&self)->u32{
        self.full_health
    }

    pub fn getCurrHealth(&self)->u32{
        self.curr_health
    }

    pub fn getFullEnergy(&self)->u32{
        self.full_energy
    }

    pub fn getCurrEnergy(&self)->u32{
        self.curr_energy
    }

    pub fn getHandSize(&self)->u32{
        self.hand_size
    }

    //pub fn getHand(&self)->&Vec<Card>{ Should we return the entire vec/vecdeque? Or abstract away?
    //    &self.hand
    //}

    //pub fn getDeck(&self)->&VecDeque<Card>{
    //    &self.deck
    //}

    pub fn setFullHealth(&mut self,h: u32){
        self.full_health = h;
    }

    pub fn setCurrHealth(&mut self,h:u32){
        self.curr_health = h;
    }

    pub fn setFullEnergy(&mut self,h:u32){
        self.full_energy = h;
    }

    pub fn setCurrEnergy(&mut self,h:u32){
        self.curr_energy = h;
    }

    pub fn setHandSize(&mut self, s:u32){
        self.hand_size = s;
    }

    pub fn addCardToHand(&mut self,c: Card<'a>){
        self.hand.push(c);
    }

    pub fn addCardToDeck(&mut self,c: Card<'a>){
        self.deck.push(c);
    }

    pub fn deckSize(&self)->usize{
        self.deck.len()
    }

    pub fn handSize(&self)->usize{
        self.hand.len()
    }

    pub fn deckDelCard(&mut self){
        self.deck.remove(0);
    }

    pub fn handDelCard(&mut self,index:usize){
        self.hand.remove(index);
    }

    pub fn drawCard(&self)->Card<'a>{
        self.deck[0].clone()
    }

    pub fn selectHand(&self,index:usize)->Card<'a>{
            self.hand[index].clone()
    }

    pub fn toString(&self)->String{
        format!("{} {} {} {} {}",self.full_health,self.curr_health,self.full_energy,self.curr_energy,self.hand_size)
    }
}
