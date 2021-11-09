use crate::cards::game_structs::Battler;
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
pub fn build_tree(){
    println!("hello there i'm going to start building the tree now");
    //get player 1's hand
    //get player 1's mana
    //get player 2's hand
    //get player 2's mana
    
    //root node is the current game state
    //create a node for each combination of cards playable
    //first row (min) is the cards player 1 could play
    //next row (max) is the cards player 2 could play in response
        //don't account for drawing new cards
    //carry on until out of cards
}