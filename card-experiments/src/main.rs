//extern crate card_experiments;
mod game_structs;

//TRY MAKING THE VECS IN PLAYER POINTERS SO THAT PLAYER NEVER HAS TO CHANGE ITSELF

use game_structs::CardType;
use game_structs::Card;
use game_structs::Player;
//For card testing

//fn drawCard<'a>(aPlayer: &'a mut Player)->Card<'a>{
    //let tmp = aPlayer.drawCard();
    //aPlayer.deckDelCard();
    //tmp
//}

fn main(){
    let aCard = Card::new("Test","This a test",3,CardType::Heal,3);
    print!("{}\n",aCard.toString());

    let mut aPlayer = Player::new(20,20,10,10,7);
    print!("{}\n",aPlayer.toString());

    aPlayer.addCardToDeck(aCard);

    //error handling here

    aPlayer.addCardToHand(aPlayer.drawCard()); //
    aPlayer.deckDelCard(); //cannot have a POP function, these 2 functions must serve as such.

    print!("{}\n",aPlayer.selectHand(0).toString());

    aPlayer.handDelCard(0);


}
