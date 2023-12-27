use tic_solved::tic_array_repr::{
    tic::*,
    alphabeta_strategy::AlphaBetaStrategy as S3,
    tic_simulator::*
};

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let board = Board::new();
    let strategy1 = S3::new(9);
    let strategy2 = S3::new(5);
    run_game(board, strategy1, strategy2);
}