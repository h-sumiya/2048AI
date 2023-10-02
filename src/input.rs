use crate::engine::set_up; //python:del
use crate::engine::Board; //python:del
use crate::engine::Data; //python:del
use std::io; //python:del

#[allow(dead_code)]
impl Board {
    fn from_input() -> Self {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let seed = buf.trim().parse::<u64>().unwrap();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut data = 0u64;
        for i in 0..4 as usize {
            buf.clear();
            io::stdin().read_line(&mut buf).unwrap();
            for (j, val) in buf.split_whitespace().enumerate() {
                let cell = val.trim().parse::<u64>().unwrap();
                let cell = match cell {
                    0 => 0u64,
                    2 => 1,
                    4 => 2,
                    _ => panic!("init errr"),
                };
                data = data | ((cell as u64) << (4 * (i * 4 + j)));
            }
        }
        set_up(seed);
        Board {
            data: Data(data),
            turn: 0,
        }
    }
}
