use crate::{
    tic_array::{tic_simulator::Strategy, tic::*, bitmap_tic::*},
    minmax::{*, alphabeta::*}
};

pub struct AlphaBetaBitBoardStrategy {
    steps_to_search: usize,
}

impl AlphaBetaBitBoardStrategy {
    pub fn new(steps_to_search: usize) -> AlphaBetaBitBoardStrategy {
        AlphaBetaBitBoardStrategy { steps_to_search }
    }
}

impl Strategy for AlphaBetaBitBoardStrategy {
    fn get_move(&self, game_state: &Board) -> Option<Board> {
        let bitboard: BitBoard = game_state.to_owned().into();
        match game_state.who_turn() {
            Player::Player1 => get_move_player1(self.steps_to_search, &bitboard),
            Player::Player2 => get_move_player2(self.steps_to_search, &bitboard),
        }
    }
}

fn get_move_player1(depth: usize, game_state: &BitBoard) -> Option<Board> {
    if game_state.next_states().is_empty() {
        return None
    }
    let (s, _) = min_score::<BitBoard, WinHeuristic>(game_state, depth, None, None);
    Some(s.into())
}

fn get_move_player2(depth: usize, game_state: &BitBoard) -> Option<Board> {
    if game_state.next_states().is_empty() {
        return None
    }
    let (s, _) = max_score::<BitBoard, WinHeuristic>(game_state, depth, None, None);
    Some(s.into())
}