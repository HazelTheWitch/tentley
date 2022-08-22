//! Matrix, AugmentedMatrix, and SquareMatrix

mod access;
mod augmented;
mod basic;
mod constructors;
mod gaussian;
mod iter;
mod ops;
#[cfg(feature = "random")]
mod random;
mod square;

use crate::scalar_traits::Scalar;

/// Any square matrix of size NxN.
pub type SquareMatrix<T, const N: usize> = Matrix<T, N, N>;

/// Represents a matrix of size RxC.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Matrix<T: Scalar, const R: usize, const C: usize> {
    data: [[T; C]; R],
}

/// Represents an augmented matrix with left and right matrices, any row operations are performed on both.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AugmentedMatrix<T: Scalar, const R: usize, const C0: usize, const C1: usize> {
    left: Matrix<T, R, C0>,
    right: Matrix<T, R, C1>,
}
