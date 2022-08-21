use crate::scalar::Scalar;

use super::Matrix;

impl<T: Scalar, const R: usize, const C: usize> Matrix<T, R, C> {
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.data.iter().enumerate().flat_map(|(row, data)| {
            data.iter()
                .enumerate()
                .map(move |(col, element)| (row, col, element))
        })
    }

    pub fn into_iter(self) -> impl Iterator<Item = (usize, usize, T)> {
        self.data.into_iter().enumerate().flat_map(|(row, data)| {
            data.into_iter()
                .enumerate()
                .map(move |(col, element)| (row, col, element))
        })
    }
}
