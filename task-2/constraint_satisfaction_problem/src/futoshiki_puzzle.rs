use crate::point::Point;

pub struct FutoshikiPuzzle {
    pub variables: Vec<Vec<Option<FutoshikiNode>>>,
    pub domain: Vec<u32>,
    len: usize,
}

pub struct FutoshikiNode {
    pub value: u32,
    pub constraints: Vec<FutoshikiConstraint>,
}

pub struct FutoshikiConstraint {
    pub relation: FutoshikiRelation,
    pub other_index: Point,
}

pub enum FutoshikiRelation {
    LessThan,
    GreaterThan,
}
