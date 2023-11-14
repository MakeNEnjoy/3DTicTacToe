pub trait GameState {
    type StateImplementation;
    fn next_states(&self) -> Vec<Self::StateImplementation>;
}

pub trait HeuristicGameState<T>: GameState {
    type Score: Copy + PartialEq + PartialOrd;
    fn score(&self) -> Self::Score;
}