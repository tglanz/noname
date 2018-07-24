use constructs::*;
use constructs::matrix::*;
use networks::network::Network;

use std::vec::Vec;

pub struct FullyConnected {

}

impl Network for FullyConnected {
    fn forward_pass(input: &Vector, weights: &Vec<Matrix>) -> Vector{
        vec![0.0; 0]
    }
}