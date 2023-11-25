use std::io::{self, Write};

use crate::tic::*;
use crate::game::*;

// TODO:
// - make a HumanStrategy and make this func take two strategies.
// - Refactor this, such that it runs the game loop but doesn't have anything to do with terminal
//   by making TerminalHumanPlayerInterface that implements HumanPlayerInterface

pub fn run_bot_player1_vs_human_player2<T: GameStrategy<Board>>(mut board: Board, strategy: T) {
    while !board.next_states().is_empty() {
        board = strategy.get_move(&board).unwrap();
        println!("=============================================");
        let moves = board.next_states();
        if moves.is_empty() {
            break;
        }
        let move_index = (0..moves.len()).collect();
        print_on_board(&board, move_index);

        //TODO: Make this a function that returns Option.
        // then while move is not valid, ask for input with good error messages.
        let mut input = String::new();
        io::stdout().flush().unwrap(); // Flush the output buffer to ensure the message is displayed before the program waits for input
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let player_move: usize = input.trim().parse().unwrap();
        board = moves.get(player_move).unwrap().to_owned();
    }
    println!("{}", board);
}
