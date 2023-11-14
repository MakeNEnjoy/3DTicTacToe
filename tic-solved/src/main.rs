use std::collections::HashSet;
use std::hash::Hash;

mod game;
mod tic;
mod minmax;

use tic::*;
use game::*;
use minmax::*;

// #[warn(dead_code)]
// fn traverse_tree_with_storing_and_generics<T>(root: T, map: &mut HashSet<T>, counter: &mut u64) 
// where T: GameState<StateImplementation = T> + Eq + Hash
// {
//     if map.contains(&root) {
//         *counter += 1;
//         return;
//     }

//     let states = root.next_states();

//     map.insert(root);

//     let size = map.len();
//     if size % 100000 == 0 {
//         println!("size={} hits={}", size, counter);
//         *counter = 0;
//     }

//     for child in states {
//         traverse_tree_with_storing_and_generics(child, map, counter);
//     }
// }

// fn main() {
//     let empty_board = Board::new();
//     let mut map: HashSet<Board> = HashSet::with_capacity(10);
//     traverse_tree_with_storing_and_generics(empty_board, &mut map, &mut 0);
// }

fn play_best_move(board: &Board, steps_to_search: usize) -> Board {
    let states = board.next_states();
    let mut best_score = None;
    let mut best_board = None;
    let mut scores = Vec::new();
    for state in states {
        let score = minmax_score(state.clone(), steps_to_search);
        scores.push(score);
        if let Some(bs) = best_score {
            if bs > score {
                best_score = Some(score);
                best_board = Some(state);
            }
        } else {
            best_score = Some(score);
            best_board = Some(state);
        }
    }

    println!("=============================================");
    println!("Bot evaluation:");
    print_on_board(&board, scores);
    best_board.unwrap()
}

fn play_worst_move(board: &Board, steps_to_search: usize) -> Board {
    let states = board.next_states();
    let mut best_score = None;
    let mut best_board = None;
    let mut scores = Vec::new();
    for state in states {
        let score = min_score(state.clone(), steps_to_search);
        scores.push(score);
        if let Some(bs) = best_score {
            if bs < score {
                best_score = Some(score);
                best_board = Some(state);
            }
        } else {
            best_score = Some(score);
            best_board = Some(state);
        }
    }

    println!("=============================================");
    println!("Bot evaluation:");
    print_on_board(&board, scores);
    best_board.unwrap()
}

#[test]
fn test_board() {
    let board = Board::new();
    play_best_move(&board, 1);
}

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

use std::io::{self, Write};

fn main() {
    let mut board = Board::new();
    while !board.next_states().is_empty() {
        board = play_best_move(&board, 5);
        println!("=============================================");
        let moves = board.next_states();
        if moves.is_empty() {
            break;
        }
        let move_index = (0..moves.len()).collect();
        print_on_board(&board, move_index);

        let mut input = String::new();
        io::stdout().flush().unwrap(); // Flush the output buffer to ensure the message is displayed before the program waits for input
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let player_move: usize = input.trim().parse().unwrap();
        board = moves.get(player_move).unwrap().to_owned();
    }
    println!("{}", board);
}