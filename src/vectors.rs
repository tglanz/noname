use std::vec::Vec;
use std::ops::{Index, IndexMut};

use types::{Scalar};

pub struct Vector {
    scalars: Vec<Scalar>,
}

impl Index<usize> for Vector {
    type Output = Scalar;

    fn index(&self, index: usize) -> &Scalar {
        &self.scalars[index]
    }
}


impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Scalar {
        &mut self.scalars[index]
    }
}

impl Vector {

    pub fn from_scalars(scalars: Vec<Scalar>) -> Vector {
        Vector {
            scalars
        }
    }

    pub fn with_capacity(capacity: usize) -> Vector {
        Vector {
            scalars: Vec::with_capacity(capacity)
        }
    }

    pub fn apply_mut<F>(&mut self, application: F)
        where F: Fn(usize, Scalar) -> Scalar {

        for (idx, value) in self.scalars.iter_mut().enumerate() {
            *value = application(idx, *value);
        }
    }
}
