//python:replace {engine.rs}
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let seed = parse_input!(input_line, u64);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let score = parse_input!(input_line, i32);
    let board = 0u64;
    for i in 0..4 as usize {
        let mut inputs = String::new();
        io::stdin().read_line(&mut inputs).unwrap();
        for (j,val) in inputs.split_whitespace().enumerate() {
            let cell = parse_input!(val, i32);
            let board = board | ((cell as u64) << (4 * (i * 4 + j)));
        }
    }
    eprintln!("Seed: {}", seed);
    let start = std::time::Instant::now();
    set_up(seed);
    eprintln!("set_up: {:?}", start.elapsed());
    let mut board = Board { data: Data(board), turn: 0 };
    let mut next = 1;
    let mut ans = String::new();
    let mut c = "U";
    loop {
        let moves = unsafe { board.moves() };
        if next == 1 {
            if board.data != moves.down {
                board = board.spawn(moves.down, moves.free_ud);
                c = "D";
            } else if board.data != moves.right {
                board = board.spawn(moves.right, moves.free_rl);
                c = "R";
            } else if board.data != moves.up {
                board = board.spawn(moves.up, moves.free_ud);
            } else {
                break;
            }
        } else {
            if board.data != moves.right {
                board = board.spawn(moves.right, moves.free_rl);
                c = "R";
            } else if board.data != moves.down {
                board = board.spawn(moves.down, moves.free_ud);
                c = "D";
            } else if board.data != moves.up {
                board = board.spawn(moves.up, moves.free_ud);
            } else {
                break;
            }
        }
        ans.push_str(c);
        next = 1 - next;
    }
    println!("{}", ans);
}
