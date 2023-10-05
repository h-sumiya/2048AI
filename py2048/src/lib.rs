use std::borrow::Cow;

use engine::Board;
use pyo3::prelude::*;
mod connect;
mod engine;
mod table;

#[pyfunction]
fn _init_board<'a>(seed: u64) -> PyResult<Cow<'a, [u8]>> {
    let board = Board::new(seed);
    Ok(board.dump().into())
}

#[pyfunction]
fn _moves(data: &[u8]) -> PyResult<Vec<Cow<[u8]>>> {
    let board = Board::load(data);
    let moves = unsafe { board.moves() };
    let mut res = Vec::with_capacity(4);
    res.push(moves.up.dump().into());
    res.push(moves.down.dump().into());
    res.push(moves.left.dump().into());
    res.push(moves.right.dump().into());
    Ok(res)
}

#[pyfunction]
fn _from_data(data: &[u8], seed: u64) -> PyResult<Cow<[u8]>> {
    let board = Board::from_vec(data, seed);
    Ok(board.dump().into())
}

#[pyfunction]
fn _seed(data: &[u8]) -> PyResult<u64> {
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&data[8..16]);
    Ok(u64::from_le_bytes(buf))
}

#[pyfunction]
fn _to_data(data: &[u8]) -> PyResult<Cow<[u8]>> {
    let board = Board::load(data);
    let data = board.to_vec();
    Ok(data.into())
}

#[pyfunction]
fn _display(data: &[u8]) -> PyResult<String> {
    let board = Board::load(data);
    Ok(format!("{}", board))
}

/// A Python module implemented in Rust.
#[pymodule]
fn py2048(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(_init_board, m)?)?;
    m.add_function(wrap_pyfunction!(_moves, m)?)?;
    m.add_function(wrap_pyfunction!(_from_data, m)?)?;
    m.add_function(wrap_pyfunction!(_seed, m)?)?;
    m.add_function(wrap_pyfunction!(_to_data, m)?)?;
    m.add_function(wrap_pyfunction!(_display, m)?)?;
    Ok(())
}
