use std::collections::HashSet;
use std::hash::Hash;

mod game;
mod tic;

use tic::*;
use game::*;

fn traverse_tree_with_storing_and_generics<T>(root: T, map: &mut HashSet<T>, counter: &mut u64) 
where T: GameState<Implementation = T> + Eq + Hash
{
    if map.contains(&root) {
        *counter += 1;
        return;
    }

    let states = root.next_states();

    map.insert(root);

    let size = map.len();
    if size % 100000 == 0 {
        println!("size={} hits={}", size, counter);
        *counter = 0;
    }

    for child in states {
        traverse_tree_with_storing_and_generics(child, map, counter);
    }
}

fn main() {
    let empty_board = Board::new();
    // let mut max = 0;
    // traverse_tree(Box::new(empty_board), &mut max);
    let mut map: HashSet<Board> = HashSet::with_capacity(10);
    traverse_tree_with_storing_and_generics(empty_board, &mut map, &mut 0);
    println!("Hello, world!");
}
