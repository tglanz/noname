use vectors::{Vector};
use matrices::{Matrix};
use types::{Tensors};
use activations::{ActivationFunction};
use layers::{ForwardPass};

pub struct FullyConnected {
    weights: Matrix,
    bias: Vector,
}

impl FullyConnected {
    pub fn new(input_size: usize, output_size: usize, batch_size: usize) -> FullyConnected {
        assert!(batch_size == 1, "batch size larger than one not implemented");

        FullyConnected {
            weights: Matrix::new(input_size, output_size),
            bias: Vector::with_capacity(output_size),
        }
    }
}

impl ForwardPass for FullyConnected {
    ///
    /// input
    ///     just a vector of scalars entering the layer doing what they do
    /// 
    /// type parameters:
    ///     - Activation - The activation function to apply
    
    fn forward_pass<Activation>(
        &self,
        input: &Tensors,
        ) -> Tensors
        where Activation: ActivationFunction {
        
        let mut output;
        match input {
            Tensors::Vector { vec } => {
                //let vec_input = input as Vector;
                output = self.weights.matrix_vector_product(&vec)
            },
            _ => assert!(false, "input for fully connected must be vector"),
        }

        output.apply_mut(|i, v| Activation::apply(v + self.bias[i]));
        let output = Tensors::Vector;
        output
    }
}