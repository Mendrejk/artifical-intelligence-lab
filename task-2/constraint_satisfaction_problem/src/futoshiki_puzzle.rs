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

impl Puzzle<FutoshikiNode> for FutoshikiPuzzle {
    fn find_next_empty(&self, variables: &[Vec<FutoshikiNode>], position: Point) -> Option<Point> {
        todo!()
    }

    fn get_next_index(&self, position: &Point) -> Option<Point> {
        todo!()
    }

    fn get_column(variables: &[Vec<FutoshikiNode>], x: usize) -> Vec<FutoshikiNode> {
        todo!()
    }

    fn check_constraints(
        variables: &mut [Vec<FutoshikiNode>],
        pos: Point,
        value: FutoshikiNode,
    ) -> bool {
        todo!()
    }

    fn backtrack(
        &mut self,
        variables: Vec<Vec<FutoshikiNode>>,
        current_pos: Point,
        solutions: Vec<Solution<FutoshikiNode>>,
    ) -> Vec<Solution<FutoshikiNode>> {
        todo!()
    }

    fn solve_with_backtracking(&mut self) -> Vec<Solution<FutoshikiNode>> {
        todo!()
    }
}
