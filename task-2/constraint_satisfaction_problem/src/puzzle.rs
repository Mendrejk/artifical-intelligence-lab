use crate::point::Point;
use crate::solution::Solution;

pub trait Puzzle<T> {
    fn solve_with_backtracking(&mut self) -> Vec<Solution<T>>;
    fn solve_with_forward_checking(&mut self) -> Vec<Solution<T>>;

    fn backtrack(
        &mut self,
        variables: Vec<Vec<T>>,
        current_pos: Point,
        solutions: Vec<Solution<T>>,
    ) -> Vec<Solution<T>>;

    fn forward_check(
        &mut self,
        variables: Vec<Vec<T>>,
        current_pos: Point,
        solutions: Vec<Solution<T>>,
    ) -> Vec<Solution<T>>;

    // checks if the value can be entered into the given coordinates
    // without violating any constraints
    fn check_constraints(variables: &mut [Vec<T>], pos: Point, inserted: &T) -> bool
    where
        Self: Sized;

    fn restrain_forward_checking_domains(
        variables: &mut [Vec<T>],
        pos: Point,
        inserted: &T,
    ) -> bool
    where
        Self: Sized;

    fn find_next_empty(&self, variables: &[Vec<T>], position: Point) -> Option<Point>;

    fn get_next_index(len: usize, position: &Point) -> Option<Point>
    where
        Self: Sized,
    {
        if position.x == len - 1 {
            if position.y == len - 1 {
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

    fn get_column(variables: &[Vec<T>], x: usize) -> Vec<&T>
    where
        Self: Sized,
    {
        variables.iter().map(|row| &row[x]).collect()
    }
}
