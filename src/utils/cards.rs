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

    pub fn calculate_number_of_card_to_drop(card_grid: &[[Option<i8>; 6]; 6], row: usize, column: usize) -> i8 {
        let current_card:Option<i8> = card_grid[row][column];
        let previous_card: Option<i8> = find_previous_card(card_grid, row, column);
        let next_card: Option<i8> = find_next_card(card_grid, row, column);
        let mut prev_sub = 0;
        let mut next_sub = 0;

        // Get the difference between current and previous card
        if  previous_card.is_some() && current_card.is_some(){
            prev_sub = current_card.unwrap_or(0) - previous_card.unwrap_or(0);
        }

        // Get the difference between current and next card
        if  next_card.is_some() && current_card.is_some(){
            next_sub = next_card.unwrap_or(0) - current_card.unwrap_or(0);
        }

        // If there's no previous card
        if prev_sub == 0 && next_sub != 0 {
            return next_sub
        }

        // If there's no next card
        if next_sub == 0 && prev_sub != 0 {
            return prev_sub
        }

        // Previous win if his smaller than next
        // ex : previous = 2 next = 4
        // the previous card win 
        if prev_sub < next_sub {
            return prev_sub
        }
        
        // Next win if his smaller than previous
        // ex : previous = 2 next = 1
        // the next card win 
        if next_sub < prev_sub {
            return next_sub
        }

        // If it's the same
        if prev_sub == next_sub {
            return prev_sub
        }

        return 0
    }

    // Return the stack whitout the user hand
    pub fn update_player_deck(mut player_deck: Vec<i8>, first_card_to_keep: usize) -> Vec<i8>{ 
        // Return only the card we keep and remove the others 
        player_deck.drain(first_card_to_keep..).collect()
    }


    fn find_previous_card(card_grid: &[[Option<i8>; 6]; 6], row: usize, column: usize) -> Option<i8>{
         // I'm in the same row
         if 0 < column && column < 5 {
            match card_grid[row][column-1] {
                Some(value) => {  return Some(value); }
                None => return None
            }
        }
        // I'm in the previous row
        if  column == 0 && row < 5 {
            match card_grid[row+1][5] {
                Some(value) => {  return Some(value); }
                None => return None
            }
        }

        return None;
    }


    fn find_next_card(card_grid: &[[Option<i8>; 6]; 6], row: usize, column: usize) -> Option<i8>{
         // I'm in the same row
         if 0 <= column && column < 5 {
            match card_grid[row][column+1] {
                Some(value) => {  return Some(value); }
                None => return None
            }
        }
        // I'm in the next row
        if  column == 5 && row > 0 {
            match card_grid[row-1][0] {
                Some(value) => {  return Some(value); }
                None => return None
            }
        }
        return None;
    }
}