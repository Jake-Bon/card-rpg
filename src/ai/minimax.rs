use crate::ai::ai_structs::*;

// Utility representing a losing game state is currently i32::MIN and winning game state
// is i32::MAX

/*
// For reference: 
// Minimax with alpha-beta pruning pseudocode pulled from https://www.geeksforgeeks.org/minimax-algorithm-in-game-theory-set-4-alpha-beta-pruning/
function minimax(node, depth, isMaximizingPlayer, alpha, beta):

    if node is a leaf node :
        return value of the node
    
    if isMaximizingPlayer :
        bestVal = -INFINITY 
        for each child node :
            value = minimax(node, depth+1, false, alpha, beta)
            bestVal = max( bestVal, value) 
            alpha = max( alpha, bestVal)
            if beta <= alpha:
                break
        return bestVal

    else :
        bestVal = +INFINITY 
        for each child node :
            value = minimax(node, depth+1, true, alpha, beta)
            bestVal = min( bestVal, value) 
            beta = min( beta, bestVal)
            if beta <= alpha:
                break
        return bestVal
*/

pub fn utility(mut node: Node) -> i32 {
    let mut status = node.getStatus();
    let player = status.get_p1().borrow().clone();
    let ai = status.get_p2().borrow().clone();
    if player.get_curr_health() <= 0 {
        return i32::MAX;
    }
    else if ai.get_curr_health() <= 0 {
        return i32::MIN;
    }
    // AI's
    let ai_health = ai.get_curr_health();
    let ai_energy = ai.get_curr_energy();
    let ai_cards = ai.get_deck_size() as i32;
    let ai_utility = ai_health + ai_energy + ai_cards;
    // Player's
    let player_health = player.get_curr_health();
    let player_energy = player.get_curr_energy();
    let player_cards = player.get_deck_size() as i32;
    let player_utility = player_health + player_energy + player_cards;
    // Combine
    let utility = ai_utility - player_utility;
    return utility;
}