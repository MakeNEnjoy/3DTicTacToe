pub trait GameState {
    type Implementation;
    fn next_states(&self) -> Vec<Self::Implementation>;
}