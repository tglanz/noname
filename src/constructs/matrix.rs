use constructs::{Scalar, Vector};

pub struct Matrix {
    values: Vector,
    column_count: usize,
    row_count: usize,
}

impl Matrix {
    fn new(row_count: usize, column_count: usize) -> Matrix {
        Matrix {
            values: vec![0.0; row_count * column_count],
            row_count,
            column_count,
        }
    }

    fn multiply_scalar_mut(&mut self, scalar: &Scalar){
        for value in &mut self.values {
            *value *= scalar;
        }
    }

    /// Matrix * vector
    fn matrix_vector_product(&self, vector: &Vector) -> Vector {
        let mut result = vec![0.0; self.row_count];

        for rowdex in 0..self.row_count {
            let row_offset = rowdex * self.column_count;
            for coldex in 0..self.column_count {
                result[rowdex] += self.values[row_offset + coldex];
            }
        }

        result
    }
}