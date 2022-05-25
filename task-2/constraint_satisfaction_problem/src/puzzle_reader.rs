use crate::binary_puzzle::BinaryPuzzle;
use crate::point::Point;
use crate::puzzle::Puzzle;

use crate::futoshiki_puzzle::{FutoshikiConstraint, FutoshikiRelation};
use std::fs::read_to_string;

pub enum PuzzleFile {
    binary_6x6,
    binary_8x8,
    binary_10x10,
    futoshiki_4x4,
    futoshiki_5x5,
    futoshiki_6x6,
}

impl PuzzleFile {
    fn get_file_path(&self) -> &str {
        match *self {
            PuzzleFile::binary_6x6 => "data/binary_6x6",
            PuzzleFile::binary_8x8 => "data/binary_8x8",
            PuzzleFile::binary_10x10 => "data/binary_10x10",
            PuzzleFile::futoshiki_4x4 => "data/futoshiki_4x4",
            PuzzleFile::futoshiki_5x5 => "data/futoshiki_5x5",
            PuzzleFile::futoshiki_6x6 => "data/futoshiki_6x6",
        }
    }
}

enum FutoshikiElement {
    LessThan,
    GreaterThan,
    Value(u32),
    Empty,
}

struct FutoshikiBoard {
    pub data: Vec<Vec<FutoshikiElement>>,
}

pub fn read_puzzle(puzzle_file: &PuzzleFile) -> Box<dyn Puzzle> {
    match puzzle_file {
        PuzzleFile::binary_6x6 | PuzzleFile::binary_8x8 | PuzzleFile::binary_10x10 => {
            read_binary_puzzle(puzzle_file)
        }
        PuzzleFile::futoshiki_4x4 | PuzzleFile::futoshiki_5x5 | PuzzleFile::futoshiki_6x6 => {
            // Box::new(BinaryPuzzle::new(variables: vec![vec![]], domain: vec![]))
            todo!()
            // TODO
        }
    }
}

fn read_binary_puzzle(puzzle_file: &PuzzleFile) -> Box<BinaryPuzzle> {
    let data = read_to_string(puzzle_file.get_file_path()).unwrap();
    let split_data: Vec<&str> = data.split("\r\n").collect();

    let domain = vec![0, 1];
    let variables: Vec<Vec<Option<u32>>> = split_data
        .into_iter()
        .map(|row| {
            row.chars()
                .map(|char| match char {
                    'x' => None,
                    _ => Some(char.to_digit(10).unwrap()),
                })
                .collect()
        })
        .collect();

    Box::new(BinaryPuzzle::new(variables, domain))
}

fn read_futoshiki_data(puzzle_file: &PuzzleFile) {
    let board_data: Vec<Vec<FutoshikiElement>> = read_to_string(puzzle_file.get_file_path())
        .unwrap()
        .split("\r\n")
        .map(|row| {
            row.chars()
                .map(|char| match char {
                    '>' => FutoshikiElement::GreaterThan,
                    '<' => FutoshikiElement::LessThan,
                    'x' => FutoshikiElement::Empty,
                    _ => FutoshikiElement::Value(char.to_digit(10).unwrap()),
                })
                .collect()
        })
        .collect();
}

fn find_futoshiki_constraints(
    board: &[Vec<FutoshikiElement>],
    point: Point,
) -> Vec<FutoshikiConstraint> {
    let mut constraints: Vec<FutoshikiConstraint> = vec![];

    if point.y > 1 {
        if let Some(constraint) =
            try_create_futoshiki_constraint(board, point.y - 1, point.x, point.y - 2, point.x)
        {
            constraints.push(constraint)
        };
    }

    if point.y < board.len() - 2 {
        if let Some(constraint) =
            try_create_futoshiki_constraint(board, point.y + 1, point.x, point.y + 2, point.x)
        {
            constraints.push(constraint)
        };
    }

    if point.x > 1 {
        if let Some(constraint) =
            try_create_futoshiki_constraint(board, point.y, point.x - 1, point.y, point.x - 2)
        {
            constraints.push(constraint)
        };
    }

    if point.x < board.len() - 2 {
        if let Some(constraint) =
            try_create_futoshiki_constraint(board, point.y, point.x + 1, point.y, point.x + 2)
        {
            constraints.push(constraint)
        };
    }

    constraints
}

fn try_create_futoshiki_constraint(
    board: &[Vec<FutoshikiElement>],
    constraint_y: usize,
    constraint_x: usize,
    other_y: usize,
    other_x: usize,
) -> Option<FutoshikiConstraint> {
    match board[constraint_y][constraint_x] {
        FutoshikiElement::LessThan => Some(FutoshikiConstraint {
            relation: FutoshikiRelation::LessThan,
            other_index: Point {
                y: other_y,
                x: other_x,
            },
        }),
        FutoshikiElement::GreaterThan => Some(FutoshikiConstraint {
            relation: FutoshikiRelation::GreaterThan,
            other_index: Point {
                y: other_y,
                x: other_x,
            },
        }),
        _ => None,
    }
}
