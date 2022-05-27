use crate::point::Point;
use crate::solution::Solution;

pub trait Puzzle<T> {
    fn find_next_empty(&self, variables: &[Vec<T>], position: Point) -> Option<Point>;
    fn get_next_index(&self, position: &Point) -> Option<Point>;

    fn get_column(variables: &[Vec<T>], x: usize) -> Vec<T>
    where
        Self: Sized,
        T: Copy,
    {
        variables.iter().map(|row| row[x]).collect()
    }

    // checks if the value can be entered into the given coordinates
    // without violating any constraints
    fn check_constraints(variables: &mut [Vec<T>], pos: Point, inserted: T) -> bool
    where
        Self: Sized;

    fn backtrack(
        &mut self,
        variables: Vec<Vec<T>>,
        current_pos: Point,
        solutions: Vec<Solution<T>>,
    ) -> Vec<Solution<T>>;
    fn solve_with_backtracking(&mut self) -> Vec<Solution<T>>;
}
