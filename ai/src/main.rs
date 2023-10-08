mod engine;
mod nn;
mod world;

use rand::prelude::*;

use crate::engine::set_seed;

fn main() {
    set_seed(123456);
    let mut rng = thread_rng();
    let network = nn::Network::new(&mut rng);
    let mut game = world::Game::new(network);
    let board = game.run();
    println!("{}", board.data);
    println!("{}", game.score);
}
