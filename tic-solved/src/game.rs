pub trait GameState {
    type StateImplementation;
    fn next_states(&self) -> Vec<Self::StateImplementation>;
}

pub trait HeuristicGameState<T>: GameState {
    type Score: Copy + PartialEq + PartialOrd;
    fn score(&self) -> Self::Score;
}

pub trait GameStrategy<T: HeuristicGameState<U>, U> {
    fn get_move(&self, game_state: &T) -> Option<T>; // None if there are no moves to make.
}

pub trait HumanPlayerInterface {
    // TODO:
    // Make a trait that could be run asynchronously for game_terminal_interface.
}