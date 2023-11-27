mod game;
mod tic;
mod minmax;
mod game_terminal_interface;
mod terminal_interface;

use tic::*;
use minmax::MinimaxPlayer2Strategy as P1;
use terminal_interface::HumanTerminalStrategy as P2;
use game_terminal_interface::*;

fn main() {
    let board = Board::new();
    let strategy1: P1<Board, AlmostWinHeuristic> = P1::new(5);
    let strategy2 = P2 {};
    run_game(board, strategy2, strategy1);
}