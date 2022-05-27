use crate::point::Point;
use crate::puzzle::Puzzle;
use crate::solution::Solution;

pub enum FutoshikiRelation {
    LessThan,
    GreaterThan,
}

pub struct FutoshikiConstraint {
    pub relation: FutoshikiRelation,
    pub other_index: Point,
}

pub struct FutoshikiNode {
    pub value: Option<u32>,
    pub constraints: Vec<FutoshikiConstraint>,
}

pub struct FutoshikiPuzzle {
    pub variables: Vec<Vec<FutoshikiNode>>,
    pub domain: Vec<u32>,
    len: usize,
}

impl FutoshikiPuzzle {
    pub fn new(variables: Vec<Vec<FutoshikiNode>>, domain: Vec<u32>) -> Self {
        Self {
            len: variables.len(),
            variables,
            domain,
        }
    }
}

impl Puzzle for FutoshikiPuzzle {
    fn solve_with_backtracking(&mut self) -> Vec<Solution<Option<u32>>> {
        todo!()
    }

    fn get_next_index(&self, position: &Point) -> Option<Point> {
        todo!()
    }
}
