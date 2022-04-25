/*
// domain -> [0, 1]
// constraints -> [0,1, x] -> 0th row sum to 3, 1st column sum to 3
// variables -> x x 1
*/

pub trait Puzzle {
    fn new<T>(variables: Vec<Vec<T>>, domain: Vec<u64>) -> Self;
}

pub struct BinaryPuzzle {
    variables: Vec<Vec<Option<u64>>>,
    domain: Vec<u64>,
}
