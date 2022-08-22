use std::{
    hint::unreachable_unchecked,
    mem::{transmute_copy, MaybeUninit},
};

use crate::scalar_traits::Scalar;

use super::Matrix;

impl<T: Scalar, const R: usize, const C: usize> Matrix<T, R, C> {
    /// Returns the shape of this [`Matrix<T, R, C>`].
    pub const fn shape(&self) -> (usize, usize) {
        (R, C)
    }

    /// Returns whether or not this [`Matrix<T, R, C>`] is square.
    pub const fn is_square(&self) -> bool {
        R == C
    }

    /// Returns the transpose of this [`Matrix<T, R, C>`].
    pub fn transpose(&self) -> Matrix<T, C, R> {
        let mut data: [[MaybeUninit<T>; R]; C] = unsafe { MaybeUninit::uninit().assume_init() };

        for (row, data) in data.iter_mut().enumerate() {
            for (col, element) in data.iter_mut().enumerate() {
                *element = MaybeUninit::new(unsafe { *self.get_unchecked(col, row) });
            }
        }

        Matrix::new(unsafe { transmute_copy(&data) })
    }

    /// Returns a new matrix where each element is equal to y where f(x) -> y.
    pub fn map<F: Fn(T) -> O + Copy, O: Scalar>(&self, f: F) -> Matrix<O, R, C> {
        Matrix::new(
            match self
                .data
                .into_iter()
                .map(
                    |row| match row.into_iter().map(f).collect::<Vec<O>>().try_into() {
                        Ok(array) => array,
                        // Safety: safe because it is impossible for an array converted to a vec then back to an array to not be the same size.
                        Err(_) => unsafe { unreachable_unchecked() },
                    },
                )
                .collect::<Vec<[O; C]>>()
                .try_into()
            {
                Ok(array) => array,
                Err(_) => unsafe { unreachable_unchecked() },
            },
        )
    }

    /// Returns a [`Matrix<Option<T>, R, C>`] where each element e -> `Some(e)` if `predicate(e)` is `true` and otherwise is `None`.
    pub fn filter<F: Fn(T) -> bool + Copy>(&self, predicate: F) -> Matrix<Option<T>, R, C> {
        self.map(|e| if predicate(e) { Some(e) } else { None })
    }
}