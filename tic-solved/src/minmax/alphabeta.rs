use crate::minmax::{GameState, Heuristic};

fn optional_op<T, F>(a: Option<T>, b: Option<T>, op: F) -> Option<T>
where
    F: Fn(T, T) -> T
{
    match (a, b) {
        (Some(a_val), Some(b_val)) => Some(op(a_val, b_val)),
        (Some(a_val), None) => Some(a_val),
        (None, Some(b_val)) => Some(b_val),
        (None, None) => None,
    }
}

fn optional_max<T: PartialOrd>(a: Option<T>, b: Option<T>) -> Option<T> {
    optional_op(a, b, |a, b| if a >= b {a} else {b})
}

fn optional_min<T: PartialOrd>(a: Option<T>, b: Option<T>) -> Option<T> {
    optional_op(a, b, |a, b| if a <= b {a} else {b})
}

#[derive(PartialEq, Clone, Debug)]
struct ScoredGameState<T, S> 
where
{
    game_state: T,
    score: S,
}

impl<T, S> PartialOrd for ScoredGameState<T, S>
where
    S: PartialOrd,
    T: PartialEq
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

pub fn max_score<T, U>(root: &T, depth: usize, mut alpha: Option<U::Score>, beta: Option<U::Score>) -> (T, U::Score)
where
    T: GameState + Clone + PartialEq,
    U: Heuristic<T>
{
    let states = root.next_states();
    if states.is_empty() || depth == 0 {
        return (root.clone(), U::score(root));
    }
    let mut best_score = None;
    for state in states {
        let (_, score) = min_score::<T, U>(&state, depth-1, alpha, beta);
        best_score = optional_max(
            best_score,
            Some(ScoredGameState{game_state: state, score: score})
        );
        
        alpha = optional_max(alpha, best_score.as_ref().map(|s| s.score));
        if let Some(ref s) = best_score {
            if let Some(b) = beta {
                if s.score >= b {
                    break;
                }
            }
        }
    }

    let value = best_score.unwrap();
    (value.game_state, value.score)
}

pub fn min_score<T, U>(root: &T, depth: usize, alpha: Option<U::Score>, mut beta: Option<U::Score>) -> (T, U::Score)
where
    T: GameState + Clone + PartialEq,
    U: Heuristic<T>
{
    let states = root.next_states();
    if states.is_empty() || depth == 0 {
        return (root.clone(), U::score(root));
    }
    let mut best_score = None;
    for state in states {
        let (_, score) = max_score::<T, U>(&state, depth-1, alpha.clone(), beta.clone());
        best_score = optional_min(
            best_score,
            Some(ScoredGameState{game_state: state, score: score})
        );
        beta = optional_min(beta, best_score.as_ref().map(|s| s.score));
        if let Some(ref s) = best_score {
            if let Some(a) = alpha {
                if s.score <= a {
                    break;
                }
            }
        }
    }

    let value = best_score.unwrap();
    (value.game_state, value.score)
}

#[cfg(test)]
mod tests {
    use crate::minmax::alphabeta::{optional_max, optional_min};

    use super::ScoredGameState;

    #[test]
    fn test_optional_max() {
        let a = Some (ScoredGameState {score: 10, game_state: 60});
        let b = Some ( ScoredGameState {score: 15, game_state: 100} );
        assert_eq!(optional_max(a, b.clone()), b);
    }

    #[test]
    fn test_optional_min() {
        let a = Some (ScoredGameState {score: 10, game_state: 60});
        let b = Some ( ScoredGameState {score: 15, game_state: 100} );
        assert_eq!(optional_min(a.clone(), b), a);
    }

    #[test]
    fn test_none_order_wtf() {
        assert!(None <= Some(-3));
        assert!(None <= Some(3));
        assert!(!(None >= Some(3)));
        assert!(!(None >= Some(-3)));
    }
}