mod convolution2d;
use convolution::variables::{Variable};

pub trait Layer {
    fn forward<T, U> (
        inputs: T,
        weights: U
    ) -> T
    where T: Variable, U: Variable;
}