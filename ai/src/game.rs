use crate::{engine::Board, nn::Network};

#[derive(Debug, Clone, Copy)]
pub struct Game {
    pub network: Network,
    pub score: usize,
}

impl Game {
    pub fn new(network: Network) -> Self {
        Game { network, score: 0 }
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn run_with_ai(&mut self, depth: usize) -> Board {
        let mut board = Board::new();
        let mut result = 0;
        loop {
            let b = self.ai(depth, board);
            if let Some(b) = b {
                board = b;
                result += 1;
            } else {
                break;
            }
        }
        self.score = result;
        board
    }

    pub fn ai(&self, depth: usize, board: Board) -> Option<Board> {
        let mut score = -100_000_001f32;
        let moves = unsafe { board.moves() };
        let mut res = None;
        if board.data != moves.down {
            let board = board.spawn(moves.down, moves.free_ud);
            let s = self.node(depth - 1, board);
            if score < s {
                res = Some(board);
                score = s;
            }
        }
        if board.data != moves.right {
            let board = board.spawn(moves.right, moves.free_rl);
            let s = self.node(depth - 1, board);
            if score < s {
                res = Some(board);
                score = s;
            }
        }
        if board.data != moves.up {
            let board = board.spawn(moves.up, moves.free_ud);
            let s = self.node(depth - 1, board);
            if score < s {
                res = Some(board);
                score = s;
            }
        }
        if board.data != moves.left {
            let board = board.spawn(moves.left, moves.free_rl);
            let s = self.node(depth - 1, board);
            if score < s {
                res = Some(board);
            }
        }
        res
    }

    fn node(&self, depth: usize, board: Board) -> f32 {
        if depth == 0 {
            return self.network.calc(&board.to_t8());
        }
        let mut score = -100_000f32;
        let moves = unsafe { board.moves() };
        if board.data != moves.down {
            let board = board.spawn(moves.down, moves.free_ud);
            score = score.max(self.node(depth - 1, board));
        }
        if board.data != moves.right {
            let board = board.spawn(moves.right, moves.free_rl);
            score = score.max(self.node(depth - 1, board));
        }
        if board.data != moves.up {
            let board = board.spawn(moves.up, moves.free_ud);
            score = score.max(self.node(depth - 1, board));
        }
        if board.data != moves.left {
            let board = board.spawn(moves.left, moves.free_rl);
            score = score.max(self.node(depth - 1, board));
        }
        score
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
