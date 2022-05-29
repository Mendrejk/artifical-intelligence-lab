extern crate core;

use crate::puzzle::Puzzle;
use crate::puzzle_reader::{read_binary_puzzle, read_futoshiki_puzzle, PuzzleFile};
mod binary_puzzle;
mod futoshiki_puzzle;
mod point;
mod puzzle;
mod puzzle_reader;
mod solution;

fn main() {
    let mut puzzle = read_futoshiki_puzzle(&PuzzleFile::Futoshiki4x4).unwrap();
    let result = puzzle.solve_with_backtracking();
    println!("{}\n", result.len());
}
