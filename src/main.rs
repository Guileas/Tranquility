use crossterm::{event::{read, Event, KeyCode, KeyEvent, KeyModifiers, KeyEventKind, KeyEventState}};
use std::io::{self, BufRead};

use utils::{
    board::Board,
    shuffle::Shuffle,
    cards::Cards
};
mod utils;

fn main(){
    println!("                                                                                               
    _|_|_|_|_|                                                    _|  _|  _|    _|                
        _|      _|  _|_|    _|_|_|  _|_|_|      _|_|_|  _|    _|      _|      _|_|_|_|  _|    _|  
        _|      _|_|      _|    _|  _|    _|  _|    _|  _|    _|  _|  _|  _|    _|      _|    _|  
        _|      _|        _|    _|  _|    _|  _|    _|  _|    _|  _|  _|  _|    _|      _|    _|  
        _|      _|          _|_|_|  _|    _|    _|_|_|    _|_|_|  _|  _|  _|      _|_|    _|_|_|  
                                                    _|                                        _|  
                                                    _|                                    _|_|    ");

    println!("Press ENTER to start the game");
    keyboard_event(read().unwrap());
}

// Read keyboard event and execute a function if necessary
fn keyboard_event(read_event: Event){
    match read_event {
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE
        }) => {
            setup();
        }
        _ => {}
    }
}

pub fn setup() {
    let grid_board: [[Option<i8>; 6]; 6] = [[None,None,None,None, None, None],[None,None,None,None, None, None],[None,None,None,None, None, None],[None,None,None,None, None, None],[None,None,None,None, None, None],[None,None,None,None, None, None]];
    let island_stack: [i8; 85] = [01,02,03,04,05,06,07,08,09,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,100,100,100,100,100];

    // Create and Display the board game
    let raw_board: [[String; 8]; 8] = Board::create_square_board();
    display_board_game(raw_board.as_slice(), &grid_board);

    // Shuffle the stack 
    let player_deck: [i8; 85] = Shuffle::shuffle(island_stack);
    
    // Create the user hand
    let mut player_hand: Vec<i8> = Vec::new();
    player_hand.extend_from_slice(&player_deck[..5]); 

    // Get the player deck whithout his first hand
    let mut player_deck: Vec<i8> = Cards::update_player_deck(player_deck.to_vec(), 5);

    //Insert the start card in the player_deck
    player_deck = Cards::insert_start_card(player_deck);

    print_hand(&player_hand);

    game(player_hand, player_deck, raw_board, grid_board)
}

fn game(player_hand: Vec<i8>, player_deck: Vec<i8>, board_game_array: [[String; 8]; 8], player_grid: [[Option<i8>; 6]; 6]) {
    let action: i8 = print_choose_action();
    match action {
        1 => { play_a_card(player_hand, player_deck, board_game_array, player_grid)},
        2 => { drop_two_card(player_hand, player_deck, board_game_array, player_grid) }
        _ => {}
    }
    loop{}
}

fn play_a_card(mut player_hand: Vec<i8>, mut pile: Vec<i8>, board_array: [[std::string::String; 8];8], mut player_grid: [[Option<i8>; 6]; 6]) {
    let mut card_selected = String::new();
    println!();
    print_hand(&player_hand);
    println!("Whitch card do you want to play?");

    io::stdin().lock().read_line(&mut card_selected).unwrap();
    let card_to_be_placed: i8 = card_selected.trim().parse::<i8>().unwrap();

    println!();
    println!("Where do you want to place it?");

    let mut row = print_row();

    // If not between 
    while row == 0 || row < 1 || row > 6 {
        row = print_row();
        println!("{}", row);
    }

    let mut column = print_column();
    // If not between 
    while column == 0 || column < 1 || column > 6 {
        column = print_column();
        println!("{}", column);
    }

    // Add the card in the board array
    // The -1 is because the col and row array start at 0 but we ask the player to type between 1 to 6 to make things more user friendly
    player_grid[row-1][column-1] = Some(card_to_be_placed);

    display_board_game(&board_array, &player_grid);
    player_hand = Cards::remove_card(player_hand, card_to_be_placed);

    // Calcul the number of card to drop after the card the user played
    // The -1 is because the col and row array start at 0 but we ask the player to type between 1 to 6 to make things more userfriendly
    Cards::calculate_number_of_card_to_drop(&player_grid, row-1, column-1);

    //Calcul how many card are missing from the player hand
    let card_to_refill: usize = Cards::calculate_number_of_card_to_refill(&player_hand);
    // Add the missing card to the player hand
    player_hand = Cards::refill_hand(player_hand.clone(), &pile, card_to_refill);
    // Remove the card add to the hand from the pile
    pile = Cards::update_player_deck(pile, card_to_refill);
    print_hand(&player_hand);

    game(player_hand, pile, board_array, player_grid);
}

