//extern crate card_experiments;

#[derive(Clone)]
pub enum CardType{
    Null, Heal, Defend, Attack,
}

pub struct Card<'a>{
    name: &'a str,
    desc: &'a str,
    cost: i32,
    action: CardType,
    value: i32,
}

impl Card<'_>{
    pub fn new<'a>(name: &'a str, desc: &'a str, cost: i32, action: CardType, value: i32)->Card<'a>{
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

    pub fn getCost(&self)->i32{
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

    pub fn getValue(&self)->i32{
        self.value
    }

    pub fn toString(&self)->String{

        format!("{} {} {} {} {}",self.name,self.desc,self.cost,self.typeToString(),self.value)
    }
}
