mod ops;
mod basic;
mod constructors;
mod access;

use crate::scalar::Scalar;

pub type SquareMatrix<T, const N: usize> = Matrix<T, N, N>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Matrix<T: Scalar, const R: usize, const C: usize> {
    data: [[T; C]; R]
}