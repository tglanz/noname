pub mod tensors;

use std::vec::Vec;
use types::{Scalar};

pub trait Variable {

    fn get_data(&self) -> &Vec<Scalar>;
    fn shape(&self) -> Vec<u32>;
}
