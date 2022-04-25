use crate::puzzle_reader::{read_puzzle, PuzzleFile};

mod puzzle;
mod puzzle_reader;

fn main() {
    read_puzzle(&PuzzleFile::binary_6x6);
}
