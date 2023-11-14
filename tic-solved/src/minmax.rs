use crate::game::*;

pub fn minmax_score<T, U>(root: T, steps_to_search: usize) -> <T as HeuristicGameState<U>>::Score
where T: HeuristicGameState<U, StateImplementation = T>,
{
    max_score(root, steps_to_search)
}

fn max_score<T, U>(root: T, steps_to_search: usize) -> <T as HeuristicGameState<U>>::Score
where T: HeuristicGameState<U, StateImplementation = T>,
{
    let states = root.next_states();
    if states.is_empty() || steps_to_search == 0 {
        return root.score();
    }
    let mut best_score = None;
    for state in states {
        let score = min_score(state, steps_to_search-1);
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

pub fn min_score<T,U>(root: T, steps_to_search: usize) -> <T as HeuristicGameState<U>>::Score
where T: HeuristicGameState<U, StateImplementation = T>,
{
    let states = root.next_states();
    if states.is_empty() || steps_to_search == 0 {
        return root.score();
    }
    let mut best_score = None;
    for state in states {
        let score = max_score(state, steps_to_search-1);
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
    use super::*;
    // use crate::tic::*;
    // fn test_empty_board() {
    //     let board = Board::new();

        
    // }


}