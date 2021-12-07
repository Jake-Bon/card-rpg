use crate::cards::game_structs::*;
use std::fs;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use rand::prelude::SliceRandom;
use rand::{thread_rng,Rng};

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
        if *action==0 {
            is_attack = true;
        }
        parse_card(*action,*value,Rc::clone(&stat));
    }
    if is_attack{
        stat.borrow_mut().get_inactive_player().borrow_mut().set_mult(1 as i32);
    }
}

pub fn parse_card (id: i32, val: i32, status: Rc<RefCell<BattleStatus>>){
    let mut stat = status.borrow_mut();



    match id as u32{ //p1 = first-person player
        0 => {let mult = stat.get_inactive_player().borrow_mut().get_mult();
            attack(val, mult, stat.get_inactive_player())},
        1 => defend(val, stat.get_active_player()),
        2 => heal(val, stat.get_active_player()),
        3 => mult_next_dmg(val, stat.get_inactive_player()), //multiplier placed on opponent
        4 => poison(val, stat.get_inactive_player()),
        5 => cure(stat.get_active_player()),
        6 => change_mana_regen(val,stat.get_active_player()),//player bumps up own regen
        7 => change_mana_regen(val,stat.get_inactive_player()),//player bumps down opponent regen
        8 => health_regen(val,stat.get_active_player()),
        9 => draw_cards(val,stat.get_active_player()),
        10 => insert_into_deck(val as u32, stat.get_active_player()), // insert into caster's deck
        11 => insert_into_deck(val as u32, stat.get_inactive_player()), // insert into caster's opponent's deck
        12 => shuffle_deck(stat.get_active_player()),   // shuffle caster's deck
        13 => shuffle_deck(stat.get_inactive_player()), // shuffle caster's opponent's deck
        14 => {let mult = stat.get_inactive_player().borrow_mut().get_mult();
            attack(val, mult, stat.get_active_player())},    // attack self (use this instead of healing for negative values to account for dmg multipliers)
        15 => remove_card_variant(val as u32,stat.get_active_player()), //Remove  one val card from self
        16 => remove_card_variant(val as u32,stat.get_inactive_player()), //Remove one val card from enemy
        17 => remove_all_card_variant(val as u32,stat.get_active_player()), //Remove all val cards from self
        18 => remove_all_card_variant(val as u32,stat.get_inactive_player()), //Remove all val cards from enemy
        19 => dup_card(val as u32,stat.get_active_player()), //Dupe own card
        20 => discard_cards_from_hand(stat.get_active_player(), val), // discard val cards from active player's hand
        21 => discard_cards_from_hand(stat.get_inactive_player(), val), // discard val cards from inactive player's hand
        22 => r_volley_effect(stat.get_inactive_player(), stat.get_active_player()),
        23 => extra_turn(stat.get_active_player(), val), //Extra turn for: SELF
        24 => extra_turn(stat.get_inactive_player(), val), //              ENEMY
        25 => add_discard(stat.get_active_player()),
        _ => unreachable_action(),
    }
}

//TODO - According to turn apply attack, defend, and heal to correct player. Check if these work properly.
//TODO - Get CARD from player deck and get card TYPE and VALUE

fn attack (val: i32, mult:f32, target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();
    let def = target.get_defense();
    
    let total_damage = def - (val as f32*mult) as i32;
    
    if total_damage < 0 {
        target.adjust_curr_health(total_damage);
    }

    //target.adjust_curr_health(def-((val as f32*mult) as i32));
    target.set_defense(0);
}

pub fn defend (val: i32, target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();
    target.add_defense(val);
}

pub fn heal (val: i32, target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();
    target.adjust_curr_health(val);
}

pub fn mult_next_dmg(val:i32, target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();

    //print!("{} healed {} hp!\n",target.get_name(),val);
    target.set_mult(val);
}

pub fn poison(val:i32,target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();
    target.add_poison(val as u32);
}

pub fn cure(target:Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();
    target.clear_poison();
}

pub fn change_mana_regen(val:i32, target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();
    target.add_energy_regen(val);
}

pub fn health_regen(val: i32, target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();
    target.add_health_regen(val);
}

fn draw_cards(val: i32, target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();
    if target.get_deck_size() > 0 {
        let mut dif = target.get_full_hand_size() - target.get_curr_hand_size()+1;
        for i in 0 as i32..val{
            if dif>0{
                //target.draw_card(true);
                target.add_draw_num(1);
            }else{
                break;
            }
            dif-=1;
        }
    }
}

// inserts the card with the given card_ID into the deck of the given target
fn insert_into_deck(card_ID: u32, target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();
    target.add_card_to_deck(card_ID);
    target.shuffle_deck();
}

// shuffles the deck of the given target player
fn shuffle_deck(target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();
    target.shuffle_deck();
}

fn remove_card_variant(card_ID: u32, target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();
    target.remove_sel_card(card_ID);
}

fn remove_all_card_variant(card_ID: u32, target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();
    target.remove_all_sel_card(card_ID);
}

fn dup_card(card_ID: u32,target: Rc<RefCell<Battler>>){
    let mut target = target.borrow_mut();
    target.dup_card(card_ID);
}

fn discard_cards_from_hand(target: Rc<RefCell<Battler>>, discard_num: i32){
    let mut target = target.borrow_mut();
    let mut rng = thread_rng();

    for i in 0 as i32..discard_num{
        if target.get_curr_hand_size() > 0 {
            let discard_index = rng.gen_range(0..target.get_curr_hand_size());
            target.hand_discard_card(discard_index);
        }
    }
}

fn r_volley_effect(target: Rc<RefCell<Battler>>, attacker: Rc<RefCell<Battler>>){
    let mut attacker = attacker.borrow_mut();
    let cur_bonus = attacker.get_volley_bonus();
    let cur_multi = attacker.get_mult();
    attacker.inc_volley_bonus();

    //let target = target.borrow_mut();
    attack((2 + cur_bonus) as i32, cur_multi, target);

}

fn extra_turn(target: Rc<RefCell<Battler>>, n_turns: i32){
    let mut target = target.borrow_mut();
    target.add_ex_turn(n_turns as u32);
}

fn add_discard(target: Rc<RefCell<Battler>>) {
    let mut target = target.borrow_mut();
    match target.get_random_discard() {
        Some(card) => { target.add_card_to_hand(card); target.dup_card_no_deck_add(card);},
        None => println!("No cards discarded"),
    }
}

fn unreachable_action(){
    print!("Hope you're happy.\n");
}
