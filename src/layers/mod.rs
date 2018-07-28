mod fully_connected;
mod convolution_2d;

use activations::{ActivationFunction};
use types::{Tensors};

pub trait ForwardPass {
    fn forward_pass<Activation>(
        &self,
        input: &Tensors,
    ) -> Tensors
    where Activation: ActivationFunction;
}