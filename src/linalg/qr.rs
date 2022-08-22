use std::ops::{Add, Div, Mul, Neg};

use num_traits::{One, Zero};

use crate::{
    prelude::{TentleyError, Matrix},
    scalar_traits::Scalar,
};

impl<T: Scalar + Mul<Output = T> + Add<Output = T> + Div<Output = T> + Neg<Output = T> + One + Zero, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn qr_decomposition(&self) -> Result<(Matrix<T, R, C>, Matrix<T, C, C>), TentleyError>{
        todo!();
    }
}