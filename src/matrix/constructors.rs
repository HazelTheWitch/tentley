use std::mem::{transmute_copy, MaybeUninit};

use num_traits::{One, Zero};

use crate::scalar_traits::Scalar;

use super::{Matrix, SquareMatrix};

impl<T: Scalar, const R: usize, const C: usize> Matrix<T, R, C> {
    /// Creates a new [`Matrix<T, R, C>`] from the given data.
    ///
    /// # Panics
    ///
    /// Panics if rows or cols equals 0.
    pub fn new(data: [[T; C]; R]) -> Self {
        assert!(R > 0, "rows must be greater than 0");
        assert!(C > 0, "cols must be greater than 0");

        Self { data }
    }

    /// Creates a matrix full of `value`.
    pub fn full(value: T) -> Self {
        Self::new([[value; C]; R])
    }

    /// Create a matrix by calling `f(row, col)` on each row, col pair and using the output as the element.
    pub fn from_fn<F: Fn(usize, usize) -> T>(f: F) -> Self {
        let mut data: [[MaybeUninit<T>; C]; R] = unsafe { MaybeUninit::uninit().assume_init() };

        for (row, data) in data.iter_mut().enumerate() {
            for (col, element) in data.iter_mut().enumerate() {
                *element = MaybeUninit::new(f(row, col));
            }
        }

        Matrix::new(unsafe { transmute_copy(&data) })
    }
}

impl<T: Scalar + Zero, const R: usize, const C: usize> Matrix<T, R, C> {
    /// Create a matrix full of zeros, equivalent to `Matrix::full(T::zero())`.
    pub fn zeros() -> Self {
        Self::full(T::zero())
    }
}

impl<T: Scalar + One, const R: usize, const C: usize> Matrix<T, R, C> {
    /// Create a matrix full of ones, equivalent to `Matrix::full(T::one())`.
    pub fn ones() -> Self {
        Self::full(T::one())
    }
}

impl<T: Scalar + Zero + One, const N: usize> SquareMatrix<T, N> {
    /// Create the identity matrix.
    /// 
    /// ```rust
    /// use tentley::prelude::*;
    /// 
    /// let i = Matrix::<f32, 2, 2>::identity();
    /// let m = mat![f32; 1, 2; 3, 4];
    /// 
    /// assert_eq!(m, i * m);
    /// ```
    pub fn identity() -> Self {
        let mut data: [[MaybeUninit<T>; N]; N] = unsafe { MaybeUninit::uninit().assume_init() };

        for (row, data) in data.iter_mut().enumerate() {
            for (col, element) in data.iter_mut().enumerate() {
                *element = MaybeUninit::new(if row == col { T::one() } else { T::zero() });
            }
        }

        Matrix::new(unsafe { transmute_copy(&data) })
    }
}
