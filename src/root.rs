//python:replace use std::fmt;
//python:replace use std::mem;
//python:replace use std::mem::transmute;
//python:replace use std::io;
//python:replace {engine.rs}
//python:replace {input.rs}

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn main() {
    let mut board = Board::from_input();
    let mut next = 1;
    let mut ans = String::new();
    loop {
        let mut c = "U";
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
