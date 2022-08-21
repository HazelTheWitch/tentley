use std::mem::{transmute_copy, MaybeUninit};

use crate::scalar::{One, Scalar, Zero};

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

    pub fn from_fn<F: Fn(usize, usize) -> T>(f: F) -> Self {
        let mut data: [[MaybeUninit<T>; C]; R] = unsafe { MaybeUninit::uninit().assume_init() };

        for i in 0..R {
            for j in 0..C {
                data[i][j] = MaybeUninit::new(f(i, j));
            }
        }

        Matrix::new(unsafe { transmute_copy(&data) })
    }
}

impl<T: Scalar + Zero, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn zeros() -> Self {
        Self::full(T::zero())
    }
}

impl<T: Scalar + One, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn ones() -> Self {
        Self::full(T::one())
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
