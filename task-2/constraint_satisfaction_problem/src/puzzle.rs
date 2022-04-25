/*
// domain -> [0, 1]
// constraints -> [0,1, x] -> 0th row sum to 3, 1st column sum to 3
// variables -> x x 1
*/

pub trait Puzzle {}

pub struct BinaryPuzzle {
    pub variables: Vec<Vec<Option<u64>>>,
    pub domain: Vec<u64>,
}

impl BinaryPuzzle {
    fn new(variables: Vec<Vec<Option<u64>>>, domain: Vec<u64>) -> Self {
        todo!()
    }
}

impl Puzzle for BinaryPuzzle {}
