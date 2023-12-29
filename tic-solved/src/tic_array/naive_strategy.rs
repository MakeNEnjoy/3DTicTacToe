use crate::{
    tic_array::{tic_simulator::Strategy, tic::*},
    minmax::{*, naive::*}
};

pub struct MinimaxStrategy {
    steps_to_search: usize,
}

impl MinimaxStrategy {
    pub fn new(steps_to_search: usize) -> MinimaxStrategy {
        MinimaxStrategy { steps_to_search }
    }
}

impl Strategy for MinimaxStrategy {
    fn get_move(&self, game_state: &Board) -> Option<Board> {
        match game_state.who_turn() {
            Player::Player1 => get_move_player1(self.steps_to_search, game_state),
            Player::Player2 => get_move_player2(self.steps_to_search, game_state)
        }
    }
}

fn get_move_player1(steps_to_search: usize, game_state: &Board) -> Option<Board> {
    let states = game_state.next_states();
    let mut best_score = None;
    let mut best_board = None;
    let mut scores = Vec::new();
    for state in states {
        let score = max_score::<Board, AlmostWinHeuristic>(&state, steps_to_search-1);
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
    best_board
}

fn get_move_player2(steps_to_search: usize, game_state: &Board) -> Option<Board> {
    let states = game_state.next_states();
    let mut best_score = None;
    let mut best_board = None;
    let mut scores = Vec::new();
    for state in states {
        let score = min_score::<Board, AlmostWinHeuristic>(&state, steps_to_search-1);
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
    best_board
}