use convolution::variables::{Variable};
use types::Scalar;


pub struct Vector {
    values:  Vec<Scalar>,
    dim0: u32,
}

impl Vector {
    pub fn new(dim0_size: u32) -> Vector {
        Vector {
            values: Vec::with_capacity(dim0_size as usize),
            dim0: dim0_size,
        }
    }    
}

impl Variable for Vector {
    fn get_data(&self) -> &Vec<Scalar> {
        &self.values
    } 

    fn shape(&self) -> Vec<u32> {
        vec![self.dim0]
    }
}


pub struct Matrix{
    values: Vec<Scalar>,
    dim0: u32,
    dim1: u32,
}

impl Matrix {
    pub fn new(dim0_size: u32, dim1_size: u32) -> Matrix {
        Matrix {
            values: Vec::with_capacity((dim0_size * dim1_size) as usize),
            dim0: dim0_size,
            dim1: dim1_size,
        }
    }    
}

impl Variable for Matrix {
    fn get_data(&self) -> &Vec<Scalar> {
        &self.values
    } 

    fn shape(&self) -> Vec<u32> {
        vec![self.dim0, self.dim1]
    }
}

pub struct Matrix3d{

    values: Vec<Scalar>,
    dim0: u32,
    dim1: u32,
    dim2: u32,
}


impl Variable for Matrix3d {
    fn get_data(&self) -> &Vec<Scalar> {
        &self.values
    } 

    fn shape(&self) -> Vec<u32> {
        vec![self.dim0, self.dim1, self.dim2]
    }
}

impl Matrix3d {
    pub fn new(dim0_size: u32, dim1_size: u32, dim2_size: u32) -> Matrix3d {
        Matrix3d {
            values: Vec::with_capacity((dim0_size * dim1_size * dim2_size) as usize),
            dim0: dim0_size,
            dim1: dim1_size,
            dim2: dim2_size,
        }
    }    
}

pub struct Matrix4d{
    values: Vec<Scalar>,
    dim0: u32,
    dim1: u32,
    dim2: u32,
    dim3: u32,
}

impl Matrix4d {
    pub fn new(dim0_size: u32, dim1_size: u32, dim2_size: u32, dim3_size: u32) -> Matrix4d {
        Matrix4d {
            values: Vec::with_capacity((dim0_size * dim1_size * dim2_size * dim3_size) as usize),
            dim0: dim0_size,
            dim1: dim1_size,
            dim2: dim2_size,
            dim3: dim3_size
        }
    }    
}

impl Variable for Matrix4d {
    fn get_data(&self) -> &Vec<Scalar> {
        &self.values
    } 

    fn shape(&self) -> Vec<u32> {
        vec![self.dim0, self.dim1, self.dim2, self.dim3]
    }
}
