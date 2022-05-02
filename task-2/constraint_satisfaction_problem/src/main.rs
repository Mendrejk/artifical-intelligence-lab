use crate::puzzle_reader::{read_puzzle, PuzzleFile};

mod puzzle;
mod puzzle_reader;

fn main() {
    let mut puzzle = read_puzzle(&PuzzleFile::binary_6x6);
    let result = puzzle.solve_with_backtracking();
    println!("{:?}", result);
}
