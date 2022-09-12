pub mod Board {
    // Prepare the array use to construct the square board
    pub fn create_square_board() -> [[String; 8]; 8] {
        
        // The first map is a square of 8x8
        let max_square_size = 8;
        let mut map: [[String; 8]; 8] =  Default::default();

        for i in 0..max_square_size {
            for j in 0..max_square_size {
                // First row
                if i == 0 && j == 0 { map[i][j] = "__".to_string(); }
                else if i == 0 && j != 0 && j != 7 {
                    map[i][j] = "C".to_string() + &j.to_string();
                } 
                else if i == 0 && j == 7 { map[i][j] = "\x1b[92m<>\x1b[0m".to_string(); }

                // First column
                else if j == 0 && i != 0 && i != 7 { 

                    map[i][j] = "R".to_string() + &i.to_string();
                }

                // Last column
                else if j == 7 && i != 0 && i != 7 { 
                    map[i][j] = "R".to_string() + &i.to_string();
                }

                // Last row
                else if i == 7 && j == 0 { map[i][j] = "\x1b[92m<>\x1b[0m".to_string(); }
                else if i == 7 && j != 0 && j != 7 {
                    map[i][j] = "C".to_string() + &j.to_string();
                } 
                else if i == 7 && j == 7 { map[i][j] = "__".to_string(); }

                // Board
                else { map[i][j] = "\x1b[93m**\x1b[0m".to_string(); }
            }   
        }
        return map;
    }
    
}