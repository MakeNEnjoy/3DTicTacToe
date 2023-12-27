use std::fmt::Debug;

pub trait GameState: Sized {
    fn next_states(&self) -> Vec<Self>;
}

pub trait Heuristic<T: GameState> {
    type Score: Copy + PartialEq + PartialOrd + Debug;
    fn score(game_state: &T) -> Self::Score;
}

pub mod naive;
pub mod alphabeta;