use std::marker::PhantomData;

pub trait GameState: Sized {
    fn next_states(&self) -> Vec<Self>;
}

pub trait Heuristic<T: GameState> {
    type Score: Copy + PartialEq + PartialOrd;
    fn score(game_state: &T) -> Self::Score;
}

pub fn max_score<T: GameState, U: Heuristic<T>>(root: &T, steps_to_search: usize) -> U::Score {
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

pub fn min_score<T: GameState, U: Heuristic<T>>(root: &T, steps_to_search: usize) -> U::Score {
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

mod tests {
    // use super::*;
    // use crate::tic::*;
    // fn test_empty_board() {
    //     let board = Board::new();

        
    // }


}