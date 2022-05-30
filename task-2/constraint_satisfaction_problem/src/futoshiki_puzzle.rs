use crate::point::Point;
use crate::puzzle::Puzzle;
use crate::solution::Solution;
use std::env::var;

#[derive(Clone)]
pub enum FutoshikiRelation {
    LessThan,
    GreaterThan,
}

#[derive(Clone)]
pub struct FutoshikiConstraint {
    pub relation: FutoshikiRelation,
    pub other_position: Point,
}

#[derive(Clone)]
pub struct FutoshikiNode {
    pub value: Option<u32>,
    pub constraints: Vec<FutoshikiConstraint>,
    pub domain: Vec<u32>,
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

    fn check_futoshiki_constraints(
        variables: &mut [Vec<FutoshikiNode>],
        node: &FutoshikiNode,
        pos: Point,
    ) -> bool {
        let node_value = node.value.unwrap();

        for constraint in &node.constraints {
            let is_first = pos.y < constraint.other_position.y
                || (pos.x < constraint.other_position.x && pos.y == constraint.other_position.y);

            let other = &variables[constraint.other_position.y][constraint.other_position.x];
            if other.value.is_none() {
                // constraint is always fulfilled
                continue;
            }

            let other_value = other.value.unwrap();
            let fulfills_constraint = match &constraint.relation {
                FutoshikiRelation::LessThan => node_value < other_value,
                FutoshikiRelation::GreaterThan => node_value > other_value,
            };

            if (is_first && !fulfills_constraint) || (!is_first && fulfills_constraint) {
                return false;
            }
        }

        true
    }
}

impl Puzzle<FutoshikiNode> for FutoshikiPuzzle {
    fn solve_with_backtracking(&mut self) -> Vec<Solution<FutoshikiNode>> {
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

    fn solve_with_forward_checking(&mut self) -> Vec<Solution<FutoshikiNode>> {
        // determine the first empty spot
        let first_point = Point { y: 0, x: 0 };
        let first_empty = if self.variables[0][0].value == None {
            Some(first_point)
        } else {
            self.find_next_empty(&self.variables, first_point)
        };

        match first_empty {
            None => vec![],
            Some(first) => self.forward_check(self.variables.clone(), first, vec![]),
        }
    }

    fn backtrack(
        &mut self,
        mut variables: Vec<Vec<FutoshikiNode>>,
        current_pos: Point,
        mut solutions: Vec<Solution<FutoshikiNode>>,
    ) -> Vec<Solution<FutoshikiNode>> {
        for value in self.domain.clone() {
            let node = FutoshikiNode {
                value: Some(value),
                constraints: variables[current_pos.y][current_pos.x].constraints.clone(),
                domain: variables[current_pos.y][current_pos.x].domain.clone(),
            };

            if Self::check_constraints(&mut variables, current_pos, &node) {
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
        variables: Vec<Vec<FutoshikiNode>>,
        current_pos: Point,
        solutions: Vec<Solution<FutoshikiNode>>,
    ) -> Vec<Solution<FutoshikiNode>> {
        todo!()
    }

    fn check_constraints(
        variables: &mut [Vec<FutoshikiNode>],
        pos: Point,
        inserted: &FutoshikiNode,
    ) -> bool {
        // ensure that the spot at pos is valid
        if variables[pos.y][pos.x].value != None {
            return false;
        }

        // check if the inserted value is unique in its row and column
        // row
        if variables[pos.y]
            .iter()
            .any(|node| node.value == inserted.value)
        {
            return false;
        }

        // column
        if Self::get_column(variables, pos.x)
            .iter()
            .any(|node| node.value == inserted.value)
        {
            return false;
        }

        Self::check_futoshiki_constraints(variables, inserted, pos)
    }

    fn restrain_forward_checking_domains(
        variables: &mut [Vec<FutoshikiNode>],
        pos: Point,
        inserted: &FutoshikiNode,
    ) -> bool
    where
        Self: Sized,
    {
        // unique row
        // variables[pos.y] = variables[pos.y]
        //     .iter()
        //     .map(|node| match node.value {
        //         None => {
        //             let restrained_domain: Vec<u32> = node
        //                 .domain
        //                 .clone()
        //                 .into_iter()
        //                 .filter(|&value| value != inserted.value.unwrap())
        //                 .collect();
        //
        //             FutoshikiNode {
        //                 value: None,
        //                 constraints: node.constraints.clone(),
        //                 domain: restrained_domain,
        //             }
        //         }
        //         Some(_) => node,
        //     })
        //     .collect();
        let foo = variables[pos.y].iter_mut().map(|node| match node.value {
            None => {}
            Some(_) => node
        })

        true
    }

    // noinspection DuplicatedCode
    fn find_next_empty(
        &self,
        variables: &[Vec<FutoshikiNode>],
        mut position: Point,
    ) -> Option<Point> {
        loop {
            position = Self::get_next_index(self.len, &position)?;
            if variables[position.y][position.x].value == None {
                return Some(position);
            }
        }
    }
}
