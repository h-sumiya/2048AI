use crate::engine::Board; //python:del

impl Board {
    pub fn score(&self) -> usize {
        let mut score = 0;
        let mut before = 1000;
        for i in [0, 1, 2, 3, 7, 6, 5, 4, 8, 9, 10, 11, 15, 14, 13, 12] {
            let shift = i * 4;
            let mask = 0xfu64 << (shift);
            let num = (self.data.0 & mask) >> shift;
            if num > before {
                score += num.pow(3);
            }
            before = num;
        }
        score as usize
    }

    pub fn auto_ai(&self) -> (char, Option<Self>) {
        let mut free = 0;
        for i in 0..16 {
            let shift = i * 4;
            let mask = 0xf << shift;
            if self.data.0 & mask == 0 {
                free += 1;
            }
        }
        let depth = (11 - free).max(5).min(9);
        self.ai(4)
    }

    pub fn ai(&self, depth: usize) -> (char, Option<Self>) {
        let mut score = 100_000_001;
        let moves = unsafe { self.moves() };
        let mut res = None;
        let mut c = 'U';
        if self.data != moves.down {
            let board = self.spawn(moves.down, moves.free_ud);
            let s = board.node(depth - 1);
            if s < score {
                res = Some(board);
                score = s;
                c = 'D';
            }
        }
        if self.data != moves.right {
            let board = self.spawn(moves.right, moves.free_rl);
            let s = board.node(depth - 1);
            if s < score {
                res = Some(board);
                score = s;
                c = 'R';
            }
        }
        if self.data != moves.up {
            let board = self.spawn(moves.up, moves.free_ud);
            let s = board.node(depth - 1);
            if s < score {
                res = Some(board);
                score = s;
                c = 'U';
            }
        }
        if self.data != moves.left {
            let board = self.spawn(moves.left, moves.free_rl);
            let s = board.node(depth - 1);
            if s < score {
                res = Some(board);
                c = 'L';
            }
        }
        (c, res)
    }

    fn node(&self, depth: usize) -> usize {
        if depth == 0 {
            return self.score();
        }
        let mut score = 100_000_000;
        let moves = unsafe { self.moves() };
        if self.data != moves.down {
            let board = self.spawn(moves.down, moves.free_ud);
            score = score.min(board.node(depth - 1));
        }
        if self.data != moves.right {
            let board = self.spawn(moves.right, moves.free_rl);
            score = score.min(board.node(depth - 1));
        }
        if self.data != moves.up {
            let board = self.spawn(moves.up, moves.free_ud);
            score = score.min(board.node(depth - 1));
        }
        if self.data != moves.left {
            let board = self.spawn(moves.left, moves.free_rl);
            score = score.min(board.node(depth - 1));
        }
        score
    }
}
