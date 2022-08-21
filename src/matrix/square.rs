use std::{ops::{Mul, Add, Div, Neg}, hint::unreachable_unchecked};

use crate::{scalar::{Scalar, One, Zero}, prelude::TentleyError};

use super::SquareMatrix;

impl<T: Scalar, const N: usize> SquareMatrix<T, N> {
    pub fn diagonal(&self) -> Vec<&T> {
        self.data.iter()
            .enumerate()
            .map(|(i, row)| { unsafe { row.get_unchecked(i) } })
            .collect()
    }

    pub fn diagonal_mut(&mut self) -> Vec<&mut T> {
        self.data.iter_mut()
            .enumerate()
            .map(|(i, row)| { unsafe { row.get_unchecked_mut(i) } })
            .collect()
    }

    pub fn into_diagonal(self) -> Vec<T> {
        self.data.into_iter()
            .enumerate()
            .map(|(i, row)| { match row.into_iter().nth(i) { Some(value) => value, None => unsafe { unreachable_unchecked() } } })
            .collect()
    }
}

impl<T: Scalar + Mul<Output = T> + Add<Output = T> + Div<Output = T> + Neg<Output = T> + One + Zero, const N: usize> SquareMatrix<T, N> {
    pub fn determinant(&self) -> Result<T, TentleyError> {
        let (l, u) = self.lu_decomposition()?;

        let mut determinant = T::one();

        for element in l.into_diagonal() {
            determinant = determinant * element;
        }

        for element in u.into_diagonal() {
            determinant = determinant * element;
        }

        Ok(determinant)
    }
}