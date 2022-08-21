use std::mem::{MaybeUninit, transmute_copy};

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

        for row in 0..R {
            for col in 0..C {
                data[col][row] = MaybeUninit::new(unsafe { *self.get_unchecked(row, col) });
            }
        }

        Matrix::new(unsafe { transmute_copy(&data) })
    }
}