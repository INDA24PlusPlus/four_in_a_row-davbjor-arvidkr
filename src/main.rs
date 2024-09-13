use std::io;

pub struct Board {
    board: [char; 42],
    turn: bool,
}

impl Board {

}

fn main() {
    
    let mut board = Board();

    // Game Loop
    loop {
        let player: String;
        if board.turn {
            // X
            player = "Yellow".to_string();
        }
        else {
            // O
            player = "Red".to_string();
        }

        println!("{}'s turn: input row 1-7 to place a piece", player);

        // Read input n
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("failed to readline");


        let num = line.trim().parse().unwrap_or(0);
        if num == 0 {
            println!("Could not read line!");
            continue;
        }
        

        

    }
}
