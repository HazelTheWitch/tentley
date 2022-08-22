use std::{
    hint::unreachable_unchecked,
    mem::{transmute_copy, MaybeUninit},
};

use crate::scalar::Scalar;

use super::Matrix;

impl<T: Scalar, const R: usize, const C: usize> Matrix<T, R, C> {
    pub const fn shape(&self) -> (usize, usize) {
        (R, C)
    }

    pub const fn is_square(&self) -> bool {
        R == C
    }

    pub fn transpose(&self) -> Matrix<T, C, R> {
        let mut data: [[MaybeUninit<T>; R]; C] = unsafe { MaybeUninit::uninit().assume_init() };

        for (row, data) in data.iter_mut().enumerate() {
            for (col, element) in data.iter_mut().enumerate() {
                *element = MaybeUninit::new(unsafe { *self.get_unchecked(row, col) });
            }
        }

        Matrix::new(unsafe { transmute_copy(&data) })
    }

    pub fn map<F: Fn(T) -> O + Copy, O: Scalar>(&self, f: F) -> Matrix<O, R, C> {
        Matrix::new(
            match self
                .data
                .into_iter()
                .map(
                    |row| match row.into_iter().map(f).collect::<Vec<O>>().try_into() {
                        Ok(array) => array,
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

    pub fn filter<F: Fn(T) -> bool + Copy>(&self, predicate: F) -> Matrix<Option<T>, R, C> {
        self.map(|e| if predicate(e) { Some(e) } else { None })
    }
}
