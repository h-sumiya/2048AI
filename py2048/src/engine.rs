use crate::table::TABLE;
use std::{arch::x86_64::_pext_u64, fmt};

#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub seed: u64,
    pub data: u64,
}

pub struct Moves {
    pub up: Board,
    pub down: Board,
    pub left: Board,
    pub right: Board,
}

const COL_MASK: u64 = 15 | (15 << 16) | (15 << 32) | (15 << 48);
const COL_MASKS: [u64; 4] = [COL_MASK, COL_MASK << 4, COL_MASK << 8, COL_MASK << 12];

impl Board {
    pub fn new(seed: u64) -> Self {
        let mut board = Board { seed, data: 0 };
        board.spawn(16);
        board.spawn(15);
        board
    }

    fn update_seed(&mut self) {
        self.seed = self.seed * self.seed % 50515093;
    }

    fn next(&self, data: u64) -> Self {
        Board {
            seed: self.seed,
            data,
        }
    }

    #[target_feature(enable = "bmi2")]
    pub unsafe fn moves(&self) -> Moves {
        let mut right = 0u64;
        let mut left = 0u64;
        let mut up = 0u64;
        let mut down = 0u64;
        let mut free_rl = 0;
        let mut free_ud = 0;
        for i in 0..4 {
            let row = (self.data >> (i * 16)) & 0xffff;
            let col = _pext_u64(self.data, COL_MASKS[i]);
            let row_data = TABLE.0.get_unchecked(row as usize);
            let col_data = TABLE.1.get_unchecked(col as usize);
            right |= (row_data.right as u64) << (i * 16);
            left |= (row_data.left as u64) << (i * 16);
            up |= col_data.up << (i * 4);
            down |= col_data.down << (i * 4);
            free_rl += row_data.free;
            free_ud += col_data.free;
        }
        let mut up = self.next(up);
        let mut down = self.next(down);
        let mut left = self.next(left);
        let mut right = self.next(right);
        if self.data != up.data {
            up.spawn(free_ud);
        }
        if self.data != down.data {
            down.spawn(free_ud);
        }
        if self.data != left.data {
            left.spawn(free_rl);
        }
        if self.data != right.data {
            right.spawn(free_rl);
        }
        Moves {
            up,
            down,
            left,
            right,
        }
    }

    fn spawn(&mut self, free: usize) {
        let index = self.seed as usize % free;
        let mask = 0xf;
        let mut pos = 0;
        for x in 0..4 {
            for y in (0..16).step_by(4) {
                let p = x + y;
                let shift = p * 4;
                let mask = mask << shift;
                if mask & self.data == 0 {
                    if pos == index {
                        let value = if self.seed & 0x10 == 0 { 1u64 } else { 2 };
                        self.data |= value << shift;
                        self.update_seed();
                        return;
                    }
                    pos += 1;
                }
            }
        }
        panic!("{} {}", index, pos);
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Seed: {}", self.seed)?;
        let mut data = [0i32; 16];
        for i in 0..16 {
            data[i] = ((self.data >> (i * 4)) & 0xf) as i32;
        }
        for (i, data) in data.iter().enumerate() {
            if i % 4 == 0 {
                write!(f, "\n")?;
            }
            if *data != 0 {
                write!(f, "{:8}", 2i32.pow(*data as u32))?;
            } else {
                write!(f, "{:8}", data)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board() {
        let board = Board::new(290797);
        println!("{}", board);
        let moves = unsafe { board.moves() };
        println!("{}", moves.down);
    }
}
