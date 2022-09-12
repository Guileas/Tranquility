pub mod Cards {
    use rand::prelude::*;

    pub fn insert_start_card(mut player_deck: Vec<i8>) -> Vec<i8>{
        // Define the random index we gonna insert the start card into
        let rand_index = thread_rng().gen_range(0..player_deck.len()+1);
        player_deck.insert(rand_index, 0);
        return player_deck
    }

    // This function return two element: [player_hand, pile]
    // - player_hand = player hand refill with new card
    // - pile = the user pile but without the card used to refill the player hand
    pub fn refill_hand(mut player_hand: Vec<i8>, pile: &Vec<i8>, number_of_card_to_refill: usize,) -> Vec<i8> {
        // Get from the pile the number of card needed
        player_hand.extend_from_slice(&pile[..number_of_card_to_refill]); 
        player_hand
    }

    pub fn remove_card(mut player_hand: Vec<i8>, card_to_remove: i8)-> Vec<i8>{
        if let Some(pos) = player_hand.iter().position(|&x| x == card_to_remove) {
            player_hand.remove(pos);
        }
        player_hand
    }

    // A player after his action must always have 5 cards
    pub fn calculate_number_of_card_to_refill(player_hand: &Vec<i8>) -> usize {
        //Calculate how much card needs to be added to the current user hand
        let number_of_card_to_refill = 5 - player_hand.len();
        number_of_card_to_refill
    }

    pub fn calculate_number_of_card_to_drop(card_grid: &[[Option<i8>; 6]; 6]) -> i8 {
        return 0
    }

    // Return the stack whitout the user hand
    pub fn update_player_deck(mut player_deck: Vec<i8>, first_card_to_keep: usize) -> Vec<i8>{ 
        // Return only the card we keep and remove the others 
        player_deck.drain(first_card_to_keep..).collect()
    }
}