use crate::binary_puzzle::{BinaryNode, BinaryPuzzle};
use crate::point::Point;
use crate::puzzle::Puzzle;

use crate::futoshiki_puzzle::{
    FutoshikiConstraint, FutoshikiNode, FutoshikiPuzzle, FutoshikiRelation,
};
use std::fs::read_to_string;
use std::iter::{Enumerate, Filter, Map};
use std::slice::Iter;

pub enum PuzzleFile {
    Binary6x6,
    Binary8x8,
    Binary10x10,
    Futoshiki4x4,
    Futoshiki5x5,
    Futoshiki6x6,
}

impl PuzzleFile {
    fn get_file_path(&self) -> &str {
        match *self {
            PuzzleFile::Binary6x6 => "data/binary_6x6",
            PuzzleFile::Binary8x8 => "data/binary_8x8",
            PuzzleFile::Binary10x10 => "data/binary_10x10",
            PuzzleFile::Futoshiki4x4 => "data/futoshiki_4x4",
            PuzzleFile::Futoshiki5x5 => "data/futoshiki_5x5",
            PuzzleFile::Futoshiki6x6 => "data/futoshiki_6x6",
        }
    }
}

#[derive(Debug)]
enum FutoshikiElement {
    Empty,
    NoRelation,
    LessThan,
    GreaterThan,
    Value(u32),
}

struct FutoshikiBoard {
    pub data: Vec<Vec<FutoshikiElement>>,
}

pub fn read_binary_puzzle(puzzle_file: &PuzzleFile) -> Option<BinaryPuzzle>
where
    BinaryPuzzle: Puzzle<BinaryNode>,
{
    if !matches!(
        puzzle_file,
        PuzzleFile::Binary6x6 | PuzzleFile::Binary8x8 | PuzzleFile::Binary10x10
    ) {
        return None;
    }

    let data = read_to_string(puzzle_file.get_file_path()).unwrap();
    let split_data = data.split("\r\n");

    let domain = vec![0, 1];
    let variables: Vec<Vec<BinaryNode>> = split_data
        .map(|row| {
            row.chars()
                .map(|char| match char {
                    'x' => BinaryNode {
                        value: None,
                        domain: domain.clone(),
                    },
                    _ => BinaryNode {
                        value: Some(char.to_digit(10).unwrap()),
                        domain: domain.clone(),
                    },
                })
                .collect()
        })
        .collect();

    Some(BinaryPuzzle::new(variables, domain))
}

pub fn read_futoshiki_puzzle(puzzle_file: &PuzzleFile) -> Option<FutoshikiPuzzle>
where
    FutoshikiPuzzle: Puzzle<FutoshikiNode>,
{
    if !matches!(
        puzzle_file,
        PuzzleFile::Futoshiki4x4 | PuzzleFile::Futoshiki5x5 | PuzzleFile::Futoshiki6x6
    ) {
        return None;
    }
    let board = FutoshikiBoard {
        data: read_to_string(puzzle_file.get_file_path())
            .unwrap()
            .split("\r\n")
            .map(|row| {
                row.chars()
                    .map(|char| match char {
                        '>' => FutoshikiElement::GreaterThan,
                        '<' => FutoshikiElement::LessThan,
                        '-' => FutoshikiElement::NoRelation,
                        'x' => FutoshikiElement::Empty,
                        _ => FutoshikiElement::Value(char.to_digit(10).unwrap()),
                    })
                    .collect()
            })
            .collect(),
    };

    let domain: Vec<u32> = (1..=((board.data.len() / 2 + 1) as u32)).collect();

    let variables: Vec<Vec<FutoshikiNode>> = board
        .data
        .iter()
        .map(|row| row.iter().enumerate())
        .enumerate()
        .map(|(y, row)| {
            row.filter(|&(_, elem)| {
                matches!(elem, FutoshikiElement::Empty | FutoshikiElement::Value(_))
            })
            .map(|(x, elem)| match elem {
                FutoshikiElement::Value(val) => FutoshikiNode {
                    value: Some(*val),
                    constraints: find_futoshiki_constraints(&board.data, Point { y, x }),
                    domain: domain.clone(),
                },
                _ => FutoshikiNode {
                    value: None,
                    constraints: find_futoshiki_constraints(&board.data, Point { y, x }),
                    domain: domain.clone(),
                },
            })
            .collect()
        })
        .filter(|row: &Vec<FutoshikiNode>| !row.is_empty())
        .collect();

    Some(FutoshikiPuzzle::new(variables, domain))
}

fn find_futoshiki_constraints(
    board: &[Vec<FutoshikiElement>],
    point: Point,
) -> Vec<FutoshikiConstraint> {
    let mut constraints: Vec<FutoshikiConstraint> = vec![];

    if point.y > 1 {
        if let Some(constraint) =
            try_create_futoshiki_constraint(board, point.y - 1, point.x / 2, point.y - 2, point.x)
        {
            constraints.push(constraint)
        };
    }

    if point.y < board.len() - 2 {
        if let Some(constraint) =
            try_create_futoshiki_constraint(board, point.y + 1, point.x / 2, point.y + 2, point.x)
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

    if point.x < board[point.y].len() - 2 {
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
    // coordinates are / 2 because in the final collection, there will be no constraint fields
    match board[constraint_y][constraint_x] {
        FutoshikiElement::LessThan => Some(FutoshikiConstraint {
            relation: FutoshikiRelation::LessThan,
            other_position: Point {
                y: other_y / 2,
                x: other_x / 2,
            },
        }),
        FutoshikiElement::GreaterThan => Some(FutoshikiConstraint {
            relation: FutoshikiRelation::GreaterThan,
            other_position: Point {
                y: other_y / 2,
                x: other_x / 2,
            },
        }),
        _ => None,
    }
}
