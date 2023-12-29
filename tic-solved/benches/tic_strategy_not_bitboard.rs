use std::{path::Path, fs};

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use tic_solved::tic_array::{
    naive_strategy::MinimaxStrategy as P1,
    alphabeta_strategy::AlphaBetaStrategy as S2,
    tic::*,
    tic_simulator::*,
};

fn tic_naive(board: &Board, depth: usize) {
    let strategy = P1::new(depth);
    let _board = strategy.get_move(board).unwrap();
}

fn tic_alphabeta(board: &Board, depth: usize) {
    let strategy = S2::new(depth);
    let _board = strategy.get_move(board).unwrap();
}

fn bench_single_positions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Single Positions");
    let positions_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("positions");
    let mut boards = Vec::new();

    for entry in fs::read_dir(positions_dir).expect("Failed to read positions directory") {
        let entry = entry.expect("Failed to read directory entry");
        if entry.path().is_file() && entry.path().extension().and_then(|s| s.to_str()) == Some("txt") {
            let filename = entry.file_name().into_string().expect("Failed to convert filename to string");
            let board_str = fs::read_to_string(entry.path()).expect("Failed to read board file");
            let board_result: BoardResult = board_str.as_str().into();
            let board = board_result.into_inner().unwrap();
            boards.push((filename, board));
        }
    }

    for (filename, board) in boards {
        group.bench_with_input(BenchmarkId::new("Naive", filename.clone()), &board, |b, board| b.iter(||  tic_naive(board, 4)));
        group.bench_with_input(BenchmarkId::new("Alphabeta", filename), &board, |b, board| b.iter(||  tic_alphabeta(board, 4)));
    }
}

fn bench_empty(c: &mut Criterion) {
    let board_str = "
    ... ... ... 
    ... ... ... 
    ... ... ... 
    
    ... ... ... 
    ... ... ... 
    ... ... ... 
    
    ... ... ... 
    ... ... ... 
    ... ... ... ";
    let board_result: BoardResult = board_str.into();
    let board = board_result.into_inner().unwrap();
    let mut group = c.benchmark_group("Empty");
    group.sample_size(10);
    // group.bench_function("Alphabeta Empty", |b| b.iter(|| tic_alphabeta(&board, 7)));

    for i in 3..15 {
        group.bench_with_input(BenchmarkId::new("Alphabeta Empty", i), &i, 
            |b, i| b.iter(|| tic_alphabeta(&board, *i)));
    }
}

criterion_group!(benches, bench_empty);
// criterion_group!(benches, bench_single_positions);
criterion_main!(benches);