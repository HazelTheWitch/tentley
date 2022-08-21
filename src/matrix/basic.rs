use std::mem::{MaybeUninit, transmute_copy};

use crate::scalar::Scalar;

use super::Matrix;

impl<T: Scalar, const R: usize, const C: usize> Matrix<T, R, C> {
    pub const fn shape(&self) -> (usize, usize) {
        (R, C)
    }

    pub fn transpose(&self) -> Matrix<T, C, R> {
        let mut data: [[MaybeUninit<T>; R]; C] = unsafe { MaybeUninit::uninit().assume_init() };

        for row in 0..R {
            for col in 0..C {
                data[col][row] = MaybeUninit::new(unsafe { *self.get_unchecked(row, col) });
            }
        }

        // TODO: Work out better way of doing this without a copy. Currently a supposed compiler bug does not allow for transmute to be used.
        Matrix::new(unsafe { transmute_copy(&data) })
    }
}