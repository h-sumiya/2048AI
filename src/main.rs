use crate::engine::temp;

mod engine;

const SIZE: usize = 4;

#[derive(Clone, Debug)]
struct Board {
    seed: usize,
    board: [usize; SIZE * SIZE],
}

fn main() {
    println!("Hello, world!");
    temp();
}
