use crate::{
    engine::{set_up, Board},
    network::load_network,
};

#[macro_use]
mod data;
mod bin;
mod engine;
mod input;
mod network;
mod score;
mod timer;

fn main() {
    let start = std::time::Instant::now();
    set_up(290797);
    load_network();
    let mut board = Board::new();
    let mut count = 0;
    loop {
        count += 1;
        let m = board.auto_ai();
        if let Some(b) = m.1 {
            board = b;
        } else {
            break;
        }
        println!("Count: {}", count);
    }
    println!("{:?}", start.elapsed());
    println!("{}", count);
    println!("{}", board.data);
}
