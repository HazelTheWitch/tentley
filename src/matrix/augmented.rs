use std::ops::{Mul, Add};

use crate::{scalar::Scalar, prelude::TentleyError};

use super::{AugmentedMatrix, Matrix};

impl<T: Scalar, const R: usize, const C0: usize, const C1: usize> AugmentedMatrix<T, R, C0, C1> {
    pub fn new(left: Matrix<T, R, C0>, right: Matrix<T, R, C1>) -> Self {
        Self { left, right }
    }

    pub fn extract(self) -> (Matrix<T, R, C0>, Matrix<T, R, C1>) {
        (self.left, self.right)
    }
}

impl<T: Scalar + Add<Output = T> + Mul<Output = T>, const R: usize, const C0: usize, const C1: usize> AugmentedMatrix<T, R, C0, C1> {
    pub fn swap_rows(&mut self, row_0: usize, row_1: usize) -> Result<(), TentleyError> {
        self.left.swap_rows(row_0, row_1)?;
        self.right.swap_rows(row_0, row_1)?;

        Ok(())
    }

    pub fn add_rows(&mut self, source: usize, target: usize, coefficient: T) -> Result<(), TentleyError> {
        self.left.add_rows(source, target, coefficient)?;
        self.right.add_rows(source, target, coefficient)?;

        Ok(())
    }

    pub fn multiply_row(&mut self, row: usize, coefficient: T) -> Result<(), TentleyError> {
        self.left.multiply_row(row, coefficient)?;
        self.right.multiply_row(row, coefficient)?;

        Ok(())
    }
}