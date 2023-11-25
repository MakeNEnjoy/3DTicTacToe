pub trait GameState: Sized {
    fn next_states(&self) -> Vec<Self>;
}

pub trait Heuristic<T: GameState> {
    type Score: Copy + PartialEq + PartialOrd;
    fn score(game_state: &T) -> Self::Score;
}

pub trait GameStrategy<T: GameState> {
    fn get_move(&self, game_state: &T) -> Option<T>; // None if there are no moves to make.
}

pub trait HumanPlayerInterface {
    // TODO:
    // Make a trait that could be run asynchronously for game_terminal_interface.
}