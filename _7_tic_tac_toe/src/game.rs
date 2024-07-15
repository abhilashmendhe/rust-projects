use std::io::{self, Write};

use board::Board;
use player::Player;

pub mod player;
pub mod board;

#[derive(Debug, Clone)]
pub struct Game {
    pub player1: Player,
    pub player2: Player,
    pub board: Board
}

impl Game {
    pub fn play_game(&mut self) {
        let player1 = self.player1.clone();
        let player2 = self.player2.clone();
        let mut turn = true;
        loop {
            if turn {
                let (x,y) = get_input_position(&player1);
                let resf = self.board.place(player1.clone(), x, y);
                if resf {
                    println!("{} won the game!", player1.name);
                    break;
                }
                self.board.print_board();
                turn = !turn;
            } else {
                let (x,y) = get_input_position(&player2);
                let resf = self.board.place(player2.clone(), x, y);
                if resf {
                    println!("{} won the game!", player2.name);
                    break;
                }
                self.board.print_board();
                turn = !turn;
            }        
        }
    }
}

fn get_input_position(player: &Player) -> (usize, usize) {
    print!("{} please enter the x position: ", player.name);
    let _ = io::stdout().flush();
    let mut inp = String::new();
    let _ = io::stdin().read_line(&mut inp).expect("Failed to read x.");
    let x = inp.trim().parse::<usize>().expect("Not able to parse to usize");

    print!("{} please enter the y position: ", player.name);
    let _ = io::stdout().flush();
    let mut inp = String::new();
    let _ = io::stdin().read_line(&mut inp).expect("Failed to read x.");
    let y = inp.trim().parse::<usize>().expect("Not able to parse to usize");
    println!("{}'s position is ({}, {})\n",player.name,x,y);

    (x, y)
}