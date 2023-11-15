use crate::game::*;

fn max_score<T, U>(root: &T, steps_to_search: usize) -> <T as HeuristicGameState<U>>::Score
where T: HeuristicGameState<U, StateImplementation = T>,
{
    let states = root.next_states();
    if states.is_empty() || steps_to_search == 0 {
        return root.score();
    }
    let mut best_score = None;
    for state in states {
        let score = min_score(&state, steps_to_search-1);
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

pub fn min_score<T,U>(root: &T, steps_to_search: usize) -> <T as HeuristicGameState<U>>::Score
where T: HeuristicGameState<U, StateImplementation = T>,
{
    let states = root.next_states();
    if states.is_empty() || steps_to_search == 0 {
        return root.score();
    }
    let mut best_score = None;
    for state in states {
        let score = max_score(&state, steps_to_search-1);
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

pub struct MinimaxPlayer1Strategy {
    pub steps_to_search: usize
}

impl<T: HeuristicGameState<U, StateImplementation = T>, U> GameStrategy<T, U> for MinimaxPlayer1Strategy {
    fn get_move(&self, game_state: &T) -> Option<T> {
        let states = game_state.next_states();
        let mut best_score = None;
        let mut best_board = None;
        let mut scores = Vec::new();
        for state in states {
            let score = max_score(&state, self.steps_to_search);
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

pub struct MinimaxPlayer2Strategy {
    pub steps_to_search: usize
}

impl<T: HeuristicGameState<U, StateImplementation = T>, U> GameStrategy<T, U> for MinimaxPlayer2Strategy {
    fn get_move(&self, game_state: &T) -> Option<T> {
        let states = game_state.next_states();
        let mut best_score = None;
        let mut best_board = None;
        let mut scores = Vec::new();
        for state in states {
            let score = min_score(&state, self.steps_to_search);
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