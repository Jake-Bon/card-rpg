use crate::cards::game_structs::*;
use std::rc::Rc;
use std::cell::RefCell;

//nodes for the decision tree
struct Node{
    //vector to hold any children of the node
    children: Vec<Node>,
    //player structs to get the player's current values
    // player1: Battler,
    // player2: Battler,//probably dont wnat this, instead want their values? eh maybe not, use copies of the initial state battlers
    //TODO: ^add when tracking health and other game state variables

    //need details about the player's current cards and energy
    //p1_energy: i32,
    p1_hand: Vec<Card>, //might want to hold cards differently
    //p2_energy: i32,
    p2_hand: Vec<Card>,
    //TODO: ^add energy consideration
}

//functions for getting nodes children yada yada
impl Node{

    pub fn get_children(&self) -> &Vec<Node>{
        return &self.children;
    }

    pub fn set_child(&mut self, child: Node){
        &self.children.push(child);
    }
}

//build the decision tree based off the current game state
//called at the start of player 2's turn in src/battle.rs::step()
pub fn build_tree(player1: Rc<RefCell<Battler>>, player2: Rc<RefCell<Battler>>, battle_handler: Rc<RefCell<BattleStatus>>){
    println!("\nai: hello there i'm going to start building the tree now\n");
    
    println!("ai: {}", player1.borrow_mut().to_string());
    println!("ai: {}", player2.borrow_mut().to_string());

    ///////////////////////////////////PLAYER 1 (HUMAN)//////////////////////////////////////
    //create the player 1 starting instance
    let p1 = Rc::clone(&player1);

    //get all the cards in player 1's hand
    let p1_hand_size = player1.borrow_mut().get_curr_hand_size();
    let mut p1_hand = Vec::<Card>::new();
    
    for i in 0..p1_hand_size{
        let card_id = player1.borrow_mut().select_hand(i).unwrap();
        let card = battle_handler.borrow_mut().get_card(card_id);
        p1_hand.push(card);
        //println!("ai: p1 card {}: {}", i, p1_hand[i].to_string());
    }
    //get player 1's energy
    let p1_energy = player1.borrow_mut().get_curr_energy();

    ///////////////////////////////////PLAYER 2 (AI)//////////////////////////////////////
    //create the player 2 starting instance
    let p2 = Rc::clone(&player2);

    //get all the cards in player 2's hand
    let p2_hand_size = player2.borrow_mut().get_curr_hand_size();
    let mut p2_hand = Vec::<Card>::new();

    for i in 0..p2_hand_size{
        let card_id = player2.borrow_mut().select_hand(i).unwrap();
        let card = battle_handler.borrow_mut().get_card(card_id);
        p2_hand.push(card);
        //println!("ai: p2 card {}: {}", i, p2_hand[i].to_string());
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

    //TODO: check if the player's hand is empty
    //TODO: playing multiple cards: card_to_play becomes a vector?
    let root = build_tree_rec(p1_hand, p2_hand, 2, 0);
    //print_tree(root);
}

//recursively build the decision tree of every possible play the ai has
//node: current node getting children added upon <- dont need this?
//p1_hand: cards in player 1's hand
//p2_hand: cards in player 2's hand
//player: 1 for human, 2 for ai
//card_to_play: index of the next card to play
//height: current number of levels in the tree, 0=base case to build the root //dont need this?
fn build_tree_rec(mut p1_hand: Vec<Card>, mut p2_hand: Vec<Card>,  player: i32, card_to_play: usize) -> Node{
    //CURRENTLY TRYING TO DO 1 CARD PER TURN, IGNORE ENERGY

    //TODO: check if the player's hand is empty, card_to_play becomes null so play nothing
    //human's turn
    if player == 1{
        //remove card_to_play from player 1's hand
            //find the card index and remove it
        p1_hand.remove(card_to_play);
        // for i in 0..p1_hand.len(){
        //     println!("ai: p1 card {}: {}", i, p1_hand[i].to_string());
        // }
        // if p1_hand.len() == 0{
        //     println!("ai tree: finished building");
        // }
    }
    //ai's turn
    if player == 2{
        //remove card_to_play from player 2's hand
            //find the card index and remove it
        p2_hand.remove(card_to_play);
        // for i in 0..p2_hand.len(){
        //     println!("ai tree: p2 card {}: {}", i, p2_hand[i].to_string());
        // }
        // if p2_hand.len() == 0{
        //     println!("ai tree: finished building");
        // }
    }

    //create the new node with the current game state
        //current hands, TODO: energy, health, etc.
    let mut n = Node{
        children: Vec::<Node>::new(),
        p1_hand: p1_hand.clone(),
        p2_hand: p2_hand.clone(),
    };

    //human's turn, loop through ai's possible plays and call recursively?
    if player == 1{
        //for card in player 2's hand
        for i in 0..p2_hand.len(){
            n.set_child(build_tree_rec(p1_hand.clone(), p2_hand.clone(), 2, i));
        }
    }

     //ai's turn, loop through human's possible plays and call recursively?
     if player == 2{
        //for card in player 1's hand
        for i in 0..p1_hand.len(){
            n.set_child(build_tree_rec(p1_hand.clone(), p2_hand.clone(), 1, i));
        }
    }
    //recursively return the node 
    return n;
}

//call print_tree_rec supplied with the root node
fn print_tree(node: Node){
    print_tree_rec(node);
}

//recursively print the tree with preorder traversal
fn print_tree_rec(node: Node){
    if node.get_children().is_empty(){
        return;
    }

    println!("ai tree: player 1's hand:");
    for i in 0..node.p1_hand.len(){
        println!("ai: p1 card {}: {}", i, node.p1_hand[i].to_string());
    }
    println!("ai tree: player 2's hand:");
    for i in 0..node.p2_hand.len(){
        println!("ai tree: p2 card {}: {}", i, node.p2_hand[i].to_string());
    }

    //TODO: fix this,  print_tree_rec can't move out of Vec<Node>
    //let next = node.get_children()[0];
    for i in 0..node.get_children().len(){
        //print_tree_rec(next);
    }
}