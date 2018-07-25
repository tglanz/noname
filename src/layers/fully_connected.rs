use vectors::{Vector};
use matrices::{Matrix};
use activations::{ActivationFunction};
use layers::{ForwardPass};

pub struct FullyConnected {
}

impl ForwardPass for FullyConnected {
    ///
    /// input
    ///     just a vector of scalars entering the layer doing what they do
    /// 
    /// kernels
    ///     the weights of the fully connected layer
    ///     kernerls[i, j] is the weight between input i to output j
    /// 
    /// bias
    ///     add to the weighted sum
    /// 
    /// type parameters:
    ///     - Activation - The activation function to apply
    fn forward_pass<Activation>(
        input: &Vector,
        kernels: &Matrix,
        bias: &Vector) -> Vector
        where Activation: ActivationFunction {

        let mut output = kernels.matrix_vector_product(&input);
        output.apply_mut(|i, v| Activation::apply(v + bias[i]));
        output
    }
}