use std::collections::HashMap;

mod game;
mod tic;

use tic::*;
use game::*;

fn traverse_tree_with_storing(root: Box<dyn GameState>) {
    let mut map: HashMap<&str , i32> = HashMap::with_capacity(10);


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

    for edge in root.get_moves() {
        let child = edge.do_move();
        traverse_tree(child, max);
    }
}

fn main() {
    let empty_board = Board::new();
    let mut max = 0;
    traverse_tree(Box::new(empty_board), &mut max);
    println!("Hello, world!");
}
