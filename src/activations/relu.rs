use types::{Scalar};
use activations::ActivationFunction;

pub struct Relu { }

impl ActivationFunction for Relu {
    fn apply(input: Scalar) -> Scalar {
        if input <= 0.0 {
            0.0
        } else {
            input
        }
    }

    fn derivative(input: Scalar) -> Scalar {
        if input <= 0.0 {
            0.0
        } else {
            1.0
        }
    }
}