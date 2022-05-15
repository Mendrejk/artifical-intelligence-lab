extern crate core;

use crate::puzzle_reader::{read_puzzle, PuzzleFile};
mod puzzle;
mod puzzle_reader;
mod solution;

fn main() {
    let mut puzzle = read_puzzle(&PuzzleFile::binary_10x10);
    let result = puzzle.solve_with_backtracking();

    println!("{}\n", result.len());

    for solution in result {
        println!("{}", solution);
        println!();
    }
}
