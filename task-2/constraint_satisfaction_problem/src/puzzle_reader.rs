/*
// domain -> [0, 1]
// constraints -> [0,1, x] -> 0th row sum to 3, 1st column sum to 3
// variables -> x x 1
*/

use crate::puzzle::{BinaryPuzzle, Puzzle};

enum PuzzleFile {
    binary_6x6,
    binary_8x8,
    binary_10x10,
    futoshiki_4x4,
    futoshiki_5x5,
    futoshiki_6x6,
}

impl PuzzleFile {
    fn file_path(&self) -> &str {
        match *self {
            PuzzleFile::binary_6x6 => "binary_6x6",
            PuzzleFile::binary_8x8 => "binary_8x8",
            PuzzleFile::binary_10x10 => "binary_10x10",
            PuzzleFile::futoshiki_4x4 => "futoshiki_4x4",
            PuzzleFile::futoshiki_5x5 => "futoshiki_5x5",
            PuzzleFile::futoshiki_6x6 => "futoshiki_6x6",
        }
    }
}

pub fn read_puzzle(puzzle_file: &PuzzleFile) -> Box<dyn Puzzle> {
    match puzzle_file {
        PuzzleFile::binary_6x6 | PuzzleFile::binary_8x8 | PuzzleFile::binary_10x10 => {
            read_binary_puzzle(puzzle_file)
        }
        PuzzleFile::futoshiki_4x4 | PuzzleFile::futoshiki_5x5 | PuzzleFile::futoshiki_6x6 => {
            Puzzle // TODO
        }
    }
}

fn read_binary_puzzle(puzzle_file: &PuzzleFile) -> Box<BinaryPuzzle> {
    BinaryPuzzle // TODO
}