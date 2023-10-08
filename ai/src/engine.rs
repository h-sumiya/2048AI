use once_cell::sync::Lazy;
use std::arch::x86_64::_pext_u64;
use std::fmt;
use std::mem::transmute;

use crate::nn::T8;

fn u16_to_data(d: u16) -> [i32; 4] {
    let mut data = [0; 4];
    for i in 0..4 {
        data[i] = ((d >> (i * 4)) & 0xf) as i32;
    }
    data
}

fn data_to_u16(data: &[i32; 4]) -> u16 {
    let mut d = 0;
    for i in 0..4 {
        d |= (data[i] as u16) << (i * 4);
    }
    d
}

fn data_to_u64(data: &[i32; 4]) -> u64 {
    let mut d = 0;
    for i in 0..4 {
        d |= (data[i] as u64) << (i * 16);
    }
    d
}

fn calc_line_l(data: &[i32; 4]) -> [i32; 4] {
    let mut res = [0; 4];
    let mut index = 0;
    let mut flag = false;
    for num in data.iter() {
        if *num != 0 {
            if flag && res[index - 1] == *num {
                if res[index - 1] != 15 {
                    res[index - 1] += 1;
                }
                flag = false;
            } else {
                res[index] = *num;
                index += 1;
                flag = true;
            }
        }
    }
    res
}

fn calc_line_r(data: &[i32; 4]) -> [i32; 4] {
    let mut res = [0; 4];
    let mut index = 4;
    let mut flag = false;
    for num in data.iter().rev() {
        if *num != 0 {
            if flag && res[index] == *num {
                if res[index] != 15 {
                    res[index] += 1;
                }
                flag = false;
            } else {
                res[index - 1] = *num;
                index -= 1;
                flag = true;
            }
        }
    }
    res
}

struct RowData {
    right: u16,
    left: u16,
    free: usize,
}

impl RowData {
    fn new(data_r: &[i32; 4], data_l: &[i32; 4]) -> Self {
        RowData {
            right: data_to_u16(data_r),
            left: data_to_u16(data_l),
            free: data_r.iter().filter(|&x| *x == 0).count(),
        }
    }
}

struct ColData {
    up: u64,
    down: u64,
    free: usize,
}

impl ColData {
    fn new(data_u: &[i32; 4], data_d: &[i32; 4]) -> Self {
        ColData {
            up: data_to_u64(data_u),
            down: data_to_u64(data_d),
            free: data_u.iter().filter(|&x| *x == 0).count(),
        }
    }
}

fn calc_line(data: &[i32; 4]) -> (RowData, ColData) {
    let data_l = calc_line_l(data);
    let data_r = calc_line_r(data);
    (
        RowData::new(&data_r, &data_l),
        ColData::new(&data_l, &data_r),
    )
}

static TABLE: Lazy<(Vec<RowData>, Vec<ColData>)> = Lazy::new(|| {
    let mut row_data: Vec<RowData> = Vec::with_capacity(65536);
    let mut col_data: Vec<ColData> = Vec::with_capacity(65536);
    for i in 0..=65535u16 {
        let data = u16_to_data(i);
        let (row, col) = calc_line(&data);
        row_data.push(row);
        col_data.push(col);
    }
    (row_data, col_data)
});

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Data(pub u64);

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut data = [0i32; 16];
        for i in 0..16 {
            data[i] = ((self.0 >> (i * 4)) & 0xf) as i32;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Seed {
    seed: usize,
    value: u8,
}

static mut SEED_TABLE: Lazy<Vec<Seed>> = Lazy::new(|| Vec::new());
pub fn set_seed(mut seed: u64) {
    let count = 100_000;
    let mut seed_table: Vec<Seed> = Vec::with_capacity(count);
    for _ in 0..count {
        let seed_data = Seed {
            seed: seed as usize,
            value: if seed & 0x10 == 0 { 1 } else { 2 },
        };
        seed_table.push(seed_data);
        seed = seed * seed % 50515093;
    }
    unsafe {
        *SEED_TABLE = seed_table;
    }
}

pub struct Board {
    pub turn: usize,
    pub data: Data,
}

#[derive(Debug)]
pub struct Moves {
    pub right: Data,
    pub left: Data,
    pub free_rl: usize,
    pub up: Data,
    pub down: Data,
    pub free_ud: usize,
}

const COL_MASK: u64 = 15 | (15 << 16) | (15 << 32) | (15 << 48);
const COL_MASKS: [u64; 4] = [COL_MASK, COL_MASK << 4, COL_MASK << 8, COL_MASK << 12];

static F_TABLE: Lazy<Vec<[f32; 4]>> = Lazy::new(|| {
    let mut f_table = Vec::with_capacity(65536);
    for i in 0..=65535u16 {
        let data = u16_to_data(i);
        let mut f = [0f32; 4];
        for j in 0..4 {
            f[j] = data[j] as f32 / 15f32;
        }
        f_table.push(f);
    }
    f_table
});

impl Board {
    pub fn new() -> Self {
        let s0 = Self {
            turn: 0,
            data: Data(0),
        };
        let s1 = s0.spawn(Data(0), 16);
        s1.spawn(s1.data, 15)
    }

    pub fn to_t8(&self) -> [T8; 2] {
        unsafe {
            let t1 = [
                F_TABLE
                    .get_unchecked(((self.data.0 >> 0) & 0xffff) as usize)
                    .clone(),
                F_TABLE
                    .get_unchecked(((self.data.0 >> 16) & 0xffff) as usize)
                    .clone(),
            ];
            let t2 = [
                F_TABLE
                    .get_unchecked(((self.data.0 >> 32) & 0xffff) as usize)
                    .clone(),
                F_TABLE
                    .get_unchecked(((self.data.0 >> 48) & 0xffff) as usize)
                    .clone(),
            ];
            [T8::new(transmute(t1)), T8::new(transmute(t2))]
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
            let row = (self.data.0 >> (i * 16)) & 0xffff;
            let col = _pext_u64(self.data.0, COL_MASKS[i]);
            let row_data = TABLE.0.get_unchecked(row as usize);
            let col_data = TABLE.1.get_unchecked(col as usize);
            right |= (row_data.right as u64) << (i * 16);
            left |= (row_data.left as u64) << (i * 16);
            up |= col_data.up << (i * 4);
            down |= col_data.down << (i * 4);
            free_rl += row_data.free;
            free_ud += col_data.free;
        }

        Moves {
            right: Data(right),
            left: Data(left),
            free_rl,
            up: Data(up),
            down: Data(down),
            free_ud,
        }
    }

    pub fn spawn(&self, data: Data, free: usize) -> Self {
        let seed = unsafe { SEED_TABLE.get_unchecked(self.turn) };
        let index = seed.seed % free;
        let mask = 0xf;
        let mut pos = 0;
        for x in 0..4 {
            for y in (0..16).step_by(4) {
                let p = x + y;
                let shift = p * 4;
                let mask = mask << shift;
                if mask & data.0 == 0 {
                    if pos == index {
                        return Self {
                            turn: self.turn + 1,
                            data: Data(data.0 | ((seed.value as u64) << shift)),
                        };
                    }
                    pos += 1;
                }
            }
        }
        panic!("{} {}", index, pos);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        set_seed(123456);
        let board = Board::new();
        println!("{}", board.data);
        println!("{:?}", board.to_t8());
    }
}
