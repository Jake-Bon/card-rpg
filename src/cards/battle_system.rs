use crate::cards::game_structs::*;
use std::fs;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub fn populate_card_map()->HashMap<u32,Card>{
    let mut cards = HashMap::new();
    let file_data = fs::read_to_string("src/cards/card-library.txt").expect("An error occurred whilst attempting to open the library.");
    for line in (file_data[4..]).split('\n'){ //Remove first character, \u was messing with things
        //println!("Currently trying to parse: {}", line);
        if line.len()==0{ //If empty line, skip
            continue;
        }else if line.starts_with("##"){ //If commented line, skip
            continue;
        }

        let line_data: Vec<&str> = line.split("::").collect();
        //Collect and parse data into new card
        cards.insert(line_data[0].parse::<u32>().unwrap(),Card::new(line_data[1].to_string(),line_data[2].to_string(),line_data[3].parse::<u32>().unwrap(),line_data[4].split(',').map(|v| v.parse::<u32>().unwrap()).collect(),line_data[5].split(',').map(|v| v.parse::<i32>().unwrap()).collect(),line_data[6].to_string()));
    }
    cards
}

pub fn populate_battler_map ()->HashMap<u32,Battler>{
    let mut battlers = HashMap::new();
    let file_data = fs::read_to_string("src/cards/battler-library.txt").expect("An error occurred whilst attempting to open the library.");
    for line in (file_data).split('\n'){ //Remove first character, \u was messing with things
        //println!("Currently trying to parse: {}", line);
        if line.len()==0{ //If empty line, skip
            continue;
        }else if line.starts_with("##"){ //If commented line, skip
            continue;
        }

        let line_data: Vec<&str> = line.split("::").collect();
        //Collect and parse data into new card
        let mut health = line_data[2].parse::<i32>().unwrap();
        let mut energy = line_data[3].parse::<i32>().unwrap();
        let mut new_battler = Battler::new(line_data[1].to_string(),health,health,energy,energy);
        new_battler.set_deck(line_data[4].split(',').map(|v| v.parse::<u32>().unwrap()).collect());
        battlers.insert(line_data[0].parse::<u32>().unwrap(),new_battler);
    }
    battlers
}

pub fn play_card(curr_status: Rc<RefCell<BattleStatus>>){
    //TODO: Play any card player gives. For loop for multiple effects on card.
}

pub fn parse_card (id: u32, val: i32, stat: Rc<RefCell<BattleStatus>>){
    let mut stat = stat.borrow_mut();

    let mut p1 = stat.get_p1();
    let mut p2 = stat.get_p2();
    let mut chosen = if stat.get_turn()==0 {p2}else{p1};

    match id{ //p1 = first-person player
        0 => attack(val,chosen),
        1 => defend(val,chosen),
        2 => heal(val,chosen),
        _ => unreachable_action(),
    }
}

//TODO - According to turn apply attack, defend, and heal to correct player. Check if these work properly.
//TODO - Get CARD from player deck and get card TYPE and VALUE

fn attack (val: i32, target: Rc<RefCell<Battler>>){
    //print!("ATTACKER\n");
    let mut target = target.borrow_mut();

    let def = target.get_defense();
    //print!("{} took {} damage!\n",target.get_name(),val-def);
    target.adjust_curr_health(def-val);
    target.set_defense(0);
    print!("{}\n",target.to_string());
}

fn defend (val: i32, target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();

    print!("{} defense value set to {}!\n",target.get_name(),val);
    target.set_defense(val);
}

fn heal (val: i32, target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();

    //print!("{} healed {} hp!\n",target.get_name(),val);
    target.adjust_curr_health(val);
    print!("{}\n",target.to_string());
}

fn unreachable_action(){
    print!("Hope you're happy.\n");
}

pub fn deal_cards(player: &mut Battler){
    print!("...\n");
    //TODO: Make player deck set-able. Need to have avail Cards
    //      May use another .txt for different classes.
}

pub fn test_libraries(){
    let card_map = populate_card_map();
    let battler_map = populate_battler_map();

    let p1 = battler_map.get(&0).unwrap();
    let cardID = p1.get_deck_card().unwrap();
    let card1 = card_map.get(&cardID).unwrap();
    print!("{}\n",card1.to_string());
}

/*
pub fn simulate_game<'a>(mut p1: Battler, mut p2: Battler){
    print!("Reading in card library data...\n");
    let fileData = &fs::read_to_string("src/cards/card-library.txt").expect("An error occurred whilst attempting to open the library.");
    print!("File read successfully!\nBuilding card map...\n\n\n");
    let card_map = populate_card_map(fileData);
    print!("Card map successfully built! Cards ready to be used!\n\n\n");


    let mut a_player = Rc::new(RefCell::new(p1));
    let mut b_player = Rc::new(RefCell::new(p2));

    let mut battle_ref = Rc::new(RefCell::new(BattleStatus::new(a_player,b_player)));

    parse_card(1,4,Rc::clone(&battle_ref));
    battle_ref.borrow_mut().turner();
    parse_card(0,10,Rc::clone(&battle_ref));
    battle_ref.borrow_mut().turner();
    parse_card(2,2,Rc::clone(&battle_ref));
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

}*/
