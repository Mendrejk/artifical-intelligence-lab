/*
// domain -> [0, 1]
// constraints -> [0,1, x] -> 0th row sum to 3, 1st column sum to 3
// variables -> x x 1
*/

use crate::solution::Solution;
use std::cmp::max;

pub trait Puzzle {
    // fn check_constraints(&mut self, pos: Point, value: u32) -> bool;

    fn solve_with_backtracking(&mut self) -> Vec<Solution<u32>>;

    // fn find_next_empty(&self, position: Point) -> Option<Point>;
    fn get_next_index(&self, position: &Point) -> Option<Point>;
}

pub struct BinaryPuzzle {
    pub variables: Vec<Vec<Option<u32>>>,
    pub domain: Vec<u32>,
    len: usize,
}

impl BinaryPuzzle {
    pub fn new(variables: Vec<Vec<Option<u32>>>, domain: Vec<u32>) -> Self {
        Self {
            len: variables.len(),
            variables,
            domain,
        }
    }

    fn get_column(variables: &[Vec<Option<u32>>], x: usize) -> Vec<Option<u32>> {
        variables.iter().map(|row| row[x]).collect()
    }

    // checks if the value can be entered into the given coordinates
    // without violating any constraints
    fn check_constraints(variables: &mut [Vec<Option<u32>>], pos: Point, value: u32) -> bool {
        if variables[pos.y][pos.x] != None {
            return false;
        }

        let len = variables.len();

        // check for more than two repetitions
        // FIXME this makes my eyes bleed
        let mut row_repetitions = 0;

        if pos.x > 0 && variables[pos.y][pos.x - 1] == Some(value) {
            row_repetitions += 1;
            if pos.x > 1 && variables[pos.y][pos.x - 2] == Some(value) {
                row_repetitions += 1;
            }
        }

        if pos.x + 1 < len && variables[pos.y][pos.x + 1] == Some(value) {
            row_repetitions += 1;
            if pos.x + 2 < len && variables[pos.y][pos.x + 2] == Some(value) {
                row_repetitions += 1;
            }
        }

        if row_repetitions > 1 {
            return false;
        }

        let mut column_repetitions = 0;

        if pos.y > 0 && variables[pos.y - 1][pos.x] == Some(value) {
            column_repetitions += 1;
            if pos.y > 1 && variables[pos.y - 2][pos.x] == Some(value) {
                column_repetitions += 1;
            }
        }

        if pos.y + 1 < len && variables[pos.y + 1][pos.x] == Some(value) {
            column_repetitions += 1;
            if pos.y + 2 < len && variables[pos.y + 2][pos.x] == Some(value) {
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

        if value == 0 {
            row_zeroes += 1;
        } else {
            row_ones += 1;
        }

        for val in variables[pos.y].iter().flatten() {
            if *val == 0 {
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

        if value == 0 {
            column_zeroes += 1;
        } else {
            column_ones += 1;
        }

        for val in BinaryPuzzle::get_column(variables, pos.x).iter().flatten() {
            if *val == 0 {
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
        variables[pos.y][pos.x] = Some(value);

        // check rows
        let mut valid_row = true;

        let current_row = &variables[pos.y];
        for elem in current_row.iter() {
            if elem.is_none() {
                valid_row = false;
                break;
            }
        }

        if valid_row {
            for (i, row) in variables.iter().enumerate() {
                if i == pos.y {
                    continue;
                }

                if row == current_row {
                    // remove the temporary value
                    variables[pos.y][pos.x] = None;
                    return false;
                }
            }
        }

        // check columns
        let mut valid_column = true;

        let current_column = BinaryPuzzle::get_column(variables, pos.x);
        for elem in &current_column {
            if elem.is_none() {
                valid_column = false;
                break;
            }
        }

        if valid_column {
            for i in 0..len {
                if i == pos.x {
                    continue;
                }

                if BinaryPuzzle::get_column(variables, i) == current_column {
                    // remove the temporary value
                    variables[pos.y][pos.x] = None;
                    return false;
                }
            }
        }

        // remove the temporary value
        variables[pos.y][pos.x] = None;

        true
    }

    fn find_next_empty(
        &self,
        variables: &[Vec<Option<u32>>],
        mut position: Point,
    ) -> Option<Point> {
        loop {
            position = self.get_next_index(&position)?;
            if variables[position.y][position.x] == None {
                return Some(position);
            }
        }
    }

    fn backtrack(
        &mut self,
        mut variables: Vec<Vec<Option<u32>>>,
        current_pos: Point,
        mut solutions: Vec<Solution<u32>>,
    ) -> Vec<Solution<u32>> {
        for value in self.domain.clone() {
            if BinaryPuzzle::check_constraints(&mut variables, current_pos, value) {
                let mut new_variables = variables.clone();
                new_variables[current_pos.y][current_pos.x] = Some(value);

                // println!(
                //     "{}",
                //     Solution {
                //         data: new_variables.clone()
                //     }
                // );

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

                // self.backtrack(new_variables)+
            }
        }

        solutions
    }
}

impl Puzzle for BinaryPuzzle {
    fn solve_with_backtracking(&mut self) -> Vec<Solution<u32>> {
        // determine the first empty spot
        let first_point = Point { y: 0, x: 0 };
        let first_empty = if self.variables[0][0] == None {
            Some(first_point)
        } else {
            self.find_next_empty(&self.variables, first_point)
        };

        match first_empty {
            None => vec![],
            Some(first) => self.backtrack(self.variables.clone(), first, vec![]),
        }
    }

    fn get_next_index(&self, position: &Point) -> Option<Point> {
        if position.x == self.len - 1 {
            if position.y == self.len - 1 {
                return None;
            }

            return Some(Point {
                y: position.y + 1,
                x: 0,
            });
        }

        Some(Point {
            y: position.y,
            x: position.x + 1,
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub y: usize,
    pub x: usize,
}
