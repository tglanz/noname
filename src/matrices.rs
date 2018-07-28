use types::{Scalar};
use vectors::{Vector};


pub struct Matrix {
    values: Vector,
    column_count: usize,
    row_count: usize,
}

impl Matrix {
    pub fn new(row_count: usize, column_count: usize) -> Matrix {
        Matrix {
            values: Vector::with_capacity(row_count * column_count),
            row_count,
            column_count,
        }
    }

    fn multiply_scalar_mut(&mut self, scalar: &Scalar){
        self.values.apply_mut(|_, v| v * scalar);
    }

    /// Matrix * vector
    pub fn matrix_vector_product(&self, vector: &Vector) -> Vector {
        let mut result = Vector::with_capacity(self.row_count);

        for rowdex in 0..self.row_count {
            let row_offset = rowdex * self.column_count;
            for coldex in 0..self.column_count {
                
                result[rowdex] += self.values[row_offset + coldex] * vector[coldex];
            }
        }

        result
    }
}

pub struct Matrix3d {
    values: Vector,
    dim0: usize,
    dim1: usize,
    dim2: usize
}

impl Matrix3d {
    pub fn new(dim0: usize, dim1: usize, dim2: usize) -> Matrix3d {

        Matrix3d {
            values: Vector::with_capacity(dim0 * dim1 * dim2),
            dim0,
            dim1,
            dim2,
        }
    }

    fn multiply_scalar_mut(&mut self, scalar: &Scalar){
        self.values.apply_mut(|_, v| v * scalar);
    }
}

pub struct Matrix4d {
    values: Vector,
    dim0: usize,
    dim1: usize,
    dim2: usize,
    dim3: usize
}

impl Matrix4d {
    fn new(dim0: usize, dim1: usize, dim2: usize, dim3: usize) -> Matrix4d {

        Matrix4d {
            values: Vector::with_capacity(dim0 * dim1 * dim2 * dim3),
            dim0,
            dim1,
            dim2,
            dim3,
        }
    }

    fn multiply_scalar_mut(&mut self, scalar: &Scalar){
        self.values.apply_mut(|_, v| v * scalar);
    }
}

