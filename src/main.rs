use rand::prelude::*;
use crossterm::{event::{read, Event, KeyCode, KeyEvent, KeyModifiers, KeyEventKind, KeyEventState}};

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
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE
        }) => {}
        Event::Key(KeyEvent {
            code: KeyCode::Char('b'),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE
        }) => {}
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

// Shuffle card using the modern the Fisher-Yates algorithm
// https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle#The_modern_algorithm
pub fn shuffle(mut cards: [i8;85]) -> [i8;85] {
    // Define the last index of the card array
    let mut last_index = cards.len() - 1;
    while last_index > 0 {
        // Generate a random index in between 0 and the last index to choose a random number in the card array
        let rand_index = thread_rng().gen_range(0..last_index);
        // Swap the number in the last index with the random number selected by the rand_index var
        cards.swap(last_index, rand_index);
        //Decrement the last index so the right part will be the shuffled part and the left part the numbers that stills need to be shuffle
        last_index -= 1;
    }

    return cards;
}

pub fn setup() {
    let island_stack: [i8; 85] = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,100,100,100,100,100];

    // Create and Display the board game
    let raw_board: [[String; 8]; 8] = create_square_board();
    display_board_game(raw_board.as_slice());

    // Shuffle the stack 
    let player_deck: [i8; 85] = shuffle(island_stack);
    
    game(player_deck, raw_board)
}

fn game(player_stack: [i8; 85], board_game_array: [[String; 8]; 8]) {

    // Create the user hand
    let mut player_hand: Vec<i8> = Vec::new();
    player_hand.extend_from_slice(&player_stack[..5]); 

    // Get the player deck whithout his first hand
    let mut player_deck: Vec<i8> = update_player_deck(player_stack.to_vec(), 5);

    //Insert the start card in the player_deck
    player_deck = insert_start_card(player_deck);

    print_hand(player_hand);
    print_choose_action();
    
    loop{}
    
}

fn print_hand(hand: Vec<i8>){
    println!();
    println!("Your hand is: ");
    println!("{:?}", hand);
}

fn print_choose_action(){
    println!();
    println!("What action would you like to do: ");
    println!("[a] - Play a card");
    println!("[b] - Drop 2 cards ");
    keyboard_event(read().unwrap());
}

// Return the stack whitout the user hand
fn update_player_deck(mut player_deck: Vec<i8>, first_card_to_keep: usize) -> Vec<i8>{ 
    // Return only the card we keep and remove the others 
    player_deck.drain(first_card_to_keep..).collect()
}

fn insert_start_card(mut player_deck: Vec<i8>) -> Vec<i8>{
    // Define the random index we gonna insert the start card into
    let rand_index = thread_rng().gen_range(0..player_deck.len()+1);
    player_deck.insert(rand_index, 0);
    return player_deck
}

// Prepare the array use to construct the square board
fn create_square_board() -> [[String; 8]; 8] {
    
    // The first map is a square of 8x8
    let max_square_size = 8;
    let mut map: [[String; 8]; 8] =  Default::default();

    for i in 0..max_square_size {
        for j in 0..max_square_size {
            // First row
            if i == 0 && j == 0 { map[i][j] = "_".to_string(); }
            else if i == 0 && j != 0 && j != 7 {
                map[i][j] = String::from(&j.to_string());
            } 
            else if i == 0 && j == 7 { map[i][j] = "x".to_string(); }

            // First column
            else if j == 0 && i != 0 && i != 7 { 
                map[i][j] = String::from(&i.to_string());
            }

            // Last column
            else if j == 7 && i != 0 && i != 7 { 
                map[i][j] = String::from(&i.to_string());
            }

            // Last row
            else if i == 7 && j == 0 { map[i][j] = "s".to_string(); }
            else if i == 7 && j != 0 && j != 7 {
                map[i][j] = String::from(&j.to_string());
            } 
            else if i == 7 && j == 7 { map[i][j] = "_".to_string(); }

            // Board
            else { map[i][j] = "\x1b[93m*\x1b[0m".to_string(); }
        }   
    }
    return map;
}

// Display the board like so:
//
// [_] [1] [2] [3] [4] [5] [6] [x]
// [1] [*] [*] [*] [*] [*] [*] [1]
// [2] [*] [*] [*] [*] [*] [*] [2]
// [3] [*] [*] [*] [*] [*] [*] [3]
// [4] [*] [*] [*] [*] [*] [*] [4]
// [5] [*] [*] [*] [*] [*] [*] [5]
// [6] [*] [*] [*] [*] [*] [*] [6]
// [s] [1] [2] [3] [4] [5] [6] [_]
//
// Using the raw board game created by the function "create_board()"
fn display_board_game(board_array: &[[std::string::String; 8]]) {
    for row_value in board_array.iter() {
        for col_value in row_value.iter() {
            print!("[{}] ", col_value);
        }
        println!();
    }
}