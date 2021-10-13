//extern crate card_experiments;
mod game_structs;

use std::collections::VecDeque;
use game_structs::CardType;
use game_structs::Card;
use game_structs::Player;
//For card testing
fn main(){
    let aCard = Card::new("Test","This a test",3,CardType::Heal,3);
    print!("{}\n",aCard.toString());

    let mut aPlayer = Player::new(20,20,10,10,7);
    print!("{}\n",aPlayer.toString());

    aPlayer.addCardToDeck(aCard);


    //error handling here

    aPlayer.addCardToHand(aPlayer.drawCard().unwrap());

    print!("{}\n",aPlayer.selectHand(0).toString());


}