fn print_row() -> usize {
    let mut row_selected = String::new();
    println!();
    println!("Select a row (R) [1 - 6]");
    io::stdin().lock().read_line(&mut row_selected).unwrap();
    return row_selected.trim().parse::<usize>().unwrap_or(0);
}

fn print_column() -> usize {
    let mut column_selected = String::new();
    println!();
    println!("Select a column (C) [1 - 6]");
    io::stdin().lock().read_line(&mut column_selected).unwrap();
    return column_selected.trim().parse::<usize>().unwrap();
}

fn drop_two_card(mut player_hand: Vec<i8>, mut pile: Vec<i8>, board_array: [[std::string::String; 8];8], player_grid: [[Option<i8>; 6]; 6]) {
    let mut first_number_selected = String::new();
    let mut second_number_selected = String::new();

    println!();
    print_hand(&player_hand);
    println!("Whitch card do you want to drop?");

    println!("The first one:");
    io::stdin().lock().read_line(&mut first_number_selected).unwrap();
    let first_card: i8 = first_number_selected.trim().parse::<i8>().unwrap();
    player_hand = Cards::remove_card(player_hand, first_card);

    println!();
    print_hand(&player_hand);
    println!("The second one:");
    io::stdin().lock().read_line(&mut second_number_selected).unwrap();
    let second_card: i8 = second_number_selected.trim().parse::<i8>().unwrap();
    player_hand = Cards::remove_card(player_hand, second_card);
    
    //Calcul how many card are missing from the player hand
    let card_to_refill: usize = Cards::calculate_number_of_card_to_refill(&player_hand);
    // Add the missing card to the player hand
    player_hand = Cards::refill_hand(player_hand, &pile, card_to_refill);
    // Remove the card add to the hand from the pile
    pile = Cards::update_player_deck(pile, card_to_refill);
    print_hand(&player_hand);

    display_board_game(&board_array, &player_grid);
    game(player_hand, pile, board_array, player_grid);
}

fn print_hand(hand: &Vec<i8>){
    println!();
    println!("Your hand is: ");
    println!("{:?}", hand);
}

fn print_choose_action() -> i8{

    println!();
    println!("What action would you like to do: ");
    println!("[1] - Play a card");
    println!("[2] - Drop 2 cards ");
    
    let mut action_selected:String = String::new();
    io::stdin().lock().read_line(&mut action_selected).unwrap();
    let action : i8 = action_selected.trim().parse::<i8>().unwrap();
    action
}

// Display the board like so:
//
// [__] [C1] [C2] [C3] [C4] [C5] [C6] [<>] 
// [R1] [**] [**] [**] [**] [**] [06] [R1] 
// [R2] [**] [**] [**] [**] [**] [**] [R2] 
// [R3] [**] [**] [**] [**] [**] [**] [R3] 
// [R4] [**] [**] [**] [**] [**] [**] [R4] 
// [R5] [**] [**] [**] [**] [**] [**] [R5] 
// [R6] [**] [**] [**] [**] [**] [**] [R6] 
// [<>] [C1] [C2] [C3] [C4] [C5] [C6] [__] 
//
// Using the raw board game and the playable grid 
fn display_board_game(board_array: &[[std::string::String; 8]], player_grid: &[[Option<i8>; 6]; 6]) {  
    println!(); 
    for (row_index, row_value) in board_array.iter().enumerate() {
        for (col_index, col_value) in row_value.iter().enumerate() {
            if 0 < row_index && row_index < 7 && 0 < col_index && col_index < 7 {
                match player_grid[row_index-1][col_index-1]{
                    None => {
                        print!("[{}] ", col_value);
                    }
                    Some(value) => {
                        if value < 10 {
                            print!("[\x1b[36m0{}\x1b[0m] ", value);
                        }else{
                            print!("[\x1b[36m{}\x1b[0m] ", value);
                        }
                    }
                }
            }else{
                print!("[{}] ", col_value);
            }
        }
        println!();
    }
}