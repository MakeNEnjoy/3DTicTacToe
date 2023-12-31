use tic_solved::{
    tic_array::{
        tic::*,
        bitmap_tic::*,
        naive_strategy::MinimaxStrategy,
        alphabeta_strategy::AlphaBetaStrategy,
        tic_simulator::*,

    },
    minmax::{
        naive,
        alphabeta,
        Heuristic
    },
};

use rstest::rstest;
use std::fs;
use std::path::PathBuf;

#[rstest]
fn test_heuristic(#[files("positions/*.txt")] file_path: PathBuf) {
    let board_str = fs::read_to_string(file_path)
            .expect("Failed to read board file");

        let board_result: BoardResult = board_str.as_str().into();
        let board = board_result.into_inner().unwrap();
        let bitmap: BitBoard = board.clone().into();
        todo!()
        // println!("{}", board);
        // assert_eq!(AlmostWinHeuristic::score(&board), AlmostWinHeuristic::score(&bitmap));
}