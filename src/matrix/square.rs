use std::{
    hint::unreachable_unchecked,
    ops::{Add, Div, Mul, Neg},
};

use crate::{
    prelude::TentleyError,
    scalar::{One, Scalar, Zero},
};

use super::SquareMatrix;

impl<T: Scalar, const N: usize> SquareMatrix<T, N> {
    /// Returns the diagonal vector of references for this [`SquareMatrix<T, N>`].
    pub fn diagonal(&self) -> Vec<&T> {
        self.data
            .iter()
            .enumerate()
            .map(|(i, row)| unsafe { row.get_unchecked(i) })
            .collect()
    }

    /// Returns the diagonal vector of mutable references for this [`SquareMatrix<T, N>`].
    pub fn diagonal_mut(&mut self) -> Vec<&mut T> {
        self.data
            .iter_mut()
            .enumerate()
            .map(|(i, row)| unsafe { row.get_unchecked_mut(i) })
            .collect()
    }

    /// Consumes this [`SquareMatrix<T, N>`] and returns a vector of its diagonal.
    pub fn into_diagonal(self) -> Vec<T> {
        self.data
            .into_iter()
            .enumerate()
            .map(|(i, row)| match row.into_iter().nth(i) {
                Some(value) => value,
                None => unsafe { unreachable_unchecked() },
            })
            .collect()
    }
}

impl<
        T: Scalar + Mul<Output = T> + Add<Output = T> + Div<Output = T> + Neg<Output = T> + One + Zero,
        const N: usize,
    > SquareMatrix<T, N>
{
    /// Returns the determinant of this [`SquareMatrix<T, N>`].
    ///
    /// # Errors
    ///
    /// - [`TentleyError::DivisionByZero`] if the matrix is singular
    pub fn determinant(&self) -> Result<T, TentleyError> {
        let (l, u) = self.lu_decomposition()?;

        let mut determinant = T::one();

        for element in l
            .into_diagonal()
            .into_iter()
            .chain(u.into_diagonal().into_iter())
        {
            determinant = determinant * element;
        }

        Ok(determinant)
    }
}
