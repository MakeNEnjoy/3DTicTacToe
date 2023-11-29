use crate::tic::*;

// TODO:
// - make a HumanStrategy and make this func take two strategies.
// - Refactor this, such that it runs the game loop but doesn't have anything to do with terminal
//   by making TerminalHumanPlayerInterface that implements HumanPlayerInterface
// - Add basic logging

pub trait Strategy {
    fn get_move(&self, game_state: &Board) -> Option<Board>; // None if there are no moves to make.
}

pub fn run_game<T: Strategy, U: Strategy>(mut board: Board, strategy1: T, strategy2: U) {
    loop {
        let legal_boards = board.get_legal_boards();
        board = strategy1.get_move(&board).unwrap();
        assert!(legal_boards.contains(&board), "Strategy 1 produced illegal board");
        
        if board.get_legal_boards().is_empty() {
            break;
        }
        
        let legal_boards = board.get_legal_boards();
        board = strategy2.get_move(&board).unwrap();
        assert!(legal_boards.contains(&board), "Strategy 2 produced illegal board");

        if board.get_legal_boards().is_empty() {
            break;
        }
    }
    println!("Game ended!");
    println!("{}", board);
}
