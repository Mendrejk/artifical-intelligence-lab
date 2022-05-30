use crate::point::Point;
use crate::puzzle::Puzzle;
use crate::solution::Solution;

use std::cmp::max;

#[derive(Clone)]
pub struct BinaryNode {
    pub value: Option<u32>,
    pub domain: Vec<u32>,
}

pub struct BinaryPuzzle {
    pub variables: Vec<Vec<BinaryNode>>,
    pub domain: Vec<u32>,
    len: usize,
}

impl BinaryPuzzle {
    pub fn new(variables: Vec<Vec<BinaryNode>>, domain: Vec<u32>) -> Self {
        Self {
            len: variables.len(),
            variables,
            domain,
        }
    }
}

impl Puzzle<BinaryNode> for BinaryPuzzle {
    fn solve_with_backtracking(&mut self) -> Vec<Solution<BinaryNode>> {
        // determine the first empty spot
        let first_point = Point { y: 0, x: 0 };
        let first_empty = if self.variables[0][0].value == None {
            Some(first_point)
        } else {
            self.find_next_empty(&self.variables, first_point)
        };

        match first_empty {
            None => vec![],
            Some(first) => self.backtrack(self.variables.clone(), first, vec![]),
        }
    }

    fn solve_with_forward_checking(&mut self) -> Vec<Solution<BinaryNode>> {
        todo!()
    }

    fn backtrack(
        &mut self,
        mut variables: Vec<Vec<BinaryNode>>,
        current_pos: Point,
        mut solutions: Vec<Solution<BinaryNode>>,
    ) -> Vec<Solution<BinaryNode>> {
        for value in self.domain.clone() {
            let node = BinaryNode {
                value: Some(value),
                domain: variables[current_pos.y][current_pos.x].domain.clone(),
            };

            if Self::check_constraints(&mut variables, current_pos, &node.clone()) {
                let mut new_variables = variables.clone();
                new_variables[current_pos.y][current_pos.x] = node;

                let next_empty = self.find_next_empty(&new_variables, current_pos);

                match next_empty {
                    None => {
                        solutions.push(Solution {
                            data: new_variables,
                        });
                    }
                    Some(next_pos) => {
                        solutions = self.backtrack(new_variables, next_pos, solutions);
                    }
                }
            }
        }

        solutions
    }

    fn forward_check(
        &mut self,
        variables: Vec<Vec<BinaryNode>>,
        current_pos: Point,
        solutions: Vec<Solution<BinaryNode>>,
    ) -> Vec<Solution<BinaryNode>> {
        todo!()
    }

    fn check_constraints(
        variables: &mut [Vec<BinaryNode>],
        pos: Point,
        inserted: &BinaryNode,
    ) -> bool {
        // ensure that the spot at pos is valid
        if variables[pos.y][pos.x].value != None {
            return false;
        }

        let node_value = inserted.value.unwrap();
        let len = variables.len();

        // check for more than two repetitions
        // FIXME this makes my eyes bleed
        let mut row_repetitions = 0;

        if pos.x > 0 && variables[pos.y][pos.x - 1].value == Some(node_value) {
            row_repetitions += 1;
            if pos.x > 1 && variables[pos.y][pos.x - 2].value == Some(node_value) {
                row_repetitions += 1;
            }
        }

        if pos.x + 1 < len && variables[pos.y][pos.x + 1].value == Some(node_value) {
            row_repetitions += 1;
            if pos.x + 2 < len && variables[pos.y][pos.x + 2].value == Some(node_value) {
                row_repetitions += 1;
            }
        }

        if row_repetitions > 1 {
            return false;
        }

        let mut column_repetitions = 0;

        if pos.y > 0 && variables[pos.y - 1][pos.x].value == Some(node_value) {
            column_repetitions += 1;
            if pos.y > 1 && variables[pos.y - 2][pos.x].value == Some(node_value) {
                column_repetitions += 1;
            }
        }

        if pos.y + 1 < len && variables[pos.y + 1][pos.x].value == Some(node_value) {
            column_repetitions += 1;
            if pos.y + 2 < len && variables[pos.y + 2][pos.x].value == Some(node_value) {
                column_repetitions += 1;
            }
        }

        if column_repetitions > 1 {
            return false;
        }

        // check if there are as many 0s as 1s

        // row
        let mut row_zeroes = 0;
        let mut row_ones = 0;

        if node_value == 0 {
            row_zeroes += 1;
        } else {
            row_ones += 1;
        }

        for val in variables[pos.y].iter().filter_map(|node| node.value) {
            if val == 0 {
                row_zeroes += 1;
            } else {
                row_ones += 1;
            }
        }

        if max(row_zeroes, row_ones) > len / 2 {
            return false;
        }

        // column
        let mut column_zeroes = 0;
        let mut column_ones = 0;

        if node_value == 0 {
            column_zeroes += 1;
        } else {
            column_ones += 1;
        }

        for val in BinaryPuzzle::get_column(variables, pos.x)
            .iter()
            .filter_map(|&elem| elem.value)
        {
            if val == 0 {
                column_zeroes += 1;
            } else {
                column_ones += 1;
            }
        }

        if max(column_zeroes, column_ones) > len / 2 {
            return false;
        }

        // check for uniqueness

        // temporarily put the value in
        variables[pos.y][pos.x].value = Some(node_value);

        // check rows
        let mut valid_row = true;

        let current_row = &variables[pos.y];
        for elem in current_row.iter() {
            if elem.value.is_none() {
                valid_row = false;
                break;
            }
        }

        if valid_row {
            for (i, row) in variables.iter().enumerate() {
                if i == pos.y {
                    continue;
                }

                if row
                    .iter()
                    .zip(current_row.iter())
                    .all(|(a, b)| a.value == b.value)
                {
                    // remove the temporary value
                    variables[pos.y][pos.x].value = None;
                    return false;
                }
            }
        }

        // check columns
        let mut valid_column = true;

        let current_column = BinaryPuzzle::get_column(variables, pos.x);
        for elem in &current_column {
            if elem.value.is_none() {
                valid_column = false;
                break;
            }
        }

        if valid_column {
            for i in 0..len {
                if i == pos.x {
                    continue;
                }

                if BinaryPuzzle::get_column(variables, i)
                    .iter()
                    .zip(current_column.iter())
                    .all(|(a, b)| a.value == b.value)
                {
                    // remove the temporary value
                    variables[pos.y][pos.x].value = None;
                    return false;
                }
            }
        }

        // remove the temporary value
        variables[pos.y][pos.x].value = None;

        true
    }

    fn restrain_forward_checking_domains(
        variables: &mut [Vec<BinaryNode>],
        pos: Point,
        inserted: &BinaryNode,
    ) -> bool
    where
        Self: Sized,
    {
        todo!()
    }

    // noinspection DuplicatedCode
    fn find_next_empty(&self, variables: &[Vec<BinaryNode>], mut position: Point) -> Option<Point> {
        loop {
            position = Self::get_next_index(self.len, &position)?;
            if variables[position.y][position.x].value == None {
                return Some(position);
            }
        }
    }
}
