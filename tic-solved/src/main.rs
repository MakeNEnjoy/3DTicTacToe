mod game;
mod tic;
mod minmax;
mod game_terminal_interface;

use tic::*;
use minmax::MinimaxPlayer1Strategy as P1;
use game_terminal_interface::*;

fn main() {
    let board = Board::new();
    let strategy: P1<Board, AlmostWinHeuristic> = P1::new(5);
    run_bot_player1_vs_human_player2(board, strategy);
}