use std::fmt;
mod board;
use crate::board::board::*;

fn main() {
    let board = ChessBoard::new();
    println!("Initial chess board:\n{}", board);
}
