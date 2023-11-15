use std::collections::HashSet;
use std::hash::Hash;

mod game;
mod tic;
mod minmax;
mod game_terminal_interface;

use tic::*;
use game_terminal_interface::*;

// fn main() {
//     let mut board = Board::new();
//     while !board.next_states().is_empty() {
//         println!("{}", board);
//         println!("============================");
//         board = play_best_move(&board, 5);
//         if board.next_states().is_empty() {
//             break;
//         }
//         println!("{}", board);
//         println!("============================");
//         board = play_worst_move(&board, 5);

//     }
//     println!("{}", board);
// }


// fn main() {
//     let mut board = Board::new();
//     while !board.next_states().is_empty() {
//         board = play_best_move(&board, 5);
//         println!("=============================================");
//         let moves = board.next_states();
//         if moves.is_empty() {
//             break;
//         }
//         let move_index = (0..moves.len()).collect();
//         print_on_board(&board, move_index);

//         let mut input = String::new();
//         io::stdout().flush().unwrap(); // Flush the output buffer to ensure the message is displayed before the program waits for input
//         io::stdin().read_line(&mut input).expect("Failed to read line");
//         let player_move: usize = input.trim().parse().unwrap();
//         board = moves.get(player_move).unwrap().to_owned();
//     }
//     println!("{}", board);
// }

fn main() {
    let board = Board::new();
    run_bot_player1_vs_human_player2(board);
}