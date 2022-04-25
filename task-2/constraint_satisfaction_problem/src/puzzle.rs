/*
// domain -> [0, 1]
// constraints -> [0,1, x] -> 0th row sum to 3, 1st column sum to 3
// variables -> x x 1
*/

pub trait Puzzle {}

pub struct BinaryPuzzle {
    pub variables: Vec<Vec<Option<u32>>>,
    pub domain: Vec<u32>,
}

impl BinaryPuzzle {
    fn new(variables: Vec<Vec<Option<u32>>>, domain: Vec<u32>) -> Self {
        todo!()
    }
}

impl Puzzle for BinaryPuzzle {}
