mod game;
mod tic;
mod minmax;
mod tic_simulator;
mod tic_terminal_interface;
mod tic_graphical_interface;
mod tic_strategies;

use tic::*;
use tic_strategies::MinimaxStrategy as P1;
use tic_terminal_interface::HumanTerminalStrategy as P2;
use tic_simulator::*;
use tic_graphical_interface::*;

fn main() {
    open_window();
    // let board = Board::new();
    // let strategy1= P1::new(5);
    // let strategy2 = P2 {};
    // run_game(board, strategy1, strategy2);
}