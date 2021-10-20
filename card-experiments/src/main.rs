//extern crate card_experiments;
mod game_structs;

//TRY MAKING THE VECS IN PLAYER POINTERS SO THAT PLAYER NEVER HAS TO CHANGE ITSELF

use game_structs::Card;
use game_structs::Player;
use std::fs;
use std::collections::HashMap;
//For card testing

//fn drawCard<'a>(a_player: &'a mut Player)->Card<'a>{
    //let tmp = a_player.drawCard();
    //a_player.deckDelCard();
    //tmp
//}

pub fn populate_card_maprary<'a>(fileData: &'a str)->HashMap<u32,Card<'a>>{
    let mut cards = HashMap::new();
    for line in (fileData[4..]).split('\n'){ //Remove first character, \u was messing with things
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

fn main(){ //File data read in then used to create library
    print!("Reading in card library data...\n");
    let fileData = &fs::read_to_string("src/card-library.txt").expect("An error occurred whilst attempting to open the library.");
    print!("File read successfully!\nBuilding card map...\n\n\n");
    let card_map = populate_card_maprary(fileData);
    print!("Card map successfully built! Cards ready to be used!\n\n\n");

    let mut a_player = Player::new(20,20,10,10,7);
    print!("Player object created.\n");
    print!("{}\n",a_player.to_string());

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

    print!("Remaining number of cards in deck: {}\nRemaining cards in hand: {}\n",a_player.get_deck_size(),a_player.get_curr_hand_size())


}
