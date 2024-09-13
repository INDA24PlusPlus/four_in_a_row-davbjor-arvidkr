use std::io;
use colored::Colorize;

pub struct Board {
    board: [char; 42],
    turn: bool,
}

impl Board {
    pub fn new() -> Self {
		Self {
			board: ['.'; 42],
			turn: false ,//turn == 0 -> Os tur
		}
	}

    pub fn place_bricka(&mut self, movi: i64) -> bool {
		let mut was_valid: bool = false;
		for i in 0..6 {
			let x = movi+i*6;
			if self.board[x as usize] == 'O' || self.board[x as usize] == 'X' {
				continue;
			}
			if self.turn == false {self.board[x as usize] = 'O'}
			else {self.board[x as usize] = 'X'}
			was_valid = true;
		}
		if !was_valid {
			println!("Invalid Move!");
			return false;
		}
        if self.turn {
            self.turn = false;
        }
        else {
            self.turn = true;
        }
		return true;
	}

    pub fn has_won(&self) -> char {
		for i in 0..42 {
			if self.board[i as usize] != '.' {
				let x : i64 = i%8;
				let y : i64 = i/8;
				let left = x-3;
				let up = y+3;
				let right = x+3;
				let mut has_winner: bool = false;
				
				let diagl1 = (x-1)+(y+1)*6;
				let diagl2 = (x-2)+(y+2)*6;
				let diagl3 = (x-3)+(y+3)*6;
				
				let left1 = i-1;
				let left2 = i-2;
				let left3 = i-3;

				let up1 = i+6;
				let up2 = i+12;
				let up3 = i+18;

				let diagr1 = (x+1)+(y+1)*6;
				let diagr2 = (x+2)+(y+2)*6;
				let diagr3 = (x+3)+(y+3)*6;

	
				if (left >= 0 && up < 7 && self.board[i as usize] == self.board[diagl1 as usize] && self.board[i as usize] == self.board[diagl2 as usize] && self.board[i as usize] == self.board[diagl3 as usize]) || (left >= 0 && self.board[i as usize] == self.board[left1 as usize] && self.board[i as usize] == self.board[left2 as usize] && self.board[i as usize] == self.board[left3 as usize]) || (up < 7 && self.board[i as usize] == self.board[up1 as usize] && self.board[i as usize] == self.board[up2 as usize] && self.board[i as usize] == self.board[up3 as usize]) || (right < 7 && up < 7 && self.board[i as usize] == self.board[diagr1 as usize] && self.board[i as usize] == self.board[diagr2 as usize] && self.board[i as usize] == self.board[diagr3 as usize]){has_winner = true;}

				if has_winner {return self.board[i as usize]}
				
			}

			
		}
        return '.';
    }



    pub fn print_board(&self) {
        print!("\n");
        for y in (0..6).rev() {
            for x in 0..7 {
                //print!("{}", self.board[y*7+x]);
                if self.board[y*7+x] == 'X' {
                    print!("{}", (self.board[y*7+x]).to_string().red());
                }
                else if self.board[y*7+x] == 'O' {
                    print!("{}", (self.board[y*7+x]).to_string().yellow());
                }
                else {
                    print!("{}", self.board[y*7+x]);
                }
            }
            print!("\n");
        }
        
        print!("\n");
        for _x in 0..8 {
            //print!("1 2 3 4 5 6 7");
        }
        print!("\n");
    }
}

fn main() {
    
    //let start: [char, 42] = ['.'; 42];
    let board = Board { board: ['.'; 42], turn: true };
    
    board.print_board();

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
        if num == 0 || num > 8 {
            println!("Could not read line!");
            continue;
        }

        /*
        if board.place_bricka(num-1) == false {
            println!("Could not place bricka at {}!\n, try again", num);
        }
        */
        board.print_board();
        
        let c: char = board.has_won();
        if c != '.' {
            let s;

            if c == 'X' {
                // X
                s = "Yellow".to_string();
            }
            else {
                // O
                s = "Red".to_string();
            };
            println!("{} has won", s);
        }

    }
}
