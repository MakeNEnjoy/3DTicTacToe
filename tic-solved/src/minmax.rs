use std::marker::PhantomData;

use crate::game::*;

fn max_score<T: GameState, U: Heuristic<T>>(root: &T, steps_to_search: usize) -> U::Score {
    let states = root.next_states();
    if states.is_empty() || steps_to_search == 0 {
        return U::score(root);
    }
    let mut best_score = None;
    for state in states {
        let score = min_score::<T, U>(&state, steps_to_search-1);
        if let Some(bs) = best_score {
            if bs < score {
                best_score = Some(score)
            }
        } else {
            best_score = Some(score)
        }
    }
    best_score.unwrap()
}

fn min_score<T: GameState, U: Heuristic<T>>(root: &T, steps_to_search: usize) -> U::Score {
    let states = root.next_states();
    if states.is_empty() || steps_to_search == 0 {
        return U::score(root);
    }
    let mut best_score = None;
    for state in states {
        let score = max_score::<T, U>(&state, steps_to_search-1);
        if let Some(bs) = best_score {
            if bs > score {
                best_score = Some(score)
            }
        } else {
            best_score = Some(score)
        }
    }
    best_score.unwrap()
}

pub struct MinimaxPlayer1Strategy<T: GameState, U: Heuristic<T>> {
    pub steps_to_search: usize,
    _phantom_game: PhantomData<T>,
    _phantom_heuristic: PhantomData<U>
}

impl<T: GameState, U: Heuristic<T>> MinimaxPlayer1Strategy<T, U> {
    pub fn new(steps_to_search: usize) -> Self {
        MinimaxPlayer1Strategy {
            steps_to_search: steps_to_search,
            _phantom_game: PhantomData,
            _phantom_heuristic: PhantomData }
    }
}

impl<T: GameState, U: Heuristic<T>> GameStrategy<T> for MinimaxPlayer1Strategy<T, U> {
    fn get_move(&self, game_state: &T) -> Option<T> {
        let states = game_state.next_states();
        let mut best_score = None;
        let mut best_board = None;
        let mut scores = Vec::new();
        for state in states {
            let score = max_score::<T, U>(&state, self.steps_to_search);
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
}

pub struct MinimaxPlayer2Strategy<T: GameState, U: Heuristic<T>> {
    pub steps_to_search: usize,
    _phantom_game: PhantomData<T>,
    _phantom_heuristic: PhantomData<U>
}

impl<T: GameState, U: Heuristic<T>> MinimaxPlayer2Strategy<T, U> {
    pub fn new(steps_to_search: usize) -> Self {
        MinimaxPlayer2Strategy {
            steps_to_search: steps_to_search,
            _phantom_game: PhantomData,
            _phantom_heuristic: PhantomData }
    }
}

impl<T: GameState, U: Heuristic<T>> GameStrategy<T> for MinimaxPlayer2Strategy<T, U> {
    fn get_move(&self, game_state: &T) -> Option<T> {
        let states = game_state.next_states();
        let mut best_score = None;
        let mut best_board = None;
        let mut scores = Vec::new();
        for state in states {
            let score = min_score::<T, U>(&state, self.steps_to_search);
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
}

mod tests {
    use super::*;
    // use crate::tic::*;
    // fn test_empty_board() {
    //     let board = Board::new();

        
    // }


}