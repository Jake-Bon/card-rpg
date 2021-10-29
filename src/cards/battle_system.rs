use crate::cards::game_structs::Card;
use crate::cards::game_structs::Battler;
use std::fs;
use std::collections::HashMap;

pub fn populate_card_map<'a>(fileData: &'a str)->HashMap<u32,Card<'a>>{
    let mut cards = HashMap::new();
    for line in (fileData[4..]).split('\n'){ //Remove first character, \u was messing with things
        println!("Currently trying to parse: {}", line);
        if line.len()==0{ //If empty line, skip
            continue;
        }else if line.starts_with("##"){ //If commented line, skip
            continue;
        }

        let line_data: Vec<&str> = line.split("::").collect();
        //Collect and parse data into new card
        cards.insert(line_data[0].parse::<u32>().unwrap(),Card::new(line_data[1],line_data[2],line_data[3].parse::<u32>().unwrap(),line_data[4].split(',').map(|v| v.parse::<u32>().unwrap()).collect(),line_data[5].split(',').map(|v| v.parse::<u32>().unwrap()).collect(),line_data[6]));
    }
    cards
}

pub fn parse_card <'a>(id: u32, val: u32, p1: &'a mut Battler, p2: &'a mut Battler){
    match id{ //p1 = first-person player
        0 => attack(val,p2),
        1 => defend(val,p1),
        2 => heal(val,p1),
        _ => unreachable_action(),
    }
}

fn attack <'a>(val: u32, target: &'a mut Battler){
    print!("{} took {} damage!\n",target.get_name(),(val as i32)-(target.get_defense() as i32));
    target.adjust_curr_health((target.get_defense() as i32)-(val as i32));
    target.set_defense(0);
}

fn defend <'a>(val: u32, target: &'a mut Battler){
    print!("{} defense value set to {}!\n",target.get_name(),val);
    target.set_defense(val);
}

fn heal <'a>(val: u32, target: &'a mut Battler){
    print!("{} healed {} hp!\n",target.get_name(),val);
    target.adjust_curr_health(val as i32);
}

fn unreachable_action(){
    print!("Hope you're happy.\n");
}

pub fn deal_cards(player: &mut Battler){
    print!("...\n");
    //TODO: Make player deck set-able. Need to have avail Cards
    //      May use another .txt for different classes.
}

pub struct BattleManager<'a> {
    file_data: String,
    pub card_map: HashMap<u32, Card<'a>>,
    pub turn: u32,
}

pub fn simulate_game(){
    print!("Reading in card library data...\n");
    let fileData = &fs::read_to_string("src/cards/card-library.txt").expect("An error occurred whilst attempting to open the library.");
    print!("File read successfully!\nBuilding card map...\n\n\n");
    let card_map = populate_card_map(fileData);
    print!("Card map successfully built! Cards ready to be used!\n\n\n");


    let mut a_player = Battler::new("Billy",20,20,10,10,7);
    print!("Battler object created.\n\n");


    let mut b_player = Battler::new("Bobby",25,25,7,7,7);
    print!("Battler object created.\n\n");

    parse_card(1,5,&mut b_player, &mut a_player); //DEF 5 @ p2
    parse_card(0,10,&mut a_player, &mut b_player); //ATTACK 10 @ p2
    parse_card(2,3,&mut b_player, &mut a_player); //HEAL 3 @ p2

    print!("\n\nP1: {}\n\n",a_player.to_string());
    print!("P2: {}\n\n",b_player.to_string());
}


pub fn demo_card_system(){ //File data read in then used to create library
    print!("Reading in card library data...\n");
    let fileData = &fs::read_to_string("src/cards/card-library.txt").expect("An error occurred whilst attempting to open the library.");
    print!("File read successfully!\nBuilding card map...\n\n\n");
    let card_map = populate_card_map(fileData);
    print!("Card map successfully built! Cards ready to be used!\n\n\n");

    let mut a_player = Battler::new("Billy",20,20,10,10,7);
    print!("Battler object created.\n\n");


    let mut b_player = Battler::new("Bobby",25,25,7,7,7);
    print!("Battler object created.\n\n");

    //print!("Adding player opponents.\n\n");
    //a_player.set_battler(&b_player);
    //b_player.set_battler(&a_player);

    print!("{}\n\n",a_player.to_string());

    print!("\n\n\n");
    for (id,card) in card_map.iter(){
        a_player.add_card_to_deck(*id);
        print!("{} added to deck!\n", card.get_name());
    }

    print!("\n\n\n");

    for _i in 0..a_player.get_full_hand_size(){ //How hand would be populated
        a_player.draw_card();
        print!("A card was transferred from DECK to HAND\n");
    }

    print!("\n\n\n");

    for _i in 0..a_player.get_curr_hand_size(){
        let c = a_player.select_hand(0);
        if !c.is_none(){
            print!("Card drawn from deck and played:\n");
            let card = card_map.get(&c.unwrap()).unwrap(); //unwrap select hand, unwrap hash
            print!("{}\nCard traits (used to play card):\n",card.to_string());
            card.play_card();
            a_player.hand_discard_card(0); //moves to discard deck in player struct
            print!("\n");
        }

    }

    print!("Remaining number of cards in deck: {}\nRemaining cards in hand: {}\n",a_player.get_deck_size(),a_player.get_curr_hand_size());

    print!("{}\n\n",a_player.to_string());
    print!("{}\n\n",b_player.to_string());

}
