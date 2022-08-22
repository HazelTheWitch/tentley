use std::ops::{Add, Mul};

use crate::{prelude::TentleyError, scalar::Scalar};

use super::{AugmentedMatrix, Matrix};

impl<T: Scalar, const R: usize, const C0: usize, const C1: usize> AugmentedMatrix<T, R, C0, C1> {
    /// Creates a new [`AugmentedMatrix<T, R, C0, C1>`].
    pub fn new(left: Matrix<T, R, C0>, right: Matrix<T, R, C1>) -> Self {
        Self { left, right }
    }

    /// Consume the [`AugmentedMatrix<T, R, C0, C1>`] and extract the left and right matrices.
    pub fn extract(self) -> (Matrix<T, R, C0>, Matrix<T, R, C1>) {
        (self.left, self.right)
    }
}

impl<
        T: Scalar + Add<Output = T> + Mul<Output = T>,
        const R: usize,
        const C0: usize,
        const C1: usize,
    > AugmentedMatrix<T, R, C0, C1>
{

    /// Swap rows `row_0` and `row_1`.
    ///
    /// # Errors
    ///
    /// - [`TentleyError::IndexOutOfBounds`] if either `row_0` or `row_1` is out of bounds.
    pub fn swap_rows(&mut self, row_0: usize, row_1: usize) -> Result<(), TentleyError> {
        self.left.swap_rows(row_0, row_1)?;
        self.right.swap_rows(row_0, row_1)?;

        Ok(())
    }

    /// Adds one row multiplied by a constant to anther.
    /// 
    /// Equivalent to A_target = A_target + coefficient * A_source.
    ///
    /// # Errors
    ///
    /// - [`TentleyError::IndexOutOfBounds`] if either `source` or `target` is out of bounds.
    pub fn add_rows(
        &mut self,
        source: usize,
        target: usize,
        coefficient: T,
    ) -> Result<(), TentleyError> {
        self.left.add_rows(source, target, coefficient)?;
        self.right.add_rows(source, target, coefficient)?;

        Ok(())
    }

    /// Multiplies a row by a coefficient.
    ///
    /// # Errors
    ///
    /// - [`TentleyError::IndexOutOfBounds`] if either `row` is out of bounds.
    pub fn multiply_row(&mut self, row: usize, coefficient: T) -> Result<(), TentleyError> {
        self.left.multiply_row(row, coefficient)?;
        self.right.multiply_row(row, coefficient)?;

        Ok(())
    }
}
