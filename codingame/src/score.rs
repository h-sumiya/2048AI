use crate::{engine::Board, network::NETWORK}; //python:del

impl Board {
    fn score(&self) -> f32 {
        unsafe { NETWORK.calc(&self.to_t8()) }
    }

    pub fn auto_ai(&self) -> (char, Option<Self>) {
        self.ai(5)
    }

    pub fn ai(&self, depth: usize) -> (char, Option<Self>) {
        let mut score = -100_000_000f32;
        let moves = unsafe { self.moves() };
        let mut res = None;
        let mut c = 'U';
        if self.data != moves.down {
            let board = self.spawn(moves.down, moves.free_ud);
            let s = board.node(depth - 1);
            if s > score {
                res = Some(board);
                score = s;
                c = 'D';
            }
        }
        if self.data != moves.right {
            let board = self.spawn(moves.right, moves.free_rl);
            let s = board.node(depth - 1);
            if s > score {
                res = Some(board);
                score = s;
                c = 'R';
            }
        }
        if self.data != moves.up {
            let board = self.spawn(moves.up, moves.free_ud);
            let s = board.node(depth - 1);
            if s > score {
                res = Some(board);
                score = s;
                c = 'U';
            }
        }
        if self.data != moves.left {
            let board = self.spawn(moves.left, moves.free_rl);
            let s = board.node(depth - 1);
            if s > score {
                res = Some(board);
                c = 'L';
            }
        }
        (c, res)
    }

    fn node(&self, depth: usize) -> f32 {
        if depth == 0 {
            return self.score();
        }
        let mut score = -100_000f32;
        let moves = unsafe { self.moves() };
        if self.data != moves.down {
            let board = self.spawn(moves.down, moves.free_ud);
            score = score.max(board.node(depth - 1));
        }
        if self.data != moves.right {
            let board = self.spawn(moves.right, moves.free_rl);
            score = score.max(board.node(depth - 1));
        }
        if self.data != moves.up {
            let board = self.spawn(moves.up, moves.free_ud);
            score = score.max(board.node(depth - 1));
        }
        if self.data != moves.left {
            let board = self.spawn(moves.left, moves.free_rl);
            score = score.max(board.node(depth - 1));
        }
        score
    }
}
