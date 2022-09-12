pub mod Shuffle {
    
    use rand::prelude::*;

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
}