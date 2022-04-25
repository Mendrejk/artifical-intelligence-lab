/*
// domain -> [0, 1]
// constraints -> [0,1, x] -> 0th row sum to 3, 1st column sum to 3
// variables -> x x 1
*/

pub trait Puzzle {}

pub struct BinaryPuzzle {
    variables: Vec<Vec<Option<u32>>>,
    domain: Vec<u32>,
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

    // checks if the value can be entered into the given coordinates
    // without violating any constraints
    pub fn check_constraints(&mut self, y: usize, x: usize, value: u32) -> bool {
        if self.variables[y][x] != None {
            return false;
        }

        // check for more than two repetitions
        // FIXME this makes my eyes bleed
        let mut row_repetitions = 0;

        if x > 0 && self.variables[y][x - 1] == Some(value) {
            row_repetitions += 1;
            if x > 1 && self.variables[y][x - 2] == Some(value) {
                row_repetitions += 1;
            }
        }

        if x + 1 < self.len && self.variables[y][x + 1] == Some(value) {
            row_repetitions += 1;
            if x + 2 < self.len && self.variables[y][x + 2] == Some(value) {
                row_repetitions += 1;
            }
        }

        if row_repetitions > 2 {
            return false;
        }

        let mut column_repetitions = 0;

        if y > 0 && self.variables[y - 1][x] == Some(value) {
            column_repetitions += 1;
            if y > 1 && self.variables[y - 2][x] == Some(value) {
                column_repetitions += 1;
            }
        }

        if y + 1 < self.len && self.variables[y + 1][x] == Some(value) {
            column_repetitions += 1;
            if y + 2 < self.len && self.variables[y + 2][x] == Some(value) {
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

        for val in self.variables[y].iter().flatten() {
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

        for val in self.get_column(x).iter().flatten() {
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
        self.variables[y][x] = Some(value);

        // check rows
        let current_row = &self.variables[y];
        for (i, row) in self.variables.iter().enumerate() {
            if i == y {
                continue;
            }

            if row == current_row {
                return false;
            }
        }

        // check columns
        let current_column = self.get_column(x);
        for i in 0..self.len {
            if i == x {
                continue;
            }

            if self.get_column(i) == current_column {
                return false;
            }
        }

        true
    }

    fn get_column(&self, x: usize) -> Vec<Option<u32>> {
        self.variables.iter().map(|row| row[x]).collect()
    }
}

impl Puzzle for BinaryPuzzle {}
