use constructs::{Vector};
use constructs::matrix::{Matrix};


pub trait Network {
    fn forward_pass(input: &Vector, weights: &Vec<Matrix>) -> Vector;
}