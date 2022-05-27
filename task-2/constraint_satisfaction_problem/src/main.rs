extern crate core;

use crate::puzzle_reader::{read_puzzle, PuzzleFile};
mod binary_puzzle;
mod futoshiki_puzzle;
mod point;
mod puzzle;
mod puzzle_reader;
mod solution;

fn main() {
    let mut puzzle = read_puzzle(&PuzzleFile::Binary10x10);
    let result = puzzle.solve_with_backtracking();

    println!("{}\n", result.len());

    // for solution in result {
    //     println!("{}", solution);
    //     println!();
    // }

    let foo = read_puzzle(&PuzzleFile::Futoshiki4x4);
}
