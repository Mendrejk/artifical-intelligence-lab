use crate::point::Point;
use crate::solution::Solution;

pub trait Puzzle {
    fn solve_with_backtracking(&mut self) -> Vec<Solution<Option<u32>>>;
    fn get_next_index(&self, position: &Point) -> Option<Point>;
}
