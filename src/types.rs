use vectors::{Vector};
use matrices::{Matrix, Matrix3d, Matrix4d};

pub type Scalar = f64;

pub enum Tensors {
    Vector {vec: Vector}, 
    Matrix {mat: Matrix},
    Matrix3d {mat3d: Matrix3d},
    Matrix4d {mat4d: Matrix4d},
}
