use crate::cards::game_structs::*;
use std::fs;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::iter::Zip;

pub fn populate_battler_map ()->HashMap<u32,Battler>{
    let mut battlers = HashMap::new();
    let file_data = fs::read_to_string("src/cards/battler-library.txt").expect("An error occurred whilst attempting to open the library.");
    for line in file_data.trim().split('\n'){ //Remove first character, \u was messing with things
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
        9 => draw_cards(val,stat.get_active_player()),
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
    target.add_defense(val);
}

fn heal (val: i32, target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();
    let mut new_val = val;
    if val<0{
        new_val = val + target.get_defense();
    }
    target.adjust_curr_health(new_val);
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

fn draw_cards(val: i32, target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();
    for i in 0 as i32..val{
        target.draw_card();
    }
}

fn unreachable_action(){
    print!("Hope you're happy.\n");
}
