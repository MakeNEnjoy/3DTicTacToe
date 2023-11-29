use std::io::{self, Write};

use crate::{tic_simulator::Strategy, tic::{Board, print_on_board}};

pub struct HumanTerminalStrategy {}

impl Strategy for HumanTerminalStrategy {
    fn get_move(&self, board: &Board) -> Option<Board> {
        let moves = board.get_legal_boards();
        if moves.is_empty() {
            return None;
        }

        let move_index = (0..moves.len()).collect();
        println!("=============================================");
        print_on_board(&board, move_index);

        let board = loop {
            let mut input: String = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let index = match input.trim().parse::<usize>() {
                Ok(num) => num,
                Err(_) => {
                    println!("You have to enter a number.");
                    continue
                },
            };
            match moves.get(index) {
                Some(board) => break board,
                None => {
                    println!("This move doesn't exist.");
                    continue
                }
            }
        };
        Some(board.to_owned())
    }
}