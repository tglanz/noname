use types::{Scalar};

mod relu;

pub trait ActivationFunction {
    fn apply(input: Scalar) -> Scalar;
    fn derivative(input: Scalar) -> Scalar;
}
