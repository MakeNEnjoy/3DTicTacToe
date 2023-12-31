use tic_solved::tic_array::{
    tic::*,
    alphabeta_strategy_bitboard::AlphaBetaBitBoardStrategy as S4,
    alphabeta_strategy::AlphaBetaStrategy as S3,
    naive_strategy::MinimaxStrategy as S2,
    tic_simulator::*
};

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let board = Board::new();
    let strategy1 = S3::new(2);
    let strategy2 = S4::new(12);
    run_game(board, strategy2, strategy1);
}