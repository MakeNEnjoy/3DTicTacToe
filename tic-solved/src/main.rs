use std::collections::HashSet;

mod game;
mod tic;

use tic::*;
use game::*;
// TODO Try make this tree thing use a generic that implements gamestate and hashable.
fn traverse_tree_with_storing(root: Box<dyn GameState>, map: &mut HashSet<String>, counter: &mut u64) {
    if map.contains(&root.id()) {
        *counter += 1;
        return;
    }
    map.insert(root.id());

    let size = map.len();
    if size % 100000 == 0 {
        println!("root.id()={:?} size={} hits={}", root.id(), size, counter);
        *counter = 0;
    }

    for child in root.next_states() {
        traverse_tree_with_storing(child, map, counter);
    }
}


fn traverse_tree(root: Box<dyn GameState>, max: &mut u32) {
    // if counter > *max {
    //     println!("{}/475", counter);
    //     *max = counter;
    // }
    if *max % 1000 == 0 {
        println!("{}", max);
    }
    *max += 1;

    for child in root.next_states() {
        traverse_tree(child, max);
    }
}

fn main() {
    let empty_board = Board::new();
    // let mut max = 0;
    // traverse_tree(Box::new(empty_board), &mut max);
    let mut map: HashSet<String> = HashSet::with_capacity(10);
    traverse_tree_with_storing(Box::new(empty_board), &mut map, &mut 0);
    println!("Hello, world!");
}
