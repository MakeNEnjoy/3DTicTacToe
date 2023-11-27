use std::io::{self, Write};

use crate::tic::*;
use crate::game::*;

// TODO:
// - make a HumanStrategy and make this func take two strategies.
// - Refactor this, such that it runs the game loop but doesn't have anything to do with terminal
//   by making TerminalHumanPlayerInterface that implements HumanPlayerInterface
// - Add basic logging

pub fn run_game<T: GameStrategy<Board>, U: GameStrategy<Board>>(mut board: Board, strategy1: T, strategy2: U) {
    loop {
        board = strategy1.get_move(&board).unwrap();
        
        if board.next_states().is_empty() {
            break;
        }
        
        board = strategy2.get_move(&board).unwrap();

        if board.next_states().is_empty() {
            break;
        }
    }
    println!("Game ended!");
    println!("{}", board);
}
