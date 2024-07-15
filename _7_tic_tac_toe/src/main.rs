use game::{board::Board, player::Player, Game};

pub mod game;

fn main() {
    let player1 = Player {
        name: "Abhi".to_string(),
        marker: 'x'
    };
    let player2 = Player {
        name: "Suchit".to_string(),
        marker: 'o'
    };
    let board = Board::new(3);
    let mut game = Game {
        player1: player1.clone(),
        player2: player2.clone(),
        board: board.clone()
    };
    board.print_board();

    game.play_game();

}


