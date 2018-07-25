mod fully_connected;
use activations::{ActivationFunction};
use vectors::{Vector};
use matrices::{Matrix};

pub trait ForwardPass {
    fn forward_pass<Activation>(
        input: &Vector,
        kernels: &Matrix,
        bias: &Vector
    ) -> Vector
    where Activation: ActivationFunction;
}