use tic_solved::{
    tic_array_repr::{
        tic::*,
        naive_strategy::MinimaxStrategy,
        alphabeta_strategy::AlphaBetaStrategy,
        tic_simulator::*,
    },
    minmax::{
        naive,
        alphabeta
    },
};

use rstest::rstest;
use std::fs;
use std::path::PathBuf;

#[rstest]
fn test_position_min(#[files("positions/*.txt")] file_path: PathBuf) {
    let board_str = fs::read_to_string(file_path)
        .expect("Failed to read board file");

    let board_result: BoardResult = board_str.as_str().into();
    let board = board_result.into_inner().unwrap();
    let score1 = naive::min_score::<Board, AlmostWinHeuristic>(&board, 3);
    let (suggested_board, score2) = alphabeta::min_score::<Board, AlmostWinHeuristic>(&board, 3, None, None);
    assert_eq!(score1, score2);
    assert_ne!(board, suggested_board);
}

#[rstest]
fn test_position_max(#[files("positions/*.txt")] file_path: PathBuf) {
    let board_str = fs::read_to_string(file_path)
        .expect("Failed to read board file");

    let board_result: BoardResult = board_str.as_str().into();
    let board = board_result.into_inner().unwrap();
    let score1 = naive::max_score::<Board, AlmostWinHeuristic>(&board, 2);
    let (suggested_board, score2) = alphabeta::max_score::<Board, AlmostWinHeuristic>(&board, 2, None, None);
    assert_eq!(score1, score2);
    assert_ne!(board, suggested_board);
}

#[rstest]
fn test_strategy(#[files("positions/*.txt")] file_path: PathBuf) {
    let board_str = fs::read_to_string(file_path)
    .expect("Failed to read board file");

    let board_result: BoardResult = board_str.as_str().into();
    let board = board_result.into_inner().unwrap(); 
    let strategy1 = MinimaxStrategy::new(3);
    let strategy2 = AlphaBetaStrategy::new(3);
    let board1 = strategy1.get_move(&board).unwrap();
    let board2 = strategy2.get_move(&board).unwrap();
    assert_eq!(board1, board2);
}