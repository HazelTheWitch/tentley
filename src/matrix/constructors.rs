use std::mem::{MaybeUninit, transmute_copy};

use crate::scalar::{Scalar, Zero, One};

use super::{Matrix, SquareMatrix};

impl<T: Scalar, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn new(data: [[T; C]; R]) -> Self {
        assert!(R > 0, "rows must be greater than 1");
        assert!(C > 0, "cols must be greater than 1");

        Self { data }
    }

    pub fn full(value: T) -> Self {
        Self::new([[value; C]; R])
    }
}

impl<T: Scalar + Zero + One, const N: usize> SquareMatrix<T, N> {
    pub fn identity() -> Self {
        let mut data: [[MaybeUninit<T>; N]; N] = unsafe { MaybeUninit::uninit().assume_init() };

        for i in 0..N {
            for j in 0..N {
                data[i][j] = MaybeUninit::new(if i == j { T::one() } else { T::zero() });
            }
        }

        Matrix::new(unsafe { transmute_copy(&data) })
    }
}
