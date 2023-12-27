use crate::{
    tic_array_repr::{tic_simulator::Strategy, tic::*},
    minmax::{*, alphabeta::*}
};

pub struct AlphaBetaStrategy {
    steps_to_search: usize,
}

impl AlphaBetaStrategy {
    pub fn new(steps_to_search: usize) -> AlphaBetaStrategy {
        AlphaBetaStrategy { steps_to_search }
    }
}

impl Strategy for AlphaBetaStrategy {
    fn get_move(&self, game_state: &Board) -> Option<Board> {
        match game_state.who_turn() {
            Player::Player1 => get_move_player1(self.steps_to_search, game_state),
            Player::Player2 => get_move_player2(self.steps_to_search, game_state),
        }
    }
}

fn get_move_player1(depth: usize, game_state: &Board) -> Option<Board> {
    if game_state.next_states().is_empty() {
        return None
    }
    let (s, score) = min_score::<Board, AlmostWinHeuristic>(game_state, depth, None, None);
    Some(s)
}

fn get_move_player2(depth: usize, game_state: &Board) -> Option<Board> {
    if game_state.next_states().is_empty() {
        return None
    }
    let (s, score) = max_score::<Board, AlmostWinHeuristic>(game_state, depth, None, None);
    Some(s)
}