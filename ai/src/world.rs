use crate::{engine::Board, nn::Network};

pub struct Game {
    pub network: Network,
    pub score: usize,
}

impl Game {
    pub fn new(network: Network) -> Self {
        Game { network, score: 0 }
    }

    pub fn run(&mut self) -> Board {
        let mut result = 0;
        let mut board = Board::new();
        let mut next = None;
        loop {
            let moves = unsafe { board.moves() };
            let mut max = -100_000f32;
            if moves.up != board.data {
                let b = board.spawn(moves.up, moves.free_ud);
                let score = self.network.calc(&b.to_t8());
                max = score;
                next = Some(b);
            }
            if moves.down != board.data {
                let b = board.spawn(moves.down, moves.free_ud);
                let score = self.network.calc(&b.to_t8());
                if score > max {
                    max = score;
                    next = Some(b);
                }
            }
            if moves.left != board.data {
                let b = board.spawn(moves.left, moves.free_rl);
                let score = self.network.calc(&b.to_t8());
                if score > max {
                    max = score;
                    next = Some(b);
                }
            }
            if moves.right != board.data {
                let b = board.spawn(moves.right, moves.free_rl);
                let score = self.network.calc(&b.to_t8());
                if score > max {
                    next = Some(b);
                }
            }
            if let Some(b) = next {
                board = b;
                next = None;
                result += 1;
            } else {
                break;
            }
        }
        self.score = result;
        board
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::engine::set_seed;
    use rand::prelude::*;
    #[test]
    fn test() {
        let mut rng = thread_rng();
        set_seed(123456);
        let network = Network::new(&mut rng);
        let mut game = Game::new(network);
        let board = game.run();
        println!("{}", board.data);
        println!("{}", game.score);
    }
}
