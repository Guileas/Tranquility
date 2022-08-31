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
    loop {
        keyword_event(read().unwrap());
    }

    
}

// Read keyword event and execute a function if necessary
fn keyword_event(read_event: Event){
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
    let mut island_map: [i8; 85] = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,100,100,100,100,100];

    island_map = shuffle(island_map);
}