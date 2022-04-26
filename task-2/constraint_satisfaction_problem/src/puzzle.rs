/*
// domain -> [0, 1]
// constraints -> [0,1, x] -> 0th row sum to 3, 1st column sum to 3
// variables -> x x 1
*/

pub trait Puzzle {
    // checks if the value can be entered into the given coordinates
    // without violating any constraints
    fn check_constraints(&mut self, pos: Point, value: u32) -> bool;

    fn solve_with_backtracking(&self) -> Vec<Vec<Vec<Option<u32>>>>;

    fn find_next_empty(&self, position: Point) -> Option<Point>;
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

    fn get_column(&self, x: usize) -> Vec<Option<u32>> {
        self.variables.iter().map(|row| row[x]).collect()
    }

    fn backtrack(
        &mut self,
        variables: Vec<Vec<Option<u32>>>,
        current_pos: Point,
        mut solutions: Vec<Vec<Vec<Option<u32>>>>,
    ) -> Vec<Vec<Vec<Option<u32>>>> {
        for value in self.domain {
            if self.check_constraints(current_pos, value) {
                let mut new_variables = variables.clone();
                new_variables[current_pos.y][current_pos.x] = Some(value);

                let next_empty = self.find_next_empty(current_pos);

                match next_empty {
                    None => {
                        solutions.push(new_variables);
                        return solutions; // fixme chyba nie return
                    }
                    Some => {
                        solutions = self.backtrack(new_variables, next_empty, solutions);
                    }
                }

                self.backtrack(new_variables)
            }
        }

        todo!()
    }
}

impl Puzzle for BinaryPuzzle {
    fn check_constraints(&mut self, pos: Point, value: u32) -> bool {
        if self.variables[pos.y][pos.x] != None {
            return false;
        }

        // check for more than two repetitions
        // FIXME this makes my eyes bleed
        let mut row_repetitions = 0;

        if pos.x > 0 && self.variables[pos.y][pos.x - 1] == Some(value) {
            row_repetitions += 1;
            if pos.x > 1 && self.variables[pos.y][pos.x - 2] == Some(value) {
                row_repetitions += 1;
            }
        }

        if pos.x + 1 < self.len && self.variables[pos.y][pos.x + 1] == Some(value) {
            row_repetitions += 1;
            if pos.x + 2 < self.len && self.variables[pos.y][pos.x + 2] == Some(value) {
                row_repetitions += 1;
            }
        }

        if row_repetitions > 2 {
            return false;
        }

        let mut column_repetitions = 0;

        if pos.y > 0 && self.variables[pos.y - 1][pos.x] == Some(value) {
            column_repetitions += 1;
            if pos.y > 1 && self.variables[pos.y - 2][pos.x] == Some(value) {
                column_repetitions += 1;
            }
        }

        if pos.y + 1 < self.len && self.variables[pos.y + 1][pos.x] == Some(value) {
            column_repetitions += 1;
            if pos.y + 2 < self.len && self.variables[pos.y + 2][pos.x] == Some(value) {
                column_repetitions += 1;
            }
        }

        if column_repetitions > 2 {
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

        for val in self.variables[pos.y].iter().flatten() {
            if *val == 0 {
                row_zeroes += 1;
            } else {
                row_ones += 1;
            }
        }

        if row_zeroes != row_ones {
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

        for val in self.get_column(pos.x).iter().flatten() {
            if *val == 0 {
                column_zeroes += 1;
            } else {
                column_ones += 1;
            }
        }

        if column_zeroes != column_ones {
            return false;
        }

        // check for uniqueness

        // temporarily put the value in
        self.variables[pos.y][pos.x] = Some(value);

        // check rows
        let current_row = &self.variables[pos.y];
        for (i, row) in self.variables.iter().enumerate() {
            if i == pos.y {
                continue;
            }

            if row == current_row {
                return false;
            }
        }

        // check columns
        let current_column = self.get_column(pos.x);
        for i in 0..self.len {
            if i == pos.x {
                continue;
            }

            if self.get_column(i) == current_column {
                return false;
            }
        }

        // remove the temporary value
        self.variables[pos.y][pos.x] = None;

        true
    }

    fn solve_with_backtracking(&self) -> Vec<Vec<Vec<Option<u32>>>> {
        todo!()
    }

    fn find_next_empty(&self, position: Point) -> Option<Point> {
        loop {
            let position = self.get_next_index(&position)?;
            if self.variables[position.y][position.x] == None {
                return Some(position);
            }
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

#[derive(Clone, Copy)]
pub struct Point {
    pub y: usize,
    pub x: usize,
}
