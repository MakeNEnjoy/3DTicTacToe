use crate::game::*;

pub fn minmax_score<T>(root: T, steps_to_search: usize) -> <T as HeuristicGameState>::Score
where T: HeuristicGameState<StateImplementation = T>,
{
    minmax_score_helper(root, steps_to_search, true)
}

pub fn minmax_score_helper<T>(root: T, steps_to_search: usize, max: bool) -> <T as HeuristicGameState>::Score
where T: HeuristicGameState<StateImplementation = T>,
{
    let states = root.next_states();
    if states.is_empty() || steps_to_search == 0 {
        return root.score();
    }
    let mut best_score = None;
    for state in states {
        let score = minmax_score_helper(state, steps_to_search-1, !max);
        if let Some(s) = best_score {
            if (s < score && max) || (s > score && !max) {
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