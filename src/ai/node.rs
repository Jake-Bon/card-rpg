use crate::cards::game_structs::*;
use std::rc::Rc;
use std::cell::RefCell;
//nodes for the decision tree
struct Node{
    //vector to hold any children of the node
    children: Vec<Node>,
    //player structs to get the player's current values
    player1: Battler,
    player2: Battler,



}

//build the decision tree based off the current game state
//called at the start of player 2's turn in src/battle.rs::step()
pub fn build_tree(player1: Rc<RefCell<Battler>>, player2: Rc<RefCell<Battler>>){
    println!("hello there i'm going to start building the tree now");
    //get player 1's hand
    //get player 1's mana
    //get player 2's hand
    //get player 2's mana
    println!("ai: {}", player1.borrow_mut().to_string());
    println!("ai: {}", player2.borrow_mut().to_string());

    //root node is the current game state
    //create a node for each combination of cards playable
    //first row (min) is the cards player 1 could play
    //next row (max) is the cards player 2 could play in response
        //don't account for drawing new cards
    //carry on until out of cards
}