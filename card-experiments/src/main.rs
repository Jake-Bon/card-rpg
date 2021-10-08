//extern crate card_experiments;
mod card;
//For card testing
fn main(){
    let aCard = card::Card::new("Test","This a test",3,card::CardType::Heal,3);
    print!("{}",aCard.toString());
}
