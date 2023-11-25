mod game;
mod tic;
mod minmax;
mod game_terminal_interface;

use tic::*;
use minmax::*;
use game_terminal_interface::*;

fn main() {
    let board = Board::new();
    let strategy: MinimaxPlayer1Strategy<Board, AlmostWinHeuristic> = MinimaxPlayer1Strategy::new(5);
    run_bot_player1_vs_human_player2(board, strategy);
}