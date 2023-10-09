//python:replace use std::fmt;
//python:replace use std::mem;
//python:replace use std::mem::transmute;
//python:replace use std::io;
//python:replace use std::arch::x86_64::*;
//python:replace use std::ops;
//python:replace {data.rs}
//python:replace {bin.rs}
//python:replace {engine.rs}
//python:replace {input.rs}
//python:replace {network.rs}
//python:replace {score.rs}
//python:replace {timer.rs}

fn main() {
    let mut timer = TimeManager::new();
    load_network();
    let mut board = Board::from_input();
    let mut ans = String::with_capacity(20000);
    loop {
        let m = board.auto_ai();
        ans.push(m.0);
        if let Some(b) = m.1 {
            board = b;
            if !timer.ok() {
                println!("{}", ans);
                ans.clear();
                timer.next();
            }
        } else {
            break;
        }
    }
    println!("{}", ans);
}
