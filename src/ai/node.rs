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

    //need details about the player's current cards and energy
    energy: i32;
    hand: Vec<u32>; //might want to hold cards differently
}

//functions for getting nodes children yada yada
impl Node{

    fn get_children(){

    }
}

//build the decision tree based off the current game state
//called at the start of player 2's turn in src/battle.rs::step()
pub fn build_tree(player1: Rc<RefCell<Battler>>, player2: Rc<RefCell<Battler>>, battle_handler: Rc<RefCell<BattleStatus>>){
    println!("\nhello there i'm going to start building the tree now\n");
    
    println!("ai: {}", player1.borrow_mut().to_string());
    println!("ai: {}", player2.borrow_mut().to_string());

    //get all the cards in player 1's hand
    let p1_hand_size = player1.borrow_mut().get_curr_hand_size();
    
    for i in 0..p1_hand_size{
        let card_id = player1.borrow_mut().select_hand(i).unwrap();
        let card = battle_handler.borrow_mut().get_card(card_id);
        println!("ai: p1 card {}: {}", i, card.to_string());
    }
    //get player 1's energy
    let p1_energy = player1.borrow_mut().get_curr_energy();

    //get player 2's hand
    let p2_hand_size = player2.borrow_mut().get_curr_hand_size();

    for i in 0..p2_hand_size{
        let card_id = player2.borrow_mut().select_hand(i).unwrap();
        let card = battle_handler.borrow_mut().get_card(card_id);
        println!("ai: p2 card {}: {}", i, card.to_string());
    }
    //get player 2's energy
    let p2_energy = player1.borrow_mut().get_curr_energy();
    

    //root node is the current game state
    //create a node for each combination of cards playable
    //first row (min) is the cards player 1 could play
    //next row (max) is the cards player 2 could play in response
        //don't account for drawing new cards
    //carry on until out of cards

    //create game state to pass into the build_tree_rec function

    //need to pass unplayed cards?
}

//recursively build the decision tree of every possible play the ai has
//node: current node getting children added upon <- dont need this?
//p1_hand: cards in player 1's hand
//p2_hand: cards in player 2's hand
//player: 1 for human, 2 for ai
//card_to_play: the card player 1 or player 2 is going to play
//height: current number of levels in the tree, 0=base case to build the root
fn build_tree_rec(node: <Node>, player: i32, height: i32, ){
    //CURRENTLY TRYING TO DO 1 CARD PER TURN, IGNORE ENERGY

    //human's turn
    if player == 1{
        //remove card_to_play from player 1's hand
    }
    //ai's turn
    if player == 2{
        //remove card_to_play from player 2's hand
    }

    //create the new node
    let n = Node{

    }

    //human's turn, loop through ai's possible plays and call recursively?
    if player == 1{
        for card in player 2's hand{
            build_tree_rec(card_to_play = i, player = player 2)
        }
    }

     //ai' turn, loop through humans possible plays and call recursively?
     if player == 2{
        for card in player 1's hand{
            build_tree_rec(card_to_play = i, player = player 1)
        }
    }
    
}