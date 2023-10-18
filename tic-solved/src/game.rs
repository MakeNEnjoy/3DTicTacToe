use std::{fmt::Display, hash::Hash};

pub trait Move {
    fn do_move(&self) -> Box<dyn GameState>;
}

pub trait GameState: Display + PartialEq + Eq + Hash {
    fn get_moves(&self) -> Vec<Box<dyn Move + '_>>;
}