use tic_solved::tic_array::{
    tic::*,
    alphabeta_strategy::AlphaBetaStrategy as S3,
    naive_strategy::MinimaxStrategy as S2,
    tic_simulator::*
};

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let board = Board::new();
    let strategy1 = S3::new(1);
    let strategy2 = S3::new(5);
    run_game(board, strategy1, strategy2);
}