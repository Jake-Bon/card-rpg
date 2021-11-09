//nodes for the decision tree
struct Node{
    //vector to hold any children of the node
    let mut children: Vec<Node> = Vec::new();
    //player structs to get the player's current values
    let mut player1 = battle_stat.get_p1();
    let mut player2 = battle_stat.get_p2();



}

pub fn build_tree(){
    println!("hello there i'm going to start building the tree now");
}