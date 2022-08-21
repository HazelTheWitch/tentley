mod access;
mod augmented;
mod basic;
mod constructors;
mod gaussian;
mod ops;
#[cfg(feature = "random")]
mod random;
mod square;

use crate::scalar::Scalar;

pub type SquareMatrix<T, const N: usize> = Matrix<T, N, N>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Matrix<T: Scalar, const R: usize, const C: usize> {
    data: [[T; C]; R],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AugmentedMatrix<T: Scalar, const R: usize, const C0: usize, const C1: usize> {
    left: Matrix<T, R, C0>,
    right: Matrix<T, R, C1>,
}
