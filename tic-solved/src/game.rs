use std::{fmt::Display, hash::Hash};

pub trait GameState: Display {
    fn next_states(&self) -> Vec<Box<dyn GameState>>;
    fn id(&self) -> String; // this is so bad.
}