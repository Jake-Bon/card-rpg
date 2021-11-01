use crate::cards::game_structs::*;
use std::fs;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::iter::Zip;

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
        new_battler.set_deck(line_data[4].split(',').map(|v| v.trim().parse::<u32>().unwrap()).collect());
        battlers.insert(line_data[0].parse::<u32>().unwrap(),new_battler);
    }
    battlers
}

pub fn play_card(stat: Rc<RefCell<BattleStatus>>,card:Card){
    //TODO: Play any card player gives. For loop for multiple effects on card.
    let types_iter = card.get_lists();
    let mut is_attack = false;
    for (action,value) in types_iter{
        if(*action==0){
            is_attack = true;
        }
        parse_card(*action,*value,Rc::clone(&stat));
    }
    if is_attack{
        stat.borrow_mut().get_inactive_player().borrow_mut().set_mult(1 as i32);
    }
}

pub fn parse_card (id: i32, val: i32, stat: Rc<RefCell<BattleStatus>>){
    let mut stat = stat.borrow_mut();

    let mut p1 = stat.get_p1();
    let mut p2 = stat.get_p2();


    match id as u32{ //p1 = first-person player
        0 => attack(val, stat.get_inactive_player()),
        1 => defend(val, stat.get_active_player()),
        2 => heal(val, stat.get_active_player()),
        3 => mult_next_dmg(val, stat.get_inactive_player()), //multiplier placed on opponent
        4 => poison(val, stat.get_inactive_player()),
        5 => cure(stat.get_active_player()),
        6 => change_mana_regen(val,stat.get_active_player()),//player bumps up own regen
        7 => change_mana_regen(val,stat.get_inactive_player()),//player bumps down opponent regen
        8 => health_regen(val,stat.get_active_player()),
        _ => unreachable_action(),
    }
}

//TODO - According to turn apply attack, defend, and heal to correct player. Check if these work properly.
//TODO - Get CARD from player deck and get card TYPE and VALUE

fn attack (val: i32, target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();
    let def = target.get_defense();
    let mult = target.get_mult();

    target.adjust_curr_health(def-((val as f64*mult) as i32));
    target.set_defense(0);
}

fn defend (val: i32, target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();
    target.set_defense(val);
}

fn heal (val: i32, target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();
    target.adjust_curr_health(val);
}

fn mult_next_dmg(val:i32, target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();

    //print!("{} healed {} hp!\n",target.get_name(),val);
    target.set_mult(val);
}

fn poison(val:i32,target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();
    target.add_poison(val as u32);
}

fn cure(target:Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();
    target.clear_poison();
}

fn change_mana_regen(val:i32, target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();
    target.add_energy_regen(val);
}

fn health_regen(val: i32, target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();
    target.add_health_regen(val);
}

fn unreachable_action(){
    print!("Hope you're happy.\n");
}

pub fn deal_cards(player: &mut Battler){
    print!("...\n");
    //TODO: Make player deck set-able. Need to have avail Cards
    //      May use another .txt for different classes.
}

/*
pub fn test_libraries(){
    let card_map = populate_card_map();
    let battler_map = populate_battler_map();

    let _p1 = battler_map.get(&0).unwrap().clone(); //Must UNWRAP AND CLONE players from map for battle use
    let _p2 = battler_map.get(&1).unwrap().clone();
    let cardID = _p1.get_deck_card().unwrap(); //Must UNWRAP given card ID when drawing from deck
    let card1 = card_map.get(&cardID).unwrap(); //MUST UNWRAP cards from map
    print!("{}\n",card1.to_string());

    let p1 = Rc::new(RefCell::new(_p1)); //Must WRAP players in refcell for battle actions
    let p2 = Rc::new(RefCell::new(_p2));

    let battle = Rc::new(RefCell::new(BattleStatus::new(Rc::clone(&p1),Rc::clone(&p2)))); //MUST use REFCELLS
    let c1 = battle.borrow_mut().get_card(2);
    let c2 = battle.borrow_mut().get_card(3);
    let c3 = battle.borrow_mut().get_card(3);
    print!("{}\n",c1.to_string());
    play_card(Rc::clone(&battle),c1);
    print!("{}\n",c2.to_string());
    play_card(Rc::clone(&battle),c2);
    print!("{}\n",c3.to_string());
    play_card(Rc::clone(&battle),c3);
    parse_card(4,4,Rc::clone(&battle));
    battle.borrow_mut().update_player_effects();
    battle.borrow_mut().update_player_effects();
    battle.borrow_mut().turner();
    parse_card(5,0,Rc::clone(&battle));
    battle.borrow_mut().update_player_effects();
    battle.borrow_mut().update_player_effects();
    battle.borrow_mut().update_player_effects();
    parse_card(8,3,Rc::clone(&battle));
    battle.borrow_mut().update_player_effects();
    battle.borrow_mut().update_player_effects();
    battle.borrow_mut().update_player_effects();
    print!("{}\n",battle.borrow_mut().get_turn());
    print!("{}\n",p1.borrow_mut().to_string());
    print!("{}\n",p2.borrow_mut().to_string());
}
*/
