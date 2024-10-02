use std::io::{stdin, stdout, Write};
use std::fmt::Display;

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            match self {
                Player::X  => "X",
                Player::O => "O",
            }
        )
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Player {
    X,
    O,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Board {
    grid: [[Option<Player>; 3] ; 3],
    current_turn: Player,
    winner: Option<Player>,
}

fn main() {
    let mut board: Board = Board { 
        grid: [[None, None, None], [None, None, None], [None, None, None]],
        current_turn: Player::X,
        winner: None,
    };
    loop{

        print!("Enter square pls {} : ", board.current_turn);
        stdout().flush().expect("uh oh");

        let mut turn = String::new();
        stdin().read_line(&mut turn).expect("uh oh");

        let guess: Result<usize, _> = turn.trim().parse();
        
        if guess.is_err() {
            continue;
        }

        let square = guess.unwrap() - 1;
        let row = square / 3;
        let column = square % 3;

        if square > 8 || board.grid[row][column].is_some() {
            continue;
        }

        board.grid[row][column] = Some(board.current_turn);


        println!("-------");
        for row in board.grid {
            for element in row {
                print!("|");
                match element {
                    Some(p) => print!("{p}"),
                    None => print!(" "),
                }
            }
            println!("|");
            println!("-------");
        };
        
        board.current_turn = match board.current_turn {
            Player::O => Player::X,
            Player::X => Player::O,
        };

        for row in board.grid {
            if row[0] == row[1] && row[1] == row[2] && row[0].is_some() {
                board.winner = row[0]
            }
        }
        for i in 0..3 {
            if board.grid[0][i] == board.grid[1][i] 
            && board.grid[1][i] == board.grid[2][i] 
            && board.grid[0][i].is_some() {
                board.winner = board.grid[0][i];
            }
        }

        if board.grid[0][0] == board.grid[1][1]
        && board.grid[1][1] == board.grid[2][2]
        && board.grid[0][0].is_some() {
            board.winner = board.grid[0][0];
        }

        if board.grid[0][2] == board.grid[1][1]
        && board.grid[1][1] == board.grid[2][0]
        && board.grid[0][2].is_some() {
            board.winner = board.grid[0][2];
        }


        match board.winner {
            None => continue,
            Some(p) => {println!("{} Wins", p); break},

        }

    }
}

